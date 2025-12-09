use minbin::{ToFromByteError, BytesWriter};

#[test]
fn writer_partial() {
    let mut buffer = [0u8; 32];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write_bytes(&[1, 2, 3]).unwrap();
    writer.write_bytes(&[4, 5]).unwrap();

    assert_eq!(writer.pos, 5);
}

#[test]
fn writer_full() {
    let mut buffer = [0u8; 4];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write_bytes(&[1u8, 1u8, 1u8, 1u8]).unwrap();
    assert_eq!(writer.pos, 4);
}

#[test]
fn writer_overflow() {
    let mut buffer = [0u8; 4];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write_bytes(&[1, 2, 3, 4]).unwrap();
    
    let err = writer.write_bytes(&[5]).unwrap_err();
    assert_eq!(err, ToFromByteError::NotEnoughBytes);
}