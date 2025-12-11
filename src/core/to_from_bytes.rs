use super::{ToFromByteError};

use crate::{BytesReader, BytesWriter};

/// The heart of minbin: one trait you implement yourself.
///
/// Why manual?
/// - You decide field order, padding, and versioning explicitly.
/// - The wire format lives in your code, not in proc-macro output you can’t easily read or predict.
/// - Changing the format never requires a crate upgrade or migration tooling.
/// - Auditing or fixing the serialization is just reading your own ~20-line implementation.

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
