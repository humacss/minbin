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
///
/// # Wire format and compatibility
///
/// Built-in encodings use the following layout. There is no automatic message
/// header, schema version, padding, or field metadata.
///
/// | Type | Encoding |
/// |---|---|
/// | `u8`–`u128`, `i8`–`i128` | Fixed-width, big-endian integers |
/// | `bool` | One byte: `0` for false, `1` for true; other values are invalid |
/// | `String`, `&str`, `StrLen32` | Big-endian `u32` UTF-8 byte length, then UTF-8 bytes |
/// | `Vec<T>`, `Slice<T>` | Big-endian `u32` element count, then elements in order |
/// | `Option<T>` | One-byte tag: `0` for `None`, `1` followed by the value for `Some` |
/// | Tuples | Fields in tuple order; unit `()` has no bytes |
/// | Struct macro | Fields in the order listed in the macro |
/// | Enum macro | Explicit `u8` discriminant followed by variant fields in macro order |
/// | Maps | Big-endian `u32` entry count, then key/value pairs sorted by key |
/// | Sets | Big-endian `u32` element count, then sorted elements |
///
/// Map and set decoders reject duplicate keys or elements. Hash collection encoding
/// requires `Ord` and sorts before writing. Custom trait implementations define
/// their own encodings.
///
/// For 1.x, changes to built-in wire layouts require a major release. Application
/// schemas remain your responsibility: changing field order, types, or enum tags
/// changes your format. Put a protocol version in your own message when needed.
///
pub trait ToFromBytes<'a> {
    /// Maximum serialized size of this type, including length prefixes and nested data.
    ///
    /// Collections, tuples, and macro-generated types use **1 MiB** (`1_048_576` bytes).
    /// `String`, `&str`, and `StrLen32` use **100 KiB** (`102_400` bytes).
    /// `Option<T>` allows one tag byte plus `T::MAX_BYTES`.
    /// All limits include length prefixes and nested values.
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
    ///
    /// For a different limit, define a wrapper type with a custom implementation.
    /// For large payloads, consider framing smaller messages or using a streaming
    /// format instead.
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
