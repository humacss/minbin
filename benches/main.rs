// Get rid of the macro
macro_rules! bencher {
    ($c:expr, $ty:ty, $name:expr, $cases:expr) => {{
        use std::hint::black_box;
        use simbin::{ToFromBytes, BytesWriter, BytesReader};

        let mut group = $c.benchmark_group($name);

        for (i, case) in $cases.iter().enumerate() {
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

use rand::{SeedableRng};
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use simbin::{ToFromBytes, ToFromByteError, BytesWriter, BytesReader};

fn deterministic_random_str(byte_len: usize) -> String {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
    let my_str = rand_utf8::rand_utf8(&mut rng, byte_len);

    my_str.to_string()
}

type BenchTuple<'a> = (u32, &'a str, Option<u64>, u16, bool);

#[derive(Debug, PartialEq)]
struct BenchStruct<'a> {
    uuid: u128, // 16 bytes
    counter: u32, // 4 bytes
    reading: &'a str,
    timestamp: i64 // 8 bytes
}

impl<'a> ToFromBytes<'a> for BenchStruct<'a> {
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.counter)?;
        writer.write(&self.reading)?;
        writer.write(&self.timestamp)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let uuid = reader.read()?;
        let counter = reader.read()?;
        let reading = reader.read()?;
        let timestamp = reader.read()?;

        Ok((BenchStruct { uuid, counter, reading, timestamp }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() + self.counter.byte_count() + self.reading.byte_count() + self.timestamp.byte_count()
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
    bencher!(runner, &str, "str_empty", &[deterministic_random_str(0).as_str()]);
    bencher!(runner, &str, "str_50b", &[deterministic_random_str(50).as_str()]);
    bencher!(runner, &str, "str_100b", &[deterministic_random_str(100).as_str()]);
    bencher!(runner, &str, "str_300b", &[deterministic_random_str(300).as_str()]);

    bencher!(runner, Option<u32>, "option_none", &[None]);
    bencher!(runner, Option<u32>, "option_some", &[Some(0), Some(42), Some(u32::MAX)]);
}

pub fn bench_structs(runner: &mut Criterion) {
    bencher!(runner, BenchStruct, "struct_78b", &[
        BenchStruct { uuid: 1,  counter: 2, timestamp: 3, reading: deterministic_random_str(50).as_str() },
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