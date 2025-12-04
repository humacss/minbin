pub mod helpers;

#[test]
fn test_u8() {
    assert_roundtrip!(u8, &[u8::MIN, 42, u8::MAX]);
}

#[test]
fn test_i8() {
    assert_roundtrip!(i8, &[i8::MIN, 42, i8::MAX]);
}

#[test]
fn test_u16() {
    assert_roundtrip!(u16, &[u16::MIN, 42, u16::MAX]);
}

#[test]
fn test_i16() {
    assert_roundtrip!(i16, &[i16::MIN, 42, i16::MAX]);
}

#[test]
fn test_u32() {
    assert_roundtrip!(u32, &[u32::MIN, 42, u32::MAX]);
}

#[test]
fn test_i32() {
    assert_roundtrip!(i32, &[i32::MIN, 42, i32::MAX]);
}

#[test]
fn test_u64() {
    assert_roundtrip!(u64, &[u64::MIN, 42, u64::MAX]);
}

#[test]
fn test_i64() {
    assert_roundtrip!(i64, &[i64::MIN, 42, i64::MAX]);
}

#[test]
fn test_u128() {
    assert_roundtrip!(u128, &[u128::MIN, 42, u128::MAX]);
}

#[test]
fn test_i128() {
    assert_roundtrip!(i128, &[i128::MIN, 42, i128::MAX]);
}

#[test]
fn test_bool() {
    assert_roundtrip!(bool, &[true, false]);
}
