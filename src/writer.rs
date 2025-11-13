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
    padding: usize,      // If > 1, pad output to be a multiple of this value
    total_written: u64,  // Total bytes written to underlying writer (for padding calculation)
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
            padding: 0,
            total_written: 0,
        }
    }

    /// Create a new Writer with padding enabled
    ///
    /// The output will be padded to be a multiple of `padding` bytes.
    /// The padding uses skippable frames filled with random data.
    /// Padding must be > 1 and <= 4MB.
    ///
    /// # Example
    ///
    /// ```
    /// use minlz::Writer;
    /// use std::io::Write;
    ///
    /// let mut compressed = Vec::new();
    /// {
    ///     let mut writer = Writer::with_padding(&mut compressed, 1024);
    ///     writer.write_all(b"Hello, World!").unwrap();
    /// } // Padding is applied when Writer is dropped
    ///
    /// // Output size will be a multiple of 1024
    /// assert_eq!(compressed.len() % 1024, 0);
    /// ```
    pub fn with_padding(writer: W, padding: usize) -> Self {
        assert!(padding > 1 && padding <= MAX_BLOCK_SIZE,
            "padding must be > 1 and <= 4MB");

        Writer {
            writer,
            buf: Vec::new(),
            block_size: DEFAULT_BLOCK_SIZE,
            wrote_header: false,
            padding,
            total_written: 0,
        }
    }

    /// Write the stream identifier if not already written
    fn write_header(&mut self) -> io::Result<()> {
        if !self.wrote_header {
            self.writer.write_all(MAGIC_CHUNK)?;
            self.total_written += MAGIC_CHUNK.len() as u64;
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

        // Track total written bytes (for padding calculation)
        self.total_written += 1 + 3 + (chunk_len as u64); // type + length + data

        // Clear the buffer
        self.buf.clear();

        Ok(())
    }

    /// Reset the writer to use a new underlying writer
    pub fn reset(&mut self, writer: W) -> W {
        self.buf.clear();
        self.wrote_header = false;
        self.total_written = 0;
        std::mem::replace(&mut self.writer, writer)
    }

    /// Calculate how many bytes of padding are needed to reach the next multiple
    fn calc_skippable_frame(written: u64, want_multiple: u64) -> usize {
        const SKIPPABLE_FRAME_HEADER: u64 = 4; // 1 byte type + 3 bytes length

        if want_multiple <= 1 {
            return 0;
        }

        let leftover = written % want_multiple;
        if leftover == 0 {
            return 0;
        }

        let mut to_add = want_multiple - leftover;

        // Make sure we have at least enough space for the frame header
        while to_add < SKIPPABLE_FRAME_HEADER {
            to_add += want_multiple;
        }

        to_add as usize
    }

    /// Write a skippable frame filled with random data
    fn write_skippable_frame(&mut self, total: usize) -> io::Result<()> {
        if total == 0 {
            return Ok(());
        }

        const SKIPPABLE_FRAME_HEADER: usize = 4;

        if total < SKIPPABLE_FRAME_HEADER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("skippable frame size ({}) < header size (4)", total),
            ));
        }

        if total >= MAX_BLOCK_SIZE + SKIPPABLE_FRAME_HEADER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("skippable frame size ({}) >= max ({})", total, MAX_BLOCK_SIZE),
            ));
        }

        // Write chunk type for padding (0xfe)
        self.writer.write_all(&[CHUNK_TYPE_PADDING])?;

        // Write chunk length (3 bytes, little-endian)
        let data_len = (total - SKIPPABLE_FRAME_HEADER) as u32;
        self.writer.write_all(&[
            (data_len & 0xff) as u8,
            ((data_len >> 8) & 0xff) as u8,
            ((data_len >> 16) & 0xff) as u8,
        ])?;

        // Write random padding data
        // Use a simple pattern for now (Go uses crypto/rand but that requires dependencies)
        // Pattern: repeating sequence of incrementing bytes
        let mut pattern = vec![0u8; data_len as usize];
        for (i, byte) in pattern.iter_mut().enumerate() {
            *byte = (i & 0xff) as u8;
        }
        self.writer.write_all(&pattern)?;

        self.total_written += total as u64;

        Ok(())
    }

    /// Apply padding if needed (called on close/drop)
    fn apply_padding(&mut self) -> io::Result<()> {
        if self.padding > 1 {
            let padding_needed = Self::calc_skippable_frame(
                self.total_written,
                self.padding as u64,
            );

            if padding_needed > 0 {
                self.write_skippable_frame(padding_needed)?;
            }
        }
        Ok(())
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
        // Flush any remaining data
        let _ = self.flush();
        // Apply padding if configured
        let _ = self.apply_padding();
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

    #[test]
    fn test_writer_with_padding() {
        let data = b"Hello, World! This is a test of padding.";
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::with_padding(&mut compressed, 1024);
            writer.write_all(data).unwrap();
        } // Drop applies padding

        // Output should be a multiple of 1024
        assert_eq!(
            compressed.len() % 1024,
            0,
            "Expected length to be multiple of 1024, got {}",
            compressed.len()
        );

        // Should have some content (not just padding)
        assert!(compressed.len() >= 1024);

        // Verify the data can still be decompressed
        use crate::Reader;
        use std::io::Read;

        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_writer_padding_multiple_blocks() {
        let data = vec![b'X'; 10000];
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::with_padding(&mut compressed, 512);
            writer.write_all(&data).unwrap();
        }

        // Output should be a multiple of 512
        assert_eq!(compressed.len() % 512, 0);

        // Verify decompression
        use crate::Reader;
        use std::io::Read;

        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
