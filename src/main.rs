use minbin::to_from_bytes;


struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<u16>,
}


to_from_bytes!{ ExampleStruct [
    self.uuid: u128,
    self.timestamp: i64,
    self.name: String,
    self.readings: Vec<u16>
] }

fn main() {
    
}