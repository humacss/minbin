use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../support.rs"]
mod support;
use support::bench_value;

#[derive(Default)]
struct Packet {
    id: u32,
    timestamp: u64,
}
minbin::minbin_struct! { Packet [self.id: u32, self.timestamp: u64] }

#[derive(Default)]
enum Event {
    #[default]
    Ping,
    Reading(u32),
}
minbin::minbin_enum! { Event [
    [0 => Self::Ping],
    [1 => Self::Reading(value: u32)],
] }

fn macro_benches(runner: &mut Criterion) {
    bench_value(runner, "macro_struct", Packet { id: 42, timestamp: 123 });
    bench_value(runner, "macro_enum", Event::Reading(42));
}

criterion_group!(benches, macro_benches);
criterion_main!(benches);
