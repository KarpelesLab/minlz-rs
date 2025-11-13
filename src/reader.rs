// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Stream reader for S2 decompression

use std::io::{self, Read, Seek, SeekFrom};

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
    // Seeking support
    current_uncompressed_offset: i64, // Current position in uncompressed stream
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
            current_uncompressed_offset: 0,
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
            current_uncompressed_offset: 0,
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
            current_uncompressed_offset: 0,
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
            (1024..=MAX_BLOCK_SIZE).contains(&alloc_block_size),
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
            current_uncompressed_offset: 0,
        }
    }

    /// Read and verify the stream identifier
    fn read_stream_identifier(&mut self) -> io::Result<()> {
        // If ignore_stream_id is set, skip verification
        if self.ignore_stream_id {
            return Ok(());
        }

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

        // Check against max_block_size limit
        if decompressed.len() > self.max_block_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "decompressed block size ({}) exceeds limit ({})",
                    decompressed.len(),
                    self.max_block_size
                ),
            ));
        }

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

        // Check against max_block_size limit
        if data_len > self.max_block_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "uncompressed block size ({}) exceeds limit ({})",
                    data_len, self.max_block_size
                ),
            ));
        }

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
        self.current_uncompressed_offset = 0;
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

        // Track uncompressed offset
        self.current_uncompressed_offset += to_copy as i64;

        Ok(to_copy)
    }
}

/// Implementation of Seek for Reader with seekable underlying reader
///
/// Note: This provides basic seeking support. For efficient random access,
/// use an Index to map uncompressed offsets to compressed positions.
impl<R: Read + Seek> Seek for Reader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        // Calculate target uncompressed position
        let target_pos = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(offset) => self.current_uncompressed_offset + offset,
            SeekFrom::End(_) => {
                // For SeekFrom::End, we would need to know the total uncompressed size
                // This requires either reading the entire stream or having an Index
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "SeekFrom::End not supported without an Index. Use Index::find() to seek from end.",
                ));
            }
        };

        if target_pos < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot seek to negative position",
            ));
        }

        // If seeking within current buffer, just adjust position
        let buffer_start_offset = self.current_uncompressed_offset - self.pos as i64;
        let buffer_end_offset = buffer_start_offset + self.buf.len() as i64;

        if target_pos >= buffer_start_offset && target_pos < buffer_end_offset {
            // Seeking within current buffer
            let new_pos = (target_pos - buffer_start_offset) as usize;
            self.pos = new_pos;
            self.current_uncompressed_offset = target_pos;
            return Ok(target_pos as u64);
        }

        // For seeks outside the current buffer, we need to reposition
        if target_pos == 0 {
            // Seek to beginning
            self.reader.seek(SeekFrom::Start(0))?;
            self.buf.clear();
            self.pos = 0;
            self.read_header = false;
            self.eof = false;
            self.current_uncompressed_offset = 0;
            return Ok(0);
        }

        if target_pos < self.current_uncompressed_offset {
            // Backward seek - need to start from beginning
            self.reader.seek(SeekFrom::Start(0))?;
            self.buf.clear();
            self.pos = 0;
            self.read_header = false;
            self.eof = false;
            self.current_uncompressed_offset = 0;
        }

        // Read forward to target position
        let mut to_skip = (target_pos - self.current_uncompressed_offset) as u64;
        let mut skip_buf = vec![0u8; 8192];

        while to_skip > 0 {
            let chunk_size = (to_skip as usize).min(skip_buf.len());
            let n = self.read(&mut skip_buf[..chunk_size])?;
            if n == 0 {
                // Reached EOF before target
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!(
                        "reached EOF at position {} before target {}",
                        self.current_uncompressed_offset, target_pos
                    ),
                ));
            }
            to_skip -= n as u64;
        }

        Ok(target_pos as u64)
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

    #[test]
    fn test_reader_seek_start() {
        use std::io::Cursor;

        // Compress some data
        let data = b"Hello, World! This is a test of seeking functionality.";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        // Create seekable reader
        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 5];

        // Read first 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"Hello");

        // Seek back to start
        let pos = reader.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(pos, 0);

        // Read again
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"Hello");
    }

    #[test]
    fn test_reader_seek_forward() {
        use std::io::Cursor;

        let data = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 5];

        // Seek to position 10
        let pos = reader.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(pos, 10);

        // Read 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"ABCDE");
    }

    #[test]
    fn test_reader_seek_current() {
        use std::io::Cursor;

        let data = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 5];

        // Read first 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"01234");

        // Seek forward 10 bytes from current position
        let pos = reader.seek(SeekFrom::Current(10)).unwrap();
        assert_eq!(pos, 15);

        // Read 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"FGHIJ");
    }

    #[test]
    fn test_reader_seek_backward() {
        use std::io::Cursor;

        let data = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 5];

        // Read to position 20
        reader.seek(SeekFrom::Start(20)).unwrap();
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"KLMNO");

        // Seek backward to position 10
        let pos = reader.seek(SeekFrom::Start(10)).unwrap();
        assert_eq!(pos, 10);

        // Read 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"ABCDE");
    }

    #[test]
    fn test_reader_seek_within_buffer() {
        use std::io::Cursor;

        let data = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 5];

        // Read first 5 bytes (this loads the buffer)
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"01234");

        // Seek to position 2 (within current buffer)
        let pos = reader.seek(SeekFrom::Start(2)).unwrap();
        assert_eq!(pos, 2);

        // Read 5 bytes
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"23456");
    }

    #[test]
    fn test_reader_seek_end_unsupported() {
        use std::io::Cursor;

        let data = b"Test data";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));

        // SeekFrom::End should return an error
        let result = reader.seek(SeekFrom::End(-5));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Unsupported);
    }

    #[test]
    fn test_reader_seek_negative() {
        use std::io::Cursor;

        let data = b"Test data";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));

        // Seeking to negative position should error
        let result = reader.seek(SeekFrom::Current(-10));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_reader_seek_beyond_eof() {
        use std::io::Cursor;

        let data = b"Short";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));

        // Seeking beyond EOF should error
        let result = reader.seek(SeekFrom::Start(1000));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_reader_seek_multiple_chunks() {
        use std::io::Cursor;

        // Create data that will span multiple chunks
        let data = vec![b'A'; 10000];
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::with_block_size(&mut compressed, 1024);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        let mut reader = Reader::new(Cursor::new(compressed));
        let mut buf = vec![0u8; 100];

        // Seek to position in a later chunk
        let pos = reader.seek(SeekFrom::Start(5000)).unwrap();
        assert_eq!(pos, 5000);

        // Read and verify
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf[..], &[b'A'; 100][..]);
    }
}
