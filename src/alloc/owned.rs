//! Implementations of `ToFromBytes` for owned collection types (`String`, `Vec<T>`).
//!
//! These live in the `alloc` crate because they require allocation during deserialization.
//! The core crate remains completely `no-std` and zero-allocation.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

impl<'a, T> ToFromBytes<'a> for Vec<T>
where
    T: ToFromBytes<'a>,
{
    const MAX_BYTES: usize = 1_048_576; // 1 MiB

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
        let mut byte_count = 4;

        for item in self.iter() {
            byte_count += item.byte_count();
        }

        byte_count
    }
}

impl<'a> ToFromBytes<'a> for String {
    const MAX_BYTES: usize = 1_048_576; // 1 MiB

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
