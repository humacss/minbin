use crate::BytesWriter;
use crate::BytesReader;
use crate::ToFromBytes;
use crate::ToFromByteError;

#[inline]
pub fn write_bytes<'a, T: ToFromBytes<'a>>(value: &T, buffer: &'a mut [u8]) -> Result<usize, ToFromByteError> {	
	if buffer.len() < value.byte_count() {
        return Err(ToFromByteError::NotEnoughBytes);
    }

	let mut writer = BytesWriter::new(buffer);
	value.to_bytes(&mut writer)?;
     
    Ok(writer.pos)
}

#[inline]
pub fn read_bytes<'a, T: ToFromBytes<'a>>(buffer: &'a [u8]) -> Result<(T, usize), ToFromByteError> {
    let mut reader = BytesReader::new(buffer);

    let value = reader.read()?;
    
    Ok((value, reader.pos))
}
