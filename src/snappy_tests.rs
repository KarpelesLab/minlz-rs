// Copyright 2024 Karpeles Lab Inc.
// Tests for Snappy encoding compatibility

use crate::{decode, encode_snappy};

#[test]
fn test_encode_snappy_roundtrip() {
    // Basic roundtrip test for Snappy encoding
    let test_data = b"Hello, World! This is a test of Snappy encoding.";
    let encoded = encode_snappy(test_data);
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, test_data);
}

#[test]
fn test_encode_snappy_empty() {
    let encoded = encode_snappy(b"");
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, b"");
}

#[test]
fn test_encode_snappy_small() {
    for i in 1..20 {
        let data = vec![b'a'; i];
        let encoded = encode_snappy(&data);
        let decoded = decode(&encoded).expect("decode failed");
        assert_eq!(decoded, data);
    }
}

#[test]
fn test_encode_snappy_repeated() {
    // Test with repeated patterns
    let data = b"aaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmm";
    let encoded = encode_snappy(data);
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, data);
}

#[test]
fn test_encode_snappy_large() {
    // Test with larger data
    let data = vec![b'X'; 100000];
    let encoded = encode_snappy(&data);
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, data);
}

#[test]
fn test_encode_snappy_incompressible() {
    // Test with random-like data (shouldn't compress well)
    let mut data = Vec::new();
    let mut x = 1u32;
    for _ in 0..1000 {
        x = x.wrapping_mul(1103515245).wrapping_add(12345);
        data.push((x >> 16) as u8);
    }

    let encoded = encode_snappy(&data);
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, data);
}

#[test]
fn test_encode_snappy_mixed() {
    // Mix of compressible and incompressible data
    let mut data = Vec::new();

    // Compressible repeated pattern
    data.extend(vec![b'a'; 100]);

    // Incompressible random bytes
    let mut x = 1u32;
    for _ in 0..100 {
        x = x.wrapping_mul(1103515245).wrapping_add(12345);
        data.push((x >> 16) as u8);
    }

    // More compressible pattern
    data.extend(vec![b'b'; 100]);

    let encoded = encode_snappy(&data);
    let decoded = decode(&encoded).expect("decode failed");
    assert_eq!(decoded, data);
}
