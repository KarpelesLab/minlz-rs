// Copyright 2024 Karpeles Lab Inc.
// MinLZ compression command-line tool (cf. github.com/minio/minlz cmd/mz).

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use minlz::minlz::{Level, Writer};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LevelArg {
    /// Fastest, lowest ratio.
    Fastest,
    /// Balanced speed and ratio.
    Balanced,
    /// Best ratio, slowest.
    Smallest,
}

impl From<LevelArg> for Level {
    fn from(l: LevelArg) -> Self {
        match l {
            LevelArg::Fastest => Level::Fastest,
            LevelArg::Balanced => Level::Balanced,
            LevelArg::Smallest => Level::Smallest,
        }
    }
}

#[derive(Parser)]
#[command(name = "mzc", about = "MinLZ compression tool", version)]
struct Args {
    /// Input files to compress (use `-` for stdin).
    #[arg(required = true)]
    files: Vec<String>,

    /// Write to stdout instead of a `.mz` file.
    #[arg(short = 'c', long)]
    stdout: bool,

    /// Output file (single input only).
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Compression level.
    #[arg(long, value_enum, default_value_t = LevelArg::Balanced)]
    level: LevelArg,

    /// Append a seek index to the stream.
    #[arg(long)]
    index: bool,

    /// Do not overwrite existing files.
    #[arg(long)]
    safe: bool,

    /// Delete source files after successful compression.
    #[arg(long)]
    rm: bool,
}

fn make_writer<W: Write>(w: W, args: &Args) -> Writer<W> {
    let w = Writer::with_level(w, args.level.into());
    if args.index {
        w.with_index()
    } else {
        w
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.files.len() > 1 && (args.output.is_some() || args.stdout) {
        anyhow::bail!("-o/-c require a single input file");
    }

    for file in &args.files {
        if file == "-" {
            let stdin = io::stdin();
            let stdout = io::stdout();
            let mut w = make_writer(stdout.lock(), &args);
            io::copy(&mut stdin.lock(), &mut w)?;
            w.finish()?.flush()?;
            continue;
        }
        compress_file(file, &args)?;
    }
    Ok(())
}

fn compress_file(input_path: &str, args: &Args) -> Result<()> {
    let input = PathBuf::from(input_path);
    if !input.is_file() {
        anyhow::bail!("not a file: {input_path}");
    }

    let output = if let Some(out) = &args.output {
        out.clone()
    } else if args.stdout {
        PathBuf::from("-")
    } else {
        let name = input.file_name().and_then(|s| s.to_str()).unwrap_or("");
        input.with_file_name(format!("{name}.mz"))
    };

    if args.safe && output != Path::new("-") && output.exists() {
        anyhow::bail!("output already exists: {}", output.display());
    }

    let mut in_file = File::open(&input).with_context(|| format!("open {}", input.display()))?;

    if output == Path::new("-") {
        let stdout = io::stdout();
        let mut w = make_writer(stdout.lock(), args);
        io::copy(&mut in_file, &mut w)?;
        w.finish()?.flush()?;
    } else {
        let out_file =
            File::create(&output).with_context(|| format!("create {}", output.display()))?;
        let mut w = make_writer(BufWriter::new(out_file), args);
        io::copy(&mut in_file, &mut w)?;
        w.finish()?.flush()?;
    }

    if args.rm && output != Path::new("-") {
        fs::remove_file(&input).with_context(|| format!("remove {}", input.display()))?;
    }
    Ok(())
}
