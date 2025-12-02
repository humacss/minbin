use simbin::{assert_roundtrip};

#[test]
fn test_tuple_1() {
    assert_roundtrip!((u32,), vec![
        (42,)
    ]);
}

#[test]
fn test_tuple_2() {
    assert_roundtrip!((u32, String), vec![
        (42, "test".to_string())
    ]);
}

#[test]
fn test_tuple_4() {
    assert_roundtrip!((u32, String, Option<u8>, Vec<u64>), vec![
        (42, "café".to_string(), Some(7), vec![1, 2]),
    ]);
}
