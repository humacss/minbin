//! Only compiled when the `alloc` feature (enabled by default) is active.
//!
//! This module exists because a majority of applications run on platforms that can allocate.
//!
//! When you're prototyping, writing tests, or just want the simplest possible API,
//! you don't want to manually manage a 256-byte stack buffer.
//!
//! All functions here allocate the exact number of bytes needed using `byte_count()`,
//! so there's no overallocation or guessing.
//!
//! Implements the common alloc types String and Vec.
//!
//! Also provides the to_bytes convenience function that abstracts away the buffer entirely.

/// Contains alloc only API functions.
pub mod api;
pub mod owned;

pub use api::{to_bytes};
