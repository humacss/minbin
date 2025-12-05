use crate::{ToFromBytes, ToFromByteError, BytesWriter, BytesReader};

// Get rid of the macro
macro_rules! implement_int {
    ($($ty:ty => $size:expr),* $(,)?) => {$(
        impl ToFromBytes<'_> for $ty {
            #[inline(always)]
            fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
                writer.write_bytes(&self.to_be_bytes())
            }

            #[inline(always)]
            fn from_bytes<'de>(reader: &mut BytesReader<'de>) -> Result<(Self, usize), ToFromByteError> {
                let bytes = reader.read_bytes($size)?;
                
                let value = Self::from_be_bytes(bytes.try_into().unwrap());
                
                Ok((value, reader.pos))
            }

            #[inline(always)]
            fn byte_count(&self) -> usize {
                $size
            }
        }
    )*}
}

implement_int! {
    u8   => 1,
    i8   => 1,
    u16  => 2,
    i16  => 2,
    u32  => 4,
    i32  => 4,
    u64  => 8,
    i64  => 8,
    u128 => 16,
    i128 => 16,
}

impl ToFromBytes<'_> for bool {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&[if *self { 1 } else { 0 }])
    }

    #[inline(always)]
    fn from_bytes<'de>(reader: &mut BytesReader<'de>) -> Result<(Self, usize), ToFromByteError> {
        let value = reader.read_bytes(1)?[0];

        match value {
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