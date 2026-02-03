/// Declarative macro that generates a `ToFromBytes` implementation for an enum
/// using a simple discriminant + payload layout.
///
/// Syntax:
///
/// ```rust
/// enum ExampleEnum {
///     Ping,
///     Temperature(i16),
///     Location(i32, i32),
///     Log { time: i64, message: String },
/// }
/// 
/// minbin::minbin_enum! { ExampleEnum [
///     [0 => Self::Ping],
///     [1 => Self::Temperature(degrees: i16)],
///     [2 => Self::Location(lat: i32, lon: i32)],
///     [3 => Self::Log{ time: i64, message: String }]
/// ] }
/// ```
///
/// Limitations / design choices:
/// - Uses `u8` discriminant (max 255 variants)
/// - Generates `if let` chains instead of `match` (to keep macro simpler)
/// - Returns `UnhandledEnumArm` when the discriminant is unknown
/// - Requires unit tests to catch discriminant duplicates and unhandled arms
///
/// For more complex enums you should write the `ToFromBytes` implementation manually.
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
		    	let value = reader.read::<u8>()?;

		    	$($crate::minbin_enum_helper!{@read reader, value, $($arm)+ })+;

				Err($crate::ToFromByteError::UnhandledEnumArm)
		    }

		    fn byte_count(&self) -> usize {
		    	let mut count = 1; // discriminator size

    			$($crate::minbin_enum_helper!{@byte_count self, count, $($arm)+ })+;

		    	count
		    }
		}
    };
}

/// This is an internal macro not intended for use outside of this crate.
///
/// Helper for `minbin_enum!` that handles different enum syntax (Unit, Tuple, Struct).
///
/// Without a helper macro we need to rely on either proc macros or a recursive muncher for 
/// matching on different syntax.
///
/// Since we focus on auditability it is important that the macro is easy to read. 
/// Munchers are hard to debug, maintain and can result in slower compile times. Proc macros add 
/// too much magic and complicates the crate, we keep it simple by using only declarative macros.
///
/// Unfortunately declarative macros are not well suited for generating match clauses so we use if 
/// clauses instead. This has some performance implications since conditions are evaluated 
/// separately, but macro simplicity takes priority. 
///
/// If you need exhaustive matchers or better performance you should implement the trait manually.
#[macro_export]
macro_rules! minbin_enum_helper {
	(@discriminant $discriminant:literal => $($tail:tt)* ) => {
		$discriminant
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident) => {
		if let Self::$arm_name = $self {
			$writer.write::<u8>(&$discriminant)?;

			return Ok(());
		}
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident($($item_name:ident: $item_type:ty),*)) => {
		if let Self::$arm_name($($item_name),*) = $self {
			$writer.write::<u8>(&$discriminant)?;

			$($writer.write::<$item_type>(&$item_name)?;)*

			return Ok(());
		}
	};

	(@write $self:expr, $writer:expr, $discriminant:literal => Self::$arm_name:ident{$($item_name:ident: $item_type:ty),*}) => {
		if let Self::$arm_name{$($item_name),*} = $self {
			$writer.write::<u8>(&$discriminant)?;

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
