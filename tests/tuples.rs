pub mod helpers;

#[test]
fn test_tuple_1() {
    assert_roundtrip!((u32,), &[(42,)]);
}

#[test]
fn test_tuple_2() {
    assert_roundtrip!((u32, &str), &[(42, "test")]);
}

#[test]
fn test_tuple_3() {
    assert_roundtrip!((u32, &str, Option<u8>), &[(42, "café", Some(7))]);
}
