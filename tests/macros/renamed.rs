// Shadow the original crate name so generated paths must use the defining crate.
extern crate minbin as wire;
mod minbin {}

#[derive(Debug, Default, PartialEq)]
struct Packet {
    id: u32,
}

wire::minbin_struct! { Packet [self.id: u32] }

#[derive(Debug, Default, PartialEq)]
enum Event {
    #[default]
    Ping,
    Reading(u16),
    Update {
        id: u32,
    },
}

wire::minbin_enum! { Event [
    [0 => Self::Ping],
    [1 => Self::Reading(value: u16)],
    [2 => Self::Update { id: u32 }],
] }

#[test]
fn struct_macro_works_through_a_crate_alias() {
    let expected = Packet { id: 42 };
    let mut buffer = [0u8; 4];
    let written = wire::write_bytes(&expected, &mut buffer).unwrap();
    assert_eq!(wire::from_bytes(Packet::default, &buffer[..written]).unwrap(), expected);
}

#[test]
fn enum_macro_works_through_a_crate_alias() {
    for expected in [Event::Ping, Event::Reading(42), Event::Update { id: 123 }] {
        let mut buffer = [0u8; 5];
        let written = wire::write_bytes(&expected, &mut buffer).unwrap();
        assert_eq!(wire::from_bytes(Event::default, &buffer[..written]).unwrap(), expected);
    }
}
