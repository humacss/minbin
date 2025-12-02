use std::hint::black_box;

use criterion::{Criterion};

use simbin::{to_bytes, from_bytes};

type BenchTuple = (
    u32,
    String,
    Option<u64>,
    Vec<i32>,
    bool,
    u16,
    (u8, u8, u8, String),
);

pub fn bench_tuples(runner: &mut Criterion) {
    let tuple = (
        42u32,
        "some benchmark string".to_string(),
        Some(999999999u64),
        vec![-1, 2, -3, 4, -5],
        true,
        65535u16,
        (1u8, 2, 3, "another benchmark string".to_string()),
    );

    let input = tuple.clone();
    runner.bench_function("tuple__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });

    let input = to_bytes(&input).unwrap();
    runner.bench_function("tuple__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<BenchTuple>(black_box(&input)))
    });
}
