/// Declarative macro that generates a `ToFromBytes` implementation for a struct
/// by simply serializing each named field in declaration order.
///
/// Example:
///
/// ```rust
/// #[derive(Debug, PartialEq)]
/// struct ExampleStruct {
///     uuid: u128,
///     timestamp: i64,
///     name: String,
///     readings: Vec<String>,
/// }
///
/// impl Default for ExampleStruct {
///     fn default() -> Self {
///         Self {
///             uuid: 0,
///             timestamp: 0,
///             name: String::new(),
///             readings: Vec::new(),
///         }
///     }
/// }
///
/// minbin::minbin_struct! { ExampleStruct [
///     self.uuid: u128,
///     self.timestamp: i64,
///     self.name: String,
///     self.readings: Vec<String>
/// ] }
/// ```
///
/// Generated code is straightforward field-by-field read/write.
/// The field list also defines the wire order, so reordering fields is a wire-format change.
///
/// For more complex structs you should write the `ToFromBytes` implementation manually.
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

		    fn from_bytes(buffer: &mut Self, reader: &mut minbin::BytesReader<'a>) -> Result<usize, minbin::ToFromByteError> {
		        $(reader.read_into(&mut buffer.$property)?;)+

		        Ok(reader.pos)
		    }

		    fn byte_count(&self) -> usize {
		    	$( self.$property.byte_count() + )+ 0
		    }
		}
    };
}
