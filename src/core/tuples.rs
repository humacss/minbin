use crate::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Didn't you say no macros in the README?
/// Yes, but we are just delegating to the implementor here so a macro is fine.
/// There is nothing special going on, just us working around the fact that there is no tuple generic in Rust.
/// Reading through 300 lines of boilerplate takes more effort than reading this macro.
macro_rules! tuple_implementation {
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

tuple_implementation!(T0);
tuple_implementation!(T0, T1);
tuple_implementation!(T0, T1, T2);
tuple_implementation!(T0, T1, T2, T3);
tuple_implementation!(T0, T1, T2, T3, T4);
tuple_implementation!(T0, T1, T2, T3, T4, T5);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_implementation!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
