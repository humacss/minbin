use std::hint::black_box;
use criterion::{Criterion, BatchSize};

use minbin::{ToFromBytes, BytesWriter, BytesReader, ToFromByteError, write_bytes, read_bytes};

pub fn bench_value<T>(runner: &mut Criterion, name: &str, value: T)
where T: for<'a> ToFromBytes<'a>
{
    let mut group = runner.benchmark_group(name);

    let size = value.byte_count(); 

    group.bench_function(&format!("{}_serialize", name), |bencher| {
        bencher.iter_batched(
            || vec![0u8; size],
            |mut buffer| {
                write_bytes(black_box(&value), black_box(&mut buffer))
            },
            BatchSize::SmallInput,
        );
    });

    let mut bytes = vec![0u8; size];
    write_bytes(&value, &mut bytes).unwrap();

    group.bench_function(&format!("{}_deserialize", name), |b| {
        b.iter_batched(
            || bytes.clone(),
            |bytes| {
                let (value, pos) = read_bytes::<T>(black_box(&bytes)).unwrap();
                black_box(value);
                black_box(pos);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}



pub struct BenchStruct {
    pub uuid: u128, 
    pub timestamp: i64,
    pub name: String,
    pub readings: Vec<u32>, 
}

impl<'a> ToFromBytes<'a> for BenchStruct {
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.timestamp)?;
        writer.write(&self.name)?;
        writer.write(&self.readings)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let (uuid, timestamp, name, readings) = reader.read()?;

        Ok((BenchStruct { uuid, timestamp, name, readings }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() + self.timestamp.byte_count() + 
        self.name.byte_count() + self.readings.byte_count()
    }
}
