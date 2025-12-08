use simbin::{ToFromBytes, write_bytes, read_bytes};

#[test]
fn test_tuple_2() {
    let expected = (0u8,1);

    let mut buffer = vec![0u8; expected.byte_count()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): ((u8, u8), usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected,                actual);
}

#[test]
fn test_tuple_12() {
    let expected = (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8);

    let mut buffer = vec![0u8; expected.byte_count()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): ((u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected,                actual);
}

#[test]
fn test_tuple_containers(){
    let expected = (Some(1u8), "");

    let mut buffer = vec![0u8; expected.byte_count()];
    let write_pos = write_bytes(&expected, &mut buffer).unwrap();
    let (actual, read_pos): ((Option<u8>, &str), usize) = read_bytes(&buffer[..write_pos]).unwrap();

    assert_eq!(expected.byte_count(),   read_pos);
    assert_eq!(expected,                actual);   
}
