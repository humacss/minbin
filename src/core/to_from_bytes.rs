use super::{ToFromByteError};

use crate::{BytesReader, BytesWriter};

/// The heart of minbin.
///
/// You implement this trait by hand. No derives or macros available except your own *(and ours for tuples)*.
///
/// Why manual?
/// - You own the format. No hidden field reordering, no padding surprises.
/// - Debugging a serialization bug is just reading pure Rust code, not reverse-engineering derives, macros or attributes.
/// - Migrating away from minbin later is trivial because the format is already explicit in your code, no surprises.
///
/// The trait is deliberately minimal:
/// - `byte_count`: Enables preallocation and exact buffer sizing. Critical for no-std/stack buffers.
/// - `to_bytes`:   Writes into an existing buffer. No intermediate allocations.
/// - `from_bytes`: Returns the final cursor position. You can parse multiple messages from one buffer.
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
