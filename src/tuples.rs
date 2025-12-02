//! Automatic implementations of `ToFromBytes` for tuples up to 12 elements.
//!
//! Each element is serialized in order with no separator or length prefix.
//! This keeps the format minimal and predictable.

use crate::{from_bytes, to_bytes, ToFromBytes, ToFromByteError};
use alloc::vec::Vec;

macro_rules! implement_tuple {
    ($($item:ident),+) => {
        // Allows single letter capitalized inputs to the implement_tuple! function
        #[allow(non_snake_case)]
        impl<$($item: ToFromBytes),+> ToFromBytes for ($($item,)+)
        {
            /// Serialize each field sequentially.
            fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
                let mut bytes = Vec::new();
                
                let ($($item,)+) = self;
                $(bytes.extend(to_bytes($item)?);)+
                
                Ok(bytes)
            }

            /// Deserialize each field sequentially.
            /// Stops and returns an error as soon as any field fails.
            #[allow(non_snake_case)]
            fn from_bytes(mut bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {                
                $(let ($item, remaining) = from_bytes::<$item>(bytes)?; bytes = remaining;)+

                Ok((($($item,)+), bytes))
            }
        }
    };
}

implement_tuple!(A);
implement_tuple!(A, B);
implement_tuple!(A, B, C);
implement_tuple!(A, B, C, D);
implement_tuple!(A, B, C, D, E);
implement_tuple!(A, B, C, D, E, F);
implement_tuple!(A, B, C, D, E, F, G);
implement_tuple!(A, B, C, D, E, F, G, H);
implement_tuple!(A, B, C, D, E, F, G, H, I);
implement_tuple!(A, B, C, D, E, F, G, H, I, J);
implement_tuple!(A, B, C, D, E, F, G, H, I, J, K);
implement_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
