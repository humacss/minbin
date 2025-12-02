use core::fmt;

/// All errors that can occur during serialization/deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToFromByteError {
    /// Not enough bytes remaining in the input buffer.
    ///
    /// Examples:
    /// - wrong type specified for deserialization
    NotEnoughBytes,
    /// The data is corrupted or semantically invalid.
    ///
    /// Examples: 
    /// - wrong type specified for deserialization
    /// - invalid UTF-8
    /// - integer overflow
    /// - container length > u32
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
