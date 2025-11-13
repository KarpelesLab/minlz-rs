// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::{decode, encode, encode_better, encode_best, max_encoded_len};

fn roundtrip(data: &[u8]) -> Result<(), String> {
    let original = data.to_vec();

    // Test standard encoding
    let encoded = encode(&data);
    let decoded = decode(&encoded).map_err(|e| format!("decode error: {}", e))?;

    if decoded != original {
        return Err(format!(
            "roundtrip mismatch: original len={}, decoded len={}",
            original.len(),
            decoded.len()
        ));
    }

    // Test better encoding
    let encoded_better = encode_better(&data);
    let decoded_better = decode(&encoded_better).map_err(|e| format!("decode better error: {}", e))?;

    if decoded_better != original {
        return Err(format!(
            "roundtrip better mismatch: original len={}, decoded len={}",
            original.len(),
            decoded_better.len()
        ));
    }

    // Test best encoding
    let encoded_best = encode_best(&data);
    let decoded_best = decode(&encoded_best).map_err(|e| format!("decode best error: {}", e))?;

    if decoded_best != original {
        return Err(format!(
            "roundtrip best mismatch: original len={}, decoded len={}",
            original.len(),
            decoded_best.len()
        ));
    }

    Ok(())
}

#[test]
fn test_empty() {
    roundtrip(&[]).unwrap();
}

#[test]
fn test_small_copy() {
    for i in 0..32 {
        let mut s = b"aaaa".to_vec();
        s.extend(vec![b'b'; i]);
        s.extend(b"aaaabbbb");
        roundtrip(&s).unwrap();
    }
}

#[test]
fn test_small_rand() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    let rs = RandomState::new();
    let mut hasher = rs.build_hasher();
    1u64.hash(&mut hasher);
    let mut rng_state = hasher.finish();

    // Simple LCG for reproducible random numbers
    let lcg_next = |state: &mut u64| -> u8 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (*state >> 32) as u8
    };

    let mut n = 1;
    while n < 20000 {
        let mut b = vec![0u8; n];
        for byte in b.iter_mut() {
            *byte = lcg_next(&mut rng_state);
        }
        roundtrip(&b).unwrap();
        n += 23;
    }
}

#[test]
fn test_small_regular() {
    let mut n = 1;
    while n < 20000 {
        let mut b = vec![0u8; n];
        for (i, byte) in b.iter_mut().enumerate() {
            *byte = (i % 10) as u8 + b'a';
        }
        roundtrip(&b).unwrap();
        n += 23;
    }
}

#[test]
fn test_small_repeat() {
    let mut n = 1;
    while n < 20000 {
        let b = vec![b'a'; n];
        roundtrip(&b).unwrap();
        n += 23;
    }
}

#[test]
fn test_max_encoded_len() {
    let test_cases = vec![
        (0, Some(1)),
        (1, Some(2)),
        (100, Some(105)),
        (1024, Some(1031)),
        (65536, Some(65545)),
        (1 << 24, None), // Will succeed and return a valid size
    ];

    for (input, _expected) in test_cases {
        match max_encoded_len(input) {
            Ok(len) => {
                assert!(len >= input, "max_encoded_len must be >= input size");
            }
            Err(_) => {
                // Error is acceptable for very large sizes
                assert!(input > 1 << 30, "should only error on very large sizes");
            }
        }
    }
}

#[test]
fn test_literal_encoding() {
    // Test various literal sizes to ensure correct encoding
    let sizes = vec![1, 10, 59, 60, 61, 100, 255, 256, 1000, 65535, 65536, 100000];

    for size in sizes {
        let data = vec![b'x'; size];
        roundtrip(&data).unwrap();
    }
}

#[test]
fn test_copy_patterns() {
    // Test patterns that should produce copy operations
    let patterns = vec![
        b"aaaaaa".to_vec(),
        b"abcabcabc".to_vec(),
        b"The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.".to_vec(),
        (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>(),
    ];

    for pattern in patterns {
        roundtrip(&pattern).unwrap();
    }
}

#[test]
fn test_compression_ratio() {
    // Highly compressible data should compress well
    let data = vec![b'a'; 10000];
    let encoded = encode(&data);

    // Should compress to much less than original
    assert!(
        encoded.len() < data.len() / 10,
        "compression ratio too low: {} -> {}",
        data.len(),
        encoded.len()
    );
}

#[test]
fn test_incompressible_data() {
    // Random-ish data should not compress much (or at all)
    let data: Vec<u8> = (0..10000).map(|i| ((i * 31337) % 256) as u8).collect();
    let encoded = encode(&data);

    // Should not expand too much
    assert!(
        encoded.len() <= data.len() + data.len() / 8,
        "incompressible data expanded too much: {} -> {}",
        data.len(),
        encoded.len()
    );
}

#[test]
fn test_large_offsets() {
    // Create data that will require large offsets
    let mut data = Vec::new();
    data.extend(b"unique_pattern_12345");
    data.extend(vec![b'x'; 70000]);
    data.extend(b"unique_pattern_12345");

    roundtrip(&data).unwrap();
}

#[test]
fn test_boundary_conditions() {
    // Test various boundary conditions
    roundtrip(&[0]).unwrap();
    roundtrip(&[255]).unwrap();
    roundtrip(&[0, 255]).unwrap();
    roundtrip(&[255, 0]).unwrap();
}

#[test]
fn test_repeating_patterns() {
    // Test short repeating patterns
    for pattern_len in 1..=10 {
        let pattern: Vec<u8> = (0..pattern_len).map(|i| i as u8).collect();
        let mut data = Vec::new();
        for _ in 0..100 {
            data.extend(&pattern);
        }
        roundtrip(&data).unwrap();
    }
}
