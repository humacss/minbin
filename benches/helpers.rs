use rand::{SeedableRng, rngs::SmallRng};
use rand_utf8::rand_utf8;
use std::hint::black_box;

use criterion::{Criterion, BatchSize};
use minbin::{ToFromBytes, write_bytes};

pub(crate) fn deterministic_random_str(byte_len: usize) -> String {
    let mut rng = SmallRng::seed_from_u64(0);
    rand_utf8(&mut rng, byte_len).to_string()
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct BenchStruct<'a> {
    pub uuid: u128,
    pub counter: u32,
    pub reading: &'a str,
    pub timestamp: i64,
}

impl<'a> minbin::ToFromBytes<'a> for BenchStruct<'a> {
    fn to_bytes(&self, writer: &mut minbin::BytesWriter<'a>) -> Result<(), minbin::ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.counter)?;
        writer.write(&self.reading)?;
        writer.write(&self.timestamp)?;
        Ok(())
    }

    fn from_bytes(reader: &mut minbin::BytesReader<'a>) -> Result<(Self, usize), minbin::ToFromByteError> {
        let uuid = reader.read()?;
        let counter = reader.read()?;
        let reading = reader.read()?;
        let timestamp = reader.read()?;
        Ok((BenchStruct { uuid, counter, reading, timestamp }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() +
        self.counter.byte_count() +
        self.reading.byte_count() +
        self.timestamp.byte_count()
    }
}