#![no_main]

use libfuzzer_sys::fuzz_target;
use minlz::decode;

fuzz_target!(|data: &[u8]| {
    // Try to decode arbitrary data - should never panic
    // Either succeeds or returns an error
    let _ = decode(data);
});
