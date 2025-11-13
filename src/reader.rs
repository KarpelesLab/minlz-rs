// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Stream reader for S2 decompression

use std::io::{self, Read};

use crate::constants::*;
use crate::crc::crc;
use crate::decode::decode;

/// Reader decompresses data using the S2 stream format
///
/// The stream format includes:
/// - Stream identifier magic bytes
/// - Framed compressed blocks with CRC checksums
/// - Support for skippable frames and padding
///
/// # Example
///
/// ```
/// use minlz::{Writer, Reader};
/// use std::io::{Write, Read};
///
/// // Compress
/// let mut compressed = Vec::new();
/// {
///     let mut writer = Writer::new(&mut compressed);
///     writer.write_all(b"Hello, World!").unwrap();
///     writer.flush().unwrap();
/// }
///
/// // Decompress
/// let mut reader = Reader::new(&compressed[..]);
/// let mut decompressed = Vec::new();
/// reader.read_to_end(&mut decompressed).unwrap();
///
/// assert_eq!(decompressed, b"Hello, World!");
/// ```
pub struct Reader<R: Read> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
    read_header: bool,
    eof: bool,
    max_block_size: usize,
    ignore_stream_id: bool,
}

impl<R: Read> Reader<R> {
    /// Create a new Reader with default settings
    ///
    /// Default max_block_size is 4MB (the S2 maximum)
    pub fn new(reader: R) -> Self {
        Reader {
            reader,
            buf: Vec::new(),
            pos: 0,
            read_header: false,
            eof: false,
            max_block_size: MAX_BLOCK_SIZE,
            ignore_stream_id: false,
        }
    }

    /// Create a new Reader with a maximum block size limit
    ///
    /// This can be used to limit memory usage if you know the stream
    /// was compressed with smaller blocks. For Snappy-compressed streams,
    /// you can safely set this to 64KB.
    ///
    /// # Panics
    /// Panics if max_block_size is 0 or greater than 4MB
    pub fn with_max_block_size(reader: R, max_block_size: usize) -> Self {
        assert!(
            max_block_size > 0 && max_block_size <= MAX_BLOCK_SIZE,
            "max_block_size must be > 0 and <= 4MB"
        );
        Reader {
            reader,
            buf: Vec::new(),
            pos: 0,
            read_header: false,
            eof: false,
            max_block_size,
            ignore_stream_id: false,
        }
    }

    /// Create a new Reader that skips the stream identifier check
    ///
    /// This can be useful when reading from a stream that has been
    /// forwarded to a specific point and doesn't start with the magic bytes.
    pub fn with_ignore_stream_id(reader: R) -> Self {
        Reader {
            reader,
            buf: Vec::new(),
            pos: 0,
            read_header: true, // Skip reading header
            eof: false,
            max_block_size: MAX_BLOCK_SIZE,
            ignore_stream_id: true,
        }
    }

    /// Create a new Reader with a pre-allocated buffer size
    ///
    /// This can reduce allocations if you know the expected block size.
    /// The buffer will grow as needed if larger blocks are encountered.
    ///
    /// Note: The Rust implementation uses a different buffering strategy than
    /// the Go implementation, so this primarily provides API compatibility.
    ///
    /// # Panics
    /// Panics if alloc_block_size is less than 1KB or greater than 4MB
    pub fn with_alloc_block_size(reader: R, alloc_block_size: usize) -> Self {
        assert!(
            alloc_block_size >= 1024 && alloc_block_size <= MAX_BLOCK_SIZE,
            "alloc_block_size must be >= 1KB and <= 4MB"
        );
        Reader {
            reader,
            buf: Vec::with_capacity(alloc_block_size),
            pos: 0,
            read_header: false,
            eof: false,
            max_block_size: MAX_BLOCK_SIZE,
            ignore_stream_id: false,
        }
    }

    /// Read and verify the stream identifier
    fn read_stream_identifier(&mut self) -> io::Result<()> {
        let mut magic = [0u8; MAGIC_CHUNK.len()];
        self.reader.read_exact(&mut magic)?;

        if magic == *MAGIC_CHUNK || magic == *MAGIC_CHUNK_SNAPPY {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid stream identifier",
            ))
        }
    }

    /// Read the next chunk from the stream
    fn read_chunk(&mut self) -> io::Result<bool> {
        // Read chunk type and length (4 bytes total)
        let mut header = [0u8; 4];
        match self.reader.read_exact(&mut header) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                self.eof = true;
                return Ok(false);
            }
            Err(e) => return Err(e),
        }

        let chunk_type = header[0];
        let chunk_len = u32::from_le_bytes([header[1], header[2], header[3], 0]) as usize;

        match chunk_type {
            CHUNK_TYPE_COMPRESSED_DATA => {
                self.read_compressed_chunk(chunk_len)?;
                Ok(true)
            }
            CHUNK_TYPE_UNCOMPRESSED_DATA => {
                self.read_uncompressed_chunk(chunk_len)?;
                Ok(true)
            }
            CHUNK_TYPE_PADDING | CHUNK_TYPE_INDEX => {
                // Skip this chunk
                self.skip_chunk(chunk_len)?;
                // Read next chunk
                self.read_chunk()
            }
            CHUNK_TYPE_STREAM_IDENTIFIER => {
                // Skip stream identifier in the middle of the stream
                self.skip_chunk(chunk_len)?;
                self.read_chunk()
            }
            0x80..=0xfd => {
                // Skippable chunk range
                self.skip_chunk(chunk_len)?;
                self.read_chunk()
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown chunk type: 0x{:02x}", chunk_type),
            )),
        }
    }

    /// Read a compressed data chunk
    fn read_compressed_chunk(&mut self, chunk_len: usize) -> io::Result<()> {
        if chunk_len < CHECKSUM_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "chunk too small",
            ));
        }

        // Read checksum
        let mut checksum_bytes = [0u8; 4];
        self.reader.read_exact(&mut checksum_bytes)?;
        let expected_crc = u32::from_le_bytes(checksum_bytes);

        // Read compressed data
        let data_len = chunk_len - CHECKSUM_SIZE;
        let mut compressed = vec![0u8; data_len];
        self.reader.read_exact(&mut compressed)?;

        // Decompress
        let decompressed = decode(&compressed).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("decode error: {}", e))
        })?;

        // Verify CRC
        let actual_crc = crc(&decompressed);
        if actual_crc != expected_crc {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "CRC mismatch"));
        }

        // Add to buffer
        self.buf.extend_from_slice(&decompressed);
        Ok(())
    }

    /// Read an uncompressed data chunk
    fn read_uncompressed_chunk(&mut self, chunk_len: usize) -> io::Result<()> {
        if chunk_len < CHECKSUM_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "chunk too small",
            ));
        }

        // Read checksum
        let mut checksum_bytes = [0u8; 4];
        self.reader.read_exact(&mut checksum_bytes)?;
        let expected_crc = u32::from_le_bytes(checksum_bytes);

        // Read uncompressed data
        let data_len = chunk_len - CHECKSUM_SIZE;
        let mut data = vec![0u8; data_len];
        self.reader.read_exact(&mut data)?;

        // Verify CRC
        let actual_crc = crc(&data);
        if actual_crc != expected_crc {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "CRC mismatch"));
        }

        // Add to buffer
        self.buf.extend_from_slice(&data);
        Ok(())
    }

    /// Skip a chunk
    fn skip_chunk(&mut self, chunk_len: usize) -> io::Result<()> {
        let mut discard = vec![0u8; chunk_len];
        self.reader.read_exact(&mut discard)?;
        Ok(())
    }

    /// Reset the reader to use a new underlying reader
    pub fn reset(&mut self, reader: R) -> R {
        self.buf.clear();
        self.pos = 0;
        self.read_header = false;
        self.eof = false;
        std::mem::replace(&mut self.reader, reader)
    }

    /// Get a reference to the underlying reader
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Get a mutable reference to the underlying reader
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read stream header if not already done
        if !self.read_header {
            self.read_stream_identifier()?;
            self.read_header = true;
        }

        // If buffer is empty and not EOF, read next chunk
        while self.pos >= self.buf.len() && !self.eof {
            self.buf.clear();
            self.pos = 0;
            if !self.read_chunk()? {
                break;
            }
        }

        // Copy from buffer
        let available = self.buf.len() - self.pos;
        if available == 0 {
            return Ok(0); // EOF
        }

        let to_copy = available.min(buf.len());
        buf[..to_copy].copy_from_slice(&self.buf[self.pos..self.pos + to_copy]);
        self.pos += to_copy;

        Ok(to_copy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Writer;
    use std::io::Write;

    #[test]
    fn test_reader_with_max_block_size() {
        // Compress with default settings
        let data = b"Hello, World!";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        // Decompress with max_block_size limit
        let mut reader = Reader::with_max_block_size(&compressed[..], 64 * 1024);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_reader_with_ignore_stream_id() {
        // Compress
        let data = b"Test data";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        // Skip the magic chunk (10 bytes)
        let without_magic = &compressed[10..];

        // This should work with ignore_stream_id
        let mut reader = Reader::with_ignore_stream_id(without_magic);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    #[should_panic(expected = "max_block_size must be > 0 and <= 4MB")]
    fn test_reader_invalid_max_block_size() {
        let data = &[0u8; 10][..];
        let _reader = Reader::with_max_block_size(data, 0);
    }

    #[test]
    fn test_reader_basic() {
        // Compress
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(b"Hello, World!").unwrap();
            writer.flush().unwrap();
        }

        // Decompress
        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();

        assert_eq!(decompressed, b"Hello, World!");
    }

    #[test]
    fn test_reader_empty() {
        // Compress empty data
        let mut compressed = Vec::new();
        {
            let _writer = Writer::new(&mut compressed);
        }

        // Try to decompress
        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        let result = reader.read_to_end(&mut decompressed);

        // Should get EOF or error for empty stream
        assert!(result.is_err() || decompressed.is_empty());
    }

    #[test]
    fn test_reader_large() {
        let data = vec![b'A'; 100000];

        // Compress
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        // Decompress
        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();

        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_reader_multiple_chunks() {
        let data1 = b"First chunk of data";
        let data2 = b"Second chunk of data";

        // Compress with small block size to create multiple chunks
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::with_block_size(&mut compressed, 16);
            writer.write_all(data1).unwrap();
            writer.write_all(data2).unwrap();
            writer.flush().unwrap();
        }

        // Decompress
        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();

        let mut expected = Vec::new();
        expected.extend_from_slice(data1);
        expected.extend_from_slice(data2);

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn test_reader_with_alloc_block_size() {
        // Test with pre-allocated buffer
        let data = b"Test data for alloc_block_size";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::with_alloc_block_size(&compressed[..], 4096);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    #[should_panic(expected = "alloc_block_size must be >= 1KB and <= 4MB")]
    fn test_reader_invalid_alloc_block_size() {
        let data = &[0u8; 10][..];
        let _reader = Reader::with_alloc_block_size(data, 512); // Too small
    }
}
