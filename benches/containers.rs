use std::hint::black_box;

use criterion::{Criterion};

use simbin::{to_bytes, from_bytes};

pub fn bench_strings(runner: &mut Criterion) {
    let input = "a".repeat(100); // change this to use different characters for each item
    runner.bench_function("string__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });
    
    let input = to_bytes(&input).unwrap();
    runner.bench_function("string__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<String>(black_box(&input)))
    });

    let input = "a".repeat(10_000); // change this to use different characters for each item
    runner.bench_function("string_10kb__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });
    
    let input = to_bytes(&input).unwrap();
    runner.bench_function("string_10kb__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<String>(black_box(&input)))
    });
}

pub fn bench_vecs(runner:  &mut Criterion) {
    let input: Vec<u8> = (0..100).collect();    
    runner.bench_function("vec__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });
    
    let input = to_bytes(&input).unwrap();
    runner.bench_function("vec__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Vec<u32>>(black_box(&input)))
    });

    let input: Vec<u16> = (0..5_000).collect();    
    runner.bench_function("vec_10kb__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });
    
    let input = to_bytes(&input).unwrap();
    runner.bench_function("vec_10kb__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Vec<u8>>(black_box(&input)))
    });
}

pub fn bench_options(runner: &mut Criterion){
    let input = Some(42u64);
    runner.bench_function("some__to_bytes", |bencher| bencher.iter(|| to_bytes(black_box(&input))));

    let input = to_bytes(&input).unwrap();
    runner.bench_function("some__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Option<u64>>(black_box(&input)))
    });

    let input: Option<u64> = None;
    runner.bench_function("none__to_bytes", |bencher| bencher.iter(|| to_bytes(black_box(&input))));

    let input = to_bytes(&input).unwrap();
    runner.bench_function("none__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Option<u64>>(black_box(&input)))
    });
}
