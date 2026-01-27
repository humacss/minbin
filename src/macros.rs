///! Placeholder

// Placeholder
#[macro_export]
macro_rules! minbin_struct {
    ($struct_name:ident [ $(self . $property:ident: $property_type:ty),+ $(,)?]) => {
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

// Placeholder
#[macro_export]
macro_rules! minbin_enum {
    ($struct_name:ident [ $($discriminant:literal => Self :: $property:ident),+ $(,)?]) => {
		impl<'a> minbin::ToFromBytes<'a> for $struct_name {
		    const MAX_BYTES: usize = 1_048_576;

		    fn to_bytes(&self, writer: &mut minbin::BytesWriter<'a>) -> Result<(), minbin::ToFromByteError> {
		    	match self {
		    		$(Self::$property => {
		    			writer.write::<u32>(&$discriminant)?;
		    		}),+
		    		_ => { return Err(minbin::ToFromByteError::UnhandledEnumArm); }
		    	}

		        Ok(())
		    }

		    fn from_bytes(reader: &mut minbin::BytesReader<'a>) -> Result<(Self, usize), minbin::ToFromByteError> {
		    	let discriminant = reader.read::<u32>()?;

		    	let result = match discriminant {
		    		$($discriminant => { Self::$property }),+,
		    		_ => { return Err(minbin::ToFromByteError::UnhandledEnumArm); }
		    	};

		        Ok((result, reader.pos))
		    }

		    fn byte_count(&self) -> usize {
		    	match self {
		    		$(Self::$property => {
		    			4
		    		}),+,
		    		_ => { 4 }
		    	}
		    }
		}
    };
}
