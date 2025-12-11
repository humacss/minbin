use criterion::{criterion_group, criterion_main, Criterion};

mod helpers;
use helpers::{bench_value, BenchStruct};

const STR_100B_1B_CHARS: &str = "SYS:2025-12-11T11:11:11.123Z|temp=23.4C|hum=56%|load=1.23|uptime=123456s";
const STR_100B_4B_CHARS: &str = "🚀🌟💜🔥🌈✨🎉🐙🦄🌺🍕🍉🎸🏄‍♂️🤖🎨🌍⚡🧠❤️🚴‍♀️🌙🍒🐳🦋";

pub struct StaticString(pub &'static str);

fn all_benches(runner: &mut Criterion) {    
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
    bench_value(runner, "String_100b_1b_chars", STR_100B_1B_CHARS.to_string());
    bench_value(runner, "String_100b_4b_chars", STR_100B_4B_CHARS.to_string());
    bench_value(runner, "Vec_100b_u32", (0u32..25).collect::<Vec<u32>>());
    bench_value(runner, "struct, 200b", BenchStruct{ 
        uuid:       0u128,                          // 16 bytes
        timestamp:  0i64,                           // 8 bytes
        name:       STR_100B_4B_CHARS.to_string(),  // 100 bytes
        readings:   (0u32..19).collect()            // 76 bytes
    } );
}

criterion_group!(
    benches,
    all_benches,
    
);
criterion_main!(benches);
