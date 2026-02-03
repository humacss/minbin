//! Implementations for fixed-width primitives and `bool`.
//!
//! All integers are serialized in big-endian byte order using `to_be_bytes`/`from_be_bytes`.
//! Fixed-width types ensure the size is always known at compile time, meaning less errors for us to deal with.

use crate::{to_from_bytes_int, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

impl ToFromBytes<'_> for bool {
    const MAX_BYTES: usize = 1;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&[if *self { 1 } else { 0 }])
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let byte = reader.read_bytes(1)?[0];

        match byte {
            0 => Ok((false, reader.pos)),
            1 => Ok((true, reader.pos)),
            _ => Err(ToFromByteError::InvalidValue),
        }
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        1
    }
}

to_from_bytes_int!(u8, 1);
to_from_bytes_int!(u16, 2);
to_from_bytes_int!(u32, 4);
to_from_bytes_int!(u64, 8);
to_from_bytes_int!(u128, 16);

to_from_bytes_int!(i8, 1);
to_from_bytes_int!(i16, 2);
to_from_bytes_int!(i32, 4);
to_from_bytes_int!(i64, 8);
to_from_bytes_int!(i128, 16);
