use criterion::{criterion_group, criterion_main, Criterion};
use mathematx::hashing::murmur3::{
    murmur3_x32::{
        murmur3_x32_16byte_chunks, murmur3_x32_32byte_chunks, murmur3_x32_4byte_chunks,
        murmur3_x32_8byte_chunks,
    },
    murmur3_x64::{
        murmur3_x64_16byte_chunks, murmur3_x64_32byte_chunks, murmur3_x64_64byte_chunks,
        murmur3_x64_8byte_chunks,
    },
};

const SMALL_INPUT: &[u8] = b"Hello World!";

fn bench_murmur3_x32_small_input_hashing(c: &mut Criterion) {
    c.bench_function("Murmur3 x32 - 4 Byte - Small Input", |b| {
        b.iter(|| murmur3_x32_4byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x32 - 8 Byte - Small Input", |b| {
        b.iter(|| murmur3_x32_8byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x32 - 16 Byte - Small Input", |b| {
        b.iter(|| murmur3_x32_16byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x32 - 32 Byte - Small Input", |b| {
        b.iter(|| murmur3_x32_32byte_chunks(SMALL_INPUT))
    });
}

fn bench_murmur3_x64_small_input_hashing(c: &mut Criterion) {
    c.bench_function("Murmur3 x64 - 8 Byte - Small input", |b| {
        b.iter(|| murmur3_x64_8byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x64 - 16 Byte - Small input", |b| {
        b.iter(|| murmur3_x64_16byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x64 - 32 Byte - Small input", |b| {
        b.iter(|| murmur3_x64_32byte_chunks(SMALL_INPUT))
    });

    c.bench_function("Murmur3 x64 - 64 Byte - Small input", |b| {
        b.iter(|| murmur3_x64_64byte_chunks(SMALL_INPUT))
    });
}

fn bench_murmur3_x32_large_input_hashing(c: &mut Criterion) {
    let file_path = "src/hashing/.utils/large_test_string_for_hashing.txt";
    let data = std::fs::read(file_path).expect("Unable to read large dataset file");

    c.bench_function("Murmur3 x32 - 4 Byte - Large Input", |b| {
        b.iter(|| murmur3_x32_4byte_chunks(&data));
    });

    c.bench_function("Murmur3 x32 - 8 Byte - Large Input", |b| {
        b.iter(|| murmur3_x32_8byte_chunks(&data));
    });

    c.bench_function("Murmur3 x32 - 16 Byte - Large Input", |b| {
        b.iter(|| murmur3_x32_16byte_chunks(&data));
    });

    c.bench_function("Murmur3 x32 - 32 Byte - Large Input", |b| {
        b.iter(|| murmur3_x32_32byte_chunks(&data));
    });
}

fn bench_murmur3_x64_large_input_hashing(c: &mut Criterion) {
    let file_path = "src/hashing/.utils/large_test_string_for_hashing.txt";
    let data = std::fs::read(file_path).expect("Unable to read large dataset file");

    c.bench_function("Murmur3 x64 - 8 Byte - Large Input", |b| {
        b.iter(|| murmur3_x64_8byte_chunks(&data));
    });

    c.bench_function("Murmur3 x64 - 16 Byte - Large Input", |b| {
        b.iter(|| murmur3_x64_16byte_chunks(&data));
    });

    c.bench_function("Murmur3 x64 - 32 Byte - Large Input", |b| {
        b.iter(|| murmur3_x64_32byte_chunks(&data));
    });

    c.bench_function("Murmur3 x64 - 64 Byte - Large Input", |b| {
        b.iter(|| murmur3_x64_64byte_chunks(&data));
    });
}

// TODO: Concurrency Test?

criterion_group!(
    benches,
    bench_murmur3_x32_small_input_hashing,
    bench_murmur3_x64_small_input_hashing,
    bench_murmur3_x32_large_input_hashing,
    bench_murmur3_x64_large_input_hashing,
);
criterion_main!(benches);
