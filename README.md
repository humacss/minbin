# simbin
Just use this one.

When you control both sides, they’re using Rust, and you just want your struct as bytes without thinking too hard.

- You don’t want to open a new repo and see a workspace with a crate list.  
- You don’t want a dependency tree that scrolls.  
- You don’t want unsafe code.  
- You don’t want any derive/attribute magic.  
- You just want the bytes.  

Today.

```toml
# Cargo.toml
[dependencies]
simbin = "0.1"
```

```rust
use simbin::{BytesReader, BytesWriter, ToFromBytes, ToFromByteError};

struct ExampleStruct<'a> {
    uuid: u128,
    timestamp: i64,
    name: &'a str,
    reading: u16,
}

impl<'a> ToFromBytes<'a> for ExampleStruct<'a> {
    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write(&self.uuid)?;
        writer.write(&self.timestamp)?;
        writer.write(&self.name)?;
        writer.write(&self.reading)?;

        Ok(())
    }

    fn from_bytes(reader: &mut BytesReader<'a>) -> Result<(Self, usize), ToFromByteError> {
        let uuid      = reader.read()?;
        let timestamp = reader.read()?;
        let name      = reader.read()?;
        let reading   = reader.read()?;

        Ok((ExampleStruct { uuid, timestamp, name, reading }, reader.pos))
    }

    fn byte_count(&self) -> usize {
        self.uuid.byte_count() + 
        self.timestamp.byte_count() +
        self.name.byte_count() +
        self.reading.byte_count()
    }
}
```

```rust
#[cfg(test)]
mod tests {    
    use simbin::{to_bytes, from_bytes};

    #[test]
    fn test_example_struct() {
        let expected = ExampleStruct{ uuid: 0, timestamp: 1, name: "example", reading: 2 };

        let mut buffer = vec![0u8; expected.byte_count()];
        let write_pos = write_bytes(&expected, &mut buffer).unwrap();
        let (actual, read_pos): (ExampleStruct, usize) = read_bytes(&buffer[..write_pos]).unwrap();

        assert_eq!(expected.byte_count(),   read_pos);
        assert_eq!(expected.uuid,           actual.uuid);
        assert_eq!(expected.timestamp,      actual.timestamp);
        assert_eq!(expected.name,           actual.name);
        assert_eq!(expected.reading,        actual.reading);
    }
}
```

## Keep it Simple
`simbin` is a binary serializer that optimizes for humans first.

Performance? It is very fast, you just don't have to trade your sanity for cycles.

What you get with `simbin`:

- <300 lines of code you can audit
- no dependencies
- no unsafe code (forbidden)
- no-std Rust support
- no configuration files
- just one tiny trait you fully control

You own every byte, every time, with zero magic in the way.

If you have to talk to another system that forces a format on you, reach for something heavier.  

Otherwise, `simbin` is the serialization crate you can start with on any project.

You’ll understand it at 3 a.m., your bugfix will be ready before sunrise, and you’ll never hesitate to add it.

Start here.  
Stay here.

You probably won’t need anything else.

## Now that I have your attention...

`simbin` is **not** trying to replace `serde`, `postcard`, `bincode`, `rkyv`, `flatbuffers`, or anything else.  

Those tools are amazing when you need what they do.

`simbin` is deliberately for the case where:
- You control both sides of the wire
- Both sides are Rust (or you’re willing to write the glue)
- You value “I can read and fix this at 3 a.m, without Stack Overflow” over maximum performance or minimum message size
- Your messages are small and don't need to be streamed
- Serialization is not your bottleneck

Adding too many moving parts will just complicate your code unnecessarily.  

That's why `simbin` focuses on simplicity first.  

If you can write Rust, you can use and debug `simbin`.

### What `simbin` actually is
- Safe, `no_std`-compatible, zero-dependency Rust
- One trait you implement by hand (yes, really)
- Zero proc macros, zero derives, zero magic
- Big-endian by default
- Fixed width types
- Max u32 length containers

### Performance in practice 
These are my results when running the benches with Criterion on an Apple M3 Pro using Rust 1.90.0.

| Type                                       | Serialize          | Deserialize        |
|--------------------------------------------|--------------------|--------------------|
| `u8-128`, `bool`                           | ~2.5 ns            | ~7.2 ns            |
| 100B `&str`                                | ~5.5 ns            | ~66 ns             |
| 78B struct (50B string + primitives).      | ~7 ns              | ~40 ns             |

*Deserialization of &str includes UTF-8 validation. A 100-byte string therefore costs ~66 ns.*

Real-world takeaway:
- Serializing a 78 byte game packet: **~143 million times / second**
- Deserializing the same packet: **~25 million times / second**

That's on one core. This should be faster than your other bottlenecks.

### Why the tone up top?
Most serialization crates greet you with feature lists, configuration options, gotchas...  

I usually have to read half the docs before I even know whether the crate is a good fit.

I wanted you to get the answer in ten seconds flat: “Is this for me? Yes or no?”

If you’re still here, I hope you already know which camp you’re in.  

If `simbin` fits your need, I hope it saves you a bunch of time and a few sleepless nights.

### If `simbin` isn’t for you, here are some alternatives

- You need maximum performance or zero-copy: **rkyv**
- You need tiny message size, no-std or serde: **postcard**
- You want to "just throw any Rust type at it": **bincode**
- You have to talk to other languages or need schema evolution: **protobuf**, **flatbuffers**, or **cap’n proto**

`simbin` deliberately gives up all of the above to stay simple, auditable and predictable.

## Why yet another serializer?

Many Rust projects need to turn a struct into bytes and back at some point.

Yet almost every other crate forces you to commit on day one to features you probably don’t need yet:

- schema evolution
- zero-copy deserialization
- cross-language support
- versioning
- compact wire format

Picking wrong means pain later: you either live with the wrong trade-offs forever or pay a heavy migration tax.

`simbin` refuses to make you choose.

It’s deliberately bare-bones, fully auditable, and has zero hidden behavior. 

You write the serialization code yourself, so you always understand exactly what’s on the wire. 

When (and only when) you hit a real limitation, migration is trivial: rewrite the clearly defined serialization code you already own against a different crate.

Start with `simbin`.   
Ship code today.  

Only graduate to something heavier when you can name the precise problem you’re solving.

For most projects, that day comes later than you would expect.
