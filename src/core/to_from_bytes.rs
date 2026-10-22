use super::ToFromByteError;

use crate::{BytesReader, BytesWriter};

/// The heart of minbin.
///
/// You can implement this trait by hand or use the crate's helper macros for simple structs and enums.
/// Manual implementations are still the intended escape hatch when the wire format needs to be explicit or custom.
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
    /// Maximum serialized size of this type, including length prefixes and nested data.
    ///
    /// `Vec<T>` and tuples use **1 MiB** (`1_048_576` bytes).
    /// `String` and `&str` use **100 KiB** (`102_400` bytes).
    ///
    /// Serialization helpers check this limit using `byte_count()`. Implementers
    /// must report the exact number of bytes that `to_bytes` will write; an
    /// incorrect count can invalidate these checks.
    ///
    /// Complete-message decoding helpers reject input longer than this limit.
    /// Reader methods check the number of bytes consumed after decoding a value.
    /// The limit applies to serialized bytes, not total memory allocated by
    /// initialization or decoding implementations.
    ///
    /// Implementers must ensure size calculations, including prefixes and nested
    /// values, fit in `usize` without overflow.
    const MAX_BYTES: usize;

    /// Serializes the value into the provided writer.
    ///
    /// Returns an error if the buffer is too small.
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError>;

    /// Deserializes the value from the reader, returning the deserialized value and final reader position.
    ///
    /// Returns an error on invalid data or insufficient bytes.
    fn from_bytes<'b>(buffer: &'b mut Self, reader: &mut BytesReader<'a>) -> Result<usize, ToFromByteError>
    where
        Self: Sized;

    /// Returns the exact number of bytes this value will occupy when serialized.
    ///
    /// Must match the number of bytes written by `to_bytes`, including all length
    /// prefixes and nested values. Serialization helpers rely on this count for
    /// buffer sizing and `MAX_BYTES` checks. The calculation must not overflow `usize`.
    fn byte_count(&self) -> usize;
}
