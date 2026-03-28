//! ToFromBytes trait implementations for container types.

/// Slice wrapper for no_std serialization.
pub mod slice;
/// Borrowed string wrapper for no_std serialization.
pub mod str;

use crate::{to_from_bytes_tuple, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

impl<'a, T: ToFromBytes<'a>> ToFromBytes<'a> for Option<T> {
    const MAX_BYTES: usize = 1 + T::MAX_BYTES;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        match self {
            Option::None => {
                writer.write(&0u8)?;

                Ok(())
            }
            Option::Some(value) => {
                writer.write(&1u8)?;
                writer.write(value)?;

                Ok(())
            }
        }
    }

    /// Deserializes an `Option<T>` from the reader.
    ///
    /// When the wire data contains `Some`, the buffer must already be `Some(value)` so that
    /// the existing value can be read into. If the buffer is `None` but the wire has `Some`,
    /// this returns `InvalidValue`.
    ///
    /// Use the convenience functions with an init closure to handle this automatically:
    /// ```ignore
    /// let opt: Option<MyType> = from_bytes(|| Some(MyType::new(..)), &bytes)?;
    /// ```
    #[inline(always)]
    fn from_bytes(buffer: &mut Option<T>, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        let mut tag: u8 = 0;
        reader.read_into(&mut tag)?;

        match tag {
            0 => *buffer = None,
            1 => {
                match buffer.as_mut() {
                    Some(inner) => reader.read_into(inner)?,
                    None => return Err(ToFromByteError::InvalidValue),
                };
            }
            _ => return Err(ToFromByteError::InvalidValue),
        }

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        match self.as_ref() {
            Some(inner) => 1 + inner.byte_count(),
            None => 1,
        }
    }
}

impl<'a> ToFromBytes<'a> for &'a str {
    const MAX_BYTES: usize = 102_400; // 100 KiB

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        let len = u32::try_from(self.len()).map_err(|_| ToFromByteError::InvalidValue)?;

        writer.write(&len)?;
        writer.write_bytes(self.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut &'a str, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        let mut len: u32 = 0;
        reader.read_into(&mut len)?;

        let bytes = reader.read_bytes(len as usize)?;

        *buffer = core::str::from_utf8(bytes).map_err(|_| ToFromByteError::InvalidValue)?;

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.len()
    }
}

to_from_bytes_tuple!();
to_from_bytes_tuple!(T0);
to_from_bytes_tuple!(T0, T1);
to_from_bytes_tuple!(T0, T1, T2);
to_from_bytes_tuple!(T0, T1, T2, T3);
to_from_bytes_tuple!(T0, T1, T2, T3, T4);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
to_from_bytes_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
