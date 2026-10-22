use criterion::{BatchSize, Criterion};
use std::hint::black_box;

use minbin::{read_bytes, write_bytes, ToFromBytes};

pub fn bench_value<T>(runner: &mut Criterion, name: &str, value: T)
where
    T: for<'a> ToFromBytes<'a> + Default,
{
    let mut group = runner.benchmark_group(name);

    let size = value.byte_count();

    group.bench_function(format!("{}_serialize", name), |bencher| {
        bencher.iter_batched(
            || vec![0u8; size],
            |mut buffer| write_bytes(black_box(&value), black_box(&mut buffer)),
            BatchSize::SmallInput,
        );
    });

    let mut bytes = vec![0u8; size];
    write_bytes(&value, &mut bytes).unwrap();

    group.bench_function(format!("{}_deserialize", name), |b| {
        b.iter_batched(
            || bytes.clone(),
            |bytes| {
                let (value, pos) = read_bytes(T::default, black_box(&bytes)).unwrap();
                black_box(value);
                black_box(pos);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}
