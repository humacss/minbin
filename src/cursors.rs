//! Internal utilities for reading and writing byte buffers.
//!
//! These are exposed publicly for advanced use cases, but most users will use the top-level functions like `to_bytes` and `from_bytes`.

use crate::error::{ToFromByteError};

/// A simple reader for deserializing data from an immutable byte slice.
///
/// Tracks position and provides remaining bytes.
pub struct BytesReader<'a> {
	/// Data
    pub data: &'a [u8],
    /// Pos
    pub pos:  usize,
}

impl<'a> BytesReader<'a> {
	/// Creates a new reader backed by the given byte slice.
    #[inline(always)]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Reads exactly `len` bytes from the current position, advancing it.
    ///
    /// Returns `Err(NotEnoughBytes)` if insufficient bytes remain.
    #[inline(always)]
    pub fn read(&mut self, byte_count: usize) -> Result<&'a [u8], ToFromByteError> {
        self.assert_enough_bytes(byte_count)?;

        let slice = &self.data[self.pos..self.pos+byte_count];

        self.pos += byte_count;

        Ok(slice)
    }

	/// Returns the remaining unread bytes in the slice.
    #[inline(always)]
    pub fn remainder(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }

    #[inline(always)]
    fn assert_enough_bytes(&self, byte_count: usize) -> Result<(), ToFromByteError> {
        if self.pos + byte_count > self.data.len() { return Err(ToFromByteError::NotEnoughBytes); }

        Ok(())
    }
}

/// A zero-cost writer over a mutable byte buffer.
///
/// Writes data sequentially and tracks how many bytes have been written.
/// Designed for serialization — never allocates, never rechecks bounds if you know the size.
pub struct BytesWriter<'a> {
	/// Data
    pub data:  &'a mut [u8],
    /// Pos
    pub pos:  usize,
}

impl<'a> BytesWriter<'a> {
	/// Creates a new writer backed by the given mutable buffer.
    #[inline(always)]
    pub const fn new(data: &'a mut [u8]) -> Self {
        Self { data, pos: 0 }
    }

	#[inline(always)]
    fn assert_enough_bytes(&self, byte_count: usize) -> Result<(), ToFromByteError> {
        if self.pos + byte_count > self.data.len() { return Err(ToFromByteError::NotEnoughBytes); }

        Ok(())
    }

	/// Writes the given bytes to the buffer, advancing the position.
    ///
    /// Returns `Err(NotEnoughBytes)` if the buffer has insufficient space.
    #[inline(always)]
    pub fn write(&mut self, src: &[u8]) -> Result<(), ToFromByteError> {
    	let byte_count = src.len();

    	self.assert_enough_bytes(byte_count)?;
        
        (&mut self.data[self.pos..self.pos + byte_count]).copy_from_slice(src);
        
        self.pos += src.len();
        
        Ok(())
    }

    /// Returns a slice of the bytes written so far.
    #[inline(always)]
    pub fn written(&'a self) -> &'a [u8] {
        &self.data[..self.pos]
    }
}
