//! Regression tests for speculative allocation from untrusted collection counts.
#[cfg(feature = "alloc")]
#[test]
fn truncated_vector_does_not_reserve_claimed_count() {
    let mut value = Vec::<u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert!(value.capacity() < 4096);
}

#[cfg(feature = "std")]
#[test]
fn truncated_hash_map_does_not_reserve_claimed_count() {
    let mut value = std::collections::HashMap::<u32, u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert_eq!(value.capacity(), 0);
}

#[cfg(feature = "std")]
#[test]
fn truncated_hash_set_does_not_reserve_claimed_count() {
    let mut value = std::collections::HashSet::<u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert_eq!(value.capacity(), 0);
}

#[cfg(feature = "alloc")]
#[test]
fn zero_byte_vector_elements_still_round_trip() {
    let expected = vec![(); 8];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(bytes.len(), 4);
    assert_eq!(minbin::from_bytes(Vec::<()>::new, &bytes).unwrap(), expected);
}

#[cfg(feature = "alloc")]
#[test]
fn nested_vectors_still_round_trip() {
    let expected = vec![vec![], vec![1u32, 2], vec![3]];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(minbin::from_bytes(Vec::<Vec<u32>>::new, &bytes).unwrap(), expected);
}

#[cfg(feature = "std")]
#[test]
fn maximum_count_without_payload_returns_an_error() {
    let bytes = u32::MAX.to_be_bytes();
    let expected = Err(minbin::ToFromByteError::NotEnoughBytes);
    assert_eq!(minbin::bytes_into(&mut Vec::<u32>::new(), &bytes), expected);
    assert_eq!(minbin::bytes_into(&mut std::collections::HashMap::<u32, u32>::new(), &bytes), expected);
    assert_eq!(minbin::bytes_into(&mut std::collections::HashSet::<u32>::new(), &bytes), expected);
}
