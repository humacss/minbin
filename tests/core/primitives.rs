use minbin::{read_bytes, write_bytes, ToFromBytes};

#[test]
fn test_u8() {
    for expected in [u8::MIN, 42, u8::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (u8, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_i8() {
    for expected in [i8::MIN, 42, i8::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (i8, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_u16() {
    for expected in [u16::MIN, 42, u16::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (u16, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_i16() {
    for expected in [i16::MIN, 42, i16::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (i16, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_u32() {
    for expected in [u32::MIN, 42, u32::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (u32, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_i32() {
    for expected in [i32::MIN, 42, i32::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (i32, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_u64() {
    for expected in [u64::MIN, 42, u64::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (u64, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_i64() {
    for expected in [i64::MIN, 42, i64::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (i64, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_u128() {
    for expected in [u128::MIN, 42, u128::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (u128, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_i128() {
    for expected in [i128::MIN, 42, i128::MAX] {
        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (i128, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(), read_pos);
        assert_eq!(expected, actual);
    }
}
