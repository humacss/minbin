use minbin::{from_bytes, to_bytes, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

#[derive(Debug, PartialEq)]
enum ExampleEnum {
    Ping,
    Temperature(i16),
    Location(i32, i32),
    Log { time: i64, message: String },
}

impl<'a> ToFromBytes<'a> for ExampleEnum {
    const MAX_BYTES: usize = 1_048_576;

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        match self {
            Self::Ping => {
                writer.write::<u8>(&0)?;
            }
            Self::Temperature(degrees) => {
                writer.write::<u8>(&1)?;

                writer.write::<i16>(&degrees)?;
            }
            Self::Location(lat, lon) => {
                writer.write::<u8>(&2)?;

                writer.write::<i32>(&lat)?;
                writer.write::<i32>(&lon)?;
            }
            Self::Log { time, message } => {
                writer.write::<u8>(&3)?;

                writer.write::<i64>(&time)?;
                writer.write::<String>(&message)?;
            }
        }

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), minbin::ToFromByteError> {
        let discriminant = reader.read::<u8>()?;

        match discriminant {
            0 => Ok((Self::Ping, reader.pos)),
            1 => {
                let degrees = reader.read::<i16>()?;

                Ok((Self::Temperature(degrees), reader.pos))
            }
            2 => {
                let lat = reader.read::<i32>()?;
                let lon = reader.read::<i32>()?;

                Ok((Self::Location(lat, lon), reader.pos))
            }
            3 => {
                let time = reader.read::<i64>()?;
                let message = reader.read::<String>()?;

                Ok((Self::Log { time, message }, reader.pos))
            }
            _ => Err(ToFromByteError::UnhandledEnumArm),
        }
    }

    fn byte_count(&self) -> usize {
        1 + match self {
            Self::Ping => 0,
            Self::Temperature(..) => 2,
            Self::Location(..) => 8,
            Self::Log { message, .. } => 8 + message.byte_count(),
        }
    }
}

fn main() {
    let cases = [
        ExampleEnum::Ping,
        ExampleEnum::Temperature(-18),
        ExampleEnum::Location(48_856_614, 2_352_221),
        ExampleEnum::Log { time: 1_738_585_200, message: "Server started".to_string() },
    ];

    for expected in cases {
        let bytes = to_bytes(&expected).unwrap();
        let decoded = from_bytes(&bytes).unwrap();
        assert_eq!(expected, decoded);
    }
}
