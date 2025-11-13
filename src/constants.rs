// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// Tag for literal chunks
pub const TAG_LITERAL: u8 = 0x00;

/// Tag for copy with 1-byte offset (11 bits)
pub const TAG_COPY1: u8 = 0x01;

/// Tag for copy with 2-byte offset (16 bits)
pub const TAG_COPY2: u8 = 0x02;

/// Tag for copy with 4-byte offset (32 bits)
pub const TAG_COPY4: u8 = 0x03;

/// Maximum block size (4MB)
pub const MAX_BLOCK_SIZE: usize = 4 << 20;

/// Minimum block size for compression (4KB)
pub const MIN_BLOCK_SIZE: usize = 4 << 10;

/// Default block size (1MB)
pub const DEFAULT_BLOCK_SIZE: usize = 1 << 20;

/// Maximum Snappy block size (64KB)
pub const MAX_SNAPPY_BLOCK_SIZE: usize = 1 << 16;

/// Input margin for encoding
pub const INPUT_MARGIN: usize = 8;

/// Minimum non-literal block size
pub const MIN_NON_LITERAL_BLOCK_SIZE: usize = 32;

/// Chunk types for stream format
pub const CHUNK_TYPE_COMPRESSED_DATA: u8 = 0x00;
pub const CHUNK_TYPE_UNCOMPRESSED_DATA: u8 = 0x01;
pub const CHUNK_TYPE_INDEX: u8 = 0x99;
pub const CHUNK_TYPE_PADDING: u8 = 0xfe;
pub const CHUNK_TYPE_STREAM_IDENTIFIER: u8 = 0xff;

/// Magic bytes for stream identification
pub const MAGIC_BODY: &[u8] = b"S2sTwO";
pub const MAGIC_BODY_SNAPPY: &[u8] = b"sNaPpY";

/// Full magic chunk for S2 streams (0xff 0x06 0x00 0x00 "S2sTwO")
pub const MAGIC_CHUNK: &[u8] = b"\xff\x06\x00\x00S2sTwO";

/// Full magic chunk for Snappy streams (0xff 0x06 0x00 0x00 "sNaPpY")
pub const MAGIC_CHUNK_SNAPPY: &[u8] = b"\xff\x06\x00\x00sNaPpY";

/// Checksum size (CRC32)
pub const CHECKSUM_SIZE: usize = 4;

/// Chunk header size
pub const CHUNK_HEADER_SIZE: usize = 4;

/// Maximum chunk size (24-bit)
pub const MAX_CHUNK_SIZE: usize = (1 << 24) - 1; // 16777215

/// Skippable frame header size
pub const SKIPPABLE_FRAME_HEADER: usize = 4;

/// Output buffer header length (checksum + chunk header)
pub const OBUF_HEADER_LEN: usize = CHECKSUM_SIZE + CHUNK_HEADER_SIZE;
