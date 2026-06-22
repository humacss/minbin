use ::std::collections::{HashMap, HashSet};

use minbin::{from_bytes, to_bytes};

#[test]
fn hash_map_round_trips() {
    let mut value = HashMap::new();
    value.insert(2_u16, "two".to_string());
    value.insert(1_u16, "one".to_string());

    let bytes = to_bytes(&value).expect("serialize map");
    let decoded = from_bytes::<HashMap<u16, String>>(HashMap::new, &bytes).expect("deserialize map");

    assert_eq!(decoded, value);
}

#[test]
fn hash_set_round_trips() {
    let mut value = HashSet::new();
    value.insert(3_u16);
    value.insert(1_u16);
    value.insert(2_u16);

    let bytes = to_bytes(&value).expect("serialize set");
    let decoded = from_bytes::<HashSet<u16>>(HashSet::new, &bytes).expect("deserialize set");

    assert_eq!(decoded, value);
}

#[test]
fn hash_collections_serialize_deterministically() {
    let mut first_map = HashMap::new();
    first_map.insert(2_u16, "two".to_string());
    first_map.insert(1_u16, "one".to_string());

    let mut second_map = HashMap::new();
    second_map.insert(1_u16, "one".to_string());
    second_map.insert(2_u16, "two".to_string());

    let mut first_set = HashSet::new();
    first_set.insert(2_u16);
    first_set.insert(1_u16);

    let mut second_set = HashSet::new();
    second_set.insert(1_u16);
    second_set.insert(2_u16);

    assert_eq!(to_bytes(&first_map).expect("serialize first map"), to_bytes(&second_map).expect("serialize second map"));
    assert_eq!(to_bytes(&first_set).expect("serialize first set"), to_bytes(&second_set).expect("serialize second set"));
}

#[test]
fn empty_hash_collections_round_trip() {
    let map = HashMap::<u16, String>::new();
    let set = HashSet::<u16>::new();

    let map_bytes = to_bytes(&map).expect("serialize empty map");
    let set_bytes = to_bytes(&set).expect("serialize empty set");

    assert_eq!(from_bytes::<HashMap<u16, String>>(HashMap::new, &map_bytes).expect("deserialize empty map"), map);
    assert_eq!(from_bytes::<HashSet<u16>>(HashSet::new, &set_bytes).expect("deserialize empty set"), set);
}
