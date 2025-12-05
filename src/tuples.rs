use crate::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter};

// Get rid of the macro
macro_rules! impl_tuple {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<'de, $($T: ToFromBytes<'de>),+> ToFromBytes<'de> for ($($T,)+) {
            #[inline(always)]
            fn to_bytes(&self, writer: &mut BytesWriter<'de>) -> Result<(), ToFromByteError> {
                let ($($T,)+) = self;
                
                $($T.to_bytes(writer)?;)+
                
                Ok(())
            }

            #[inline(always)]
            fn from_bytes(reader: &mut BytesReader<'de>) -> Result<(Self, usize), ToFromByteError> {
                let value = ($( {
                    let (item, _) = $T::from_bytes(reader)?;

                    item
                }, )+);

                Ok((value, reader.pos))
            }

            #[inline(always)]
            fn byte_count(&self) -> usize {
                let ($($T,)+) = self;

                0 $(+ $T.byte_count())+
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
