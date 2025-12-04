use super::{ToFromByteError};

use crate::cursors::{BytesReader, BytesWriter};

/// Trait for types that can be serialized to and deserialized from bytes.
///
/// Implement this for your custom structs to enable serialization with `simbin`.
/// 
/// # Examples
///
/// ```rust
/// use simbin::{ToFromBytes, BytesWriter, BytesReader, ToFromByteError};
///
/// struct MyStruct {
///     field: u32,
/// }
///
/// impl<'de> ToFromBytes<'de> for MyStruct {
///     fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
///         self.field.to_bytes(writer);
///
///         Ok(())
///     }
///
///     fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, &'de [u8]), ToFromByteError> {
///         let (field, remainder) = u32::from_bytes(reader)?;
///         Ok((MyStruct { field }, remainder))
///     }
///
///     fn byte_count(&self) -> usize {
///         self.field.byte_count()
///     }
/// }
/// ```
pub trait ToFromBytes<'a> {    
    /// Serializes the value into the provided writer.
    ///
    /// Returns `Ok(())` on success, or an error if the buffer is too small.
    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError>;

    /// Deserializes the value from the reader, returning the deserialized value and any remaining bytes.
    ///
    /// Returns an error on invalid data or insufficient bytes.
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, &'a [u8]), ToFromByteError> where Self: Sized;

    /// Returns the exact number of bytes this value will occupy when serialized.
    fn byte_count(&self) -> usize;
}

/// Convenience function that calls `T::to_bytes()`.
pub fn to_bytes<'a, T: ToFromBytes<'a>>(value: &'a T, bytes: &mut [u8]) -> Result<(), ToFromByteError> {
    let mut writer = BytesWriter::new(bytes);
    
    value.to_bytes(&mut writer)?;

    Ok(())
}

/// Convenience function that calls `T::from_bytes()`.
pub fn from_bytes<'a, T: ToFromBytes<'a>>(bytes: &'a [u8]) -> Result<(T, &'a [u8]), ToFromByteError> {
    let mut reader = BytesReader::new(bytes);

    let (value, remainder) = T::from_bytes(&mut reader)?;

    Ok((value, remainder))
}
