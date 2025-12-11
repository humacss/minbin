use minbin::{ToFromBytes, write_bytes, read_bytes};

#[test]
fn test_option() {
    for expected in [None, Some(u32::MIN), Some(42), Some(u32::MAX)] {
        let mut buffer = vec![0u8; expected.byte_count().unwrap()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (Option<u32>, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count().unwrap(),  read_pos);
        assert_eq!(expected,                        actual);
    }   
}

#[test]
fn test_str() {
    for expected in ["", "something", "else"] {
        let mut buffer = vec![0u8; expected.byte_count().unwrap()];
        
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        
        assert_eq!(expected.byte_count().unwrap(),   write_pos);

        let (actual, read_pos): (&str, usize) = read_bytes(&buffer).unwrap();

        assert_eq!(expected.byte_count().unwrap(),  read_pos);
        assert_eq!(expected,                        actual);
    }    
}
