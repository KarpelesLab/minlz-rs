use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use minlz::{decode, encode, encode_best, encode_better, Encoder};

fn generate_test_data(size: usize, pattern: &str) -> Vec<u8> {
    match pattern {
        "random" => (0..size).map(|i| ((i * 7919) % 256) as u8).collect(),
        "repeated" => vec![b'a'; size],
        "text" => {
            let text = b"The quick brown fox jumps over the lazy dog. ";
            text.iter().cycle().take(size).copied().collect()
        }
        "sequential" => (0..size).map(|i| (i % 256) as u8).collect(),
        _ => vec![0; size],
    }
}

fn bench_encode_standard(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_standard");

    for size in [1024, 10 * 1024, 100 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        for pattern in ["random", "repeated", "text", "sequential"] {
            let data = generate_test_data(size, pattern);
            group.bench_with_input(BenchmarkId::new(pattern, size), &data, |b, data| {
                b.iter(|| encode(black_box(data)));
            });
        }
    }
    group.finish();
}

fn bench_encode_better(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_better");

    for size in [1024, 10 * 1024, 100 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        for pattern in ["random", "repeated", "text"] {
            let data = generate_test_data(size, pattern);
            group.bench_with_input(BenchmarkId::new(pattern, size), &data, |b, data| {
                b.iter(|| encode_better(black_box(data)));
            });
        }
    }
    group.finish();
}

fn bench_encode_best(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_best");

    for size in [1024, 10 * 1024, 100 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        for pattern in ["repeated", "text"] {
            let data = generate_test_data(size, pattern);
            group.bench_with_input(BenchmarkId::new(pattern, size), &data, |b, data| {
                b.iter(|| encode_best(black_box(data)));
            });
        }
    }
    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");

    for size in [1024, 10 * 1024, 100 * 1024] {
        for pattern in ["random", "repeated", "text", "sequential"] {
            let data = generate_test_data(size, pattern);
            let compressed = encode(&data);

            group.throughput(Throughput::Bytes(size as u64));
            group.bench_with_input(
                BenchmarkId::new(pattern, size),
                &compressed,
                |b, compressed| {
                    b.iter(|| decode(black_box(compressed)));
                },
            );
        }
    }
    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    for size in [1024, 10 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        for pattern in ["text", "repeated"] {
            let data = generate_test_data(size, pattern);
            group.bench_with_input(BenchmarkId::new(pattern, size), &data, |b, data| {
                b.iter(|| {
                    let compressed = encode(black_box(data));
                    decode(black_box(&compressed)).unwrap()
                });
            });
        }
    }
    group.finish();
}

fn bench_encoder_reused(c: &mut Criterion) {
    // Compare stateful Encoder against the free function on the same
    // patterns, capturing the buffer-reuse win on hot loops.
    let mut group = c.benchmark_group("encoder_reused");

    for size in [1024, 10 * 1024, 100 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        for pattern in ["random", "text"] {
            let data = generate_test_data(size, pattern);
            let id = BenchmarkId::new(format!("standard/{pattern}"), size);
            group.bench_with_input(id, &data, |b, data| {
                let mut enc = Encoder::new();
                b.iter(|| enc.encode(black_box(data)));
            });

            let id = BenchmarkId::new(format!("better/{pattern}"), size);
            group.bench_with_input(id, &data, |b, data| {
                let mut enc = Encoder::new();
                b.iter(|| enc.encode_better(black_box(data)));
            });
        }

        // Best mode is the headline beneficiary because its hash tables
        // are 4.5 MiB and reuse skips the cold-cache memset.
        for pattern in ["repeated", "text"] {
            let data = generate_test_data(size, pattern);
            let id = BenchmarkId::new(format!("best/{pattern}"), size);
            group.bench_with_input(id, &data, |b, data| {
                let mut enc = Encoder::new();
                b.iter(|| enc.encode_best(black_box(data)));
            });
        }
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_encode_standard,
    bench_encode_better,
    bench_encode_best,
    bench_decode,
    bench_roundtrip,
    bench_encoder_reused,
);
criterion_main!(benches);
