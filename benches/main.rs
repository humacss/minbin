macro_rules! bencher {
    ($c:expr, $ty:ty, $name:expr, $cases:expr) => {{
        use std::hint::black_box;
        use simbin::{ToFromBytes, BytesWriter, BytesReader};

        let mut group = $c.benchmark_group($name);

        for (i, case) in $cases.iter().enumerate() {
            // Pre-compute size once
            let size = <$ty as ToFromBytes>::byte_count(case);

            group.bench_function(format!("ser_{i}"), |b| {
                b.iter_batched(
                    || vec![0u8; size],                                    
                    |mut buffer| {
                        let mut writer = BytesWriter::new(&mut buffer);
                        case.to_bytes(&mut writer).unwrap();
                        black_box(buffer)                                  
                    },
                    criterion::BatchSize::SmallInput,
                )
            });

            // Serialize once to get real data for deserialization
            let mut real_buffer = vec![0u8; size];
            {
                let mut writer = BytesWriter::new(&mut real_buffer);
                case.to_bytes(&mut writer).unwrap();
            }

            group.bench_function(format!("de_{i}"), |b| {
                b.iter_batched(
                    || real_buffer.clone(),                                
                    |data| {
                        let mut reader = BytesReader::new(&data);
                        let _ = black_box(<$ty as ToFromBytes>::from_bytes(&mut reader).unwrap());
                    },
                    criterion::BatchSize::SmallInput,
                )
            });
        }

        group.finish();
    }};
}

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use simbin::{ToFromBytes, ToFromByteError, BytesWriter, BytesReader};


type BenchTuple<'a> = (u32, &'a str, Option<u64>, u16, bool);

#[derive(Debug, PartialEq)]
struct BenchStruct<'a> {
    id: u32,
    name: &'a str,
}

impl<'de> ToFromBytes<'de> for BenchStruct<'de> {
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        self.id.to_bytes(writer)?;
        self.name.to_bytes(writer)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, &'de [u8]), ToFromByteError> {
        let (id, _) = u32::from_bytes(reader)?;
        let (name, remainder) = <&str>::from_bytes(reader)?;
        Ok((BenchStruct { id, name }, remainder))
    }

    fn byte_count(&self) -> usize {
        self.id.byte_count() + self.name.byte_count()
    }
}

pub fn bench_primitives(runner: &mut Criterion) {
    bencher!(runner, u8,   "u8",   &[0, 1, 42, 255]);
    bencher!(runner, i8,   "i8",   &[-128, -1, 0, 127]);
    bencher!(runner, u16,  "u16",  &[0, 1234, u16::MAX]);
    bencher!(runner, u32,  "u32",  &[0, 123456, u32::MAX]);
    bencher!(runner, u64,  "u64",  &[0, 1234567890123, u64::MAX]);
    bencher!(runner, bool, "bool", &[false, true]);
}

pub fn bench_containers(runner: &mut Criterion) {
    bencher!(runner, &str, "str_small", &["", "a", "hello", "simbin", "rust"]);
    bencher!(runner, &str, "str_10kb", &["a".repeat(10_000).as_str()]); // should repeat unique characters

    bencher!(runner, Option<u32>, "option_none", &[None]);
    bencher!(runner, Option<u32>, "option_some", &[Some(0), Some(42), Some(u32::MAX)]);
}

pub fn bench_structs(runner: &mut Criterion) {
    bencher!(runner, BenchStruct,"struct", &[
        BenchStruct { id: 1,   name: "ping" },
    ]);
}

pub fn bench_tuples(runner: &mut Criterion) {
    bencher!(runner, BenchTuple, "tuple_mixed", &[
        (0,      "",       None,       0,     false),
        (42,     "hello",  Some(123456789), 65535, true),
        (999999, "longer benchmark string", None, 1, false),
    ]);
}


criterion_group!(
    benches,
    bench_primitives,
    bench_containers,
    bench_structs,
    bench_tuples,
);

criterion_main!(benches);