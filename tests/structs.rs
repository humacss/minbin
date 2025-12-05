use simbin::{BytesWriter, BytesReader, ToFromBytes, ToFromByteError, write_bytes, read_bytes};

struct ExampleStruct<'a> {
    uuid: u128,
    timestamp: i64,
    name: &'a str,
    reading: u16,
}

impl<'a> ToFromBytes<'a> for ExampleStruct<'a> {
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.timestamp)?;
        writer.write(&self.name)?;
        writer.write(&self.reading)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let uuid      = reader.read()?;
        let timestamp = reader.read()?;
        let name      = reader.read()?;
        let reading   = reader.read()?;

        Ok((ExampleStruct { uuid, timestamp, name, reading }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() + 
        self.timestamp.byte_count() +
        self.name.byte_count() +
        self.reading.byte_count()
    }
}

#[test]
fn test_struct_stack() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example", reading: 2 };

    let mut buffer = [0u8; 1024];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected.uuid,           actual.uuid);
    assert_eq!(expected.timestamp,      actual.timestamp);
    assert_eq!(expected.name,           actual.name);
    assert_eq!(expected.reading,        actual.reading);
}

#[test]
fn test_struct_heap() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example", reading: 2 };

    let mut buffer = vec![0u8; expected.byte_count()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected.uuid,           actual.uuid);
    assert_eq!(expected.timestamp,      actual.timestamp);
    assert_eq!(expected.name,           actual.name);
    assert_eq!(expected.reading,        actual.reading);
}

