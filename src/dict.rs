// Copyright 2024 Karpeles Lab Inc.
// Dictionary support for S2 compression

use crate::varint::{decode_varint, encode_varint, varint_size};

/// Minimum dictionary size
pub const MIN_DICT_SIZE: usize = 16;

/// Maximum dictionary size
pub const MAX_DICT_SIZE: usize = 65536;

/// Maximum offset where a dictionary entry can start
pub const MAX_DICT_SRC_OFFSET: usize = 65535;

/// Dictionary for S2 compression
///
/// A dictionary allows better compression of similar data by pre-seeding
/// the compression hash tables with common patterns.
pub struct Dict {
    /// Dictionary data
    dict: Vec<u8>,
    /// Repeat offset into dictionary
    repeat: usize,

    // Hash tables for different compression levels (lazy initialized)
    // These will be used when full dictionary encoding is implemented
    #[allow(dead_code)]
    fast_table: Option<Box<[u16; 1 << 14]>>,
    #[allow(dead_code)]
    better_table_short: Option<Box<[u16; 1 << 14]>>,
    #[allow(dead_code)]
    better_table_long: Option<Box<[u16; 1 << 17]>>,
    #[allow(dead_code)]
    best_table_short: Option<Box<[u32; 1 << 16]>>,
    #[allow(dead_code)]
    best_table_long: Option<Box<[u32; 1 << 19]>>,
}

impl Dict {
    /// Create a dictionary from serialized bytes
    ///
    /// The format is: uvarint(repeat_offset) followed by dictionary bytes.
    /// Returns None if the dictionary is invalid.
    pub fn new(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        // Decode repeat offset
        let (repeat, n) = decode_varint(data).ok()?;
        let dict_data = &data[n..];

        if dict_data.len() < MIN_DICT_SIZE || dict_data.len() > MAX_DICT_SIZE {
            return None;
        }

        if repeat as usize > dict_data.len() {
            return None;
        }

        // Ensure capacity for extra bytes (for safe reading)
        let mut dict = Vec::with_capacity(dict_data.len() + 16);
        dict.extend_from_slice(dict_data);

        Some(Dict {
            dict,
            repeat: repeat as usize,
            fast_table: None,
            better_table_short: None,
            better_table_long: None,
            best_table_short: None,
            best_table_long: None,
        })
    }

    /// Serialize dictionary to bytes
    ///
    /// Output format: uvarint(repeat_offset) followed by dictionary bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = varint_size(self.repeat as u64);
        let mut result = vec![0u8; size + self.dict.len()];
        let n = encode_varint(&mut result, self.repeat as u64);
        result[n..].copy_from_slice(&self.dict);
        result
    }

    /// Get dictionary data
    pub fn data(&self) -> &[u8] {
        &self.dict
    }

    /// Get repeat offset
    pub fn repeat(&self) -> usize {
        self.repeat
    }

    /// Get fast hash table, initializing if needed
    #[allow(dead_code)]
    pub(crate) fn get_fast_table(&mut self) -> &[u16; 1 << 14] {
        if self.fast_table.is_none() {
            self.init_fast();
        }
        self.fast_table.as_ref().unwrap()
    }

    /// Get better hash tables, initializing if needed
    #[allow(dead_code)]
    pub(crate) fn get_better_tables(&mut self) -> (&[u16; 1 << 14], &[u16; 1 << 17]) {
        if self.better_table_short.is_none() || self.better_table_long.is_none() {
            self.init_better();
        }
        (
            self.better_table_short.as_ref().unwrap(),
            self.better_table_long.as_ref().unwrap(),
        )
    }

    /// Get best hash tables, initializing if needed
    #[allow(dead_code)]
    pub(crate) fn get_best_tables(&mut self) -> (&[u32; 1 << 16], &[u32; 1 << 19]) {
        if self.best_table_short.is_none() || self.best_table_long.is_none() {
            self.init_best();
        }
        (
            self.best_table_short.as_ref().unwrap(),
            self.best_table_long.as_ref().unwrap(),
        )
    }

    /// Initialize fast hash table
    fn init_fast(&mut self) {
        const TABLE_BITS: u32 = 14;
        let mut table = Box::new([0u16; 1 << TABLE_BITS]);

        // Hash every 3rd byte to populate table
        let mut i = 0;
        while i < self.dict.len().saturating_sub(10) {
            let x = load64(&self.dict, i);
            let h0 = hash6(x, TABLE_BITS);
            let h1 = hash6(x >> 8, TABLE_BITS);
            let h2 = hash6(x >> 16, TABLE_BITS);
            table[h0 as usize] = i as u16;
            table[h1 as usize] = (i + 1) as u16;
            table[h2 as usize] = (i + 2) as u16;
            i += 3;
        }

        self.fast_table = Some(table);
    }

    /// Initialize better hash tables
    fn init_better(&mut self) {
        const L_TABLE_BITS: u32 = 17;
        const S_TABLE_BITS: u32 = 14;

        let mut l_table = Box::new([0u16; 1 << L_TABLE_BITS]);
        let mut s_table = Box::new([0u16; 1 << S_TABLE_BITS]);

        // Hash every byte
        for i in 0..self.dict.len().saturating_sub(8) {
            let cv = load64(&self.dict, i);
            l_table[hash7(cv, L_TABLE_BITS) as usize] = i as u16;
            s_table[hash4(cv, S_TABLE_BITS) as usize] = i as u16;
        }

        self.better_table_long = Some(l_table);
        self.better_table_short = Some(s_table);
    }

    /// Initialize best hash tables
    fn init_best(&mut self) {
        const L_TABLE_BITS: u32 = 19;
        const S_TABLE_BITS: u32 = 16;

        let mut l_table = Box::new([0u32; 1 << L_TABLE_BITS]);
        let mut s_table = Box::new([0u32; 1 << S_TABLE_BITS]);

        // Hash every byte with chaining
        for i in 0..self.dict.len().saturating_sub(8) {
            let cv = load64(&self.dict, i);
            let hash_l = hash8(cv, L_TABLE_BITS) as usize;
            let hash_s = hash4(cv, S_TABLE_BITS) as usize;

            let candidate_l = l_table[hash_l];
            let candidate_s = s_table[hash_s];

            l_table[hash_l] = (i as u32) | (candidate_l << 16);
            s_table[hash_s] = (i as u32) | (candidate_s << 16);
        }

        self.best_table_long = Some(l_table);
        self.best_table_short = Some(s_table);
    }
}

/// Create a dictionary from data
///
/// If `data` is longer than MAX_DICT_SIZE, only the last MAX_DICT_SIZE bytes are used.
/// If `search_start` is provided, the repeat offset will be set to the last occurrence
/// of that pattern in the dictionary (or a shorter prefix if exact match not found).
/// If no match >= 4 bytes is found, repeat is set to 0.
pub fn make_dict(data: &[u8], search_start: Option<&[u8]>) -> Option<Dict> {
    if data.is_empty() {
        return None;
    }

    // Trim to max size
    let dict_data = if data.len() > MAX_DICT_SIZE {
        &data[data.len() - MAX_DICT_SIZE..]
    } else {
        data
    };

    if dict_data.len() < MIN_DICT_SIZE {
        return None;
    }

    // Find repeat offset
    let mut repeat = 0;
    if let Some(search) = search_start {
        // Find longest match, trying progressively shorter lengths
        for len in (4..=search.len()).rev() {
            if let Some(pos) = find_last_occurrence(dict_data, &search[..len]) {
                // Make sure we have at least 8 bytes remaining
                if pos <= dict_data.len().saturating_sub(8) {
                    repeat = pos;
                    break;
                }
            }
        }
    }

    // Ensure capacity for extra bytes
    let mut dict = Vec::with_capacity(dict_data.len() + 16);
    dict.extend_from_slice(dict_data);

    Some(Dict {
        dict,
        repeat,
        fast_table: None,
        better_table_short: None,
        better_table_long: None,
        best_table_short: None,
        best_table_long: None,
    })
}

/// Create a dictionary with manual repeat offset
///
/// The `first_idx` must be less than `data.len() - 8`.
pub fn make_dict_manual(data: &[u8], first_idx: u16) -> Option<Dict> {
    if data.len() < MIN_DICT_SIZE
        || data.len() > MAX_DICT_SIZE
        || first_idx as usize >= data.len().saturating_sub(8)
    {
        return None;
    }

    // Ensure capacity for extra bytes
    let mut dict = Vec::with_capacity(data.len() + 16);
    dict.extend_from_slice(data);

    Some(Dict {
        dict,
        repeat: first_idx as usize,
        fast_table: None,
        better_table_short: None,
        better_table_long: None,
        best_table_short: None,
        best_table_long: None,
    })
}

/// Find last occurrence of needle in haystack
fn find_last_occurrence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }

    // Manual implementation is clearer than iterator chaining for this case
    #[allow(clippy::manual_find)]
    for i in (0..=haystack.len() - needle.len()).rev() {
        if &haystack[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

// Helper functions for loading and hashing (matching encode.rs patterns)
// These will be used when full dictionary encoding is implemented

#[allow(dead_code)]
#[inline(always)]
fn load64(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap())
}

#[allow(dead_code)]
#[inline(always)]
fn hash4(v: u64, bits: u32) -> u32 {
    ((v.wrapping_mul(0x1e35a7bd)) >> (32 - bits)) as u32
}

#[allow(dead_code)]
#[inline(always)]
fn hash6(v: u64, bits: u32) -> u32 {
    ((v.wrapping_mul(0xcf1bbcdcb7a56463)) >> (64 - bits)) as u32
}

#[allow(dead_code)]
#[inline(always)]
fn hash7(v: u64, bits: u32) -> u32 {
    ((v.wrapping_mul(0xcf1bbcdcb7a56463)) >> (64 - bits)) as u32
}

#[allow(dead_code)]
#[inline(always)]
fn hash8(v: u64, bits: u32) -> u32 {
    ((v.wrapping_mul(0x1e35a7bd1e35a7bd)) >> (64 - bits)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::decode_with_dict;
    use crate::encode::encode;

    #[test]
    fn test_dict_roundtrip_simple() {
        // Create a dictionary
        let dict_data = b"Hello, World! This is common text that appears frequently.";
        let dict = make_dict(dict_data, Some(b"Hello")).unwrap();

        // Encode some data (currently falls back to standard encoding)
        let data = b"Hello, World! Testing dictionary compression.";
        let compressed = encode(data);

        // Decode with dictionary should still work
        let decompressed = decode_with_dict(&compressed, &dict).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_dict_api_compatibility() {
        // Verify all encoding APIs work (even if they fall back for now)
        let dict_data = b"Common prefix data that repeats often in our dataset.";
        let dict = make_dict(dict_data, None).unwrap();

        let data = b"Test data for compression.";

        // All encode functions should work
        let _c1 = crate::encode::encode_with_dict(data, &dict);
        let _c2 = crate::encode::encode_better_with_dict(data, &dict);
        let _c3 = crate::encode::encode_best_with_dict(data, &dict);
    }

    #[test]
    fn test_dict_new() {
        // Create a simple dictionary
        let data = b"Hello, World! This is a test.";
        let dict = make_dict(data, None).unwrap();

        assert_eq!(dict.data(), data);
        assert_eq!(dict.repeat(), 0);
    }

    #[test]
    fn test_dict_serialization() {
        let data = b"Test dictionary data for serialization";
        let dict = make_dict(data, Some(b"dict")).unwrap();

        // Serialize and deserialize
        let bytes = dict.to_bytes();
        let dict2 = Dict::new(&bytes).unwrap();

        assert_eq!(dict.data(), dict2.data());
        assert_eq!(dict.repeat(), dict2.repeat());
    }

    #[test]
    fn test_dict_size_limits() {
        // Too small
        let small = b"tiny";
        assert!(make_dict(small, None).is_none());

        // Minimum size
        let min_size = vec![b'X'; MIN_DICT_SIZE];
        assert!(make_dict(&min_size, None).is_some());

        // Maximum size
        let max_size = vec![b'X'; MAX_DICT_SIZE];
        assert!(make_dict(&max_size, None).is_some());

        // Over maximum - should trim
        let over_max = vec![b'X'; MAX_DICT_SIZE + 1000];
        let dict = make_dict(&over_max, None).unwrap();
        assert_eq!(dict.data().len(), MAX_DICT_SIZE);
    }

    #[test]
    fn test_make_dict_with_search() {
        let data = b"The quick brown fox jumps over the lazy dog. The quick brown fox.";

        // Search for "The quick"
        let dict = make_dict(data, Some(b"The quick")).unwrap();

        // Should find the last occurrence
        assert!(dict.repeat() > 0);

        // Verify it's actually at that position
        assert!(dict.data()[dict.repeat()..].starts_with(b"The quick"));
    }

    #[test]
    fn test_make_dict_manual() {
        let data = vec![b'A'; 100];
        let dict = make_dict_manual(&data, 10).unwrap();

        assert_eq!(dict.repeat(), 10);
        assert_eq!(dict.data().len(), 100);
    }

    #[test]
    fn test_find_last_occurrence() {
        let haystack = b"Hello world, hello Rust, hello!";

        assert_eq!(find_last_occurrence(haystack, b"hello"), Some(25));
        assert_eq!(find_last_occurrence(haystack, b"Hello"), Some(0));
        assert_eq!(find_last_occurrence(haystack, b"xyz"), None);
    }
}
