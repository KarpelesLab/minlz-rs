// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Stream writer for S2 compression

use std::io::{self, Write};

use crate::constants::*;
use crate::crc::crc;
use crate::encode::encode;

/// Writer compresses data using the S2 stream format
///
/// The stream format includes:
/// - Stream identifier magic bytes
/// - Framed compressed blocks with CRC checksums
/// - Support for flushing and proper stream termination
///
/// # Example
///
/// ```
/// use minlz::Writer;
/// use std::io::Write;
///
/// let mut compressed = Vec::new();
/// {
///     let mut writer = Writer::new(&mut compressed);
///     writer.write_all(b"Hello, World!").unwrap();
///     writer.flush().unwrap();
/// } // Writer is dropped and finalized here
///
/// assert!(compressed.len() > 0);
/// ```
pub struct Writer<W: Write> {
    writer: W,
    buf: Vec<u8>,
    block_size: usize,
    wrote_header: bool,
}

impl<W: Write> Writer<W> {
    /// Create a new Writer with default block size (1MB)
    pub fn new(writer: W) -> Self {
        Self::with_block_size(writer, DEFAULT_BLOCK_SIZE)
    }

    /// Create a new Writer with a specific block size
    ///
    /// Block size must be between 4KB and 4MB
    pub fn with_block_size(writer: W, block_size: usize) -> Self {
        let block_size = block_size.clamp(MIN_BLOCK_SIZE, MAX_BLOCK_SIZE);

        Writer {
            writer,
            buf: Vec::new(),
            block_size,
            wrote_header: false,
        }
    }

    /// Write the stream identifier if not already written
    fn write_header(&mut self) -> io::Result<()> {
        if !self.wrote_header {
            self.writer.write_all(MAGIC_CHUNK)?;
            self.wrote_header = true;
        }
        Ok(())
    }

    /// Flush any buffered data as a compressed block
    fn flush_block(&mut self) -> io::Result<()> {
        if self.buf.is_empty() {
            return Ok(());
        }

        self.write_header()?;

        // Compress the block
        let compressed = encode(&self.buf);

        // Calculate CRC of uncompressed data
        let checksum = crc(&self.buf);

        // Write chunk: type (1 byte) + length (3 bytes little-endian) + checksum (4 bytes) + data
        let chunk_len = compressed.len() + CHECKSUM_SIZE;
        if chunk_len > MAX_CHUNK_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "compressed block too large",
            ));
        }

        // Chunk type: compressed data
        self.writer.write_all(&[CHUNK_TYPE_COMPRESSED_DATA])?;

        // Chunk length (24-bit little-endian)
        let len_bytes = [
            (chunk_len & 0xff) as u8,
            ((chunk_len >> 8) & 0xff) as u8,
            ((chunk_len >> 16) & 0xff) as u8,
        ];
        self.writer.write_all(&len_bytes)?;

        // CRC32 checksum (little-endian)
        self.writer.write_all(&checksum.to_le_bytes())?;

        // Compressed data
        self.writer.write_all(&compressed)?;

        // Clear the buffer
        self.buf.clear();

        Ok(())
    }

    /// Reset the writer to use a new underlying writer
    pub fn reset(&mut self, writer: W) -> W {
        self.buf.clear();
        self.wrote_header = false;
        std::mem::replace(&mut self.writer, writer)
    }

    /// Get a reference to the underlying writer
    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    /// Get a mutable reference to the underlying writer
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;

        while written < buf.len() {
            let remaining = buf.len() - written;
            let space_in_buf = self.block_size - self.buf.len();

            if space_in_buf == 0 {
                // Buffer is full, flush it
                self.flush_block()?;
                continue;
            }

            let to_write = remaining.min(space_in_buf);
            self.buf
                .extend_from_slice(&buf[written..written + to_write]);
            written += to_write;
        }

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_block()?;
        self.writer.flush()
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        // Try to flush on drop, but ignore errors since we can't handle them
        let _ = self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_writer_basic() {
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(b"Hello, World!").unwrap();
            writer.flush().unwrap();
        }

        // Should have magic header + at least one chunk
        assert!(compressed.len() > MAGIC_CHUNK.len());
        assert_eq!(&compressed[..MAGIC_CHUNK.len()], MAGIC_CHUNK);
    }

    #[test]
    fn test_writer_empty() {
        let mut compressed = Vec::new();
        {
            let _writer = Writer::new(&mut compressed);
            // Write nothing
        }

        // Should not write anything for empty stream
        assert_eq!(compressed.len(), 0);
    }

    #[test]
    fn test_writer_multiple_writes() {
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(b"Hello, ").unwrap();
            writer.write_all(b"World!").unwrap();
            writer.flush().unwrap();
        }

        assert!(compressed.len() > MAGIC_CHUNK.len());
        assert_eq!(&compressed[..MAGIC_CHUNK.len()], MAGIC_CHUNK);
    }

    #[test]
    fn test_writer_large_data() {
        let data = vec![b'A'; 100000];
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        // Should compress well
        assert!(compressed.len() < data.len() / 2);
    }
}
