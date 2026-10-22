//! Hash set encoding in sorted element order.

use super::sort::sorted_by_key;
use crate::rust_std::collections::HashSet;
use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};
use core::hash::Hash;

impl<'a, T> ToFromBytes<'a> for HashSet<T>
where
    T: ToFromBytes<'a> + Default + Ord + Eq + Hash,
{
    const MAX_BYTES: usize = 1_048_576;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;

        writer.write(&len)?;

        for item in sorted_by_key(self.iter(), |item| *item) {
            writer.write(item)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut HashSet<T>, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        buffer.clear();

        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        if reader.can_reserve_elements(len, Self::MAX_BYTES) {
            buffer.reserve(len as usize);
        }

        for _ in 0..len {
            let mut item = T::default();
            reader.read_into(&mut item)?;

            if !buffer.insert(item) {
                return Err(ToFromByteError::InvalidValue);
            }
        }

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        let mut byte_count = 4;

        for item in self {
            byte_count += item.byte_count();
        }

        byte_count
    }
}
