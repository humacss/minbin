use std::hint::black_box;

use criterion::{Criterion};

use simbin::{to_bytes, from_bytes, ToFromBytes, ToFromByteError};

#[derive(Debug, PartialEq, Clone)]
pub struct Packet {
    pub magic: u32,
    pub seq: u64,
    pub kind: u8,
    pub flags: u16,
    pub name: String,
    pub payload: Vec<u8>,
    pub checksum: u32,
}

impl ToFromBytes for Packet {
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        let mut bytes = Vec::new();

        bytes.extend(to_bytes(&self.magic)?);
        bytes.extend(to_bytes(&self.seq)?);
        bytes.extend(to_bytes(&self.kind)?);
        bytes.extend(to_bytes(&self.flags)?);
        bytes.extend(to_bytes(&self.name)?);
        bytes.extend(to_bytes(&self.payload)?);
        bytes.extend(to_bytes(&self.checksum)?);

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        let (magic, bytes) = from_bytes(bytes)?;
        let (seq, bytes) = from_bytes(bytes)?;
        let (kind, bytes) = from_bytes(bytes)?;
        let (flags, bytes) = from_bytes(bytes)?;
        let (name, bytes) = from_bytes(bytes)?;
        let (payload, bytes) = from_bytes(bytes)?;
        let (checksum, bytes) = from_bytes(bytes)?;

        Ok((Packet { magic, seq, kind, flags, name, payload, checksum }, bytes))
    }
}

pub fn bench_structs(runner: &mut Criterion) {
    let packet = Packet {
        magic: 0xDEADBEEF,
        seq: 123456789,
        kind: 1,
        flags: 0x00FF,
        name: "ping".to_string(),
        payload: vec![0xAA; 64], // change this to use different values for each item
        checksum: 0x12345678,
    };

    let input = packet.clone();
    runner.bench_function("struct__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });

    let input = to_bytes(&input).unwrap();
    runner.bench_function("struct__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Packet>(black_box(&input)))
    });

    let input = Packet { payload: vec![0x55; 10_000],  ..packet.clone() }; // change this to use different values for each item
    runner.bench_function("struct_10kb__to_bytes", |bencher| {
        bencher.iter(|| to_bytes(black_box(&input)))
    });

    let input = to_bytes(&input).unwrap();
    runner.bench_function("struct_10kb__from_bytes", |bencher| {
        bencher.iter(|| from_bytes::<Packet>(black_box(&input)))
    });
}