use super::ToFromByteError;

use crate::{BytesReader, BytesWriter};

/// The heart of minbin.
///
/// You implement this trait by hand. No derives or macros available except your own.
/// *(Actually we do use ONE macro for our tuple implementations, but that's it)*.
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
    /// Hard upper bound on the serialized size of this type (including length prefixes
    /// and all nested data).
    ///
    /// Default for `String`, `&str`, `Vec<T>` and tuples: **1 MiB** (`1_048_576` bytes).
    /// This prevents accidental or intentional denial of service attacks.
    ///
    /// # Important Safety Note
    /// The library **centrally enforces** `MAX_BYTES` in `write_bytes` and `read_bytes`.
    /// Even if a user's `byte_count()` lies, the limit is still respected.
    ///
    /// # When can `usize` overflow happen?
    /// Only if you manually set `MAX_BYTES` close to or equal to `usize::MAX`
    /// **and** your type adds extra overhead (e.g. a length prefix) that pushes
    /// the final size over the edge.
    ///
    /// Examples:
    /// - A `String` with `MAX_BYTES = usize::MAX`. `byte_count()` can return
    ///   `4 + usize::MAX` leading to overflow.
    /// - A deeply nested `Vec<Vec<...>>` with `MAX_BYTES = usize::MAX`, same problem but nested.
    ///
    /// The default 1 MiB limit is safe on all systems 32-bit and above, since it is enforced on
    /// the top level. If you raise `MAX_BYTES` significantly, just keep the total serialized size
    /// comfortably below `usize::MAX` and you’ll be fine.
    const MAX_BYTES: usize;

    /// Serializes the value into the provided writer.
    ///
    /// Returns an error if the buffer is too small.
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError>;

    /// Deserializes the value from the reader, returning the deserialized value and final reader position.
    ///
    /// Returns an error on invalid data or insufficient bytes.
    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError>
    where
        Self: Sized;

    /// Returns the exact number of bytes this value will occupy when serialized.
    ///
    /// The total byte_count including recursive types and length prefixes may not exceed usize.
    fn byte_count(&self) -> usize;
}
