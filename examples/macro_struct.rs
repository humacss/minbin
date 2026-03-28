use minbin::{from_bytes, to_bytes};

#[derive(Debug, PartialEq)]
struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<String>,
}

#[allow(clippy::derivable_impls)]
impl Default for ExampleStruct {
    fn default() -> Self {
        Self { uuid: 0, timestamp: 0, name: String::new(), readings: Vec::new() }
    }
}

minbin::minbin_struct! { ExampleStruct [
    self.uuid: u128,
    self.timestamp: i64,
    self.name: String,
    self.readings: Vec<String>
] }

fn main() {
    let expected = ExampleStruct {
        uuid: u128::MAX,
        timestamp: i64::MAX,
        name: "Name".to_string(),
        readings: vec!["Reading1".to_string(), "Reading2".to_string()],
    };
    let bytes = to_bytes(&expected).unwrap();
    let actual = from_bytes(ExampleStruct::default, &bytes).unwrap();
    assert_eq!(expected, actual);
}
