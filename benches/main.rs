mod containers;
mod primitives;
mod structs;
mod tuples;


use criterion::{criterion_main, criterion_group};

criterion_group!(
    benches,
    
    containers::bench_strings,
    containers::bench_vecs,
    containers::bench_options,

    primitives::bench_ints,
    primitives::bench_bools,

    structs::bench_structs,

    tuples::bench_tuples,
);

criterion_main!(benches);
