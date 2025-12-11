use crate::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter};

impl<'a, T: ToFromBytes<'a>> ToFromBytes<'a> for Option<T> {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        match self {
            Option::None => {
                writer.write(&0u8)?;

                Ok(())
            },
            Option::Some(value) => {
                writer.write(&1u8)?;
                writer.write(value)?;

                Ok(())
            }
        }
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let option_byte: u8 = reader.read()?;

        match option_byte {
            0 => Ok((None, reader.pos)),
            1 => {
                let value = reader.read()?;
                
                Ok((Some(value), reader.pos))
            }
            _ => Err(ToFromByteError::InvalidValue),
        }
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        // or is hit if the option is None
        1 + self.as_ref().map_or(0, T::byte_count)
    }
}

impl<'a> ToFromBytes<'a> for &'a str {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;
        
        writer.write(&len)?;
        writer.write_bytes(self.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let len: u32 = reader.read()?;

        let bytes = reader.read_bytes(len as usize)?;

        let value = core::str::from_utf8(bytes).map_err(|_| ToFromByteError::InvalidValue)?;

        Ok((value, reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        // usize overflow issues needs to be handled
        4 + self.len()
    }
}



