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
//! ## How to Work With String and &str
//!
//! In general, you will want to work with `String` if you can. 
//! `String` is slightly less performant but simplifies the API a **lot**.
//!
//! If you have to work with `&str` for performance reasons or because you don't have `alloc`, 
//! then you will need to also keep the bytes buffer in memory for as long as the `&str` is, 
//! because the lifetimes are tied.
//! 
//! This is often fine because you can just keep both in scope until you finish processing. 
//! However, sometimes you need to keep the `&str` for longer, in which case you have 2 options:
//! - Keep the entire bytes buffer in memory for as long as you need the `&str`
//! - Write the `&str` bytes to a smaller, dedicated, byte buffer and read the `&str` again from that buffer. 
//!   Keep both in memory for as long as the `&str` is needed.
//!
//! The borrow checker guarantees safety: you cannot use the `&str` after the buffer is dropped. 
//! If you try, the code simply won’t compile.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

// The core implementation, always included
pub mod core;
pub use core::{ToFromBytes, ToFromByteError, BytesReader, BytesWriter, read_bytes, write_bytes, from_bytes};

// Implementations requiring the alloc crate.
#[cfg(feature = "alloc")]
pub mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::{to_bytes};