//! Tests for BytesReader and BytesWriter


use simbin::{ToFromByteError, BytesReader, BytesWriter};

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
fn writer_and_reader() {
    let mut buffer = [0u8; 16];
    let pos = {
        let mut writer = BytesWriter::new(&mut buffer);

        writer.write_bytes(&[0u8, 1u8]).unwrap();
        writer.write_bytes(&[2u8, 3u8, 4u8]).unwrap();

        writer.pos
    };

    let mut reader = BytesReader::new(&buffer[..pos]);
    assert_eq!(reader.read_bytes(2).unwrap(), &[0u8, 1u8]);
    assert_eq!(reader.read_bytes(3).unwrap(), &[2u8, 3u8, 4u8]);
    assert_eq!(reader.pos, 2 + 3);
}
