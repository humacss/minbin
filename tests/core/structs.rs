use minbin::{BytesWriter, BytesReader, ToFromBytes, ToFromByteError, write_bytes, read_bytes};

struct ExampleStruct<'a> {
    uuid: u128,
    timestamp: i64,
    name: &'a str,
    reading: u16,
}

impl<'a> ToFromBytes<'a> for ExampleStruct<'a> {
    const MAX_BYTES: usize = 1_000_000;

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&(self.uuid, self.timestamp, self.name, self.reading))?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let (uuid, timestamp, name, reading) = reader.read()?;

        Ok((ExampleStruct { uuid, timestamp, name, reading }, reader.pos))
    }

    fn byte_count(&self) -> Result<usize, ToFromByteError> {
        let byte_count = self.uuid.byte_count()? + self.timestamp.byte_count()? + 
        self.name.byte_count()? + self.reading.byte_count()?;

        if byte_count > Self::MAX_BYTES {
            return Err(ToFromByteError::MaxBytesExceeded);
        }

        Ok(byte_count)
    }
}

#[test]
fn test_struct_stack() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example", reading: 2 };

    let mut buffer = [0u8; 1024];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    
    assert_eq!(expected.byte_count().unwrap(),   write_pos);

    let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer).unwrap();

    assert_eq!(expected.byte_count().unwrap(),  read_pos);
    assert_eq!(expected.uuid,                   actual.uuid);
    assert_eq!(expected.timestamp,              actual.timestamp);
    assert_eq!(expected.name,                   actual.name);
    assert_eq!(expected.reading,                actual.reading);
}

#[test]
fn test_struct_heap() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example", reading: 2 };

    let mut buffer = vec![0u8; expected.byte_count().unwrap()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    
    assert_eq!(expected.byte_count().unwrap(),   write_pos);

    let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer).unwrap();

    assert_eq!(expected.byte_count().unwrap(),  read_pos);
    assert_eq!(expected.uuid,                   actual.uuid);
    assert_eq!(expected.timestamp,              actual.timestamp);
    assert_eq!(expected.name,                   actual.name);
    assert_eq!(expected.reading,                actual.reading);
}

