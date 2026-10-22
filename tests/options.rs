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

#[cfg(feature = "alloc")]
#[test]
fn vector_of_options_round_trips() {
    let expected = vec![None, Some(42u32), Some(0), None];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(from_bytes(Vec::<Option<u32>>::new, &bytes).unwrap(), expected);
}

#[cfg(feature = "alloc")]
#[test]
fn option_reuses_existing_string_allocation() {
    let mut existing = String::with_capacity(64);
    existing.push_str("old");
    let pointer = existing.as_ptr();
    let mut value = Some(existing);
    bytes_into(&mut value, &[1, 0, 0, 0, 3, b'n', b'e', b'w']).unwrap();
    let actual = value.unwrap();
    assert_eq!(actual, "new");
    assert_eq!(actual.as_ptr(), pointer);
}

#[cfg(feature = "alloc")]
#[test]
fn map_with_optional_values_round_trips() {
    let expected = std::collections::BTreeMap::from([(1u32, Some(42u32)), (2, None)]);
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(from_bytes(std::collections::BTreeMap::<u32, Option<u32>>::new, &bytes).unwrap(), expected);
}
