//! # Minbin
//! Minimal, manual, platform-agnostic binary serialization.
//!
//! Predictable encoding across platforms, with a wire format you own.
//! Built-in types use explicit encodings rather than native memory layouts.
//! Your implementation defines the fields, their order, and any custom encoding.
//!
//! Designed for cases where you control both sides and value auditability over maximum speed or minimum size.
//!
//! You implement one small trait by hand or with the helper macros. No derives, no hidden layout rules.
//!
//! This gives you full ownership of the wire format while keeping the crate small, zero-dependency, and `no-std`.
//!
//! # Quick Start
//!
//! The easiest path for a typical Rust application is:
//! - implement [`ToFromBytes`] manually or with [`minbin_struct!`] / [`minbin_enum!`]
//! - serialize with `to_bytes` (available with `alloc`, included by the default `std` feature)
//! - deserialize with [`from_bytes`], passing an initializer such as `MyType::default`
//!
//! If you do not want allocation, use [`write_bytes`] and [`read_into`] with your own byte buffers instead.
//! The core API is always buffer-based; the alloc and std APIs are convenience layers on top.
//!
//! # Further reading
//!
//! - [Initialization and custom defaults](core::api#initialization-and-custom-defaults)
//! - [Wire format and compatibility](ToFromBytes#wire-format-and-compatibility)
//! - [Serialized-size limits](ToFromBytes::MAX_BYTES)
//! - [Allocation-free slices](core::containers::slice::Slice)
//! - [Borrowed strings and lifetimes](core::containers::str)

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std as rust_std;

/// Re-exports everything needed for typical usage.
pub mod core;
pub use core::{bytes_into, from_bytes, read_bytes, read_into, write_bytes, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Convenience macros for implementing `ToFromBytes` on structs and enums.
pub mod macros;

/// Implementations requiring the alloc crate.
#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::to_bytes;

/// Implementations requiring the standard library.
#[cfg(feature = "std")]
pub mod std;

/// Predictable hash collection encodings with explicit sorting and allocation.
#[cfg(feature = "hash")]
pub mod hash;
