use simbin::{assert_roundtrip};

#[test]
fn test_strings() {
    assert_roundtrip!(String, vec![
        String::from(""),
        String::from("test string")
    ]);    
}

#[test]
fn test_vecs() {
    assert_roundtrip!(Vec<u32>, vec![
        vec![],
        vec![u32::MIN, 42, u32::MAX],
    ]);
}

#[test]
fn test_options() {
    assert_roundtrip!(Option<u32>, vec![
        None,
        Some(u32::MIN),
        Some(42),
        Some(u32::MAX),
    ]);
}
