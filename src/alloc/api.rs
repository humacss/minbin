
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::{ToFromBytes, ToFromByteError, write_bytes};

/// Convenience function.
///
/// Automatically allocates a `Vec<u8>` containing the serialized value and returns it.
///
/// Use this when the default implementation here is good enough for your needs.
///
/// Returns early with `TrailingBytes` if the writer didn't consume the entire buffer 
/// (this catches bugs in `byte_count` implementations instantly)
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, ToFromByteError>
where T: for<'a> ToFromBytes<'a>
{
    let mut bytes = vec![0u8; value.byte_count()];

    let writer_pos = write_bytes(value, &mut bytes)?;

    if writer_pos < value.byte_count() {
        return Err(ToFromByteError::TrailingBytes);
    }

    Ok(bytes)
}
