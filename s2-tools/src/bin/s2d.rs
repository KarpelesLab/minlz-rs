// Copyright 2024 Karpeles Lab Inc.
// S2 decompression command-line tool
// Based on klauspost/compress/s2/cmd/s2d

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use minlz::{decode, Reader};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "s2d")]
#[command(about = "S2 decompression tool", long_about = None)]
#[command(version)]
struct Args {
    /// Input files to decompress
    #[arg(required = true)]
    files: Vec<String>,

    /// Write output to stdout (use with single file or -)
    #[arg(short = 'c', long)]
    stdout: bool,

    /// Output file (use with single input file)
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Do not overwrite existing files
    #[arg(long)]
    safe: bool,

    /// Delete source files after successful decompression
    #[arg(long)]
    rm: bool,

    /// Quiet mode - don't print progress
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Verify files only, don't write output
    #[arg(long)]
    verify: bool,

    /// Decompress as a single block (loads into memory)
    #[arg(long)]
    block: bool,

    /// Return last N bytes (e.g., 92, 64K, 256K, 1M, 4M) - requires index
    #[arg(long)]
    tail: Option<String>,

    /// Start at offset (e.g., 92, 64K, 256K, 1M, 4M) - requires index
    #[arg(long)]
    offset: Option<String>,

    /// Number of concurrent decompression threads
    #[arg(long)]
    cpu: Option<usize>,

    /// Run benchmark n times (no output will be written)
    #[arg(long)]
    bench: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate arguments
    if args.files.len() > 1 && args.output.is_some() {
        anyhow::bail!("Cannot use -o with multiple input files");
    }

    if args.files.len() > 1 && args.stdout {
        anyhow::bail!("Cannot use -c with multiple input files");
    }

    if args.tail.is_some() && args.offset.is_some() {
        anyhow::bail!("Cannot use both --tail and --offset");
    }

    // Check for unsupported features
    if args.tail.is_some() {
        eprintln!("Warning: --tail is not yet implemented (requires index support)");
    }
    if args.offset.is_some() {
        eprintln!("Warning: --offset is not yet implemented (requires index support)");
    }
    if args.cpu.is_some() {
        eprintln!("Warning: --cpu is not yet implemented (single-threaded decompression)");
    }

    // Handle benchmark mode
    if let Some(bench_count) = args.bench {
        return run_benchmark(&args, bench_count);
    }

    // Handle stdin/stdout case
    if args.files.len() == 1 && args.files[0] == "-" {
        return decompress_stdio(&args);
    }

    // Decompress each file
    for file in &args.files {
        decompress_file(file, &args)?;
    }

    Ok(())
}

fn run_benchmark(args: &Args, iterations: usize) -> Result<()> {
    use std::time::Instant;

    for file_path in &args.files {
        if file_path == "-" {
            anyhow::bail!("Cannot benchmark stdin");
        }

        let input = PathBuf::from(file_path);
        if !input.exists() {
            anyhow::bail!("File not found: {}", file_path);
        }

        // Read compressed file into memory
        let mut file_data = Vec::new();
        File::open(&input)
            .with_context(|| format!("Failed to open file: {}", input.display()))?
            .read_to_end(&mut file_data)?;

        let compressed_size = file_data.len();

        if args.block {
            // Block mode benchmark
            println!(
                "Benchmarking {} ({} bytes compressed, {} iterations):",
                input.display(),
                compressed_size,
                iterations
            );

            let start = Instant::now();
            let mut decompressed_size = 0;
            for _ in 0..iterations {
                let decompressed = decode(&file_data)?;
                decompressed_size = decompressed.len();
            }
            let elapsed = start.elapsed();

            let avg_time = elapsed.as_secs_f64() / iterations as f64;
            let throughput = decompressed_size as f64 / avg_time / 1024.0 / 1024.0;

            println!(
                "  Average: {:.3}s per iteration ({:.2} MB/s decompressed)",
                avg_time, throughput
            );
        } else {
            // Stream mode benchmark
            println!(
                "Benchmarking {} ({} bytes compressed, {} iterations):",
                input.display(),
                compressed_size,
                iterations
            );

            let start = Instant::now();
            let mut decompressed_size = 0;
            for _ in 0..iterations {
                let mut s2_reader = Reader::new(&file_data[..]);
                let mut output = Vec::new();
                s2_reader.read_to_end(&mut output)?;
                decompressed_size = output.len();
            }
            let elapsed = start.elapsed();

            let avg_time = elapsed.as_secs_f64() / iterations as f64;
            let throughput = decompressed_size as f64 / avg_time / 1024.0 / 1024.0;

            println!(
                "  Average: {:.3}s per iteration ({:.2} MB/s decompressed)",
                avg_time, throughput
            );
        }
    }

    Ok(())
}

fn decompress_stdio(args: &Args) -> Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    if args.verify {
        // Verify mode: just read and discard
        if args.block {
            let mut data = Vec::new();
            reader.read_to_end(&mut data)?;
            let _ = decode(&data)?;
        } else {
            let mut s2_reader = Reader::new(reader);
            io::copy(&mut s2_reader, &mut io::sink())?;
        }
        if !args.quiet {
            eprintln!("Verification successful");
        }
        return Ok(());
    }

    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if args.block {
        // Block mode: read all into memory and decompress
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let decompressed = decode(&data)?;
        writer.write_all(&decompressed)?;
    } else {
        // Stream mode
        let mut s2_reader = Reader::new(reader);
        io::copy(&mut s2_reader, &mut writer)?;
    }

    Ok(())
}

fn decompress_file(input_path: &str, args: &Args) -> Result<()> {
    let input = PathBuf::from(input_path);

    if !input.exists() {
        anyhow::bail!("File not found: {}", input_path);
    }

    if !input.is_file() {
        anyhow::bail!("Not a file: {}", input_path);
    }

    // Determine output path
    let output = if args.verify {
        PathBuf::from("-") // Don't write anything in verify mode
    } else if let Some(ref out) = args.output {
        out.clone()
    } else if args.stdout {
        PathBuf::from("-")
    } else {
        // Remove .s2 or .sz extension
        let path_str = input.to_string_lossy();
        if let Some(stripped) = path_str.strip_suffix(".s2") {
            PathBuf::from(stripped)
        } else if let Some(stripped) = path_str.strip_suffix(".sz") {
            PathBuf::from(stripped)
        } else {
            anyhow::bail!(
                "Input file must have .s2 or .sz extension: {}",
                input.display()
            );
        }
    };

    // Check if output exists in safe mode
    if args.safe && !args.verify && output != Path::new("-") && output.exists() {
        anyhow::bail!("Output file already exists: {}", output.display());
    }

    // Get file size for progress bar
    let file_size = fs::metadata(&input)?.len();

    let pb = if !args.quiet && !args.stdout && !args.verify {
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

        let decompressed = decode(&data).context("Decompression failed")?;

        if args.verify {
            if !args.quiet {
                println!("Verification successful: {}", input.display());
            }
        } else if output == Path::new("-") {
            io::stdout().write_all(&decompressed)?;
        } else {
            let mut output_file = File::create(&output)
                .with_context(|| format!("Failed to create output file: {}", output.display()))?;
            output_file.write_all(&decompressed)?;
        }
    } else {
        // Stream mode
        let mut s2_reader = Reader::new(input_file);

        if args.verify {
            let mut buffer = vec![0u8; 128 * 1024];
            loop {
                let n = s2_reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                if let Some(ref pb) = pb {
                    pb.inc(n as u64);
                }
            }
            if !args.quiet {
                println!("Verification successful: {}", input.display());
            }
        } else if output == Path::new("-") {
            let stdout = io::stdout();
            let mut stdout_lock = stdout.lock();
            let mut buffer = vec![0u8; 128 * 1024];
            loop {
                let n = s2_reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                stdout_lock.write_all(&buffer[..n])?;
                if let Some(ref pb) = pb {
                    pb.inc(n as u64);
                }
            }
        } else {
            let mut output_file = File::create(&output)
                .with_context(|| format!("Failed to create output file: {}", output.display()))?;
            let mut buffer = vec![0u8; 128 * 1024];
            loop {
                let n = s2_reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                output_file.write_all(&buffer[..n])?;
                if let Some(ref pb) = pb {
                    pb.inc(n as u64);
                }
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Done");
    }

    // Print decompression stats
    if !args.quiet && !args.stdout && !args.verify {
        let output_size = if output != Path::new("-") {
            fs::metadata(&output)?.len()
        } else {
            0
        };

        if output_size > 0 {
            let ratio = (file_size as f64 / output_size as f64) * 100.0;
            println!(
                "{} -> {} (compressed to {:.2}%)",
                input.display(),
                output.display(),
                ratio
            );
        }
    }

    // Remove source file if requested
    if args.rm && output != Path::new("-") && !args.verify {
        fs::remove_file(&input)
            .with_context(|| format!("Failed to remove source file: {}", input.display()))?;
    }

    Ok(())
}
