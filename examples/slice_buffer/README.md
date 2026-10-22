# Decoding into part of a fixed buffer

Wrap a subslice to update a selected region of a larger buffer. The encoded
element count must exactly match that region's length; otherwise decoding returns
`SliceLengthMismatch` before updating any elements.

See [the example source](main.rs) for applying incoming
updates to different regions of a larger buffer and continuing after a length mismatch:

```bash
cargo run --example slice_buffer --no-default-features
```

Other decoding errors can leave elements partially updated; decoding does not
roll back changes when an error occurs.

