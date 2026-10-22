# Core benchmarks

Fixed-width integers and booleans, with Minbin's optional features disabled.

```sh
cargo bench --bench core --no-default-features
```

Criterion point estimates from the October 22, 2026 local run with Rust 1.90.0.
All times are nanoseconds per operation.

| Value | Serialize | Deserialize |
|---|---:|---:|
| `u8` | 9.38 | 8.90 |
| `i8` | 9.24 | 8.74 |
| `u16` | 9.36 | 9.43 |
| `i16` | 9.27 | 9.43 |
| `u32` | 9.25 | 8.84 |
| `i32` | 9.27 | 8.75 |
| `u64` | 9.36 | 8.81 |
| `i64` | 9.34 | 8.83 |
| `u128` | 9.28 | 9.67 |
| `i128` | 9.41 | 9.78 |
| `true` | 9.34 | 8.76 |
| `false` | 9.29 | 8.75 |

Integer samples contain 42. Each encoding occupies the integer's fixed byte
width; a boolean occupies one byte. These types take roughly 9–10 ns in this run.

The shared [measurement harness](../support.rs) uses `write_bytes` and `read_bytes`.
Output-buffer allocation and input-buffer cloning happen in Criterion's untimed
setup. The timed closures include the call and cleanup; decoding constructs the
value through `Default`. The benchmark executable uses std even though Minbin's
std and alloc features are disabled.

Each case uses Criterion's default three-second warm-up, five-second measurement
window, and 100 samples. Results depend on the machine and load; small differences
between runs need not indicate a regression. CI uses `-- --test` for smoke checks
without measuring timing.
