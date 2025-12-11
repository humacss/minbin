extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::{ToFromBytes, ToFromByteError, write_bytes};

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
