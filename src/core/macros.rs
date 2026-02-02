
/// Big endian
#[macro_export]
macro_rules! to_from_bytes_int {
	($int: ty, $byte_count: literal) => {
		impl ToFromBytes<'_> for $int {
		    const MAX_BYTES: usize = $byte_count;

		    #[inline(always)]
		    fn to_bytes(&self, writer: &mut BytesWriter<'_>) -> Result<(), ToFromByteError> {
		        writer.write_bytes(&self.to_be_bytes())
		    }

		    #[inline(always)]
		    fn from_bytes(reader: &mut BytesReader<'_>) -> Result<(Self, usize), ToFromByteError> {
		        let bytes = reader.read_bytes($byte_count)?;
		        let bytes = bytes.try_into().map_err(|_| ToFromByteError::NotEnoughBytes)?;

		        Ok((<$int>::from_be_bytes(bytes), reader.pos))
		    }

		    #[inline(always)]
		    fn byte_count(&self) -> usize {
		        $byte_count
		    }
		}
	}
}

/// Placeholder
#[macro_export]
macro_rules! to_from_bytes_tuple {
    ($($name:ident),*) => {
        #[allow(non_snake_case)]
        impl<'a, $($name: ToFromBytes<'a>),*> ToFromBytes<'a> for ($($name,)*)
        {
            const MAX_BYTES: usize = 1_048_576; // 1 MiB

            #[inline(always)]
            fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
                let ($($name,)*) = self;
                $($name.to_bytes(writer)?;)*
                Ok(())
            }

            #[inline(always)]
            fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
                Ok((
                    ($(reader.read::<$name>()?,)*),
                    reader.pos
                ))
            }

            #[inline(always)]
            fn byte_count(&self) -> usize {
                let ($($name,)*) = self;
                0 $(+ $name.byte_count())*
            }
        }
    };
}
