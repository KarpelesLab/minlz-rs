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
pub const CHUNK_TYPE_PADDING: u8 = 0xfe;
pub const CHUNK_TYPE_STREAM_IDENTIFIER: u8 = 0xff;

/// Magic chunk headers
pub const MAGIC_BODY: &[u8] = b"S2sTwO";
pub const MAGIC_BODY_SNAPPY: &[u8] = b"sNaPpY";

/// Checksum size
pub const CHECKSUM_SIZE: usize = 4;

/// Chunk header size
pub const CHUNK_HEADER_SIZE: usize = 4;
