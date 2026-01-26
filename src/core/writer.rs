//! Position-tracking writer over a `&mut [u8]`.
//!
//! Why not just pass `&mut [u8]` and an index everywhere?
//!
//! Because manually slicing and passing indices leads to off-by-one bugs and lifetime hell.
//! This wrapper simplifies working with the Rust compiler and introduces zero overhead.
//!
//! All writes return early on overflow instead of silently truncating.
//! Which should prevent otherwise common surprises in production.

use crate::{ToFromByteError, ToFromBytes};

/// Writes into a mutable byte slice.
///
/// The buffer is borrowed for `'a` — no copies, no allocation, works in `no_std`.
pub struct BytesWriter<'a> {
    /// The underlying buffer we're writing into. Borrowed, never reallocated.
    pub data: &'a mut [u8],
    /// Current write position. Always ≤ data.len().
    pub pos: usize,
}

impl<'a> BytesWriter<'a> {
    /// Create a new writer starting at position 0.
    #[inline(always)]
    pub const fn new(data: &'a mut [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Convenience function.
    ///
    /// Write bytes from any type that implements `ToFromBytes`.
    #[inline(always)]
    pub fn write<T: ToFromBytes<'a>>(&mut self, value: &T) -> Result<(), ToFromByteError> {
        if value.byte_count() > T::MAX_BYTES {
            return Err(ToFromByteError::MaxBytesExceeded);
        }

        value.to_bytes(self)
    }

    /// Write raw bytes. Used by primitives and length-prefixed containers.
    ///
    /// Performs a single bounds check, then `copy_from_slice`.
    ///
    /// Used by all base implementations.
    #[inline(always)]
    pub fn write_bytes(&mut self, src: &[u8]) -> Result<(), ToFromByteError> {
        let byte_count = src.len();

        self.assert_enough_bytes(byte_count)?;

        self.data[self.pos..self.pos + byte_count].copy_from_slice(src);

        self.pos += src.len();

        Ok(())
    }

    #[inline(always)]
    fn assert_enough_bytes(&self, byte_count: usize) -> Result<(), ToFromByteError> {
        if self.pos + byte_count > self.data.len() {
            return Err(ToFromByteError::NotEnoughBytes);
        }

        Ok(())
    }
}
