// Copyright 2024 Karpeles Lab Inc.
// MinLZ decompression command-line tool (cf. github.com/minio/minlz cmd/mz).

use anyhow::{Context, Result};
use clap::Parser;
use minlz::minlz::{seek_decompress, Index, Reader};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "mzd", about = "MinLZ decompression tool", version)]
struct Args {
    /// Input `.mz` files to decompress (use `-` for stdin).
    #[arg(required = true)]
    files: Vec<String>,

    /// Write to stdout instead of stripping the `.mz` suffix.
    #[arg(short = 'c', long)]
    stdout: bool,

    /// Output file (single input only).
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Start decompressing at this uncompressed byte offset (needs an indexed
    /// stream and a regular file input). Implies `-c`.
    #[arg(long)]
    offset: Option<u64>,

    /// Do not overwrite existing files.
    #[arg(long)]
    safe: bool,

    /// Delete source files after successful decompression.
    #[arg(long)]
    rm: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.files.len() > 1 && (args.output.is_some() || args.stdout || args.offset.is_some()) {
        anyhow::bail!("-o/-c/--offset require a single input file");
    }

    for file in &args.files {
        if let Some(off) = args.offset {
            if file == "-" {
                anyhow::bail!("--offset requires a regular file, not stdin");
            }
            let bytes = fs::read(file).with_context(|| format!("read {file}"))?;
            let index = Index::load(&bytes)
                .context("stream has no seek index (compress with `mzc --index`)")?;
            let out = seek_decompress(&bytes, &index, off)?;
            io::stdout().write_all(&out)?;
            continue;
        }

        if file == "-" {
            let stdin = io::stdin();
            let stdout = io::stdout();
            let mut r = Reader::new(stdin.lock());
            let mut w = stdout.lock();
            io::copy(&mut r, &mut w)?;
            continue;
        }
        decompress_file(file, &args)?;
    }
    Ok(())
}

fn decompress_file(input_path: &str, args: &Args) -> Result<()> {
    let input = PathBuf::from(input_path);
    if !input.is_file() {
        anyhow::bail!("not a file: {input_path}");
    }

    let output = if let Some(out) = &args.output {
        out.clone()
    } else if args.stdout {
        PathBuf::from("-")
    } else {
        match input.extension().and_then(|s| s.to_str()) {
            Some("mz") => input.with_extension(""),
            _ => anyhow::bail!("input does not end in .mz; use -o or -c"),
        }
    };

    if args.safe && output != Path::new("-") && output.exists() {
        anyhow::bail!("output already exists: {}", output.display());
    }

    let in_file = File::open(&input).with_context(|| format!("open {}", input.display()))?;
    let mut r = Reader::new(BufReader::new(in_file));

    if output == Path::new("-") {
        let stdout = io::stdout();
        let mut w = stdout.lock();
        io::copy(&mut r, &mut w)?;
    } else {
        let mut out_file =
            File::create(&output).with_context(|| format!("create {}", output.display()))?;
        io::copy(&mut r, &mut out_file)?;
    }

    if args.rm && output != Path::new("-") {
        fs::remove_file(&input).with_context(|| format!("remove {}", input.display()))?;
    }
    Ok(())
}
