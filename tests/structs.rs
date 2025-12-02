use simbin::{ToFromBytes, ToFromByteError, to_bytes, from_bytes, assert_roundtrip};

#[derive(Debug, PartialEq)]
struct TestStruct {
    id: u32,
    name: String,
    values: Vec<i16>,
}

impl ToFromBytes for TestStruct {
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        let mut bytes = Vec::new();
        
        bytes.extend(to_bytes(&self.id)?);
        bytes.extend(to_bytes(&self.name)?);
        bytes.extend(to_bytes(&self.values)?);

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        let (id, bytes) = from_bytes(bytes)?;
        let (name, bytes) = from_bytes(bytes)?;
        let (values, bytes) = from_bytes(bytes)?;

        Ok((TestStruct { id, name, values }, bytes))
    }
}

#[test]
fn test_struct() {
    assert_roundtrip!(TestStruct, vec![
        TestStruct {
            id: 0,
            name: "first".to_string(),
            values: vec![],
        },
        TestStruct {
            id: 42,
            name: "second".to_string(),
            values: vec![i16::MIN, 42, i16::MAX],
        }
    ]);
}