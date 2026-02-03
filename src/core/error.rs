use core::fmt;

/// Single error type used throughout minbin.
///
/// Custom enum keeps us `no-std` and zero-size overhead compared to `std::io::Error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToFromByteError {
    /// The buffer or data slice does not have enough bytes for the operation.
    NotEnoughBytes,
    /// The value was read successfully, but the buffer contained extra bytes afterward.
    ///
    /// These bytes would be silently dropped if we returned Ok().
    TrailingBytes,
    /// The value is too large, exceeding the `MAX_BYTES` setting on `ToFromBytes`.
    ///
    /// This is a security setting intended to prevent denial of service attacks.
    MaxBytesExceeded,
    /// The deserialized value is invalid for the type (e.g., invalid bool, invalid UTF-8).
    InvalidValue,
    /// Returned when the enum macro ToFromBytes implementation calls an unhandled enum arm.
    UnhandledEnumArm,
}

impl fmt::Display for ToFromByteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToFromByteError::NotEnoughBytes => formatter.write_str("not enough bytes"),
            ToFromByteError::TrailingBytes => formatter.write_str("trailing bytes"),
            ToFromByteError::MaxBytesExceeded => formatter.write_str("max bytes exceeded"),
            ToFromByteError::InvalidValue => formatter.write_str("invalid value"),
            ToFromByteError::UnhandledEnumArm => formatter.write_str("unhandled enum arm"),
        }
    }
}
