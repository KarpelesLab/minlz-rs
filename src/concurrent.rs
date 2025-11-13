// Copyright 2024 Karpeles Lab Inc.
// Concurrent compression support using Rayon

#[cfg(feature = "concurrent")]
use std::io::{self, Write};

#[cfg(feature = "concurrent")]
use rayon::prelude::*;

#[cfg(feature = "concurrent")]
use crate::constants::*;
#[cfg(feature = "concurrent")]
use crate::crc::crc;
#[cfg(feature = "concurrent")]
use crate::encode::encode;

/// Concurrent writer that compresses blocks in parallel
///
/// Uses Rayon for parallel compression of multiple blocks.
/// Best for large data where multiple blocks can be compressed simultaneously.
///
/// # Example
///
/// ```ignore
/// use minlz::ConcurrentWriter;
/// use std::io::Write;
///
/// let mut compressed = Vec::new();
/// {
///     let mut writer = ConcurrentWriter::new(&mut compressed, 4); // 4 workers
///     writer.write_all(&vec![0u8; 1024 * 1024]).unwrap();
///     writer.flush().unwrap();
/// }
/// ```
#[cfg(feature = "concurrent")]
pub struct ConcurrentWriter<W: Write> {
    writer: W,
    buffers: Vec<Vec<u8>>,
    block_size: usize,
    concurrency: usize,
    wrote_header: bool,
}

#[cfg(feature = "concurrent")]
impl<W: Write> ConcurrentWriter<W> {
    /// Create a new concurrent writer with specified number of workers
    ///
    /// `concurrency` determines how many blocks can be compressed in parallel
    pub fn new(writer: W, concurrency: usize) -> Self {
        Self::with_block_size(writer, DEFAULT_BLOCK_SIZE, concurrency)
    }

    /// Create a new concurrent writer with specific block size and worker count
    pub fn with_block_size(writer: W, block_size: usize, concurrency: usize) -> Self {
        let block_size = block_size.clamp(MIN_BLOCK_SIZE, MAX_BLOCK_SIZE);
        let concurrency = concurrency.max(1);

        ConcurrentWriter {
            writer,
            buffers: Vec::new(),
            block_size,
            concurrency,
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

    /// Compress and write blocks in parallel
    fn flush_blocks(&mut self) -> io::Result<()> {
        if self.buffers.is_empty() {
            return Ok(());
        }

        self.write_header()?;

        // Compress all blocks in parallel
        let compressed_blocks: Vec<(Vec<u8>, u32)> = self
            .buffers
            .par_iter()
            .map(|buf| {
                let compressed = encode(buf);
                let checksum = crc(buf);
                (compressed, checksum)
            })
            .collect();

        // Write compressed blocks in order
        for (compressed, checksum) in compressed_blocks {
            let chunk_len = compressed.len() + CHECKSUM_SIZE;
            if chunk_len > MAX_CHUNK_SIZE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "compressed block too large",
                ));
            }

            // Write chunk header
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
        }

        self.buffers.clear();
        Ok(())
    }
}

#[cfg(feature = "concurrent")]
impl<W: Write> Write for ConcurrentWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut remaining = buf;

        while !remaining.is_empty() {
            // Get or create current buffer
            if self.buffers.is_empty() {
                self.buffers.push(Vec::new());
            }

            let current = self.buffers.last_mut().unwrap();
            let available = self.block_size.saturating_sub(current.len());

            if available == 0 {
                // Current buffer is full, start a new one
                self.buffers.push(Vec::new());
                continue;
            }

            // Write as much as possible to current buffer
            let to_write = available.min(remaining.len());
            current.extend_from_slice(&remaining[..to_write]);
            remaining = &remaining[to_write..];

            // If we have enough buffers for parallel compression, flush them
            if self.buffers.len() >= self.concurrency {
                self.flush_blocks()?;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_blocks()?;
        self.writer.flush()
    }
}

#[cfg(feature = "concurrent")]
impl<W: Write> Drop for ConcurrentWriter<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

#[cfg(test)]
#[cfg(feature = "concurrent")]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_writer_basic() {
        let mut compressed = Vec::new();
        {
            let mut writer = ConcurrentWriter::new(&mut compressed, 2);
            writer.write_all(b"Hello, World!").unwrap();
            writer.write_all(b" This is a test.").unwrap();
            writer.flush().unwrap();
        }

        // Should be able to decompress
        assert!(!compressed.is_empty());

        // Decode using Reader to handle stream format
        use crate::Reader;
        use std::io::Read;

        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();

        let expected = b"Hello, World! This is a test.";
        assert_eq!(decompressed, expected);
    }

    #[test]
    fn test_concurrent_writer_large() {
        let data = vec![b'X'; 1024 * 1024]; // 1MB of data
        let mut compressed = Vec::new();

        {
            let mut writer = ConcurrentWriter::with_block_size(&mut compressed, 256 * 1024, 4);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        // Decode and verify
        use crate::Reader;
        use std::io::Read;

        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).unwrap();

        assert_eq!(decompressed.len(), data.len());
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_concurrent_vs_serial() {
        let data = vec![b'A'; 512 * 1024];

        // Compress with concurrent writer
        let mut concurrent_compressed = Vec::new();
        {
            let mut writer =
                ConcurrentWriter::with_block_size(&mut concurrent_compressed, 128 * 1024, 4);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        // Compress with regular writer
        let mut serial_compressed = Vec::new();
        {
            let mut writer = crate::Writer::with_block_size(&mut serial_compressed, 128 * 1024);
            writer.write_all(&data).unwrap();
            writer.flush().unwrap();
        }

        // Both should decompress to the same data
        use crate::Reader;
        use std::io::Read;

        let mut reader1 = Reader::new(&concurrent_compressed[..]);
        let mut decompressed1 = Vec::new();
        reader1.read_to_end(&mut decompressed1).unwrap();

        let mut reader2 = Reader::new(&serial_compressed[..]);
        let mut decompressed2 = Vec::new();
        reader2.read_to_end(&mut decompressed2).unwrap();

        assert_eq!(decompressed1, decompressed2);
        assert_eq!(decompressed1, data);
    }
}
