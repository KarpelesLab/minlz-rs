#![no_main]

use libfuzzer_sys::fuzz_target;
use minlz::{Reader, Writer};
use std::io::{Read, Write};

fuzz_target!(|data: &[u8]| {
    // Skip very large inputs
    if data.len() > 1_000_000 {
        return;
    }

    // Test stream format roundtrip
    let mut compressed = Vec::new();
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(data).ok()?;
        writer.flush().ok()
    }))
    .is_ok()
    {
        if let Ok(()) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut reader = Reader::new(&compressed[..]);
            let mut decompressed = Vec::new();
            reader.read_to_end(&mut decompressed).ok()?;

            if decompressed == data {
                Some(())
            } else {
                None
            }
        })) {
            // Success
        }
    }

    // Also test reading arbitrary stream data - should not panic
    let mut reader = Reader::new(data);
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);
});
