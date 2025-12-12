//! Implementations for fixed-width primitives and `bool`.
//!
//! All integers are serialized in big-endian byte order using `to_be_bytes`/`from_be_bytes`.
//! Fixed-width types ensure the size is always known at compile time, meaning less errors for us to deal with.

use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

impl ToFromBytes<'_> for u8 {
    const MAX_BYTES: usize = 1;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&self.to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(1)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((u8::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        1
    }
}

impl ToFromBytes<'_> for i8 {
    const MAX_BYTES: usize = 1;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&(*self as u8).to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(1)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((i8::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        1
    }
}

impl ToFromBytes<'_> for u16 {
    const MAX_BYTES: usize = 2;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&self.to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(2)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((u16::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        2
    }
}

impl ToFromBytes<'_> for i16 {
    const MAX_BYTES: usize = 2;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&(*self as u16).to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(2)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((i16::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        2
    }
}

impl ToFromBytes<'_> for u32 {
    const MAX_BYTES: usize = 4;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&self.to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(4)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((u32::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4
    }
}

impl ToFromBytes<'_> for i32 {
    const MAX_BYTES: usize = 4;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&(*self as u32).to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(4)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((i32::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4
    }
}

impl ToFromBytes<'_> for u64 {
    const MAX_BYTES: usize = 8;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&self.to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(8)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((u64::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        8
    }
}

impl ToFromBytes<'_> for i64 {
    const MAX_BYTES: usize = 8;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&(*self as u64).to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(8)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((i64::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        8
    }
}

impl ToFromBytes<'_> for u128 {
    const MAX_BYTES: usize = 16;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&self.to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(16)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((u128::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        16
    }
}

impl ToFromBytes<'_> for i128 {
    const MAX_BYTES: usize = 16;

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write_bytes(&(*self as u128).to_be_bytes())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
        let bytes = reader.read_bytes(16)?;
        let bytes = bytes
            .try_into()
            .map_err(|_| ToFromByteError::NotEnoughBytes)?;

        Ok((i128::from_be_bytes(bytes), reader.pos))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        16
    }
}

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
