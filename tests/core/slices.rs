use minbin::core::containers::slice::Slice;
use minbin::{bytes_into, ToFromByteError};

#[test]
fn shorter_sequence_is_rejected_without_updating_elements() {
    let mut storage = [99u32; 3];
    let result = bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 1, 0, 0, 0, 42]);
    assert_eq!(result, Err(ToFromByteError::SliceLengthMismatch));
    assert_eq!(storage, [99; 3]);
}

#[test]
fn longer_sequence_is_rejected_without_updating_elements() {
    let mut storage = [99u32; 1];
    let result = bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 2, 0, 0, 0, 42, 0, 0, 0, 43]);
    assert_eq!(result, Err(ToFromByteError::SliceLengthMismatch));
    assert_eq!(storage, [99]);
}

#[test]
fn empty_sequence_is_rejected_for_nonempty_destination() {
    let mut storage = [99u32];
    assert_eq!(bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 0]), Err(ToFromByteError::SliceLengthMismatch));
    assert_eq!(storage, [99]);
}

#[test]
fn exact_length_decodes_into_selected_region() {
    let mut storage = [99u32; 3];
    bytes_into(&mut Slice(&mut storage[1..2]), &[0, 0, 0, 1, 0, 0, 0, 42]).unwrap();
    assert_eq!(storage, [99, 42, 99]);
}

#[test]
fn empty_sequence_decodes_into_empty_destination() {
    bytes_into(&mut Slice(&mut [] as &mut [u32]), &[0, 0, 0, 0]).unwrap();
}

#[test]
fn truncated_matching_sequence_reports_not_enough_bytes() {
    let mut storage = [99u32];
    assert_eq!(bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 1]), Err(ToFromByteError::NotEnoughBytes));
}

#[test]
fn truncated_payload_can_leave_earlier_elements_updated() {
    let mut storage = [99u32; 2];
    let result = bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 2, 0, 0, 0, 42]);
    assert_eq!(result, Err(ToFromByteError::NotEnoughBytes));
    assert_eq!(storage, [42, 99]);
}

#[test]
fn trailing_bytes_are_rejected_after_elements_are_updated() {
    let mut storage = [99u32];
    let result = bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 1, 0, 0, 0, 42, 0]);
    assert_eq!(result, Err(ToFromByteError::TrailingBytes));
    assert_eq!(storage, [42]);
}

#[test]
fn invalid_element_encoding_is_rejected() {
    let mut storage = [false];
    let result = bytes_into(&mut Slice(&mut storage), &[0, 0, 0, 1, 2]);
    assert_eq!(result, Err(ToFromByteError::InvalidValue));
    assert_eq!(storage, [false]);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn unrepresentable_slice_count_is_rejected_before_writing() {
    use minbin::{BytesWriter, ToFromBytes};

    // Unit elements occupy no memory, even when the count exceeds u32::MAX.
    let mut elements = vec![(); u32::MAX as usize + 1];
    let value = Slice(&mut elements);
    for buffer in [&mut [0xAA; 3][..], &mut [0xAA; 4][..]] {
        let mut writer = BytesWriter::new(buffer);
        assert_eq!(value.to_bytes(&mut writer), Err(ToFromByteError::InvalidValue));
        assert_eq!(writer.pos, 0);
        assert!(buffer.iter().all(|byte| *byte == 0xAA));
    }
}
