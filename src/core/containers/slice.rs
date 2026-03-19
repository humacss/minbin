use crate::{ToFromBytes, ToFromByteError, BytesWriter, BytesReader};

/// A wrapper around a mutable slice for serialization/deserialization.
///
/// Use this for `no_std` slice support where `Vec<T>` is not available.
///
/// The buffer must be pre-allocated with enough capacity for the expected elements.
/// Deserialization will fail with `NotEnoughBytes` if the wire length exceeds the buffer size.
pub struct Slice<'a, T>(pub &'a mut [T]);

impl<'a, T> From<&'a mut [T]> for Slice<'a, T> {
    fn from(value: &'a mut [T]) -> Slice<'a, T> {
        Slice(value)
    }
}

impl<'a, T: ToFromBytes<'a>> ToFromBytes<'a> for Slice<'a, T> {
    const MAX_BYTES: usize = 1_048_576; // 1 MiB

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write::<u32>(&(self.0.len() as u32))?;

        for item in &*self.0 {
            item.to_bytes(writer)?;
        }

        Ok(())
    }

    fn from_bytes(buffer: &mut Self, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError> {
        let mut len: u32 = 0;
        reader.read_into::<u32>(&mut len)?;

        if len as usize > buffer.0.len() {
            return Err(ToFromByteError::NotEnoughBytes);
        }

        for i in 0..len as usize {
            reader.read_into(&mut buffer.0[i])?;
        }

        Ok(reader.pos)
    }

    fn byte_count(&self) -> usize {
        4 + self.0.iter().map(|item| item.byte_count()).sum::<usize>()
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{read_into, write_bytes};
    use super::*;
    use crate::core::containers::str::StrLen32;


    #[rstest]
    #[case::empty(&mut [], &mut[0u32; 0])]
    #[case::min_max(&mut[u32::MIN, u32::MAX], &mut [0u32; 2])]
    #[case::many(&mut [47, 819, 305, 92, 674, 158, 933, 21, 446, 787, 603, 139, 528, 964, 312, 85, 751, 204, 638, 376], &mut [0u32; 20])]
    fn test_slice(
        #[case] expected: &mut [u32],
        #[case] default: &mut [u32]
    ){
        let value: Slice<'_, u32> = expected.into();
        let mut bytes = [0u8; 1000];

        let write_pos = write_bytes(&value, &mut bytes).unwrap();

        assert_eq!(value.byte_count(), write_pos);

        let mut actual = Slice(default);
        let read_pos = read_into(&mut actual, &bytes).unwrap();

        assert_eq!(value.byte_count(), read_pos);
        assert_eq!(expected, actual.0);

    }

    #[test]
    fn test_str_slice_empty() {
        let mut bytes = [0u8; 1000];
        let write_pos = {
            let mut expected = [StrLen32(""); 0];
            let value: Slice<'_, StrLen32<'_>> = (&mut expected[..]).into();
            let wp = write_bytes(&value, &mut bytes).unwrap();
            assert_eq!(value.byte_count(), wp);
            wp
        };

        let mut actual_buf = [StrLen32(""); 0];
        let mut actual = Slice(&mut actual_buf[..]);
        let read_pos = read_into(&mut actual, &bytes).unwrap();

        assert_eq!(write_pos, read_pos);
    }

    #[test]
    fn test_str_slice_with_values() {
        let mut bytes = [0u8; 1000];
        let write_pos = {
            let mut expected = [StrLen32("hello"), StrLen32("world")];
            let value: Slice<'_, StrLen32<'_>> = (&mut expected[..]).into();
            let wp = write_bytes(&value, &mut bytes).unwrap();
            assert_eq!(value.byte_count(), wp);
            wp
        };

        let mut actual_buf = [StrLen32(""); 2];
        let mut actual = Slice(&mut actual_buf[..]);
        let read_pos = read_into(&mut actual, &bytes).unwrap();

        assert_eq!(write_pos, read_pos);
        assert_eq!("hello", actual.0[0].0);
        assert_eq!("world", actual.0[1].0);
    }

}
