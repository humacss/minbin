use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../support.rs"]
mod support;
use support::bench_value;

mod struct_value;
use struct_value::BenchStruct;

const STR_100B_1B_CHARS: &str = "SYS:2025-12-11T11:11:11.123Z|temp=23.4C|hum=56%|load=1.23|uptime=123456s";
const STR_100B_4B_CHARS: &str = "🚀🌟💜🔥🌈✨🎉🐙🦄🌺🍕🍉🎸🏄‍♂️🤖🎨🌍⚡🧠❤️🚴‍♀️🌙🍒🐳🦋";

fn alloc_benches(runner: &mut Criterion) {
    bench_value(runner, "String_100b_1b_chars", STR_100B_1B_CHARS.to_string());
    bench_value(runner, "String_100b_4b_chars", STR_100B_4B_CHARS.to_string());
    bench_value(runner, "Vec_100b_u32", (0u32..25).collect::<Vec<u32>>());
    bench_value(
        runner,
        "struct, 200b",
        BenchStruct {
            uuid: 0u128,                         // 16 bytes
            timestamp: 0i64,                     // 8 bytes
            name: STR_100B_4B_CHARS.to_string(), // 118 UTF-8 bytes
            readings: (0u32..19).collect(),      // 76 bytes
        },
    );
}

criterion_group!(benches, alloc_benches);
criterion_main!(benches);
