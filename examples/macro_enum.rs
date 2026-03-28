use minbin::{from_bytes, to_bytes};

#[derive(Debug, PartialEq)]
enum ExampleEnum {
    Ping,
    Temperature(i16),
    Location(i32, i32),
    Log { time: i64, message: String },
}

#[allow(clippy::derivable_impls)]
impl Default for ExampleEnum {
    fn default() -> Self {
        ExampleEnum::Ping
    }
}

minbin::minbin_enum! { ExampleEnum [
    [0 => Self::Ping],
    [1 => Self::Temperature(degrees: i16 = 0)],
    [2 => Self::Location(lat: i32 = 0, lon: i32 = 0)],
    [3 => Self::Log{ time: i64 = 0, message: String = String::new() }]
] }

fn main() {
    let cases = [
        ExampleEnum::Ping,
        ExampleEnum::Temperature(-18),
        ExampleEnum::Location(48_856_614, 2_352_221),
        ExampleEnum::Log { time: 1_738_585_200, message: "Server started".to_string() },
    ];

    for expected in cases {
        let bytes = to_bytes(&expected).unwrap();
        let decoded = from_bytes(ExampleEnum::default, &bytes).unwrap();
        assert_eq!(expected, decoded);
    }
}
