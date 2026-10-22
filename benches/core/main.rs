use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../support.rs"]
mod support;
use support::bench_value;

fn core_benches(runner: &mut Criterion) {
    bench_value(runner, "u8", 42u8);
    bench_value(runner, "i8", 42i8);
    bench_value(runner, "u16", 42u16);
    bench_value(runner, "i16", 42i16);
    bench_value(runner, "u32", 42u32);
    bench_value(runner, "i32", 42i32);
    bench_value(runner, "u64", 42u64);
    bench_value(runner, "i64", 42i64);
    bench_value(runner, "u128", 42u128);
    bench_value(runner, "i128", 42i128);
    bench_value(runner, "bool_true", true);
    bench_value(runner, "bool_false", false);
}

criterion_group!(benches, core_benches);
criterion_main!(benches);
