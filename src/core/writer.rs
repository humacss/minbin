use crate::core::{ToFromBytes, ToFromByteError};

/// Position-tracking writer over a `&mut [u8]`.
///
/// Why a writer struct instead of writing directly to the buffer?
/// - Rust’s borrow rules make it painful to mutate a slice while keeping track of position safely.
/// - Without a cursor you’d constantly pass around index + slice, risking off-by-one errors or lifetime fights.
/// - The writer lets you borrow the buffer once and pass `&mut writer` around cleanly for nesting.
/// - It’s still zero-cost (just a usize + reference) and gives early overflow errors instead of silent truncation.
/// 
/// Using borrows instead of owned `Vec<u8>` avoids cloning or moving data unnecessarily and works in `no-std`.
/// The tiny abstraction pays for itself in ergonomics and safety while staying fully auditable.
pub struct BytesWriter<'a> {
    pub data:  &'a mut [u8],
    pub pos:  usize,
}

impl<'a> BytesWriter<'a> {
    #[inline(always)]
    pub fn write<T: ToFromBytes<'a>>(&mut self, value: &T) -> Result<(), ToFromByteError> {
        value.to_bytes(self)
    }

    #[inline(always)]
    pub const fn new(data: &'a mut [u8]) -> Self {
        Self { data, pos: 0 }
    }

	#[inline(always)]
    fn assert_enough_bytes(&self, byte_count: usize) -> Result<(), ToFromByteError> {
        if self.pos + byte_count > self.data.len() { return Err(ToFromByteError::NotEnoughBytes); }

        Ok(())
    }

    #[inline(always)]
    pub fn write_bytes(&mut self, src: &[u8]) -> Result<(), ToFromByteError> {
    	let byte_count = src.len();

    	self.assert_enough_bytes(byte_count)?;
        
        (&mut self.data[self.pos..self.pos + byte_count]).copy_from_slice(src);
        
        self.pos += src.len();
        
        Ok(())
    }
}
