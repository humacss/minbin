///! Placeholder
#[macro_export]
macro_rules! minbin_enum {
    ($name:ident [ $([$($arm:tt)+]),+ $(,)?]) => {
		impl<'a> minbin::ToFromBytes<'a> for $name {
		    const MAX_BYTES: usize = 1_048_576;

		    fn to_bytes(&self, writer: &mut minbin::BytesWriter<'a>) -> Result<(), minbin::ToFromByteError> {
		    	$($crate::minbin_enum_helper!{@write self, writer, $($arm)+ })+;

		        Err($crate::ToFromByteError::UnhandledEnumArm)
		    }

		    fn from_bytes(reader: &mut minbin::BytesReader<'a>) -> Result<(Self, usize), minbin::ToFromByteError> {
		    	let value = reader.read::<u32>()?;

		    	$($crate::minbin_enum_helper!{@read reader, value, $($arm)+ })+;

				Err($crate::ToFromByteError::UnhandledEnumArm)
		    }

		    fn byte_count(&self) -> usize {
		    	let mut count = 4; // discriminator size

    			$($crate::minbin_enum_helper!{@byte_count self, count, $($arm)+ })+;

		    	count
		    }
		}
    };
}

///! Placeholder
#[macro_export]
macro_rules! minbin_enum_helper {
	(@discriminant $discriminant:literal => $($tail:tt)* ) => {
		$discriminant
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident) => {
		if let Self::$arm_name = $self {
			$writer.write::<u32>(&$discriminant)?;

			return Ok(());
		}
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident($($item_name:ident: $item_type:ty),*)) => {
		if let Self::$arm_name($($item_name),*) = $self {
			$writer.write::<u32>(&$discriminant)?;	

			$($writer.write::<$item_type>(&$item_name)?;)*

			return Ok(());
		}
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident{$($item_name:ident: $item_type:ty),*}) => {
		if let Self::$arm_name{$($item_name),*} = $self {
			$writer.write::<u32>(&$discriminant)?;	

			$($writer.write::<$item_type>(&$item_name)?;)*

			return Ok(());
		}
	};

	(@read $reader:expr, $value:expr, $discriminant:literal => Self::$arm_name:ident) => {
		if $discriminant == $value {
			return Ok((Self::$arm_name, $reader.pos));
		}
	};

	(@read $reader:expr, $value:expr, $discriminant:literal => Self::$arm_name:ident($($item_name:ident: $item_type:ty),*)) => {
		if $discriminant == $value {
			$(let $item_name = $reader.read::<$item_type>()?;)*
			
			return Ok((Self::$arm_name($($item_name),*), $reader.pos));
		}
	};

	(@read $reader:expr, $value:expr, $discriminant:literal => Self::$arm_name:ident{$($item_name:ident: $item_type:ty),*}) => {
		if $discriminant == $value {
			$(let $item_name = $reader.read::<$item_type>()?;)*
			
			return Ok((Self::$arm_name{$($item_name),*}, $reader.pos));
		}
	};

	(@byte_count $self:expr, $count:expr, $discriminant:literal => Self::$arm_name:ident) => {
		if let Self::$arm_name = $self {
			$count += 0;
		}
	};

	(@byte_count $self:expr, $count:expr, $discriminant:literal => Self::$arm_name:ident($($item_name:ident: $item:ty),*)) => {
		if let Self::$arm_name($($item_name),*) = $self {
			$($count += $item_name.byte_count();)*
		}
	};

	(@byte_count $self:expr, $count:expr, $discriminant:literal => Self::$arm_name:ident{$($item_name:ident: $item:ty),*}) => {
		if let Self::$arm_name{$($item_name),*} = $self {
			$($count += $item_name.byte_count();)*
		}
	};
}
