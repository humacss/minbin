use std::collections::{BTreeMap, BTreeSet};

use minbin::{from_bytes, to_bytes, ToFromBytes};

#[test]
fn btree_map_round_trips() {
    let mut value = BTreeMap::new();
    value.insert(2_u16, "two".to_string());
    value.insert(1_u16, "one".to_string());

    let bytes = to_bytes(&value).expect("serialize map");
    let decoded = from_bytes::<BTreeMap<u16, String>>(BTreeMap::new, &bytes).expect("deserialize map");

    assert_eq!(decoded, value);
}

#[test]
fn btree_set_round_trips() {
    let mut value = BTreeSet::new();
    value.insert(3_u16);
    value.insert(1_u16);
    value.insert(2_u16);

    let bytes = to_bytes(&value).expect("serialize set");
    let decoded = from_bytes::<BTreeSet<u16>>(BTreeSet::new, &bytes).expect("deserialize set");

    assert_eq!(decoded, value);
}

#[test]
fn empty_btree_collections_round_trip() {
    let map = BTreeMap::<u16, String>::new();
    let set = BTreeSet::<u16>::new();

    let map_bytes = to_bytes(&map).expect("serialize empty map");
    let set_bytes = to_bytes(&set).expect("serialize empty set");

    assert_eq!(from_bytes::<BTreeMap<u16, String>>(BTreeMap::new, &map_bytes).expect("deserialize empty map"), map);
    assert_eq!(from_bytes::<BTreeSet<u16>>(BTreeSet::new, &set_bytes).expect("deserialize empty set"), set);
}

#[test]
fn btree_collection_byte_count_matches_serialized_length() {
    let mut map = BTreeMap::new();
    map.insert(7_u16, "seven".to_string());
    map.insert(11_u16, "eleven".to_string());

    let mut set = BTreeSet::new();
    set.insert(7_u16);
    set.insert(11_u16);

    let map_bytes = to_bytes(&map).expect("serialize map");
    let set_bytes = to_bytes(&set).expect("serialize set");

    assert_eq!(map.byte_count(), map_bytes.len());
    assert_eq!(set.byte_count(), set_bytes.len());
}
