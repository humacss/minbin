//! **minbin**: Minimal, manual, big-endian binary serialization.
//!
//! Designed for cases where you control both sides and value auditability over maximum speed or minimum size.
//! You implement one small trait by hand – no derives, no macros, no hidden layout rules.
//! This gives you full ownership of the wire format while keeping the crate tiny (<300 LOC), zero-dependency, and `no-std`.
//!
//! The API is built around borrows (`&[u8]`, `&mut [u8]`) instead of owned buffers because:
//! - It avoids unnecessary cloning or moving data you already own.
//! - It works efficiently in `no-std` and embedded environments where allocation is expensive or impossible.
//! - It keeps lifetimes explicit and safe without forcing `'static` or owned types everywhere.
//!
//! Everything is deliberately simple enough to read and debug at 3 a.m.

#![no_std]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

// The core implementation, always included
pub mod core;
pub use core::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter, read_bytes, write_bytes, from_bytes};

// The core implementation, only with alloc feature
#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::{to_bytes};