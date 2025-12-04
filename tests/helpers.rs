#[macro_export]
macro_rules! assert_roundtrip {
    ($type:ty, $cases:expr) => {{
        use simbin::{to_bytes, from_bytes};

        for expected in $cases {
            let size = <$type as simbin::ToFromBytes>::byte_count(expected);
            let mut buffer = vec![0u8; size];  // alloc in tests = fine

            to_bytes(expected, &mut buffer)
                .expect("serialization failed");

            let (actual, remainder) = from_bytes::<$type>(&buffer)
                .expect("deserialization failed");

            assert_eq!(*expected, actual);
            assert!(remainder.is_empty());
        }
    }};
}
