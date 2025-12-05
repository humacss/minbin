#![no_std]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

/// Contains all error states for the crate.
pub mod error;
/// The trait used for serialization, implement the trait for serialization support.
pub mod bytes;
/// ToFromBytes trait implementations for primitive types.
pub mod primitives;
/// ToFromBytes trait implementations for container types.
pub mod containers;
/// ToFromBytes trait implementations for tuples;
pub mod tuples;
/// Used for traversing a byte slice for reading and writing.
pub mod cursors;

pub use error::ToFromByteError;
pub use bytes::{ToFromBytes};
pub use cursors::{BytesReader, BytesWriter};

#[inline]
pub fn write_bytes<'a, T: ToFromBytes<'a>>(value: &T, buffer: &'a mut [u8]) -> Result<usize, ToFromByteError> {	
	if buffer.len() < value.byte_count() {
        return Err(ToFromByteError::NotEnoughBytes);
    }

	let mut writer = BytesWriter::new(buffer);
	value.to_bytes(&mut writer)?;
     
    Ok(writer.pos)
}

#[inline]
pub fn read_bytes<'a, T: ToFromBytes<'a>>(buffer: &'a [u8]) -> Result<(T, usize), ToFromByteError> {
    let mut reader = BytesReader::new(buffer);

    let value = reader.read()?;
    
    Ok((value, reader.pos))
}
