#![no_std]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

// The core implementation, always included
pub mod core;
pub use core::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter, write_bytes, read_bytes};

// The core implementation, only with alloc feature
#[cfg(feature = "alloc")]
pub mod alloc;