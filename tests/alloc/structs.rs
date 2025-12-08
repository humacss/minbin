use simbin::{BytesWriter, BytesReader, ToFromBytes, ToFromByteError, write_bytes, read_bytes};

struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<u16>,
}

impl<'a> ToFromBytes<'a> for ExampleStruct {
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.timestamp)?;
        writer.write(&self.name)?;
        writer.write(&self.readings)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let (uuid, timestamp, name, readings) = reader.read()?;

        Ok((ExampleStruct { uuid, timestamp, name, readings }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        (self.uuid, self.timestamp, self.name.clone(), self.readings.clone()).byte_count()
    }
}

#[test]
fn test_struct() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example".to_string(), readings: vec![1, 2, 3, 4] };

    let mut buffer = vec![0u8; expected.byte_count()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected.uuid,           actual.uuid);
    assert_eq!(expected.timestamp,      actual.timestamp);
    assert_eq!(expected.name,           actual.name);
    assert_eq!(expected.readings,       actual.readings);
}

