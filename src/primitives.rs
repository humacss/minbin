//! Implementations of `ToFromBytes` for primitive types.
//! All integers are serialized in **big-endian** (network order).

use alloc::vec;
use alloc::vec::Vec;

use crate::{ToFromBytes, ToFromByteError};

macro_rules! implement_int {
    ($($int_type:ty, $byte_count:expr;)*) => {$(
        impl ToFromBytes for $int_type {
            /// Serialize as big-endian bytes.
            fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
                Ok(self.to_be_bytes().to_vec())
            }
            
            /// Deserialize from big-endian bytes.
            /// Fails with `NotEnoughBytes` if fewer than `$byte_count` bytes remain.
            fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
                if bytes.len() < $byte_count {
                    return Err(ToFromByteError::NotEnoughBytes);
                }
                
                let arr = <[u8; $byte_count]>::try_from(&bytes[..$byte_count]).unwrap();
                
                Ok((Self::from_be_bytes(arr), &bytes[$byte_count..]))
            }
        }
    )*};
}

implement_int! {
    u8, 1;  i8, 1;
    u16, 2; i16, 2;
    u32, 4; i32, 4;
    u64, 8; i64, 8;
    u128, 16; i128, 16;
}

impl ToFromBytes for bool {
    /// `true` → `[1]`, `false` → `[0]`
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        Ok(vec![*self as u8])
    }

    /// Any non-zero byte is interpreted as `true`. (change to error if not 0 or 1)
    /// Returns `NotEnoughBytes` if input is empty.
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        if bytes.is_empty() {
            return Err(ToFromByteError::NotEnoughBytes);
        }
        Ok((bytes[0] == 1, &bytes[1..]))
    }
}
