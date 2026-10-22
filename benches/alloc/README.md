# Alloc benchmarks

Strings, a vector, and a manually implemented struct, with only Minbin's `alloc`
feature enabled.

```sh
cargo bench --bench alloc --no-default-features --features alloc
```

Criterion point estimates from the October 22, 2026 local run with Rust 1.90.0.
All times are nanoseconds per operation.

| Sample | Serialized bytes | Serialize | Deserialize |
|---|---:|---:|---:|
| ASCII string | 76 | 13.25 | 48.71 |
| Emoji string | 122 | 14.69 | 103.77 |
| Vector, 25 × `u32` | 104 | 21.19 | 65.37 |
| Mixed struct | 226 | 31.09 | 157.08 |

The ASCII string has 72 UTF-8 bytes. The emoji string has 118 UTF-8 bytes, with a
mix of three- and four-byte Unicode scalar values. Each serialized string adds a
four-byte length prefix. The vector contains integers 0 through 24, plus a
four-byte element-count prefix.

The [mixed struct](struct_value.rs) contains a `u128`, an `i64`, the emoji string,
and 19 `u32` readings: 16 + 8 + 122 + 80 = 226 serialized bytes. Criterion's
historical group labels (`String_100b_*`, `Vec_100b_u32`, and `struct, 200b`) are
retained for comparison with earlier runs; the sizes above describe the actual
samples.

The [shared harness](../support.rs) measures `write_bytes` and `read_bytes`, not
the allocating `to_bytes` helper. Output-buffer allocation and input cloning are
outside the timed section. Decoded strings and vectors are constructed from
`Default`; their allocation, UTF-8 validation, and cleanup are included in the
timed closure. Serialization includes cleanup of its prepared output buffer.

Each case uses a three-second warm-up, five-second measurement window, and 100
samples. Results depend on the machine and load. CI smoke checks do not impose
performance thresholds.
