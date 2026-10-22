# Hash benchmarks

Standard-library hash collections, with Minbin's `hash` feature enabled. That
feature also enables std and alloc.

```sh
cargo bench --bench hash --no-default-features --features hash
```

Criterion point estimates from the October 22, 2026 local run with Rust 1.90.0.
All times are nanoseconds per operation.

| Sample | Serialized bytes | Serialize | Deserialize |
|---|---:|---:|---:|
| Map, 25 × `(u32, u32)` | 204 | 182.63 | 286.65 |
| Set, 25 × `u32` | 104 | 173.38 | 304.09 |

Map keys are 0 through 24, with each value twice its key. The set contains 0
through 24. Both encodings add a four-byte element-count prefix.

Serialization collects references into a temporary vector and sorts by key or
set element before writing. Its temporary allocation and sorting are included
in the measured time. Decoding builds a new collection using the standard
library's default hasher; allocation, hashing, insertion, and cleanup are also
included. Hash seeds can differ between runs.

As with the other suites, the [shared harness](../support.rs) prepares output
buffers and clones input outside the timed section. Timed closures include the
serialization or decoding call and cleanup. Each case uses a three-second
warm-up, five-second measurement window, and 100 samples. Results depend on the
machine and load; CI smoke checks do not impose performance thresholds.
