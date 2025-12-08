use crate::{ToFromBytes, ToFromByteError};

pub struct BytesReader<'a> {
    pub data: &'a [u8],
    pub pos:  usize,
}

impl<'a> BytesReader<'a> {
    #[inline(always)]
    pub fn read<T: ToFromBytes<'a>>(&mut self) -> Result<T, ToFromByteError> {
        let (value, _) = T::from_bytes(self)?;

        Ok(value)
    }

    #[inline(always)]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

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
