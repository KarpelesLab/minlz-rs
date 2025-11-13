// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::{decode, encode, encode_best, encode_better, max_encoded_len};

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
    let decoded_better =
        decode(&encoded_better).map_err(|e| format!("decode better error: {}", e))?;

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

// Critical tests ported from Go implementation

#[test]
fn test_decode_copy4() {
    // Tests 4-byte offset copy operations
    // This tests the tagCopy4 encoding with length=5 offset=65540
    let dots = ".".repeat(65536);

    let mut input = Vec::new();
    // decodedLen = 65545 as varint
    input.extend(&[0x89, 0x80, 0x04]);
    // 4-byte literal "pqrs"
    input.extend(&[0x0c]);
    input.extend(b"pqrs");
    // 65536-byte literal dots
    input.extend(&[0xf4, 0xff, 0xff]);
    input.extend(dots.as_bytes());
    // tagCopy4; length=5 offset=65540
    input.extend(&[0x13, 0x04, 0x00, 0x01, 0x00]);

    let got = decode(&input).expect("decode failed");
    let want = format!("pqrs{}pqrs.", dots);

    assert_eq!(got.len(), want.len(), "length mismatch");
    assert_eq!(got, want.as_bytes(), "content mismatch");
}

#[test]
fn test_invalid_varint() {
    // Tests that invalid varints are properly rejected
    let test_cases = vec![
        (
            "invalid varint, final byte has continuation bit set",
            vec![0xff],
        ),
        (
            "invalid varint, value overflows uint64",
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00],
        ),
        (
            "valid varint (as uint64), but value overflows uint32",
            vec![0x80, 0x80, 0x80, 0x80, 0x10],
        ),
    ];

    for (desc, input) in test_cases {
        // decode should return an error, not panic
        match decode(&input) {
            Err(_) => {}, // Expected
            Ok(_) => panic!("{}: expected error but got success", desc),
        }
    }
}

#[test]
fn test_decode_length_offset() {
    // Tests decoding with various length and offset combinations
    // This validates variable-length integer encoding
    const PREFIX: &[u8] = b"abcdefghijklmnopqr";
    const SUFFIX: &[u8] = b"ABCDEFGHIJKLMNOPQR";
    const NOT_PRESENT_BASE: u8 = 0xa0;
    const NOT_PRESENT_LEN: usize = 37;
    const TAG_LITERAL: u8 = 0x00;
    const TAG_COPY2: u8 = 0x02;

    for length in 1..=18 {
        for offset in 1..=18 {
            'suffix_loop: for suffix_len in 0..=18 {
                let total_len = PREFIX.len() + length + suffix_len;

                // Build input
                let mut input_buf = [0u8; 128];
                let mut input_len = 0;

                // Encode length as varint
                let mut n = total_len as u64;
                while n >= 0x80 {
                    input_buf[input_len] = (n as u8) | 0x80;
                    input_len += 1;
                    n >>= 7;
                }
                input_buf[input_len] = n as u8;
                input_len += 1;

                // Literal prefix
                input_buf[input_len] = TAG_LITERAL + 4 * ((PREFIX.len() - 1) as u8);
                input_len += 1;
                input_buf[input_len..input_len + PREFIX.len()].copy_from_slice(PREFIX);
                input_len += PREFIX.len();

                // Copy2 operation
                input_buf[input_len] = TAG_COPY2 + 4 * ((length - 1) as u8);
                input_buf[input_len + 1] = offset as u8;
                input_buf[input_len + 2] = 0x00;
                input_len += 3;

                // Literal suffix
                if suffix_len > 0 {
                    input_buf[input_len] = TAG_LITERAL + 4 * ((suffix_len - 1) as u8);
                    input_len += 1;
                    input_buf[input_len..input_len + suffix_len]
                        .copy_from_slice(&SUFFIX[..suffix_len]);
                    input_len += suffix_len;
                }

                let input = &input_buf[..input_len];

                // Initialize output buffer with sentinel values
                let mut got_buf = [0u8; 128];
                for i in 0..got_buf.len() {
                    got_buf[i] = NOT_PRESENT_BASE + (i % NOT_PRESENT_LEN) as u8;
                }

                // Decode
                let got = match decode(input) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!(
                            "length={}, offset={}, suffix_len={}: decode error: {}",
                            length, offset, suffix_len, e
                        );
                    }
                };

                // Build expected output
                let mut want = Vec::new();
                want.extend_from_slice(PREFIX);
                for _ in 0..length {
                    want.push(want[want.len() - offset]);
                }
                want.extend_from_slice(&SUFFIX[..suffix_len]);

                // Verify input doesn't contain sentinel values
                for &x in input.iter() {
                    if x >= NOT_PRESENT_BASE && x < NOT_PRESENT_BASE + NOT_PRESENT_LEN as u8 {
                        continue 'suffix_loop;
                    }
                }

                // Verify output doesn't contain sentinel values
                for &x in want.iter() {
                    if x >= NOT_PRESENT_BASE && x < NOT_PRESENT_BASE + NOT_PRESENT_LEN as u8 {
                        continue 'suffix_loop;
                    }
                }

                // Compare
                assert_eq!(
                    got, want,
                    "length={}, offset={}, suffix_len={}: mismatch",
                    length, offset, suffix_len
                );
            }
        }
    }
}

#[test]
fn test_slow_forward_copy_overrun() {
    // Tests overlapping copy operations where the copy reads from data being written
    // This simulates the internal decoder logic for handling small offsets
    const BASE: usize = 100;

    for length in 1..18 {
        for offset in 1..18 {
            // Build input that will trigger overlapping copy
            let mut input_buf = Vec::new();

            // Total length
            let total_len = BASE + length;
            let mut n = total_len as u64;
            while n >= 0x80 {
                input_buf.push((n as u8) | 0x80);
                n >>= 7;
            }
            input_buf.push(n as u8);

            // Literal: BASE bytes of 'x'
            if BASE <= 60 {
                input_buf.push(0x00 + 4 * ((BASE - 1) as u8));
            } else if BASE <= 256 {
                input_buf.push(0xf0);
                input_buf.push((BASE - 1) as u8);
            } else {
                input_buf.push(0xf4);
                input_buf.push(((BASE - 1) & 0xff) as u8);
                input_buf.push((((BASE - 1) >> 8) & 0xff) as u8);
            }
            for _ in 0..BASE {
                input_buf.push(b'x');
            }

            // Copy with small offset (overlapping)
            // Use tagCopy1 for lengths 4-11 with small offsets
            // Use tagCopy2 for other cases
            if length >= 4 && length <= 11 && offset < 2048 {
                input_buf.push(0x01 + 4 * ((length - 4) as u8));
                input_buf.push((offset & 0xff) as u8);
            } else {
                input_buf.push(0x02 + 4 * ((length - 1) as u8));
                input_buf.push((offset & 0xff) as u8);
                input_buf.push(((offset >> 8) & 0xff) as u8);
            }

            let got = match decode(&input_buf) {
                Ok(v) => v,
                Err(e) => {
                    panic!("length={}, offset={}: decode error: {}", length, offset, e);
                }
            };

            // Build expected output
            let mut want = vec![b'x'; BASE];
            for _ in 0..length {
                want.push(want[want.len() - offset]);
            }

            assert_eq!(
                got, want,
                "length={}, offset={}: mismatch",
                length, offset
            );
        }
    }
}

#[test]
fn test_reader_reset() {
    use crate::{Reader, Writer};
    use std::io::{Read, Write};

    // Test that Reader can be reset and reused with different data
    let test_data_1 = b"First test data with some repeated patterns aaaaabbbbcccc";
    let test_data_2 = b"Second test data completely different xxxxyyyyzzzz";
    let test_data_3 = b"Third data set for testing reset functionality 123456789";

    // Compress all test data
    let mut compressed1 = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed1);
        writer.write_all(test_data_1).unwrap();
        writer.flush().unwrap();
    }

    let mut compressed2 = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed2);
        writer.write_all(test_data_2).unwrap();
        writer.flush().unwrap();
    }

    let mut compressed3 = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed3);
        writer.write_all(test_data_3).unwrap();
        writer.flush().unwrap();
    }

    // Create reader and read first data
    let mut reader = Reader::new(&compressed1[..]);
    let mut output1 = Vec::new();
    reader.read_to_end(&mut output1).unwrap();
    assert_eq!(output1, test_data_1);

    // Reset with second data
    reader.reset(&compressed2[..]);
    let mut output2 = Vec::new();
    reader.read_to_end(&mut output2).unwrap();
    assert_eq!(output2, test_data_2);

    // Reset with third data
    reader.reset(&compressed3[..]);
    let mut output3 = Vec::new();
    reader.read_to_end(&mut output3).unwrap();
    assert_eq!(output3, test_data_3);
}

#[test]
fn test_writer_reset() {
    use crate::{Reader, Writer};
    use std::io::{Read, Write};

    // Test that Writer can be reset and reused with different destinations
    let test_data_1 = b"First chunk of data to compress with pattern aaabbbccc";
    let test_data_2 = b"Second chunk with different content xxxxyyyyzzzz";
    let test_data_3 = b"Third chunk for reset testing 123456789";

    let mut output1 = Vec::new();
    let mut output2 = Vec::new();
    let mut output3 = Vec::new();

    // Create writer and write to all outputs, using reset
    {
        let mut writer = Writer::new(&mut output1);
        writer.write_all(test_data_1).unwrap();
        writer.flush().unwrap();

        // Reset and write to second output
        writer.reset(&mut output2);
        writer.write_all(test_data_2).unwrap();
        writer.flush().unwrap();

        // Reset and write to third output
        writer.reset(&mut output3);
        writer.write_all(test_data_3).unwrap();
        writer.flush().unwrap();
    } // Drop writer here

    // Verify all compressions are independent and correct
    let mut reader = Reader::new(&output1[..]);
    let mut decoded1 = Vec::new();
    reader.read_to_end(&mut decoded1).unwrap();
    assert_eq!(decoded1, test_data_1);

    reader.reset(&output2[..]);
    let mut decoded2 = Vec::new();
    reader.read_to_end(&mut decoded2).unwrap();
    assert_eq!(decoded2, test_data_2);

    reader.reset(&output3[..]);
    let mut decoded3 = Vec::new();
    reader.read_to_end(&mut decoded3).unwrap();
    assert_eq!(decoded3, test_data_3);
}
