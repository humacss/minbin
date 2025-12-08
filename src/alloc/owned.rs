extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use crate::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter};

impl<'a, T> ToFromBytes<'a> for Vec<T>
where T: ToFromBytes<'a>
{
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;
        
        writer.write(&len)?;

        for item in self {
            writer.write(item)?;    
        }

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let len: u32 = reader.read()?;

        let mut value = Vec::with_capacity(len as usize);

        for _i in 0..len {
            value.push(reader.read()?);
        }

        Ok((value, reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.iter().map(T::byte_count).sum::<usize>()
    }
}


impl<'a> ToFromBytes<'a> for String
{
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;
        
        writer.write(&len)?;
        writer.write_bytes(self.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let len: u32 = reader.read()?;

        let bytes = reader.read_bytes(len as usize)?;

        let value = String::from_utf8(bytes.to_vec()).map_err(|_| ToFromByteError::InvalidValue)?;

        Ok((value, reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.len()
    }
}
