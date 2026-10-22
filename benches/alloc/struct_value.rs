use minbin::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

#[derive(Default)]
pub struct BenchStruct {
    pub uuid: u128,
    pub timestamp: i64,
    pub name: String,
    pub readings: Vec<u32>,
}

impl<'a> ToFromBytes<'a> for BenchStruct {
    const MAX_BYTES: usize = 1_048_576;

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.timestamp)?;
        writer.write(&self.name)?;
        writer.write(&self.readings)?;

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
        self.uuid.byte_count() + self.timestamp.byte_count() + self.name.byte_count() + self.readings.byte_count()
    }
}
