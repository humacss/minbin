use minbin::core::containers::slice::Slice;
use minbin::{bytes_into, from_bytes, write_bytes, ToFromByteError, ToFromBytes};

const STRING_LIMIT: usize = <&str as ToFromBytes<'static>>::MAX_BYTES;

#[test]
fn tuple_rejects_a_string_over_its_own_limit() {
    // The string exceeds its limit by one byte, including its u32 prefix.
    // The complete tuple still fits comfortably within the tuple's 1 MiB limit.
    let text = "x".repeat(STRING_LIMIT - 4 + 1);
    let value = (7u8, text.as_str(), 9u8);
    let mut buffer = vec![0u8; value.byte_count()];

    assert_eq!(write_bytes(&value, &mut buffer), Err(ToFromByteError::MaxBytesExceeded));
}

#[test]
fn tuple_round_trips_a_string_exactly_at_its_own_limit() {
    let text = "x".repeat(STRING_LIMIT - 4);
    let value = (7u8, text.as_str(), 9u8);
    let mut buffer = vec![0u8; value.byte_count()];

    let written = write_bytes(&value, &mut buffer).unwrap();
    let decoded = from_bytes(|| (0u8, "", 0u8), &buffer[..written]).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn slice_rejects_a_string_over_its_own_limit() {
    let text = "x".repeat(STRING_LIMIT - 4 + 1);
    let mut elements = [text.as_str()];
    let value = Slice(&mut elements);
    let mut buffer = vec![0u8; value.byte_count()];

    assert_eq!(write_bytes(&value, &mut buffer), Err(ToFromByteError::MaxBytesExceeded));
}

#[test]
fn slice_round_trips_a_string_exactly_at_its_own_limit() {
    let text = "x".repeat(STRING_LIMIT - 4);
    let mut buffer = vec![0u8; 4 + STRING_LIMIT];
    let written = {
        let mut elements = [text.as_str()];
        let value = Slice(&mut elements);
        write_bytes(&value, &mut buffer).unwrap()
    };

    let mut destination = [""];
    let mut decoded = Slice(&mut destination);
    bytes_into(&mut decoded, &buffer[..written]).unwrap();
    assert_eq!(decoded.0, &[text.as_str()]);
}
