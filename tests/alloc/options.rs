use minbin::{bytes_into, from_bytes};

#[test]
fn vector_of_options_round_trips() {
    let expected = vec![None, Some(42u32), Some(0), None];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(from_bytes::<Vec<Option<u32>>>(&bytes).unwrap(), expected);
}

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

#[test]
fn map_with_optional_values_round_trips() {
    let expected = std::collections::BTreeMap::from([(1u32, Some(42u32)), (2, None)]);
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(from_bytes::<std::collections::BTreeMap<u32, Option<u32>>>(&bytes).unwrap(), expected);
}
