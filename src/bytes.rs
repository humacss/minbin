use super::{ToFromByteError};

use crate::{BytesReader, BytesWriter};

/// Trait for types that can be serialized to and deserialized from bytes.
///
/// Implement this for your custom structs to enable serialization with `simbin`.
pub trait ToFromBytes<'a> {    
    /// Serializes the value into the provided writer.
    ///
    /// Returns `Ok(())` on success, or an error if the buffer is too small.
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError>;

    /// Deserializes the value from the reader, returning the deserialized value and final reader position.
    ///
    /// Returns an error on invalid data or insufficient bytes.
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> where Self: Sized;

    /// Returns the exact number of bytes this value will occupy when serialized.
    fn byte_count(&self) -> usize;
}
