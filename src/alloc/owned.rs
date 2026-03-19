//! Implementations of `ToFromBytes` for owned collection types (`String`, `Vec<T>`).
//!
//! These live in the `alloc` crate because they require allocation during deserialization.
//! The core crate remains completely `no-std` and zero-allocation.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// `Vec<T>` requires `T: Default` because the element count is encoded in the wire
/// format and only known at deserialization time. Each element is created via
/// `T::default()` and then overwritten by the deserialized data.
///
/// If your element type does not implement `Default`, use `Slice<T>` from the core
/// module instead — it works with a pre-allocated buffer and has no trait requirements
/// beyond `ToFromBytes`.
///
/// We plan to explore removing the `Default` requirement in a future version.
impl<'a, T> ToFromBytes<'a> for Vec<T>
where
    T: ToFromBytes<'a> + Default,
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
    fn from_bytes(buffer: &mut Vec<T>, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        buffer.clear();

        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        buffer.reserve(len as usize);

        for _ in 0..len {
            buffer.push(T::default());
            reader.read_into(buffer.last_mut().unwrap())?;
        }

        Ok(reader.pos)
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
    const MAX_BYTES: usize = 102_400; // 100 KiB

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;

        writer.write(&len)?;
        writer.write_bytes(self.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut String, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        buffer.clear();

        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        let bytes = reader.read_bytes(len as usize)?;

        let s = core::str::from_utf8(bytes).map_err(|_| ToFromByteError::InvalidValue)?;
        buffer.push_str(s);

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.len()
    }
}
