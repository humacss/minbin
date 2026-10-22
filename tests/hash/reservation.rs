#[test]
fn truncated_hash_map_does_not_reserve_claimed_count() {
    let mut value = std::collections::HashMap::<u32, u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert_eq!(value.capacity(), 0);
}

#[test]
fn truncated_hash_set_does_not_reserve_claimed_count() {
    let mut value = std::collections::HashSet::<u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert_eq!(value.capacity(), 0);
}

#[test]
fn maximum_count_without_payload_returns_an_error() {
    let bytes = u32::MAX.to_be_bytes();
    let expected = Err(minbin::ToFromByteError::NotEnoughBytes);
    assert_eq!(minbin::bytes_into(&mut Vec::<u32>::new(), &bytes), expected);
    assert_eq!(minbin::bytes_into(&mut std::collections::HashMap::<u32, u32>::new(), &bytes), expected);
    assert_eq!(minbin::bytes_into(&mut std::collections::HashSet::<u32>::new(), &bytes), expected);
}
