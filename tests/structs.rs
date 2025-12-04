pub mod helpers;

use simbin::{ToFromBytes, ToFromByteError, BytesWriter, BytesReader};

#[derive(Debug, PartialEq)]
struct TestStruct<'a> {
    id: u32,
    name: &'a str,
}

impl<'de> ToFromBytes<'de> for TestStruct<'de> {
    #[inline(always)]
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
        self.id.to_bytes(writer)?;
        self.name.to_bytes(writer)?;

        Ok(())
    }

    #[inline(always)]
    fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, &'de [u8]), ToFromByteError> {
        let (id, _) = u32::from_bytes(reader)?;
        let (name, remainder) = <&str>::from_bytes(reader)?;

        Ok((TestStruct { id, name }, remainder))
    }

    #[inline(always)]
    fn byte_count(&self) -> usize {
        self.id.byte_count() + self.name.byte_count()
    }
}

#[test]
fn test_struct() {
    assert_roundtrip!(TestStruct, &[
        TestStruct {
            id: 0,
            name: "first",
        },
        TestStruct {
            id: 42,
            name: "second",
        }
    ]);
}