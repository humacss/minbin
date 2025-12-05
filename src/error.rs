//! Error types for serialization and deserialization operations.
use core::fmt;

/// Errors that can occur during serialization or deserialization with `simbin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToFromByteError {
    /// The buffer or data slice does not have enough bytes for the operation.
    NotEnoughBytes,
    /// The deserialized value is invalid for the type (e.g., invalid bool, UTF-8).
    InvalidValue,
}

impl fmt::Display for ToFromByteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToFromByteError::NotEnoughBytes => formatter.write_str("not enough bytes"),
            ToFromByteError::InvalidValue => formatter.write_str("invalid value"),
        }
    }
}
