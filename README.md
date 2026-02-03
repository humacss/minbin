[![Crates.io](https://img.shields.io/crates/v/minbin.svg)](https://crates.io/crates/minbin)
[![Docs](https://docs.rs/minbin/badge.svg)](https://docs.rs/minbin)

# minbin
When both sides are Rust and you just want your structs as bytes without committing to a more complex crate.

`minbin` is a tiny (~500 LOC), zero-dependency, safe, no-std binary serializer prioritizing predictability and auditability.
It is intended as a sensible default choice for new Rust projects before you know which trade-offs actually matter for your use-case.

`minbin` is a dependency you can audit in minutes, instead of untangling proc macros, dependency trees, and thousands of lines of code.
It is usually all you need until you hit a limitation that justifies committing to something bigger. 

Specialized crates are excellent when you have a concrete serialization bottleneck, `minbin` lets you defer that choice until you do.
`minbin` supports manual implementation and can handle millions of packets per second (see [benchmarks](#performance-in-practice)), that's often all you need.

## Usage

Install the crate:
```
cargo add minbin
```
Now use the helper macro to implement the [`ToFromBytes`](https://github.com/humacss/minbin/tree/main/src/core/to_from_bytes.rs) trait for your structs and enums.

### Enum example
```rust
#[derive(Debug, PartialEq)]
enum ExampleEnum {
    Ping,
    Temperature(i16),
    Location(i32, i32),
    Log{ time: i64, message: String }
}

minbin::minbin_enum!{ ExampleEnum [
    [0 => Self::Ping],
    [1 => Self::Temperature(degrees: i16)],
    [2 => Self::Location(lat: i32, lon: i32)],
    [3 => Self::Log{ time: i64, message: String }]
] }

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn test_enum_roundtrip() {
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
}

```

### Struct example
```rust
#[derive(Debug, PartialEq)]
struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<ExampleEnum>,
}

minbin::minbin_struct!{ ExampleStruct [
    self.uuid: u128,
    self.timestamp: i64,
    self.name: String,
    self.readings: Vec<ExampleEnum>
] }

#[cfg(test)]
mod tests {    
    use super::*;

    use minbin::{to_bytes, from_bytes};

    #[test]
    fn test_struct_roundtrip() {
        let expected = ExampleStruct {
            uuid: u128::MAX,
            timestamp: i64::MAX,
            name: "Name".to_string(),
            readings: vec![ExampleEnum::Ping],
        };
        let bytes = to_bytes(&expected).unwrap();
        let actual = from_bytes(&bytes).unwrap();
        assert_eq!(expected, actual);
    }
}
```

You can also implement the trait [manually](#manual-implementations) for complex cases not supported by the macro. The macro is just a convenience that helps you reduce boilerplate, manual implementations are straightforward and encouraged.

## Why yet another serializer?
Many Rust projects need to turn a struct into bytes and back at some point.

Yet almost every other crate forces you to commit on day one to features and trade-offs you probably don’t need yet.

Picking wrong means pain later: you either live with the wrong trade-offs forever or pay a heavy migration tax.

`minbin` refuses to make you choose.

It’s deliberately bare-bones, fully auditable, and has zero hidden behavior.

You write the serialization code yourself, so you always understand exactly what’s on the wire. 

When (and only when) you hit a real limitation, migration is trivial: rewrite the clearly defined serialization code you already own against a different crate.

Start with `minbin`.   
Ship code today.  

Only graduate to something heavier when you can name the precise problem you’re solving.

For most projects, that day comes later than you would expect.

## When to use `minbin`

- You control both sides of the wire
- Both sides are Rust (or you’re willing to write the glue)
- You value “I can read and fix this at 3 a.m. without Stack Overflow” over maximum performance or minimum message size
- Your messages are small and don't need to be streamed
- Serialization is not your bottleneck

Adding too many moving parts will just complicate your code unnecessarily.

That's why `minbin` focuses on simplicity first.

If you can write Rust, you can use and debug `minbin`.

## When NOT to use `minbin`

| You need                                   | Better alternatives                      |
|--------------------------------------------|------------------------------------------|
| zero-copy deserialization                  | `rkyv`                                   |
| smallest possible on-wire size             | `postcard`                               |
| cross-language support                     | `prost`, `flatbuffers`, `cap'n proto`    |
| serde integration                          | `bincode`, `postcard`                    |

`minbin` deliberately gives up all of the above to stay simple, auditable and predictable.

### Performance in practice 
These are my results when running the benches with Criterion on an Apple M3 Pro using Rust 1.90.0.

| Type                                             | Serialize | Deserialize |
|--------------------------------------------------|-----------|-------------|
| `u8-128`, `i8-i128`, `bool`                      | ~8 ns     | ~8 ns       |
| 100B Vec<u32>                                    | ~24 ns    | ~38 ns      |
| 100B `String`     (1 byte chars)                 | ~13 ns    | ~30 ns      |
| 100B `String`     (4 byte chars)                 | ~17 ns    | ~77 ns      |
| 200B struct       (includes 100B, 4 char string) | ~37 ns    | ~115 ns     |

*Deserialization of String includes UTF-8 validation.[^strings]*

[^strings]: Deserializing strings(`String`, `&str`) is often more expensive than other types due to UTF-8 validation. This cannot be avoided without using `unsafe`. To avoid this cost you can use a `Vec<u8>` representing the UTF-8 bytes instead.

Real-world takeaway:
- Serializing a 200 byte game packet: **~27 million times / second**
- Deserializing the same packet: **~8.7 million times / second**

That's on one core. This is usually fast enough that it won't be your bottleneck.

## Examples

### Manual implementations

Manual implementations are not difficult, it's just a lot of unnecessary boilerplate for very simple cases.

See the examples here:
    - [Structs](https://github.com/humacss/minbin/tree/main/examples/manual-struct)
    - [Enum](https://github.com/humacss/minbin/tree/main/examples/manual-enum)

For trivial cases the `minbin_struct!` and `minbin_enum!` macros work well. 
For anything complicated manual implementation is recommended. 

### Inspecting macro implementations
You can inspect the output of the code generated by the macro using the two macro examples.

Install cargo-expand on your system:
```bash
cargo install cargo-expand              
```

Output the code generated from [examples/macro-enum](https://github.com/humacss/minbin/tree/main/examples/macro-enum.rs)
```bash
cargo expand --example macro_enum    
```

Output the code generated from [examples/macro-struct](https://github.com/humacss/minbin/tree/main/examples/macro-struct.rs)
```bash
cargo expand --example macro_struct    
```

## Future work
- optional little-endian feature (currently only big-endian is supported)
- &[T] support (for primitive types without `unsafe`)
- more examples showcasing common use-cases (versioning, server)
