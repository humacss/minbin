//! # simbin — simple, zero-dependency, big-endian binary serialization
//!
//! `simbin` is a tiny (~180 LOC), `no_std`-compatible, big-endian serialization library
//! with zero dependencies and no unsafe code.
//!
//! ## Design goals
//!
//! - As few dependencies as possible (zero when possible)
//! - Predictable big-endian layout (perfect for network and embedded)
//! - Simple, composable trait
//! - Work everywhere: `no_std`, wasm, embedded, desktop
//!
//! ## Supported types
//!
//! - All primitive integers (`u8` … `u128`, signed and unsigned) — big-endian
//! - `bool`
//! - `String` (UTF-8 validated)
//! - `Vec<T>` where `T: ToFromBytes` (length prefixed with `u32`)
//! - `Option<T>` (1-byte discriminant)
//! - Tuples up to 12 elements
//! - Any custom type that implements `ToFromBytes`
//!
//! ## Important limitation
//!
//! Container lengths (`String`, `Vec<T>`, etc.) are prefixed with a **u32**.
//! This means the maximum length is 4,294,967,295 bytes/elements.
//! This will be made configurable in a future version.
//!
//! ## Example
//!
//! ```rust
//! use simbin::{to_bytes, from_bytes};
//! 
//! let expected = (
//!     42_u32,
//!     Some(42u8),
//!     vec!["hello".to_string(), "world".to_string()],
//! );
//! 
//! let bytes = to_bytes(&expected).unwrap();
//! let (actual, bytes) = from_bytes::<(u32, Option<u8>, Vec<String>)>(&bytes).unwrap();
//! 
//! assert_eq!(expected, actual);
//! assert!(bytes.is_empty());
//! ```
//!
//! ## Implementing for your own types
//!
//! ```rust
//! use simbin::{ToFromBytes, ToFromByteError, to_bytes, from_bytes};
//!
//! struct Player { id: u32, name: String, health: u8 }
//!
//! impl ToFromBytes for Player {
//!     fn to_bytes(&self) -> Result<Vec<u8>, ToFromByteError> {
//!         let mut out = Vec::new();
//!
//!         out.extend(to_bytes(&self.id)?);
//!         out.extend(to_bytes(&self.name)?);
//!         out.extend(to_bytes(&self.health)?);
//!
//!         Ok(out)
//!     }
//!
//!     fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), ToFromByteError> {
//!         let (id, b) = from_bytes(bytes)?;
//!         let (name, b) = from_bytes(b)?;
//!         let (health, b) = from_bytes(b)?;
//!
//!         Ok((Player { id, name, health }, b))
//!     }
//! }
//! ```

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
/// Test helpers used for asserting accurate serializations.
pub mod test;

pub use error::ToFromByteError;
pub use bytes::{ToFromBytes, to_bytes, from_bytes};
