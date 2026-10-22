[![Crates.io](https://img.shields.io/crates/v/minbin.svg)](https://crates.io/crates/minbin)
[![Docs](https://docs.rs/minbin/badge.svg)](https://docs.rs/minbin)
[![Qlty | Maintainability ](https://qlty.sh/gh/humacss/projects/minbin/maintainability.svg)](https://qlty.sh/gh/humacss/projects/minbin)
[![Qlty | Code Coverage](https://qlty.sh/gh/humacss/projects/minbin/coverage.svg)](https://qlty.sh/gh/humacss/projects/minbin)

# minbin
When both sides are Rust and you just want your structs as bytes without committing to a more complex crate.

`minbin` is a small, zero-dependency, safe, no-std binary serializer with platform-agnostic encoding, prioritizing predictability and auditability.
It is intended as a sensible default choice for new Rust projects before you know which trade-offs actually matter for your use-case.

`minbin` is a dependency you can audit in minutes, instead of untangling proc macros, dependency trees, and thousands of lines of code.
It is usually all you need until you hit a limitation that justifies committing to something bigger. 

Specialized crates are excellent when you have a concrete serialization bottleneck, `minbin` lets you defer that choice until you do.
`minbin` supports manual implementation and can handle millions of packets per second (see [benchmarks](#performance-in-practice)), that's often all you need.

## Usage

Install the crate:
```sh
cargo add minbin
```
For most projects, start with the default standard-library path:
- implement [`ToFromBytes`](https://github.com/humacss/minbin/tree/main/src/core/to_from_bytes.rs) manually or with the helper macros
- call `to_bytes(&value)` to serialize
- call `from_bytes(Type::default, &bytes)` to deserialize

The default `std` and `hash` features provide strings, vectors, tree and hash
collections, and `to_bytes` out of the box. `std` includes `alloc`; `hash` includes
`std`. Hash collection encoding allocates temporary storage and sorts entries for
predictable output. See the [hash module documentation](https://docs.rs/minbin/1.0.0/minbin/hash/index.html)
for the tradeoffs and custom encoding options.

Without allocation, use `write_bytes` / `read_into` with your own buffers and
borrowed strings or `Slice<T>`. Rust 1.90 or newer is supported.

For simple structs and enums, the macros are the easiest place to start.
For anything non-trivial, manual implementations are straightforward and give you full control over the wire format.

### Enum example
```rust
use minbin::{from_bytes, to_bytes};

#[derive(Debug, PartialEq)]
enum ExampleEnum {
    Ping,
    Temperature(i16),
    Location(i32, i32),
    Log { time: i64, message: String }
}

impl Default for ExampleEnum {
    fn default() -> Self {
        ExampleEnum::Ping
    }
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
            let decoded = from_bytes(ExampleEnum::default, &bytes).unwrap();
            assert_eq!(expected, decoded);
        }
    }
}

```

### Struct example
```rust
use minbin::{from_bytes, to_bytes};

#[derive(Debug, PartialEq)]
struct ExampleStruct {
    uuid: u128,
    timestamp: i64,
    name: String,
    readings: Vec<String>,
}

impl Default for ExampleStruct {
    fn default() -> Self {
        Self {
            uuid: 0,
            timestamp: 0,
            name: String::new(),
            readings: Vec::new(),
        }
    }
}

minbin::minbin_struct!{ ExampleStruct [
    self.uuid: u128,
    self.timestamp: i64,
    self.name: String,
    self.readings: Vec<String>
] }

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn test_struct_roundtrip() {
        let expected = ExampleStruct {
            uuid: u128::MAX,
            timestamp: i64::MAX,
            name: "Name".to_string(),
            readings: vec!["Reading1".to_string()],
        };
        let bytes = to_bytes(&expected).unwrap();
        let actual = from_bytes(ExampleStruct::default, &bytes).unwrap();
        assert_eq!(expected, actual);
    }
}
```

You can also implement the trait [manually](#manual-implementations) for complex cases not supported by the macro. The macro is just a convenience that helps you reduce boilerplate, manual implementations are straightforward and encouraged.

### How to start

- If you want the shortest path to working code, start with `minbin_struct!` / `minbin_enum!`
- If you need custom validation or more control over the wire format, implement `ToFromBytes` manually
- If you do not want allocation, use `write_bytes` and `read_into` instead of `to_bytes`

See the Rust documentation for [initialization and custom defaults](https://docs.rs/minbin/1.0.0/minbin/core/api/index.html#initialization-and-custom-defaults)
and [wire format and compatibility](https://docs.rs/minbin/1.0.0/minbin/trait.ToFromBytes.html#wire-format-and-compatibility).

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

| Type | Serialize | Deserialize |
|---|---:|---:|
| `u8`–`u128`, `i8`–`i128`, `bool` | ~9–10 ns | ~9–10 ns |
| 104B `Vec<u32>` (25 elements) | ~21 ns | ~65 ns |
| 72B `String` (ASCII) | ~13 ns | ~49 ns |
| 118B `String` (emoji) | ~15 ns | ~104 ns |
| 226B struct (includes 118B emoji string) | ~31 ns | ~157 ns |
| `HashMap<u32, u32>` (25 entries) | ~183 ns | ~287 ns |
| `HashSet<u32>` (25 elements) | ~173 ns | ~304 ns |

*Deserialization of String includes UTF-8 validation.[^strings] Hash serialization includes sorting and temporary allocation.[^hash]*

[^strings]: Deserializing strings (`String`, `&str`) is often more expensive than other types due to UTF-8 validation. To avoid this cost you can use a `Vec<u8>` representing the UTF-8 bytes instead.

[^hash]: Hash maps and sets do not iterate in a predictable order. Minbin collects references into a temporary vector, sorts them by key or element, then writes them to the output buffer in that order. The keys and values are not cloned. Tree collections already iterate in sorted order and do not need this step.

Real-world takeaway:

- Serializing a 226 byte game packet: **~32 million times / second**
- Deserializing the same packet: **~6.4 million times / second**

That's on one core. This is usually fast enough that it won't be your bottleneck.

For individual results, sample layouts, and measurement details, see the
[core](benches/core/README.md), [alloc](benches/alloc/README.md),
[hash](benches/hash/README.md), and [macro](benches/macros/README.md) benchmark notes.

## Examples

### Manual implementations

Manual implementations are not difficult, it's just a lot of unnecessary boilerplate for very simple cases.

See the examples here:
- [Struct](https://github.com/humacss/minbin/tree/main/examples/manual_struct.rs)
- [Enum](https://github.com/humacss/minbin/tree/main/examples/manual_enum.rs)

For trivial cases the `minbin_struct!` and `minbin_enum!` macros work well. 
For anything complicated manual implementation is recommended. 

### Inspecting macro implementations
You can inspect the output of the code generated by the macro using the two macro examples.

Install cargo-expand on your system:
```bash
cargo install cargo-expand              
```

Output the code generated from [examples/macro_enum](https://github.com/humacss/minbin/tree/main/examples/macro_enum.rs)
```bash
cargo expand --example macro_enum    
```

Output the code generated from [examples/macro_struct](https://github.com/humacss/minbin/tree/main/examples/macro_struct.rs)
```bash
cargo expand --example macro_struct    
```

See [the slice buffer example](examples/slice_buffer/README.md) for decoding into a region of a fixed buffer.

## Tests and benchmarks

Integration tests and benchmarks have separate `core`, `alloc`, `hash`, and
`macros` targets. Run each with only its required Minbin features:

| Target | Integration tests | Benchmarks |
|---|---|---|
| Core | `cargo test --test core --no-default-features` | `cargo bench --bench core --no-default-features` |
| Macros | `cargo test --test macros --no-default-features` | `cargo bench --bench macros --no-default-features` |
| Alloc | `cargo test --test alloc --no-default-features --features alloc` | `cargo bench --bench alloc --no-default-features --features alloc` |
| Hash | `cargo test --test hash --no-default-features --features hash` | `cargo bench --bench hash --no-default-features --features hash` |

Allocation-dependent macro tests live in the alloc suite. The std feature adds
alloc support without additional implementations, so CI checks it as a feature
combination. `cargo test --all-features` runs all integration suites, library
unit tests, and documentation examples together.

The test and Criterion harnesses use the standard library, including when Minbin
is built without its `std` or `alloc` features. CI smoke-tests benchmark targets
with `-- --test`; it does not use timing results as a pass/fail threshold.

## Future work
- direct `&[T]` support beyond the existing mutable `Slice<T>` wrapper
- more examples showcasing common use-cases (versioning, server)

## Real-World Usage

Currently used in [Arcanex](https://arcanex.online/), a tickless event-driven multiplayer game engine, where server processes and clients exchange events using `minbin`.
