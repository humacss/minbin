use minbin::{read_bytes, write_bytes, ToFromBytes};

#[test]
fn test_option() {
    for expected in [None, Some(u32::MIN), Some(42), Some(u32::MAX)] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();

        let init = || expected.as_ref().map(|_| 0u32);
        let (actual, read_pos): (Option<u32>, usize) = read_bytes(init, &buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_str() {
    for expected in ["", "something", "else"] {
        let mut buffer = vec![0u8; expected.byte_count()];

        let write_pos = write_bytes(&expected, &mut buffer).unwrap();

        assert_eq!(expected.byte_count(), write_pos);

        let (actual, read_pos): (&str, usize) = read_bytes(<&str>::default, &buffer).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}
