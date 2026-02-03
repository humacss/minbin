//! Re-exports the convenience macros that reduce boilerplate when implementing
//! `ToFromBytes` for simple structs and enums.
//!
//! These macros are **optional**.
//! You can (and sometimes should) write the implementations by hand for full 
//! control and easier debugging.

/// Macro for implementing `Enum`s.
pub mod minbin_enum;
/// Macro for implementing `Struct`s.
pub mod minbin_struct;
