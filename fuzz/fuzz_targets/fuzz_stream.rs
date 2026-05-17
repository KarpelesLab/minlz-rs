#![no_main]

use libfuzzer_sys::fuzz_target;
use minlz::{Reader, Writer};
use std::io::{Read, Write};

fuzz_target!(|data: &[u8]| {
    // Skip very large inputs to keep the fuzzer's memory under the
    // libFuzzer default limit.
    if data.len() > 1_000_000 {
        return;
    }

    // Stream roundtrip: compress through Writer, then decompress through
    // Reader and assert we get the original bytes back. Writer/Reader I/O
    // must not panic; the inner closure is intentionally infallible (any
    // panic will be reported as a fuzz crash).
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        let _ = writer.write_all(data);
        let _ = writer.flush();
    }
    let mut decompressed = Vec::new();
    let mut reader = Reader::new(&compressed[..]);
    let _ = reader.read_to_end(&mut decompressed);
    assert_eq!(
        data, &decompressed[..],
        "stream roundtrip produced different bytes"
    );

    // Independently: reading arbitrary attacker-controlled bytes through
    // the Reader must not panic — it should either decode successfully
    // or return an io::Error.
    let mut reader = Reader::new(data);
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);
});
