use minbin::{bytes_into, from_bytes, ToFromByteError};

#[test]
fn some_decodes_into_none() {
    assert_eq!(from_bytes(Option::<u32>::default, &[1, 0, 0, 0, 42]), Ok(Some(42)));
}

#[test]
fn nested_options_decode_from_default() {
    assert_eq!(from_bytes(Option::<Option<u32>>::default, &[1, 1, 0, 0, 0, 42]), Ok(Some(Some(42))));
    assert_eq!(from_bytes(Option::<Option<u32>>::default, &[1, 0]), Ok(Some(None)));
}

#[test]
fn none_replaces_existing_some() {
    let mut value = Some(42u32);
    assert_eq!(bytes_into(&mut value, &[0]), Ok(()));
    assert_eq!(value, None);
}

#[test]
fn option_rejects_invalid_tag_and_truncated_payload() {
    assert_eq!(from_bytes(Option::<u32>::default, &[2]), Err(ToFromByteError::InvalidValue));
    assert_eq!(from_bytes(Option::<u32>::default, &[1]), Err(ToFromByteError::NotEnoughBytes));
}
