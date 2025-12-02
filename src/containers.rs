use alloc::string::String;
use alloc::vec::Vec;

use crate::{ToFromBytes, ToFromByteError, to_bytes, from_bytes};

/// `String` is serialized as: `u32` length (big-endian) + raw UTF-8 bytes.
/// Fails if the string is not valid UTF-8 on deserialization.
/// Maximum length: 4,294,967,295 elements.
impl ToFromBytes for String {
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        let mut bytes = to_bytes(&(self.len() as u32))?;

        bytes.extend(self.as_bytes());
        
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        let (len, bytes) = from_bytes::<u32>(bytes)?;
        let len = len as usize;

        if bytes.len() < len {
            return Err(ToFromByteError::NotEnoughBytes);
        }

        let s = String::from_utf8(bytes[..len].to_vec())
            .map_err(|_| ToFromByteError::InvalidValue)?;
        
        Ok((s, &bytes[len..]))
    }
}

/// `Vec<T>` is serialized as: `u32` length + each element serialized in order.
/// Maximum length: 4,294,967,295 elements.
impl<T: ToFromBytes> ToFromBytes for Vec<T> {
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        let mut bytes = to_bytes(&(self.len() as u32))?;
        
        for item in self {
            bytes.extend(to_bytes(item)?);
        }
        
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        let (len, mut bytes) = from_bytes::<u32>(bytes)?;
        let mut vec = Vec::with_capacity(len as usize);
        
        for _ in 0..len {
            let (item, rest) = from_bytes(bytes)?;
            bytes = rest;
            
            vec.push(item);
        }

        Ok((vec, bytes))
    }
}

/// `Option<T>` uses a 1-byte tag: `0` = None, `1` = Some, followed by the value.
impl<T: ToFromBytes> ToFromBytes for Option<T> {
    fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
        match self {
            None => to_bytes(&0u8),
            Some(v) => {
                let mut bytes = to_bytes(&1u8)?;
                
                bytes.extend(to_bytes(v)?);
                
                Ok(bytes)
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
        let (tag, bytes) = from_bytes::<u8>(bytes)?;

        match tag {
            0 => Ok((None, bytes)),
            1 => {
                let (option, bytes) = from_bytes(bytes)?;
                Ok((Some(option), bytes))
            }
            _ => Err(ToFromByteError::InvalidValue),
        }
    }
}