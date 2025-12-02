use alloc::vec::Vec;

use super::{ToFromByteError};

/// Core trait for types that can be serialized to/from bytes.
///
/// Implement this for your own structs and enums.
/// The trait is deliberately minimal and composable.
pub trait ToFromBytes {    
    /// Serialize `self` into a new `Vec<u8>`.
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError>;

    /// Deserialize an instance of `Self` from `bytes`.
    ///
    /// Returns the value and any remaining unconsumed bytes.
    /// This allows parsing multiple concatenated objects.
    ///
    /// Errors on invalid values, most commonly due to a type mismatch due to the type used for 
    /// serialization not being the same as the one used for deserialization due to programmer error.
    fn from_bytes<'a>(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), ToFromByteError> where Self: Sized;
}

/// Convenience function that calls `T::to_bytes()`.
pub fn to_bytes<T: ToFromBytes>(input: &T) -> Result<Vec<u8>, ToFromByteError> {
    input.to_bytes()
}

/// Convenience function that calls `T::from_bytes()`.
pub fn from_bytes<'a, T: ToFromBytes>(bytes: &'a [u8]) -> Result<(T, &'a [u8]), ToFromByteError> {
    T::from_bytes(bytes)
}
