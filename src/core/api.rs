//! High-level helpers built on top of [`BytesReader`] and [`BytesWriter`].
//!
//! The four entry points map to two common workflows:
//! - [`from_bytes`] / [`bytes_into`] for decoding a complete value from a buffer
//! - [`read_bytes`] / [`read_into`] for incremental parsing when you manage framing yourself
//!
//! Pair these with `crate::to_bytes` or [`write_bytes`] depending on whether you want the
//! alloc convenience path or the explicit buffer-based path.
//!
//! # Initialization and custom defaults
//!
//! `from_bytes` and `read_bytes` accept an initializer such as `MyType::default`.
//! `bytes_into` and `read_into` instead update an existing value.
//!
//! Vectors, maps, sets, and `Option<T>` require their decoded elements to implement
//! `Default`. Enum macro fields also use `Default`; no placeholder expressions are
//! needed. When decoding `Some`, an existing inner value is reused, or `T::default()`
//! constructs one if the destination is `None`.
//!
//! If your application's `Default` is unsuitable for decoding, define a wrapper
//! with its own `Default` and a delegating `ToFromBytes` implementation. For example,
//! `Option<DecodeValue>` can use a different initializer from `Option<MyValue>`.
//! Manual implementations can also call `reader.read(|| custom_initializer())`.
//!
//! For example, `i32::default()` is `0`, but a decoding wrapper can use `-1`:
//!
//! ```rust
//! use minbin::{from_bytes, write_bytes};
//!
//! #[derive(Debug, PartialEq)]
//! struct DecodeValue {
//!     value: i32,
//! }
//!
//! impl Default for DecodeValue {
//!     fn default() -> Self {
//!         Self { value: -1 }
//!     }
//! }
//!
//! minbin::minbin_struct! { DecodeValue [self.value: i32] }
//!
//! let original = Some(DecodeValue { value: 42 });
//! let mut buffer = [0u8; 5]; // Option tag and i32 value
//! let written = write_bytes(&original, &mut buffer).unwrap();
//!
//! // Reading Some into None constructs the wrapper with -1, then replaces
//! // that placeholder with the incoming value, 42.
//! let decoded = from_bytes(Option::<DecodeValue>::default, &buffer[..written]).unwrap();
//! assert_eq!(decoded, original);
//! assert_eq!(i32::default(), 0);
//! ```
//!
//! Defaults provide initial storage; each type's decoder determines how it is
//! updated. Decoding is not transactional: errors can leave values partially updated.
//!
//!
//! # Framing and errors
//!
//! [`from_bytes`] and [`bytes_into`] expect exactly one value and reject trailing
//! bytes. [`read_bytes`] and [`read_into`] return the consumed byte count so the
//! caller can parse multiple values from a buffer.
//!
//! [The `Slice` wrapper](crate::core::containers::slice::Slice) requires an exact destination
//! element count. Wrap a subslice to update a region of a larger buffer. A count
//! mismatch is rejected before updating elements. Other decoding errors can leave
//! values partially updated; callers must handle the error before using them.

use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Convenience function.
///
/// Deserializes a complete value from a byte slice.
///
/// The `init` closure constructs the initial value that gets read into.
/// This is lazy — the closure is only called once, right before reading.
/// Passing `Type::default` is the most common case.
///
/// Fails with `TrailingBytes` if the input contains extra data after the value.
/// This is intentional, silently ignoring trailing bytes is a common source of errors and security bugs.
///
/// Use this when you expect exactly one message per buffer (most common case).
#[inline]
pub fn from_bytes<'a, T>(init: impl FnOnce() -> T, bytes: &'a [u8]) -> Result<T, ToFromByteError>
where
    T: ToFromBytes<'a>,
{
    if bytes.len() > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    let mut value = init();
    bytes_into(&mut value, bytes)?;

    Ok(value)
}

/// Buffer-fill variant of `from_bytes`.
///
/// Deserializes into an existing buffer and checks for trailing bytes.
///
/// Use this when you already have a reusable buffer or want to avoid constructing a new value.
pub fn bytes_into<'a, T: ToFromBytes<'a>>(buffer: &mut T, bytes: &'a [u8]) -> Result<(), ToFromByteError> {
    if bytes.len() > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    let pos = read_into(buffer, bytes)?;

    if pos < bytes.len() {
        return Err(ToFromByteError::TrailingBytes);
    }

    Ok(())
}

/// Convenience function.
///
/// Low-level read: deserialize a value and return it along with how many bytes were consumed.
///
/// The `init` closure constructs the initial value that gets read into.
/// This is lazy — the closure is only called once, right before reading.
///
/// Does NOT check for trailing bytes. Use this when:
/// - You're parsing multiple messages from one buffer
/// - You're implementing streaming parsers
/// - You have a length prefix and want to stop exactly there
///
/// Do NOT use this function without confirming that the reader.pos is at the correct position afterwards.
/// Silently ignoring trailing bytes is a common source of errors and security bugs.
#[inline]
pub fn read_bytes<'a, T>(init: impl FnOnce() -> T, bytes: &'a [u8]) -> Result<(T, usize), ToFromByteError>
where
    T: ToFromBytes<'a>,
{
    let mut value = init();
    let pos = read_into(&mut value, bytes)?;
    Ok((value, pos))
}

/// Buffer-fill variant of `read_bytes`.
///
/// Deserializes into an existing buffer and returns how many bytes were consumed.
///
/// Does NOT check for trailing bytes.
///
/// The buffer's mutable borrow is decoupled from the data lifetime,
/// allowing types like `Slice<'a, StrLen32<'a>>` to work without
/// the `&'a mut Thing<'a>` anti-pattern.
#[inline]
pub fn read_into<'a, T: ToFromBytes<'a>>(buffer: &mut T, bytes: &'a [u8]) -> Result<usize, ToFromByteError> {
    let mut reader = BytesReader::new(bytes);

    reader.read_into(buffer)?;

    if reader.pos > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    Ok(reader.pos)
}

/// Serialize a value into an existing buffer.
///
/// Returns the number of bytes written on success.
///
/// Fails early with `NotEnoughBytes` if the buffer is too small.
/// This is checked using `value.byte_count()` before touching the buffer,
/// so you get predictable errors instead of silent truncation or panics.
///
/// Preferred over `to_bytes` (the alloc version) in hot paths, embedded code, and other `no_std` use cases.
#[inline]
pub fn write_bytes<'a, T: ToFromBytes<'a>>(value: &T, buffer: &'a mut [u8]) -> Result<usize, ToFromByteError> {
    let buffer_len = buffer.len();

    if buffer_len < value.byte_count() {
        return Err(ToFromByteError::NotEnoughBytes);
    }

    if value.byte_count() > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    let mut writer = BytesWriter::new(buffer);
    value.to_bytes(&mut writer)?;

    Ok(writer.pos)
}
