use minbin::{bytes_into, from_bytes, ToFromByteError, ToFromBytes};

#[derive(Debug, Default, PartialEq)]
struct Packet {
    value: u32,
}
minbin::minbin_struct! { Packet [self.value: u32] }

#[test]
fn result_type_selects_the_default_destination() {
    let packet: Packet = from_bytes(&[0, 0, 0, 42]).unwrap();
    assert_eq!(packet, Packet { value: 42 });
}

#[test]
fn explicit_type_selects_the_default_destination() {
    assert_eq!(from_bytes::<u32>(&[0, 0, 0, 42]), Ok(42));
}

#[test]
fn borrowed_output_can_be_inferred() {
    let text: &str = from_bytes(&[0, 0, 0, 2, b'h', b'i']).unwrap();
    assert_eq!(text, "hi");
}

#[test]
fn incomplete_or_trailing_bytes_are_rejected() {
    assert_eq!(from_bytes::<Packet>(&[0, 0, 0]), Err(ToFromByteError::NotEnoughBytes));
    assert_eq!(from_bytes::<Packet>(&[0, 0, 0, 42, 0]), Err(ToFromByteError::TrailingBytes));
}

#[test]
fn oversized_input_is_rejected_before_default_is_constructed() {
    struct NeverInitialize {
        value: u8,
    }
    impl Default for NeverInitialize {
        fn default() -> Self {
            panic!("oversized input must be rejected before initialization")
        }
    }
    minbin::minbin_struct! { NeverInitialize [self.value: u8] }
    let bytes = vec![0; NeverInitialize::MAX_BYTES + 1];
    assert!(matches!(from_bytes::<NeverInitialize>(&bytes), Err(ToFromByteError::MaxBytesExceeded)));
}

#[test]
fn custom_destination_does_not_require_default() {
    #[derive(Debug, PartialEq)]
    struct CustomPacket {
        value: u32,
    }
    minbin::minbin_struct! { CustomPacket [self.value: u32] }
    let mut packet = CustomPacket { value: 99 };
    bytes_into(&mut packet, &[0, 0, 0, 42]).unwrap();
    assert_eq!(packet, CustomPacket { value: 42 });
}
