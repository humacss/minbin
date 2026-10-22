//! Predictable encodings for standard-library hash collections.
//!
//! The `hash` feature is enabled by default and enables `std` (and `alloc`).
//! Hash maps and sets are common, so Minbin provides these implementations as a
//! convenience with an explicit ordering policy.
//!
//! # Encoding choices
//!
//! Serialization collects references into a temporary vector and sorts them by
//! key (`HashMap`) or element (`HashSet`). It then writes a `u32` count followed
//! by the sorted entries. This requires `Ord`, additional allocation, and sorting
//! work even when the output buffer has already been provided by the caller.
//! Decoding requires `Default` and rejects duplicate keys or elements.
//!
//! For naturally ordered iteration without this temporary sorting step, use
//! `BTreeMap` or `BTreeSet` with the `alloc` feature.
//!
//! # Custom encodings
//!
//! To omit these implementations while retaining standard-library support, use:
//!
//! ```toml
//! minbin = { version = "1.0", default-features = false, features = ["std"] }
//! ```
//!
//! Define a local wrapper around your collection and implement `ToFromBytes` on
//! that wrapper to choose your own encoding. A wrapper works whether `hash` is
//! enabled or disabled. Rust's orphan rules prevent a downstream crate from
//! implementing this trait directly for standard-library collections. Cargo
//! features are additive, so another dependency can also enable `hash`.

pub mod hashmap;
pub mod hashset;

mod sort;
