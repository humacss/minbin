//! Tests for BytesReader and BytesWriter
pub mod helpers;

use simbin::{ToFromByteError, BytesReader, BytesWriter};

#[test]
fn writer_basic() {
    let mut buffer = [0u8; 32];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write(&[1, 2, 3]).unwrap();
    writer.write(&[4, 5]).unwrap();

    assert_eq!(writer.written(), &[1, 2, 3, 4, 5]);
    assert_eq!(writer.pos, 5);
}

#[test]
fn writer_exact_fit() {
    let mut buffer = [0u8; 4];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write(&[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
    assert_eq!(writer.written(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn writer_overflow() {
    let mut buffer = [0u8; 4];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write(&[1, 2, 3, 4]).unwrap();
    let err = writer.write(&[5]).unwrap_err();
    assert_eq!(err, ToFromByteError::NotEnoughBytes);
}

#[test]
fn reader_basic() {
    let data = [1, 2, 3, 4, 5, 6];
    let mut reader = BytesReader::new(&data);

    assert_eq!(reader.read(3).unwrap(), &[1, 2, 3]);
    assert_eq!(reader.read(2).unwrap(), &[4, 5]);
    assert_eq!(reader.remainder(), &[6]);
}

#[test]
fn reader_exact_consume() {
    let data = [0xCA, 0xFE, 0xBA, 0xBE];
    let mut reader = BytesReader::new(&data);

    assert_eq!(reader.read(4).unwrap(), &[0xCA, 0xFE, 0xBA, 0xBE]);
    assert!(reader.remainder().is_empty());
}

#[test]
fn reader_not_enough_bytes() {
    let data = [1, 2, 3];
    let mut reader = BytesReader::new(&data);

    let err = reader.read(4).unwrap_err();
    assert_eq!(err, ToFromByteError::NotEnoughBytes);
}

#[test]
fn reader_remainder_is_correct() {
    let data = b"hello world";
    let mut reader = BytesReader::new(data);

    assert_eq!(reader.read(5).unwrap(), b"hello");
    assert_eq!(reader.remainder(), b" world");
    assert_eq!(reader.read(6).unwrap(), b" world");
    assert!(reader.remainder().is_empty());
}

#[test]
fn writer_and_reader_together() {
    let mut buffer = [0u8; 16];
    let mut writer = BytesWriter::new(&mut buffer);

    writer.write(&[0x01, 0x02]).unwrap();
    writer.write(&[0x03, 0x04, 0x05]).unwrap();

    let mut reader = BytesReader::new(writer.written());
    assert_eq!(reader.read(2).unwrap(), &[0x01, 0x02]);
    assert_eq!(reader.read(3).unwrap(), &[0x03, 0x04, 0x05]);
    assert!(reader.remainder().is_empty());
}