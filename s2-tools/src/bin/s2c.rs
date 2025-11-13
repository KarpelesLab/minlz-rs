// Copyright 2024 Karpeles Lab Inc.
// S2 compression command-line tool
// Based on klauspost/compress/s2/cmd/s2c

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use minlz::{encode, encode_best, encode_better, ConcurrentWriter, Reader, Writer};
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
    #[arg(long, default_value = "4M")]
    blocksize: String,

    /// Generate Snappy-compatible output
    #[arg(long)]
    snappy: bool,

    /// Compress as a single block (loads into memory)
    #[arg(long)]
    block: bool,

    /// Add seek index (default: true, use --index=false to disable)
    #[arg(long, default_value_t = true, num_args = 0..=1, default_missing_value = "true", action = clap::ArgAction::Set)]
    index: bool,

    /// Pad size to a multiple of this value (e.g., 500, 64K, 256K, 1M, 4M)
    #[arg(long, default_value = "1")]
    pad: String,

    /// Number of concurrent compression threads
    #[arg(long)]
    cpu: Option<usize>,

    /// Verify written files by decompressing them
    #[arg(long)]
    verify: bool,

    /// Run benchmark n times (no output will be written)
    #[arg(long)]
    bench: Option<usize>,

    /// Recompress Snappy or S2 input
    #[arg(long)]
    recomp: bool,
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

    // Parse pad size
    let pad_size = if args.pad != "1" {
        parse_size(&args.pad).context("Invalid pad size")?
    } else {
        1
    };

    // Check for unsupported features
    if args.recomp {
        eprintln!("Warning: --recomp is not yet implemented");
    }

    // Parse block size
    let block_size = parse_size(&args.blocksize).context("Invalid block size")?;

    // Handle benchmark mode
    if let Some(bench_count) = args.bench {
        return run_benchmark(&args, block_size, bench_count);
    }

    // Handle stdin/stdout case
    if args.files.len() == 1 && args.files[0] == "-" {
        return compress_stdio(&args);
    }

    // Compress each file
    for file in &args.files {
        compress_file(file, &args, block_size, pad_size)?;
    }

    Ok(())
}

fn run_benchmark(args: &Args, block_size: usize, iterations: usize) -> Result<()> {
    use std::time::Instant;

    for file_path in &args.files {
        if file_path == "-" {
            anyhow::bail!("Cannot benchmark stdin");
        }

        let input = PathBuf::from(file_path);
        if !input.exists() {
            anyhow::bail!("File not found: {}", file_path);
        }

        // Read file into memory
        let mut file_data = Vec::new();
        File::open(&input)
            .with_context(|| format!("Failed to open file: {}", input.display()))?
            .read_to_end(&mut file_data)?;

        let file_size = file_data.len();

        if args.block {
            // Block mode benchmark
            println!(
                "Benchmarking {} ({} bytes, {} iterations):",
                input.display(),
                file_size,
                iterations
            );

            let start = Instant::now();
            for _ in 0..iterations {
                let _compressed = if args.slower {
                    encode_best(&file_data)
                } else if args.faster {
                    encode(&file_data)
                } else {
                    encode_better(&file_data)
                };
            }
            let elapsed = start.elapsed();

            let avg_time = elapsed.as_secs_f64() / iterations as f64;
            let throughput = file_size as f64 / avg_time / 1024.0 / 1024.0;

            println!(
                "  Average: {:.3}s per iteration ({:.2} MB/s)",
                avg_time, throughput
            );
        } else {
            // Stream mode benchmark
            println!(
                "Benchmarking {} ({} bytes, {} iterations):",
                input.display(),
                file_size,
                iterations
            );

            let start = Instant::now();
            for _ in 0..iterations {
                let mut output = Vec::new();
                let mut s2_writer = Writer::with_block_size(&mut output, block_size);
                s2_writer.write_all(&file_data)?;
                s2_writer.flush()?;
            }
            let elapsed = start.elapsed();

            let avg_time = elapsed.as_secs_f64() / iterations as f64;
            let throughput = file_size as f64 / avg_time / 1024.0 / 1024.0;

            println!(
                "  Average: {:.3}s per iteration ({:.2} MB/s)",
                avg_time, throughput
            );
        }
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

fn compress_file(input_path: &str, args: &Args, block_size: usize, pad_size: usize) -> Result<()> {
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

            compress_stream(
                &mut input_file,
                &mut stdout_lock,
                args,
                block_size,
                pad_size,
                pb.as_ref(),
            )?;
        } else {
            let mut output_file = File::create(&output)
                .with_context(|| format!("Failed to create output file: {}", output.display()))?;

            compress_stream(
                &mut input_file,
                &mut output_file,
                args,
                block_size,
                pad_size,
                pb.as_ref(),
            )?;
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

    // Verify compressed file if requested
    if args.verify && output != Path::new("-") {
        verify_compressed_file(&input, &output)?;
    }

    // Remove source file if requested
    if args.rm && output != Path::new("-") {
        fs::remove_file(&input)
            .with_context(|| format!("Failed to remove source file: {}", input.display()))?;
    }

    Ok(())
}

fn verify_compressed_file(original: &Path, compressed: &Path) -> Result<()> {
    // Read original file
    let mut original_data = Vec::new();
    File::open(original)
        .with_context(|| {
            format!(
                "Failed to open original file for verification: {}",
                original.display()
            )
        })?
        .read_to_end(&mut original_data)?;

    // Decompress compressed file
    let compressed_file = File::open(compressed).with_context(|| {
        format!(
            "Failed to open compressed file for verification: {}",
            compressed.display()
        )
    })?;

    let mut reader = Reader::new(compressed_file);
    let mut decompressed_data = Vec::new();
    reader
        .read_to_end(&mut decompressed_data)
        .with_context(|| {
            format!(
                "Failed to decompress file for verification: {}",
                compressed.display()
            )
        })?;

    // Compare
    if original_data != decompressed_data {
        anyhow::bail!(
            "Verification failed: decompressed data does not match original (original: {} bytes, decompressed: {} bytes)",
            original_data.len(),
            decompressed_data.len()
        );
    }

    Ok(())
}

fn compress_stream<R: Read, W: Write>(
    input: &mut R,
    output: &mut W,
    args: &Args,
    block_size: usize,
    pad_size: usize,
    pb: Option<&ProgressBar>,
) -> Result<()> {
    let buffer_size = 128 * 1024;
    let mut buffer = vec![0u8; buffer_size];

    // Use concurrent compression if --cpu > 1
    if let Some(cpu_count) = args.cpu {
        if cpu_count > 1 {
            // For concurrent writer, we need to handle padding separately
            // since ConcurrentWriter doesn't support padding directly
            if pad_size > 1 {
                // Write to a buffer, then add padding
                let mut temp_output = Vec::new();
                let mut s2_writer =
                    ConcurrentWriter::with_block_size(&mut temp_output, block_size, cpu_count);

                loop {
                    let n = input.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    s2_writer.write_all(&buffer[..n])?;
                    if let Some(pb) = pb {
                        pb.inc(n as u64);
                    }
                }
                s2_writer.flush()?;
                drop(s2_writer);

                // Apply padding if needed
                let padding_needed = calc_padding(temp_output.len(), pad_size);
                if padding_needed > 0 {
                    write_padding(&mut temp_output, padding_needed)?;
                }

                // Apply index if needed
                if args.index {
                    // Note: Index support requires deeper integration with the writer
                    // For now, we skip index when using concurrent compression
                    eprintln!("Warning: --index is not supported with --cpu > 1");
                }

                output.write_all(&temp_output)?;
            } else {
                let mut s2_writer =
                    ConcurrentWriter::with_block_size(output, block_size, cpu_count);

                loop {
                    let n = input.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    s2_writer.write_all(&buffer[..n])?;
                    if let Some(pb) = pb {
                        pb.inc(n as u64);
                    }
                }
                s2_writer.flush()?;

                if args.index {
                    eprintln!("Warning: --index is not supported with --cpu > 1");
                }
            }
            return Ok(());
        }
    }

    // Single-threaded compression
    if pad_size > 1 && args.index {
        // Padding + index: need to use temp buffer
        let mut temp_output = Vec::new();
        let mut s2_writer = Writer::with_index_and_block_size(&mut temp_output, block_size);

        loop {
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            s2_writer.write_all(&buffer[..n])?;
            if let Some(pb) = pb {
                pb.inc(n as u64);
            }
        }
        s2_writer.flush()?;
        drop(s2_writer);

        // Apply padding manually after index
        let padding_needed = calc_padding(temp_output.len(), pad_size);
        if padding_needed > 0 {
            write_padding(&mut temp_output, padding_needed)?;
        }

        output.write_all(&temp_output)?;
    } else if pad_size > 1 {
        // Padding only
        let mut s2_writer = Writer::with_padding(output, pad_size);

        loop {
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            s2_writer.write_all(&buffer[..n])?;
            if let Some(pb) = pb {
                pb.inc(n as u64);
            }
        }
        s2_writer.flush()?;
    } else if args.index {
        // Index only
        let mut s2_writer = Writer::with_index_and_block_size(output, block_size);

        loop {
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            s2_writer.write_all(&buffer[..n])?;
            if let Some(pb) = pb {
                pb.inc(n as u64);
            }
        }
        s2_writer.flush()?;
    } else {
        // No padding, no index
        let mut s2_writer = Writer::with_block_size(output, block_size);

        loop {
            let n = input.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            s2_writer.write_all(&buffer[..n])?;
            if let Some(pb) = pb {
                pb.inc(n as u64);
            }
        }
        s2_writer.flush()?;
    }

    Ok(())
}

fn calc_padding(written: usize, want_multiple: usize) -> usize {
    if want_multiple <= 1 {
        return 0;
    }
    let leftover = written % want_multiple;
    if leftover == 0 {
        return 0;
    }
    want_multiple - leftover
}

fn write_padding<W: Write>(output: &mut W, padding_needed: usize) -> Result<()> {
    if padding_needed == 0 {
        return Ok(());
    }

    const SKIPPABLE_FRAME_HEADER: usize = 4;
    if padding_needed < SKIPPABLE_FRAME_HEADER {
        anyhow::bail!("padding size too small");
    }

    // Write chunk type for padding (0xfe)
    output.write_all(&[0xfe])?;

    // Write chunk length (3 bytes, little-endian)
    let data_len = (padding_needed - SKIPPABLE_FRAME_HEADER) as u32;
    output.write_all(&[
        (data_len & 0xff) as u8,
        ((data_len >> 8) & 0xff) as u8,
        ((data_len >> 16) & 0xff) as u8,
    ])?;

    // Write padding data (zeros or pattern)
    let pattern = vec![0u8; data_len as usize];
    output.write_all(&pattern)?;

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
