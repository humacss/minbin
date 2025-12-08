use crate::core::{ToFromBytes, ToFromByteError};

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
