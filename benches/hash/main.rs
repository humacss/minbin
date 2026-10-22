use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::{HashMap, HashSet};

#[path = "../support.rs"]
mod support;
use support::bench_value;

fn hash_benches(runner: &mut Criterion) {
    bench_value(runner, "HashMap_u32_u32", (0u32..25).map(|key| (key, key * 2)).collect::<HashMap<_, _>>());
    bench_value(runner, "HashSet_u32", (0u32..25).collect::<HashSet<_>>());
}

criterion_group!(benches, hash_benches);
criterion_main!(benches);
