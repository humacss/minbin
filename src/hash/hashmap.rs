//! Hash map encoding in sorted key order.

use super::sort::sorted_by_key;
use crate::rust_std::collections::HashMap;
use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};
use core::hash::Hash;

impl<'a, K, V> ToFromBytes<'a> for HashMap<K, V>
where
    K: ToFromBytes<'a> + Default + Ord + Eq + Hash,
    V: ToFromBytes<'a> + Default,
{
    const MAX_BYTES: usize = 1_048_576;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;

        writer.write(&len)?;

        for (key, value) in sorted_by_key(self.iter(), |(key, _)| *key) {
            writer.write(key)?;
            writer.write(value)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut HashMap<K, V>, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        buffer.clear();

        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        if reader.can_reserve_elements(len, Self::MAX_BYTES) {
            buffer.reserve(len as usize);
        }

        for _ in 0..len {
            let mut key = K::default();
            let mut value = V::default();

            reader.read_into(&mut key)?;
            reader.read_into(&mut value)?;

            if buffer.insert(key, value).is_some() {
                return Err(ToFromByteError::InvalidValue);
            }
        }

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        let mut byte_count = 4;

        for (key, value) in self {
            byte_count += key.byte_count();
            byte_count += value.byte_count();
        }

        byte_count
    }
}
