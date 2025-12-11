use minbin::{BytesWriter, BytesReader, ToFromBytes, ToFromByteError, to_bytes, from_bytes};

struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<u16>,
}

impl<'a> ToFromBytes<'a> for ExampleStruct {
    const MAX_BYTES: usize = 1_048_576; // 1 MiB
    
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

    fn byte_count(&self) -> Result<usize, ToFromByteError> {
        let byte_count = self.uuid.byte_count()? + self.timestamp.byte_count()? + 
        self.name.byte_count()? + self.readings.byte_count()?;

        if byte_count > Self::MAX_BYTES {
            return Err(ToFromByteError::MaxBytesExceeded);
        }

        Ok(byte_count)
    }
}

#[test]
fn test_struct() {
    let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example".to_string(), readings: vec![1, 2, 3, 4] };

    let bytes = to_bytes(&expected).unwrap();
    let actual: ExampleStruct = from_bytes(&bytes).unwrap();

    assert_eq!(expected.uuid,           actual.uuid);
    assert_eq!(expected.timestamp,      actual.timestamp);
    assert_eq!(expected.name,           actual.name);
    assert_eq!(expected.readings,       actual.readings);
}
