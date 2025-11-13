// Copyright 2024 Karpeles Lab Inc.
// S2 compression command-line tool
// Based on klauspost/compress/s2/cmd/s2c

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use minlz::{encode, encode_best, encode_better, Writer};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "s2c")]
#[command(about = "S2 compression tool", long_about = None)]
#[command(version)]
struct Args {
    /// Input files to compress
    #[arg(required = true)]
    files: Vec<String>,

    /// Write output to stdout (use with single file or -)
    #[arg(short = 'c', long)]
    stdout: bool,

    /// Output file (use with single input file)
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Compress faster with slightly less compression
    #[arg(long)]
    faster: bool,

    /// Compress slower but achieve better compression
    #[arg(long)]
    slower: bool,

    /// Do not overwrite existing files
    #[arg(long)]
    safe: bool,

    /// Delete source files after successful compression
    #[arg(long)]
    rm: bool,

    /// Quiet mode - don't print progress
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Block size (e.g., 64K, 256K, 1M, 4M)
    #[arg(long, default_value = "1M")]
    blocksize: String,

    /// Generate Snappy-compatible output
    #[arg(long)]
    snappy: bool,

    /// Compress as a single block (loads into memory)
    #[arg(long)]
    block: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate arguments
    if args.faster && args.slower {
        anyhow::bail!("Cannot use both --faster and --slower");
    }

    if args.files.len() > 1 && args.output.is_some() {
        anyhow::bail!("Cannot use -o with multiple input files");
    }

    if args.files.len() > 1 && args.stdout {
        anyhow::bail!("Cannot use -c with multiple input files");
    }

    // Parse block size
    let block_size = parse_size(&args.blocksize).context("Invalid block size")?;

    // Handle stdin/stdout case
    if args.files.len() == 1 && args.files[0] == "-" {
        return compress_stdio(&args);
    }

    // Compress each file
    for file in &args.files {
        compress_file(file, &args, block_size)?;
    }

    Ok(())
}

fn compress_stdio(args: &Args) -> Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if args.block {
        // Block mode: read all into memory and compress
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let compressed = if args.slower {
            encode_best(&data)
        } else if args.faster {
            encode(&data)
        } else {
            encode_better(&data)
        };

        writer.write_all(&compressed)?;
    } else {
        // Stream mode
        let block_size = parse_size(&args.blocksize)?;
        let mut s2_writer = Writer::with_block_size(&mut writer, block_size);
        io::copy(&mut reader, &mut s2_writer)?;
        s2_writer.flush()?;
    }

    Ok(())
}

fn compress_file(input_path: &str, args: &Args, block_size: usize) -> Result<()> {
    let input = PathBuf::from(input_path);

    if !input.exists() {
        anyhow::bail!("File not found: {}", input_path);
    }

    if !input.is_file() {
        anyhow::bail!("Not a file: {}", input_path);
    }

    // Determine output path
    let output = if let Some(ref out) = args.output {
        out.clone()
    } else if args.stdout {
        PathBuf::from("-")
    } else {
        let ext = if args.snappy { ".sz" } else { ".s2" };
        input.with_extension(format!(
            "{}{}",
            input.extension().and_then(|s| s.to_str()).unwrap_or(""),
            ext
        ))
    };

    // Check if output exists in safe mode
    if args.safe && output != Path::new("-") && output.exists() {
        anyhow::bail!("Output file already exists: {}", output.display());
    }

    // Get file size for progress bar
    let file_size = fs::metadata(&input)?.len();

    let pb = if !args.quiet && !args.stdout {
        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec})")?
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    // Open input file
    let mut input_file = File::open(&input)
        .with_context(|| format!("Failed to open input file: {}", input.display()))?;

    if args.block {
        // Block mode: read all into memory
        let mut data = Vec::new();
        input_file.read_to_end(&mut data)?;

        if let Some(ref pb) = pb {
            pb.set_position(file_size);
        }

        let compressed = if args.slower {
            encode_best(&data)
        } else if args.faster {
            encode(&data)
        } else {
            encode_better(&data)
        };

        if output == Path::new("-") {
            io::stdout().write_all(&compressed)?;
        } else {
            let mut output_file = File::create(&output)
                .with_context(|| format!("Failed to create output file: {}", output.display()))?;
            output_file.write_all(&compressed)?;
        }
    } else {
        // Stream mode
        if output == Path::new("-") {
            let stdout = io::stdout();
            let mut stdout_lock = stdout.lock();
            let mut s2_writer = Writer::with_block_size(&mut stdout_lock, block_size);

            let mut buffer = vec![0u8; 128 * 1024];
            loop {
                let n = input_file.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                s2_writer.write_all(&buffer[..n])?;
                if let Some(ref pb) = pb {
                    pb.inc(n as u64);
                }
            }
            s2_writer.flush()?;
        } else {
            let output_file = File::create(&output)
                .with_context(|| format!("Failed to create output file: {}", output.display()))?;
            let mut s2_writer = Writer::with_block_size(output_file, block_size);

            let mut buffer = vec![0u8; 128 * 1024];
            loop {
                let n = input_file.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                s2_writer.write_all(&buffer[..n])?;
                if let Some(ref pb) = pb {
                    pb.inc(n as u64);
                }
            }
            s2_writer.flush()?;
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Done");
    }

    // Print compression stats
    if !args.quiet && !args.stdout {
        let output_size = if output != Path::new("-") {
            fs::metadata(&output)?.len()
        } else {
            0
        };

        if output_size > 0 {
            let ratio = (output_size as f64 / file_size as f64) * 100.0;
            println!(
                "{} -> {} ({:.2}%)",
                input.display(),
                output.display(),
                ratio
            );
        }
    }

    // Remove source file if requested
    if args.rm && output != Path::new("-") {
        fs::remove_file(&input)
            .with_context(|| format!("Failed to remove source file: {}", input.display()))?;
    }

    Ok(())
}

fn parse_size(s: &str) -> Result<usize> {
    let s = s.trim().to_uppercase();

    if let Some(num) = s.strip_suffix('K') {
        Ok(num.parse::<usize>()? * 1024)
    } else if let Some(num) = s.strip_suffix('M') {
        Ok(num.parse::<usize>()? * 1024 * 1024)
    } else if let Some(num) = s.strip_suffix('G') {
        Ok(num.parse::<usize>()? * 1024 * 1024 * 1024)
    } else {
        s.parse::<usize>().context("Invalid size format")
    }
}
