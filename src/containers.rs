//! Implementations of `ToFromBytes` for common container types.
//!
//! These are re-exported in the crate root for convenience.

use crate::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter};


impl<'de, T: ToFromBytes<'de>> ToFromBytes<'de> for Option<T> {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        match self {
            Option::None => writer.write(&[0]),
            Option::Some(value) => {
                writer.write(&[1])?;
                value.to_bytes(writer)
            }
        }
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, &'de [u8]), ToFromByteError> {
        let option_byte = reader.read(1)?[0];

        match option_byte {
            0 => Ok((None, reader.remainder())),
            1 => {
                let (value, remainder) = T::from_bytes(reader)?;
                
                Ok((Some(value), remainder))
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

impl<'de> ToFromBytes<'de> for &'de str {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;
        len.to_bytes(writer)?;
        writer.write(self.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, &'de [u8]), ToFromByteError> {
        let (len, _remainder) = u32::from_bytes(reader)?;

        let bytes = reader.read(len as usize)?;

        let value = core::str::from_utf8(bytes).map_err(|_| ToFromByteError::InvalidValue)?;

        Ok((value, reader.remainder()))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.len()
    }
}

