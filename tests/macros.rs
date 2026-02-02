use minbin::{minbin_struct, minbin_enum, to_bytes, from_bytes};

#[derive(Debug, PartialEq)]
enum ExampleEnum {
    Ping,
    Temperature(i16),
    Location(i32, i32),
    Log{ time: i64, message: String }
}

minbin_enum!{ ExampleEnum [
    [0 => Self::Ping],
    [1 => Self::Temperature(degrees: i16)],
    [2 => Self::Location(lat: i32, lon: i32)],
    [3 => Self::Log{ time: i64, message: String }],
] }

#[derive(Debug, PartialEq)]
struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<ExampleEnum>,
}

minbin_struct!{ ExampleStruct [
    self.uuid: u128,
    self.timestamp: i64,
    self.name: String,
    self.readings: Vec<ExampleEnum>,
] }

#[test]
fn test_macros() {
    let expected = ExampleStruct{ 
        uuid: 0, 
        timestamp: 1, 
        name: "example".to_string(), 
        readings: vec![ExampleEnum::Ping, ExampleEnum::Temperature(1), ExampleEnum::Location(0, 1), ExampleEnum::Log{ time: 0, message: "Logging...".to_string() }]
    };
    let bytes = to_bytes(&expected).unwrap();
    let actual: ExampleStruct = from_bytes(&bytes).unwrap();

    assert_eq!(expected, actual);
}
