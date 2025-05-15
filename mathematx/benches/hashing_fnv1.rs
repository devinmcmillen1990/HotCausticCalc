use criterion::{criterion_group, criterion_main, Criterion};
use mathematx::hashing::fowler_noll_vo_1::{
    fowler_noll_vo_1_x32::{
        fnv1_x32_16byte_chunks, fnv1_x32_32byte_chunks, fnv1_x32_4byte_chunks,
        fnv1_x32_8byte_chunks,
    },
    fowler_noll_vo_1_x64::{
        fnv1_x64_16byte_chunks, fnv1_x64_32byte_chunks, fnv1_x64_64byte_chunks,
        fnv1_x64_8byte_chunks,
    },
};

const SMALL_INPUT: &[u8] = b"Hello World!";

fn bench_fnv1_x32_small_input_hashing(c: &mut Criterion) {
    c.bench_function("FNV-1 x32 - 4 Byte - Small Input", |b| {
        b.iter(|| fnv1_x32_4byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FNV-1 x32 - 8 Byte - Small Input", |b| {
        b.iter(|| fnv1_x32_8byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FMV-1 x32 - 16 Byte - Small Input", |b| {
        b.iter(|| fnv1_x32_16byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FNV-1 x32 - 32 Byte - Small Input", |b| {
        b.iter(|| fnv1_x32_32byte_chunks(SMALL_INPUT))
    });
}

fn bench_fnv1_x64_small_input_hashing(c: &mut Criterion) {
    c.bench_function("FNV-1 x64 - 8 Byte - Small input", |b| {
        b.iter(|| fnv1_x64_8byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FNV-1 x64 - 16 Byte - Small input", |b| {
        b.iter(|| fnv1_x64_16byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FNV-1 x64 - 32 Byte - Small input", |b| {
        b.iter(|| fnv1_x64_32byte_chunks(SMALL_INPUT))
    });

    c.bench_function("FNV-1 x64 - 64 Byte - Small input", |b| {
        b.iter(|| fnv1_x64_64byte_chunks(SMALL_INPUT))
    });
}

// TODO: Concurrency Test?

criterion_group!(
    benches,
    bench_fnv1_x32_small_input_hashing,
    bench_fnv1_x64_small_input_hashing
);
criterion_main!(benches);
