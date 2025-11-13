use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use minlz::{decode, encode, encode_best, encode_better};

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

criterion_group!(
    benches,
    bench_encode_standard,
    bench_encode_better,
    bench_encode_best,
    bench_decode,
    bench_roundtrip
);
criterion_main!(benches);
