/// Placeholder
#[macro_export]
macro_rules! minbin_struct {
    ($name:ident [ $(self . $property:ident: $property_type:ty),+ $(,)?]) => {
		impl<'a> minbin::ToFromBytes<'a> for $name {
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
