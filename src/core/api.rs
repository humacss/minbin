use crate::{BytesWriter, BytesReader, ToFromBytes, ToFromByteError};

#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, ToFromByteError>
where T: for<'a> ToFromBytes<'a>
{
    let (value, pos) = read_bytes(bytes)?; 

    if pos < bytes.len(){
        return Err(ToFromByteError::TrailingBytes);
    }

    Ok(value)
}

#[inline]
pub fn read_bytes<'a, T: ToFromBytes<'a>>(buffer: &'a [u8]) -> Result<(T, usize), ToFromByteError> {
    let mut reader = BytesReader::new(buffer);

    let value = reader.read()?;
    
    Ok((value, reader.pos))
}

#[inline]
pub fn write_bytes<'a, T: ToFromBytes<'a>>(value: &T, buffer: &'a mut [u8]) -> Result<usize, ToFromByteError> {	
    let buffer_len = buffer.len();

	if buffer_len < value.byte_count() {
        return Err(ToFromByteError::NotEnoughBytes);
    }

	let mut writer = BytesWriter::new(buffer);
	value.to_bytes(&mut writer)?;
     
    Ok(writer.pos)
}
