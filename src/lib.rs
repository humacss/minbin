//! `simbin` is a simple, zero-dependency, big-endian binary serializer optimized for ease of use and auditability.
//! 
//! It is designed for scenarios where you control both sides of the serialization, both are in Rust, and simplicity is prioritized over advanced features like versioning or cross-language support.
//! See the [README](https://github.com/humacss/simbin) for examples and more details.


#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

/// Contains all error states for the crate.
pub mod error;
/// The trait used for serialization, implement the trait for serialization support.
pub mod bytes;
/// ToFromBytes trait implementations for primitive types.
pub mod primitives;
/// ToFromBytes trait implementations for container types.
pub mod containers;
/// ToFromBytes trait implementations for tuples;
pub mod tuples;
/// Used for traversing a byte slice for reading and writing.
pub mod cursors;

pub use error::ToFromByteError;
pub use bytes::{ToFromBytes, to_bytes, from_bytes};
pub use cursors::{BytesReader, BytesWriter};
