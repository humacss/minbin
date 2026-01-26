///! Placeholder

// Placeholder
#[macro_export]
macro_rules! to_from_bytes {
    ($struct_name:ident [ $(self . $property:ident: $property_type:ty),+ ]) => {
		impl<'a> minbin::ToFromBytes<'a> for $struct_name {
		    const MAX_BYTES: usize = 1_048_576;

		    fn to_bytes(&self, writer: &mut minbin::BytesWriter<'a>) -> Result<(), minbin::ToFromByteError> {
		    	$(
	    			writer.write::<$property_type>(&self.$property)?;
    			)+

		        Ok(())
		    }

		    fn from_bytes(reader: &mut minbin::BytesReader<'a>) -> Result<(Self, usize), minbin::ToFromByteError> {
		        $(
                    let $property = reader.read::<$property_type>()?;
                )+
		        
		        Ok((Self { $($property,)+ }, reader.pos))
		    }

		    fn byte_count(&self) -> usize {
		    	$( self.$property.byte_count() + )+ 0
		    }
		}
    };
}

/// Placeholder
#[macro_export]
macro_rules! to_from_bytes_tuple {
    ($($name:ident),+) => {
        #[allow(non_snake_case)]
        impl<'a, $($name: ToFromBytes<'a>),+> ToFromBytes<'a> for ($($name,)+)
        {
            const MAX_BYTES: usize = 1_048_576; // 1 MiB

            #[inline(always)]
            fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
                let ($($name,)+) = self;
                $($name.to_bytes(writer)?;)+
                Ok(())
            }

            #[inline(always)]
            fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
                Ok((
                    ($(reader.read::<$name>()?,)+),
                    reader.pos
                ))
            }

            #[inline(always)]
            fn byte_count(&self) -> usize {
                let ($($name,)+) = self;
                0 $(+ $name.byte_count())+
            }
        }
    };
}
