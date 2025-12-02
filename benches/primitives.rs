use std::hint::black_box;

use criterion::{Criterion};

use simbin::{to_bytes, from_bytes};

pub fn bench_ints(runner: &mut Criterion) {
    macro_rules! bench_int {
        ($name:ident, $int_type:ty, $input:expr) => {
            let name = stringify!($name);

            runner.bench_function(&format!("{}__to_bytes", name), |bencher| {
                bencher.iter(|| to_bytes(black_box(&$input)))
            });

            let input = to_bytes(&$input).unwrap();
            runner.bench_function(&format!("{}__from_bytes", name), |bencher| {
                bencher.iter(|| from_bytes::<$int_type>(black_box(&input)))
            });
        };
    }

    bench_int!(u8,   u8,   42u8);
    bench_int!(i8,   i8,   -42i8);
    bench_int!(u16,  u16,  42u16);
    bench_int!(i16,  i16,  -42i16);
    bench_int!(u32,  u32,  42u32);
    bench_int!(i32,  i32,  -42i32);
    bench_int!(u64,  u64,  42u64);
    bench_int!(i64,  i64,  -42i64);
    bench_int!(u128, u128, 42u128);
}

pub fn bench_bools(runner: &mut Criterion) {
    let input = true;
    runner.bench_function("bool_true__to_bytes", |bencher| bencher.iter(|| to_bytes(black_box(&input))));

    let input = false;
    runner.bench_function("bool_false__to_bytes", |bencher| bencher.iter(|| to_bytes(black_box(&input))));
}
