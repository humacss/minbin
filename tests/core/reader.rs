use minbin::{BytesReader, ToFromByteError};

#[test]
fn reader_partial() {
    let data = [1, 2, 3, 4, 5, 6];
    let mut reader = BytesReader::new(&data);

    assert_eq!(reader.read_bytes(3).unwrap(), &[1, 2, 3]);
    assert_eq!(reader.read_bytes(2).unwrap(), &[4, 5]);
    assert_eq!(reader.pos, 5);
}

#[test]
fn reader_full() {
    let data = [0u8, 1u8, 2u8, 3u8];
    let mut reader = BytesReader::new(&data);

    assert_eq!(reader.read_bytes(4).unwrap(), &[0u8, 1u8, 2u8, 3u8]);
    assert_eq!(reader.pos, data.len());
}

#[test]
fn reader_overflow() {
    let data = [1, 2, 3];
    let mut reader = BytesReader::new(&data);

    let err = reader.read_bytes(4).unwrap_err();
    assert_eq!(err, ToFromByteError::NotEnoughBytes);
}

#[test]
fn reader_chunks() {
    let data = b"simbin";
    let mut reader = BytesReader::new(data);

    assert_eq!(reader.read_bytes(3).unwrap(), b"sim");
    assert_eq!(reader.pos, 3);
    assert_eq!(reader.read_bytes(3).unwrap(), b"bin");
    assert_eq!(reader.pos, 3 + 3);
}

#[test]
fn reader_rejects_an_overflowing_length_without_advancing() {
    let mut reader = BytesReader::new(&[1, 2]);
    reader.read_bytes(1).unwrap();

    assert_eq!(reader.read_bytes(usize::MAX), Err(ToFromByteError::NotEnoughBytes));
    assert_eq!(reader.pos, 1);
    assert_eq!(reader.read_bytes(1).unwrap(), &[2]);
}

#[test]
fn malformed_string_length_is_rejected() {
    let buffer = [255, 255, 255, 255];
    assert_eq!(minbin::from_bytes::<&str>(&buffer), Err(ToFromByteError::NotEnoughBytes));
    assert_eq!(minbin::from_bytes::<minbin::core::containers::str::StrLen32>(&buffer), Err(ToFromByteError::NotEnoughBytes));
}
