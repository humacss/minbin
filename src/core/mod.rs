//! Core functionality, always included, no allocation required.
//!
//! Everything in this module works with `&[u8]` / `&mut [u8]` borrows only.
//!
//! Why? Because owning a `Vec<u8>` for every packet is inefficient and `no-std` 
//! environments often can’t allocate at all.
//!
//! By forcing borrows we get zero-cost, zero-allocation, and explicit lifetimes.
//! This gives us close to as good performance as we can get with safe Rust.

/// The public API intended for consumption
pub mod api;
/// ToFromBytes trait implementations for container types.
pub mod containers;
/// Contains all error states for the crate.
pub mod error;
/// ToFromBytes trait implementations for primitive types.
pub mod primitives;
/// Used for traversing a byte slice for reading.
pub mod reader;
/// The trait used for serialization, implement the trait for serialization support.
pub mod to_from_bytes;
/// ToFromBytes trait implementations for tuples;
pub mod tuples;
/// Used for traversing a byte slice for writing.
pub mod writer;

pub use api::{read_bytes, write_bytes, from_bytes};
pub use error::{ToFromByteError};
pub use to_from_bytes::{ToFromBytes};
pub use reader::{BytesReader};
pub use writer::{BytesWriter};
