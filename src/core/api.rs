use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Convenience function.
///
/// Deserializes a complete value from a byte slice.
///
/// Fails with `TrailingBytes` if the input contains extra data after the value.
/// This is intentional, silently ignoring trailing bytes is a common source of errors and security bugs.
///
/// Use this when you expect exactly one message per buffer (most common case).
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, ToFromByteError>
where
    T: for<'a> ToFromBytes<'a>,
{
    if bytes.len() > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    let (value, pos) = read_bytes(bytes)?;

    if pos < bytes.len() {
        return Err(ToFromByteError::TrailingBytes);
    }

    Ok(value)
}

/// Low-level read: deserialize a value and return how many bytes were consumed.
///
/// Does NOT check for trailing bytes. Use this when:
/// - You're parsing multiple messages from one buffer
/// - You're implementing streaming parsers
/// - You have a length prefix and want to stop exactly there
///
/// Do NOT use this function without confirming that the reader.pos is at the correct position afterwards.
/// Silently ignoring trailing bytes is a common source of errors and security bugs.
#[inline]
pub fn read_bytes<'a, T: ToFromBytes<'a>>(buffer: &'a [u8]) -> Result<(T, usize), ToFromByteError> {
    let mut reader = BytesReader::new(buffer);

    let value = reader.read()?;

    // We already did the work but should drop the buffer as soon as possible
    if reader.pos > T::MAX_BYTES {
        return Err(ToFromByteError::MaxBytesExceeded);
    }

    Ok((value, reader.pos))
}

/// Serialize a value into an existing buffer.
///
/// Returns the number of bytes written on success.
///
/// Fails early with `NotEnoughBytes` if the buffer is too small.
/// This is checked using `value.byte_count()` before touching the buffer,
/// so you get predictable errors instead of silent truncation or panics.
///
/// Preferred over `to_bytes` (the alloc version) in hot paths and no-std code.
#[inline]
pub fn write_bytes<'a, T: ToFromBytes<'a>>(
    value: &T,
    buffer: &'a mut [u8],
) -> Result<usize, ToFromByteError> {
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
