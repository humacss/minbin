#[test]
fn truncated_vector_does_not_reserve_claimed_count() {
    let mut value = Vec::<u32>::new();
    let result = minbin::bytes_into(&mut value, &4096u32.to_be_bytes());
    assert_eq!(result, Err(minbin::ToFromByteError::NotEnoughBytes));
    assert!(value.capacity() < 4096);
}

#[test]
fn zero_byte_vector_elements_still_round_trip() {
    let expected = vec![(); 8];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(bytes.len(), 4);
    assert_eq!(minbin::from_bytes::<Vec<()>>(&bytes).unwrap(), expected);
}

#[test]
fn nested_vectors_still_round_trip() {
    let expected = vec![vec![], vec![1u32, 2], vec![3]];
    let bytes = minbin::to_bytes(&expected).unwrap();
    assert_eq!(minbin::from_bytes::<Vec<Vec<u32>>>(&bytes).unwrap(), expected);
}
