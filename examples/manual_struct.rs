use minbin::{from_bytes, to_bytes, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

#[derive(Debug, Default, PartialEq)]
struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<String>,
}

impl<'a> ToFromBytes<'a> for ExampleStruct {
    const MAX_BYTES: usize = 1_048_576;

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write::<u128>(&self.uuid)?;
        writer.write::<i64>(&self.timestamp)?;
        writer.write::<String>(&self.name)?;
        writer.write::<Vec<String>>(&self.readings)?;

        Ok(())
    }

    fn from_bytes(buffer: &mut Self, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        reader.read_into(&mut buffer.uuid)?;
        reader.read_into(&mut buffer.timestamp)?;
        reader.read_into(&mut buffer.name)?;
        reader.read_into(&mut buffer.readings)?;

        Ok(reader.pos)
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() +
        self.timestamp.byte_count() +
        self.name.byte_count() +
        self.readings.byte_count()
    }
}

fn main() {
    let expected = ExampleStruct {
        uuid: u128::MAX,
        timestamp: i64::MAX,
        name: "Name".to_string(),
        readings: vec!["Reading1".to_string(), "Reading2".to_string()],
    };
    let bytes = to_bytes(&expected).unwrap();
    let actual = from_bytes(ExampleStruct::default, &bytes).unwrap();
    assert_eq!(expected, actual);
}
