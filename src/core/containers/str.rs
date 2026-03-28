#![deny(elided_lifetimes_in_paths)]

use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Borrowed string wrapper for `no_std` and other allocation-free use cases.
///
/// This stores a `&str` as a `u32` byte length followed by UTF-8 bytes.
/// When the default `alloc` feature is available, plain `String` is usually the simpler choice.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct StrLen32<'a>(pub &'a str);

impl<'a> ToFromBytes<'a> for StrLen32<'a> {
    const MAX_BYTES: usize = 102_400; // 100 KiB

    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        writer.write(&(self.0.len() as u32))?;
        writer.write_bytes(self.0.as_bytes())?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(buffer: &mut Self, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        let mut len: u32 = 0;
        reader.read_into::<u32>(&mut len)?;

        let bytes = reader.read_bytes(len as usize)?;
        buffer.0 = core::str::from_utf8(bytes).map_err(|_| ToFromByteError::InvalidValue)?;

        Ok(reader.pos)
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        4 + self.0.len() // len() returns number of bytes
    }
}

impl<'a> From<&'a str> for StrLen32<'a> {
    fn from(value: &'a str) -> Self {
        StrLen32(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_into, write_bytes};
    use rstest::rstest;

    #[rstest]
    #[case::empty("")]
    #[case::something("something")]
    fn test_str(#[case] expected: &str) {
        let value: StrLen32<'_> = expected.into();
        let mut bytes = [0u8; 1000];

        let write_pos = write_bytes(&value, &mut bytes).unwrap();

        assert_eq!(value.byte_count(), write_pos);

        let mut actual: StrLen32<'_> = StrLen32("");
        let read_pos = read_into(&mut actual, &bytes).unwrap();

        assert_eq!(value.byte_count(), read_pos);
        assert_eq!(expected, actual.0);
    }
}
