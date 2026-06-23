// Copyright 2024 Karpeles Lab Inc.
// Integration tests for the mzc/mzd CLI tools.

use std::io::Write;
use std::process::{Command, Stdio};

fn pipe(exe: &str, args: &[&str], input: &[u8]) -> (Vec<u8>, bool) {
    let mut child = Command::new(exe)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input)
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait");
    (out.stdout, out.status.success())
}

fn mzc(args: &[&str], input: &[u8]) -> (Vec<u8>, bool) {
    pipe(env!("CARGO_BIN_EXE_mzc"), args, input)
}
fn mzd(args: &[&str], input: &[u8]) -> (Vec<u8>, bool) {
    pipe(env!("CARGO_BIN_EXE_mzd"), args, input)
}

#[test]
fn stdio_roundtrip_all_levels() {
    let data: Vec<u8> = (0..300_000u32).map(|i| (i / 11) as u8).collect();
    for level in ["fastest", "balanced", "smallest"] {
        let (comp, ok) = mzc(&["-c", "--level", level, "-"], &data);
        assert!(ok, "mzc failed at {level}");
        assert!(comp.len() < data.len(), "{level} did not compress");
        let (back, ok) = mzd(&["-c", "-"], &comp);
        assert!(ok, "mzd failed at {level}");
        assert_eq!(back, data, "roundtrip mismatch at {level}");
    }
}

#[test]
fn incompressible_roundtrip() {
    let data: Vec<u8> = (0..50_000u32)
        .map(|i| (i.wrapping_mul(2654435761) >> 13) as u8)
        .collect();
    let (comp, ok) = mzc(&["-c", "-"], &data);
    assert!(ok);
    let (back, ok) = mzd(&["-c", "-"], &comp);
    assert!(ok);
    assert_eq!(back, data);
}
