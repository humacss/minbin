//! Implementations of `ToFromBytes` for standard-library hash collections.
//!
//! `HashMap` and `HashSet` serialize deterministically by sorting keys/items
//! before writing.

use core::hash::Hash;

use crate::rust_std::collections::{HashMap, HashSet};
use crate::rust_std::vec::Vec;
use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

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

        let mut entries = self.iter().collect::<Vec<_>>();
        entries.sort_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));

        for (key, value) in entries {
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

        buffer.reserve(len as usize);

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

impl<'a, T> ToFromBytes<'a> for HashSet<T>
where
    T: ToFromBytes<'a> + Default + Ord + Eq + Hash,
{
    const MAX_BYTES: usize = 1_048_576;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;

        writer.write(&len)?;

        let mut items = self.iter().collect::<Vec<_>>();
        items.sort();

        for item in items {
            writer.write(item)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut HashSet<T>, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        buffer.clear();

        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        buffer.reserve(len as usize);

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
