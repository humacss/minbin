use minbin::{from_bytes, write_bytes};

#[derive(Debug, Default, PartialEq)]
enum Message {
    #[default]
    Empty,
    Tuple(Option<u32>, u16),
    Named {
        value: Option<u32>,
    },
}

minbin::minbin_enum! { Message [
    [0 => Self::Empty],
    [1 => Self::Tuple(value: Option<u32>, code: u16)],
    [2 => Self::Named { value: Option<u32> }],
] }

#[test]
fn enum_fields_decode_from_default() {
    for expected in [Message::Empty, Message::Tuple(Some(42), 7), Message::Tuple(None, 0), Message::Named { value: Some(123) }] {
        let mut bytes = [0u8; 32];
        let len = write_bytes(&expected, &mut bytes).unwrap();
        assert_eq!(from_bytes::<Message>(&bytes[..len]).unwrap(), expected);
    }
}

#[derive(Debug, Default, PartialEq)]
struct Record {
    value: Option<u32>,
}
minbin::minbin_struct! { Record [self.value: Option<u32>] }

#[test]
fn struct_optional_field_decodes_from_default() {
    assert_eq!(from_bytes::<Record>(&[1, 0, 0, 0, 42]).unwrap(), Record { value: Some(42) });
}
