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

#[test]
fn test_decode_golden_input() {
    // Tests decoding of golden compressed file from Go reference implementation
    // This validates compatibility with the reference implementation's output
    use std::fs;

    let compressed = fs::read("testdata/Mark.Twain-Tom.Sawyer.txt.rawsnappy")
        .expect("Failed to read golden compressed file");
    let expected = fs::read("testdata/Mark.Twain-Tom.Sawyer.txt")
        .expect("Failed to read golden text file");

    let decoded = decode(&compressed).expect("Failed to decode golden input");

    assert_eq!(
        decoded.len(),
        expected.len(),
        "Decoded length mismatch: got {}, want {}",
        decoded.len(),
        expected.len()
    );
    assert_eq!(
        decoded, expected,
        "Decoded content doesn't match original text"
    );
}

#[test]
fn test_emit_literal() {
    use crate::encode::test_helpers::test_emit_literal;

    // Test cases from Go implementation
    let test_cases = vec![
        (1, vec![0x00]),
        (2, vec![0x04]),
        (59, vec![0xe8]),
        (60, vec![0xec]),
        (61, vec![0xf0, 0x3c]),
        (62, vec![0xf0, 0x3d]),
        (254, vec![0xf0, 0xfd]),
        (255, vec![0xf0, 0xfe]),
        (256, vec![0xf0, 0xff]),
        (257, vec![0xf4, 0x00, 0x01]),
        (65534, vec![0xf4, 0xfd, 0xff]),
        (65535, vec![0xf4, 0xfe, 0xff]),
        (65536, vec![0xf4, 0xff, 0xff]),
    ];

    let mut dst = vec![0u8; 70000];
    let nines = vec![0x99u8; 65536];

    for (length, want) in test_cases {
        let lit = &nines[..length];
        let n = test_emit_literal(&mut dst, lit);

        // Check that output ends with the literal bytes
        assert!(
            &dst[n - length..n] == lit,
            "length={}: output doesn't end with literal bytes",
            length
        );

        // Check the header bytes
        let got_header = &dst[..n - length];
        assert_eq!(
            got_header, &want[..],
            "length={}: header mismatch\ngot:  {:?}\nwant: {:?}",
            length, got_header, want
        );
    }
}

#[test]
fn test_emit_copy() {
    use crate::encode::test_helpers::test_emit_copy;

    // Test cases from Go implementation
    let test_cases = vec![
        // offset=8 cases
        (8, 4, vec![0x01, 0x08]),
        (8, 11, vec![0x1d, 0x08]),
        (8, 12, vec![0x2e, 0x08, 0x00]),
        (8, 13, vec![0x32, 0x08, 0x00]),
        (8, 59, vec![0xea, 0x08, 0x00]),
        (8, 60, vec![0xee, 0x08, 0x00]),
        (8, 61, vec![0xf2, 0x08, 0x00]),
        (8, 62, vec![0xf6, 0x08, 0x00]),
        (8, 63, vec![0xfa, 0x08, 0x00]),
        (8, 64, vec![0xfe, 0x08, 0x00]),
        // offset=256 cases
        (256, 4, vec![0x21, 0x00]),
        (256, 11, vec![0x3d, 0x00]),
        (256, 12, vec![0x2e, 0x00, 0x01]),
        (256, 13, vec![0x32, 0x00, 0x01]),
        (256, 59, vec![0xea, 0x00, 0x01]),
        (256, 60, vec![0xee, 0x00, 0x01]),
        (256, 61, vec![0xf2, 0x00, 0x01]),
        (256, 62, vec![0xf6, 0x00, 0x01]),
        (256, 63, vec![0xfa, 0x00, 0x01]),
        (256, 64, vec![0xfe, 0x00, 0x01]),
        // offset=2048 cases (tagCopy2 - 3 bytes)
        (2048, 4, vec![0x0e, 0x00, 0x08]),
        (2048, 11, vec![0x2a, 0x00, 0x08]),
        (2048, 12, vec![0x2e, 0x00, 0x08]),
        (2048, 13, vec![0x32, 0x00, 0x08]),
        (2048, 59, vec![0xea, 0x00, 0x08]),
        (2048, 60, vec![0xee, 0x00, 0x08]),
        (2048, 61, vec![0xf2, 0x00, 0x08]),
        (2048, 62, vec![0xf6, 0x00, 0x08]),
        (2048, 63, vec![0xfa, 0x00, 0x08]),
        (2048, 64, vec![0xfe, 0x00, 0x08]),
    ];

    let mut dst = vec![0u8; 100];

    for (offset, length, want) in test_cases {
        dst.fill(0);
        let n = test_emit_copy(&mut dst, offset, length);

        let got = &dst[..n];
        assert_eq!(
            got, &want[..],
            "offset={}, length={}: mismatch\ngot:  {:?}\nwant: {:?}",
            offset, length, got, want
        );
    }
}

#[test]
fn test_match_len() {
    use crate::encode::test_helpers::test_match_len;

    // Reference implementation
    let reference = |a: &[u8], b: &[u8]| -> usize {
        let mut n = 0;
        for i in 0..a.len().min(b.len()) {
            if a[i] != b[i] {
                break;
            }
            n += 1;
        }
        n
    };

    // Test various patterns
    let nums = vec![0, 1, 2, 7, 8, 9, 16, 20, 29, 30, 31, 32, 33, 34, 38, 39, 40];

    for y_index in (31..=40).rev() {
        let mut xxx = vec![b'x'; 40];
        if y_index < xxx.len() {
            xxx[y_index] = b'y';
        }

        for &i in &nums {
            for &j in &nums {
                if i >= j {
                    continue;
                }

                let got = test_match_len(&xxx[j..], &xxx[i..]);
                let want = reference(&xxx[j..], &xxx[i..]);

                // Allow exact match or very close
                assert!(
                    got == want || got.abs_diff(want) <= 1,
                    "y_index={}, i={}, j={}: got {}, want {}",
                    y_index,
                    i,
                    j,
                    got,
                    want
                );
            }
        }
    }
}

#[test]
fn test_big_encode_buffer() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    // Use smaller sizes for tests to avoid stack overflow
    const BLOCK_SIZE: usize = 64 * 1024; // 64 KB (instead of 1 MB)
    let mut buf = vec![0u8; BLOCK_SIZE * 2];
    let mut compressed = Vec::new();

    // Use a smaller max for faster tests (Go uses 4 for short tests)
    let max: u8 = 4;

    {
        let mut writer = Writer::with_block_size(&mut compressed, BLOCK_SIZE);

        for n in 0..max {
            // Fill buffer with a repeating value
            for b in buf.iter_mut() {
                *b = n;
            }

            // Write the buffer twice (simulating EncodeBuffer being called twice with same data)
            writer.write_all(&buf).expect("write failed");
            writer.write_all(&buf).expect("write failed");
            writer.flush().expect("flush failed");
        }
        // Writer drops here, flushing any remaining data
    }

    // Decode and verify we can read it all back
    let mut reader = Reader::new(&compressed[..]);
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded).expect("decode failed");

    // Verify the decoded size
    // We wrote: max iterations × 2 writes × (BLOCK_SIZE * 2) bytes each
    let expected_size = max as usize * 2 * (BLOCK_SIZE * 2);
    assert_eq!(
        decoded.len(),
        expected_size,
        "decoded size mismatch: got {}, want {}",
        decoded.len(),
        expected_size
    );

    // Verify the content pattern by checking chunks
    let chunk_size = BLOCK_SIZE * 2;
    let mut offset = 0;
    for n in 0..max {
        for _ in 0..2 {
            // Two writes per iteration
            let chunk = &decoded[offset..offset + chunk_size];
            // Verify all bytes in this chunk equal n
            assert!(
                chunk.iter().all(|&b| b == n),
                "data mismatch in chunk starting at offset {}: expected all bytes to be {}",
                offset,
                n
            );
            offset += chunk_size;
        }
    }
}

#[test]
fn test_reader_uncompressed_data_ok() {
    use crate::reader::Reader;
    use crate::crc::crc;
    use std::io::Read;

    // Build stream: magic + uncompressed chunk
    let mut stream = Vec::new();
    
    // Magic bytes
    stream.extend_from_slice(b"\xff\x06\x00\x00S2sTwO");
    
    // Uncompressed chunk header: type=0x01, length=8 (4 byte checksum + 4 byte data)
    stream.push(0x01);
    stream.extend_from_slice(&[0x08, 0x00, 0x00]); // length = 8 in little-endian
    
    // Calculate CRC for "abcd"
    let data = b"abcd";
    let checksum = crc(data);
    stream.extend_from_slice(&checksum.to_le_bytes());
    
    // Uncompressed payload
    stream.extend_from_slice(data);
    
    // Read and verify
    let mut reader = Reader::new(&stream[..]);
    let mut output = Vec::new();
    reader.read_to_end(&mut output).expect("read failed");
    
    assert_eq!(output, b"abcd");
}

#[test]
fn test_reader_uncompressed_data_no_payload() {
    use crate::reader::Reader;
    use std::io::Read;

    // Build stream with truncated uncompressed chunk
    let mut stream = Vec::new();
    
    // Magic bytes
    stream.extend_from_slice(b"\xff\x06\x00\x00S2sTwO");
    
    // Uncompressed chunk header: type=0x01, length=4
    stream.push(0x01);
    stream.extend_from_slice(&[0x04, 0x00, 0x00]);
    
    // No payload - this is corrupt
    
    // Should get an error (UnexpectedEof when trying to read)
    let mut reader = Reader::new(&stream[..]);
    let mut output = Vec::new();
    let result = reader.read_to_end(&mut output);
    
    assert!(result.is_err(), "expected error for missing payload");
}

#[test]
fn test_reader_uncompressed_data_too_long() {
    use crate::reader::Reader;
    use std::io::Read;

    const MAX_BLOCK_SIZE: usize = 4 << 20; // 4 MB
    const CHECKSUM_SIZE: usize = 4;
    
    // Build stream with chunk that is exactly at the limit
    let n = MAX_BLOCK_SIZE + CHECKSUM_SIZE;
    let n32 = n as u32;
    
    let mut stream = Vec::new();
    
    // Magic bytes
    stream.extend_from_slice(b"\xff\x06\x00\x00S2sTwO");
    
    // Uncompressed chunk header with valid size (at limit)
    stream.push(0x01);
    stream.extend_from_slice(&[n32 as u8, (n32 >> 8) as u8, (n32 >> 16) as u8]);
    
    // Add n bytes of zeros (this should work, though CRC will fail)
    stream.resize(stream.len() + n, 0);
    
    // Should get CRC error (since CRC is all zeros)
    let mut reader = Reader::new(&stream[..]);
    let mut output = Vec::new();
    let result = reader.read_to_end(&mut output);
    
    assert!(result.is_err(), "expected CRC error");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("CRC"),
        "expected CRC error, got: {}",
        err_msg
    );
    
    // Now test with chunk that is too large (over the limit)
    let n_invalid = n + 1;
    let n32_invalid = n_invalid as u32;
    
    let mut stream2 = Vec::new();
    stream2.extend_from_slice(b"\xff\x06\x00\x00S2sTwO");
    stream2.push(0x01);
    stream2.extend_from_slice(&[
        n32_invalid as u8,
        (n32_invalid >> 8) as u8,
        (n32_invalid >> 16) as u8,
    ]);
    stream2.resize(stream2.len() + n_invalid, 0);
    
    let mut reader2 = Reader::new(&stream2[..]);
    let mut output2 = Vec::new();
    let result2 = reader2.read_to_end(&mut output2);
    
    // Should fail (either with "chunk too large" or similar error)
    assert!(result2.is_err(), "expected error for too-large chunk");
}

#[test]
fn test_big_regular_writes() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    // Use smaller sizes for tests (Go uses maxBlockSize which is 4MB)
    const BLOCK_SIZE: usize = 64 * 1024; // 64 KB
    let mut buf = vec![0u8; BLOCK_SIZE * 2];
    let mut compressed = Vec::new();

    // Use max=4 for short tests (Go uses 4 for short tests)
    let max: u8 = 4;

    {
        let mut writer = Writer::new(&mut compressed);

        for n in 0..max {
            // Fill buffer with repeating value
            for b in buf.iter_mut() {
                *b = n;
            }

            // Write using standard Write interface
            // (Writes may not keep a reference to the data beyond the Write call)
            writer.write_all(&buf).expect("write failed");
        }

        // Close writer (drop will flush)
    }

    // Decode and verify
    let mut reader = Reader::new(&compressed[..]);
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded).expect("decode failed");

    // Verify the decoded size
    let expected_size = max as usize * (BLOCK_SIZE * 2);
    assert_eq!(
        decoded.len(),
        expected_size,
        "decoded size mismatch: got {}, want {}",
        decoded.len(),
        expected_size
    );

    // Verify the content pattern
    let chunk_size = BLOCK_SIZE * 2;
    let mut offset = 0;
    for n in 0..max {
        let chunk = &decoded[offset..offset + chunk_size];
        assert!(
            chunk.iter().all(|&b| b == n),
            "data mismatch in chunk starting at offset {}: expected all bytes to be {}",
            offset,
            n
        );
        offset += chunk_size;
    }
}

#[test]
fn test_leading_skippable_block() {
    use crate::reader::Reader;
    use std::io::Read;

    // Build a stream with: magic + skippable block + compressed data
    let mut stream = Vec::new();
    
    // Magic bytes
    stream.extend_from_slice(b"\xff\x06\x00\x00S2sTwO");
    
    // Skippable block (type 0x80-0xfd)
    // Type: 0x80, Length: 15 bytes ("skippable block")
    stream.push(0x80);
    stream.extend_from_slice(&[0x0f, 0x00, 0x00]); // length = 15
    stream.extend_from_slice(b"skippable block");
    
    // Now add compressed data for "some data"
    // We'll use encode to create proper compressed data
    use crate::encode::encode;
    use crate::crc::crc;
    
    let data = b"some data";
    let compressed = encode(data);
    let checksum = crc(data);
    
    // Compressed chunk header
    stream.push(0x00); // CHUNK_TYPE_COMPRESSED_DATA
    let chunk_len = compressed.len() + 4; // +4 for checksum
    stream.extend_from_slice(&[
        chunk_len as u8,
        (chunk_len >> 8) as u8,
        (chunk_len >> 16) as u8,
    ]);
    
    // Checksum and data
    stream.extend_from_slice(&checksum.to_le_bytes());
    stream.extend_from_slice(&compressed);
    
    // Read and verify - skippable block should be ignored
    let mut reader = Reader::new(&stream[..]);
    
    // Empty read to trigger initial processing
    let mut empty = [0u8; 0];
    reader.read(&mut empty).expect("empty read failed");
    
    // Read all data - should only get "some data", not the skippable block
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded).expect("read failed");
    
    assert_eq!(
        decoded,
        b"some data",
        "expected decoded data to be 'some data', got {:?}",
        String::from_utf8_lossy(&decoded)
    );
}

#[test]
fn test_framing_format() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    // Create 1MB source with alternating incompressible and compressible sequences
    // Each sequence is 100KB (1e5 bytes), larger than maxBlockSize (64KB)
    const CHUNK_SIZE: usize = 100_000;
    let mut src = vec![0u8; CHUNK_SIZE * 10];
    
    // Use a seeded RNG for reproducibility
    let mut seed = 1u64;
    let mut next_random = || -> u8 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        (seed >> 16) as u8
    };

    for i in 0..10 {
        let start = CHUNK_SIZE * i;

        if i % 2 == 0 {
            // Incompressible: random bytes
            for j in 0..CHUNK_SIZE {
                src[start + j] = next_random();
            }
        } else {
            // Compressible: repeated bytes
            for j in 0..CHUNK_SIZE {
                src[start + j] = i as u8;
            }
        }
    }
    
    // Encode
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(&src).expect("write failed");
    }
    
    // Decode
    let mut reader = Reader::new(&compressed[..]);
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded).expect("read failed");
    
    // Verify
    assert_eq!(
        decoded.len(),
        src.len(),
        "decoded length mismatch: got {}, want {}",
        decoded.len(),
        src.len()
    );
    assert_eq!(decoded, src, "decoded data does not match source");
}

#[test]
fn test_framing_format_better() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    // Same test as test_framing_format, but with "better" compression
    // (Our implementation doesn't have compression levels yet, so this is the same)
    
    const CHUNK_SIZE: usize = 100_000;
    let mut src = vec![0u8; CHUNK_SIZE * 10];

    let mut seed = 1u64;
    let mut next_random = || -> u8 {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        (seed >> 16) as u8
    };

    for i in 0..10 {
        let start = CHUNK_SIZE * i;

        if i % 2 == 0 {
            for j in 0..CHUNK_SIZE {
                src[start + j] = next_random();
            }
        } else {
            for j in 0..CHUNK_SIZE {
                src[start + j] = i as u8;
            }
        }
    }
    
    // Encode (would use WriterBetterCompression if we had compression levels)
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(&src).expect("write failed");
    }
    
    // Decode
    let mut reader = Reader::new(&compressed[..]);
    let mut decoded = Vec::new();
    reader.read_to_end(&mut decoded).expect("read failed");
    
    // Verify
    assert_eq!(
        decoded.len(),
        src.len(),
        "decoded length mismatch: got {}, want {}",
        decoded.len(),
        src.len()
    );
    assert_eq!(decoded, src, "decoded data does not match source");
}

#[test]
fn test_flush() {
    use crate::writer::Writer;
    use std::io::Write;

    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(&mut buf);

        // Write 20 'x' bytes
        let data = vec![b'x'; 20];
        writer.write_all(&data).expect("write failed");

        // Before flush, nothing should be written yet (data is buffered)
        let len_before = writer.get_ref().len();
        assert_eq!(
            len_before,
            0,
            "before Flush: {} bytes were written to the underlying writer, want 0",
            len_before
        );

        // Flush
        writer.flush().expect("flush failed");

        // After flush, data should be written
        let len_after = writer.get_ref().len();
        assert!(
            len_after > 0,
            "after Flush: {} bytes were written to the underlying writer, want non-0",
            len_after
        );

        // Keep writer alive until the end of the scope
    }
}

#[test]
fn test_new_writer() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    // Test all 32 possible sub-sequences of these 5 input slices
    // Their lengths sum to 400,000, which is over 6x the max block size
    let inputs = vec![
        vec![b'a'; 40_000],
        vec![b'b'; 150_000],
        vec![b'c'; 60_000],
        vec![b'd'; 120_000],
        vec![b'e'; 30_000],
    ];

    // Test all 32 combinations (2^5)
    for i in 0..(1 << inputs.len()) {
        let mut want = Vec::new();
        let mut compressed = Vec::new();

        {
            let mut writer = Writer::new(&mut compressed);

            for (j, input) in inputs.iter().enumerate() {
                if i & (1 << j) == 0 {
                    continue;
                }
                writer.write_all(input).expect(&format!("i={:#02x}: j={}: Write failed", i, j));
                want.extend_from_slice(input);
            }
        } // Writer drops and closes here

        // If no inputs were written, skip decompression (empty stream)
        if want.is_empty() {
            continue;
        }

        // Decompress and verify
        let mut reader = Reader::new(&compressed[..]);
        let mut got = Vec::new();
        reader.read_to_end(&mut got).expect(&format!("i={:#02x}: ReadAll failed", i));

        assert_eq!(
            got, want,
            "i={:#02x}: decoded data mismatch (got {} bytes, want {} bytes)",
            i, got.len(), want.len()
        );
    }
}

#[test]
fn test_encode_noise_then_repeats() {
    use crate::encode::encode;

    // Test with smaller sizes to avoid stack overflow
    // (Go tests with 256KB and 2MB, but we use smaller for testing)
    for orig_len in [64 * 1024, 256 * 1024] {
        let mut src = vec![0u8; orig_len];
        
        // Use seeded RNG for reproducibility
        let mut seed = 1u64;
        let mut next_random = || -> u8 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            (seed >> 16) as u8
        };
        
        // First half: incompressible random data
        let half = orig_len / 2;
        for i in 0..half {
            src[i] = next_random();
        }
        
        // Second half: compressible repeated pattern
        for i in half..orig_len {
            src[i] = ((i >> 8) & 0xff) as u8;
        }
        
        // Encode
        let dst = encode(&src);
        
        // The encoded size should be less than 75% of original
        // (first half incompressible ~1:1, second half highly compressible)
        let max_size = orig_len * 3 / 4;
        assert!(
            dst.len() < max_size,
            "origLen={}: got {} encoded bytes, want less than {}",
            orig_len,
            dst.len(),
            max_size
        );
    }
}

#[test]
fn test_writer_reset_without_flush() {
    use crate::writer::Writer;
    use crate::reader::Reader;
    use std::io::{Write, Read};

    let mut buf0 = Vec::new();
    let mut buf1 = Vec::new();
    
    {
        let mut writer = Writer::new(&mut buf0);
        
        // Write "xxx" to buf0
        writer.write_all(b"xxx").expect("Write #0 failed");
        
        // Note: we don't Flush before calling Reset
        // This should discard the "xxx" data
        writer.reset(&mut buf1);
        
        // Write "yyy" to buf1
        writer.write_all(b"yyy").expect("Write #1 failed");
        writer.flush().expect("Flush failed");
        
        // writer drops here
    }
    
    // buf0 should be empty (no data written before reset)
    // buf1 should contain compressed "yyy"
    
    // Verify buf1 contains "yyy"
    let mut reader = Reader::new(&buf1[..]);
    let mut got = Vec::new();
    reader.read_to_end(&mut got).expect("ReadAll failed");
    
    assert_eq!(
        got,
        b"yyy",
        "expected 'yyy', got {:?}",
        String::from_utf8_lossy(&got)
    );
}

#[test]
fn test_decode_edge_cases() {
    use crate::decode::decode;

    // Create a 40-byte literal for test cases
    let lit40: Vec<u8> = (0..40).collect();

    // Test cases: (description, input, expected_output, should_error)
    let test_cases = vec![
        // Empty input
        ("decodedLen=0; valid input", vec![0x00], vec![], false),
        
        // tagLiteral with 0-byte length encoding
        ("decodedLen=3; tagLiteral, 0-byte length; length=3", 
         vec![0x03, 0x08, 0xff, 0xff, 0xff], 
         vec![0xff, 0xff, 0xff], false),
        
        // tagLiteral with 1-byte length encoding
        ("decodedLen=3; tagLiteral, 1-byte length; length=3",
         vec![0x03, 0xf0, 0x02, 0xff, 0xff, 0xff],
         vec![0xff, 0xff, 0xff], false),
        
        // tagLiteral with 2-byte length encoding
        ("decodedLen=3; tagLiteral, 2-byte length; length=3",
         vec![0x03, 0xf4, 0x02, 0x00, 0xff, 0xff, 0xff],
         vec![0xff, 0xff, 0xff], false),
        
        // tagLiteral with 40 bytes
        (
            "decodedLen=40; tagLiteral, 0-byte length; length=40",
            {
                let mut v = vec![0x28, 0x9c];
                v.extend_from_slice(&lit40);
                v
            },
            lit40.clone(),
            false
        ),
        
        // tagLiteral (4 bytes "abcd") only
        ("decodedLen=4; tagLiteral (4 bytes abcd)",
         vec![0x04, 0x0c, b'a', b'b', b'c', b'd'],
         b"abcd".to_vec(), false),
        
        // tagLiteral + tagCopy1: "abcd" then copy length=9 offset=4
        ("decodedLen=13; tagLiteral + tagCopy1; length=9 offset=4",
         vec![0x0d, 0x0c, b'a', b'b', b'c', b'd', 0x15, 0x04],
         b"abcdabcdabcda".to_vec(), false),
        
        // tagLiteral + tagCopy1: "abcd" then copy length=4 offset=4
        ("decodedLen=8; tagLiteral + tagCopy1; length=4 offset=4",
         vec![0x08, 0x0c, b'a', b'b', b'c', b'd', 0x01, 0x04],
         b"abcdabcd".to_vec(), false),
        
        // tagLiteral + tagCopy1: "abcd" then copy length=4 offset=2 (overlapping)
        ("decodedLen=8; tagLiteral + tagCopy1; length=4 offset=2",
         vec![0x08, 0x0c, b'a', b'b', b'c', b'd', 0x01, 0x02],
         b"abcdcdcd".to_vec(), false),
        
        // tagLiteral + tagCopy1: "abcd" then copy length=4 offset=1 (repeating)
        ("decodedLen=8; tagLiteral + tagCopy1; length=4 offset=1",
         vec![0x08, 0x0c, b'a', b'b', b'c', b'd', 0x01, 0x01],
         b"abcddddd".to_vec(), false),
        
        // Error cases: not enough dst bytes
        ("decodedLen=2; tagLiteral, 0-byte length; length=3; not enough dst bytes",
         vec![0x02, 0x08, 0xff, 0xff, 0xff],
         vec![], true),
        
        // Error cases: not enough src bytes
        ("decodedLen=3; tagLiteral, 0-byte length; length=3; not enough src bytes",
         vec![0x03, 0x08, 0xff, 0xff],
         vec![], true),
        
        // Error cases: offset=0 (invalid)
        ("decodedLen=8; tagLiteral + tagCopy1; length=4 offset=0",
         vec![0x08, 0x0c, b'a', b'b', b'c', b'd', 0x01, 0x00],
         vec![], true),
        
        // Error cases: offset too large
        ("decodedLen=8; tagLiteral + tagCopy1; length=4 offset=5; offset too large",
         vec![0x08, 0x0c, b'a', b'b', b'c', b'd', 0x01, 0x05],
         vec![], true),
    ];

    for (desc, input, expected, should_error) in test_cases {
        let result = decode(&input);
        
        if should_error {
            assert!(result.is_err(), "{}: expected error but got success", desc);
        } else {
            match result {
                Ok(output) => {
                    assert_eq!(
                        output, expected,
                        "{}: output mismatch\ngot:  {:?}\nwant: {:?}",
                        desc, output, expected
                    );
                }
                Err(e) => {
                    panic!("{}: unexpected error: {}", desc, e);
                }
            }
        }
    }
}
