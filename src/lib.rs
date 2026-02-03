//! # Minbin
//! Minimal, manual, big-endian binary serialization.
//!
//! Designed for cases where you control both sides and value auditability over maximum speed or minimum size.
//!
//! You implement one small trait by hand. No derives, no macros, no hidden layout rules.
//!
//! This gives you full ownership of the wire format while keeping the crate tiny (<500 LOC), zero-dependency, and `no-std`.
//!
//! The API is mainly built around borrows (`&[u8]`, `&mut [u8]`) instead of owned buffers because:
//! - It avoids unnecessary cloning or moving data you already own.
//! - It works efficiently in `no-std` and embedded environments where allocation is expensive or impossible.
//! - It keeps lifetimes explicit and safe without forcing `'static` or owned types everywhere.
//!
//! Everything is deliberately simple and documented enough to read and debug at 3 a.m.
//!
//! # Size limits
//!
//! By default, all container types (`Option<T>`, `Vec<T>`, etc.) are limited to
//! **1 MiB** (1,048,576 bytes) of total serialized data.
//! This is a safeguard against memory exhaustion in constrained environments.
//!
//! Strings (`String` and `&str`) are capped at **100 KiB** (102,400 bytes) instead.
//! This is because deserializing a `String` requires a full UTF-8 validation pass,
//! which is significantly more expensive than just copying raw bytes.
//!
//! On the worst-case input (all 4-byte UTF-8 characters), a 100 KiB string deserializes
//! in roughly 50 µs on the same machine we used for benchmarking, keeping the worst case
//! reasonably fast and predictable.
//!
//! `minbin` is made for small, frequent packets. If you regularly send larger strings, you have several options:
//! - Override the default `String`/`&str` implementations and skip UTF-8 validation if you trust the source.
//! - Override them and validate incrementally or lazily instead of all at once.
//! - Send raw bytes directly in your trait definition and handle validation manually.
//! - Consider a more feature-rich crate with streaming support.
//!
//! ## How to Work With String and &str
//!
//! In general, you will want to work with `String` if you can.
//! `String` is slightly less performant but simplifies the API a **lot**.
//!
//! If you have to work with `&str` for performance reasons or because you don't have `alloc`,
//! then you will need to also keep the bytes buffer in memory for as long as the `&str` is,
//! because they use the same lifetime.
//!
//! This is often fine because you can just keep both in scope until you finish processing.
//! However, sometimes you need to keep the `&str` for longer than just one processing step, in which case you have 2 options:
//! - Keep the entire bytes buffer in memory for as long as you need the `&str`
//! - Write the `&str` bytes to a smaller, dedicated, byte buffer and read the `&str` again from that buffer.
//!   Keep both in memory for as long as the `&str` is needed.
//!
//! The borrow checker guarantees safety: you cannot use the `&str` after the buffer is dropped.
//! If you try, the code simply won’t compile.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Re-exports everything needed for typical usage.
pub mod core;
pub use core::{from_bytes, read_bytes, write_bytes, BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

/// Placeholder
pub mod macros;

// Implementations requiring the alloc crate.
#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::to_bytes;
