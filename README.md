# simbin — simple, zero-dependency, big-endian binary serialization

**simbin** is a tiny (~180 LOC), `no_std`, zero-dependency, big-endian binary serialization library that just works.

- **Zero dependencies** – not even `std` (only `alloc` when you need `String`/`Vec`)
- **`#![no_std]` + `#![forbid(unsafe_code)]`**
- Big-endian everywhere (network/embedded friendly)
- Built-in support for all primitive integers, `bool`, `String`, `Vec<T>`, `Option<T>`, tuples up to 12 elements
- Simple trait, easy to implement for your own types
- Length prefix is `u32` → maximum container size **4,294,967,295** elements/bytes

If you want the smallest, simplest, most predictable binary format without pulling in half of crates.io, this is it.


## Example Usage

```rust
use simbin::{to_bytes, from_bytes};

let expected = (
    42_u32,
    Some(42u8),
    vec!["hello".to_string(), "world".to_string()],
);

let bytes = to_bytes(&expected).unwrap();
let (actual, bytes) = from_bytes::<(u32, Option<u8>, Vec<String>)>(&bytes).unwrap();

assert_eq!(expected, actual);
assert!(bytes.is_empty());
```
