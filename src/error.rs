// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

/// Result type for S2 operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for S2 compression/decompression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input data is corrupt
    Corrupt,

    /// The decoded block is too large
    TooLarge,

    /// The input format is unsupported
    Unsupported,

    /// CRC mismatch (streams only)
    CrcMismatch,

    /// Buffer too small
    BufferTooSmall,

    /// Invalid input
    InvalidInput(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Corrupt => write!(f, "s2: corrupt input"),
            Error::TooLarge => write!(f, "s2: decoded block is too large"),
            Error::Unsupported => write!(f, "s2: unsupported input"),
            Error::CrcMismatch => write!(f, "s2: corrupt input, crc mismatch"),
            Error::BufferTooSmall => write!(f, "s2: buffer too small"),
            Error::InvalidInput(msg) => write!(f, "s2: invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
