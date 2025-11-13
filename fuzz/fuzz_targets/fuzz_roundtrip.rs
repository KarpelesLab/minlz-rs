#![no_main]

use libfuzzer_sys::fuzz_target;
use minlz::{decode, encode, encode_better, encode_best};

fuzz_target!(|data: &[u8]| {
    // Skip very large inputs to avoid OOM
    if data.len() > 1_000_000 {
        return;
    }

    // Test standard compression
    if let Ok(compressed) = std::panic::catch_unwind(|| encode(data)) {
        if let Ok(decompressed) = decode(&compressed) {
            assert_eq!(data, &decompressed[..], "Standard roundtrip failed");
        }
    }

    // Test better compression
    if let Ok(compressed) = std::panic::catch_unwind(|| encode_better(data)) {
        if let Ok(decompressed) = decode(&compressed) {
            assert_eq!(data, &decompressed[..], "Better roundtrip failed");
        }
    }

    // Test best compression
    if let Ok(compressed) = std::panic::catch_unwind(|| encode_best(data)) {
        if let Ok(decompressed) = decode(&compressed) {
            assert_eq!(data, &decompressed[..], "Best roundtrip failed");
        }
    }
});
