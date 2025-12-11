//! Position-tracking reader over a `&[u8]`.
//!
//! Why not just pass `&[u8]` and an index everywhere?
//!
//! Because manually slicing and passing indices leads to off-by-one bugs and lifetime hell.
//! This wrapper simplifies working with the Rust compiler and introduces zero overhead.

use crate::{ToFromBytes, ToFromByteError};

/// Reads from an immutable byte slice.
pub struct BytesReader<'a> {
    /// The underlying buffer we're reading from.
    pub data: &'a [u8],
    /// Current read position. Always ≤ data.len().
    pub pos:  usize,
}

impl<'a> BytesReader<'a> {
    /// Create a new reader starting at position 0.
    #[inline(always)]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Convenience function.
    ///
    /// Reads a complete value, advancing the cursor.
    ///
    /// Equivalent to calling `T::from_bytes(self)` and discarding the returned position.
    ///
    /// Use only when you don't need to know how many bytes were consumed.
    #[inline(always)]
    pub fn read<T: ToFromBytes<'a>>(&mut self) -> Result<T, ToFromByteError> {
        let (value, _pos) = T::from_bytes(self)?;

        Ok(value)
    }

    /// Read exactly `byte_count` raw bytes, advancing the cursor.
    ///
    /// Returns a `&'a [u8]` slice that borrows from the original buffer.
    /// Zero-copy, needed for `no_std` and zero-allocation parsing.
    ///
    /// Used by all base implementations.
    #[inline(always)]
    pub fn read_bytes(&mut self, byte_count: usize) -> Result<&'a [u8], ToFromByteError> {
        self.assert_enough_bytes(byte_count)?;

        let slice = &self.data[self.pos..self.pos+byte_count];

        self.pos += byte_count;

        Ok(slice)
    }

    #[inline(always)]
    fn assert_enough_bytes(&self, byte_count: usize) -> Result<(), ToFromByteError> {
        if self.pos + byte_count > self.data.len() { return Err(ToFromByteError::NotEnoughBytes); }

        Ok(())
    }
}
