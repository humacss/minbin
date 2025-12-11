//! Core functionality – always available, no allocation required.
//!
//! Each module is kept separate so you can see exactly why it exists and jump straight to the code that matters.
//! The design favors explicit borrows and small, composable pieces over convenience that would hide details.

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
