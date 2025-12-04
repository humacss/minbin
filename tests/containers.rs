pub mod helpers;

#[test]
fn test_options() {
    assert_roundtrip!(Option<u32>, &[None, Some(u32::MIN), Some(42), Some(u32::MAX)]);
}

#[test]
fn test_strs() {
    assert_roundtrip!(&str, &["", "something", "else"]);
}
