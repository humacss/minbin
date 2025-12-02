/// Test helper macro: asserts that serialization → deserialization is lossless
/// and fully consumes the byte buffer.
///
/// ## Example
///
/// ```rust
/// use simbin::assert_roundtrip;
///
/// assert_roundtrip!(u32, vec![u32::MIN, 42, u32::MAX]);
/// assert_roundtrip!(String, vec!["".to_string(), "hello".to_string()]);
/// ```
#[macro_export]
macro_rules! assert_roundtrip {
    ($type:ty, $cases:expr) => {{
        for expected in $cases {
            let bytes = $crate::to_bytes(&expected).unwrap();
            let (actual, bytes) = $crate::from_bytes::<$type>(&bytes).unwrap();
            
            assert_eq!(expected, actual);
            assert!(bytes.is_empty());
        }
    }};
}
