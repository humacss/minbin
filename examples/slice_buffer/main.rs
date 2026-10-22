//! Decode a fixed-length sequence into a selected region of reusable storage.
//!
//! Run with: cargo run --example slice_buffer --no-default-features
//! The example executable uses std for printing; the Minbin calls need no alloc.

use minbin::core::containers::slice::Slice;
use minbin::{bytes_into, write_bytes, ToFromByteError};

fn main() -> Result<(), ToFromByteError> {
    let mut readings = [10u32, 20];
    let mut bytes = [0u8; 12]; // u32 element count + two u32 values
    let written = write_bytes(&Slice(&mut readings), &mut bytes)?;
    let message = &bytes[..written];

    // The destination region must match the encoded element count, even when
    // the backing storage is larger. Select the region using normal slicing.
    let mut storage = [99u32; 5];
    bytes_into(&mut Slice(&mut storage[..2]), message)?;
    assert_eq!(storage, [10, 20, 99, 99, 99]);
    println!("Update the first two slots: {storage:?}");

    let mut storage = [99u32; 5];
    bytes_into(&mut Slice(&mut storage[2..4]), message)?;
    assert_eq!(storage, [99, 99, 10, 20, 99]);
    println!("Update two slots starting at index 2: {storage:?}");

    // A destination that is too large or too small returns the same error.
    // The count is checked before decoding elements, so storage is unchanged.
    let mut storage = [99u32; 5];
    let error = bytes_into(&mut Slice(&mut storage), message).unwrap_err();
    assert_eq!(error, ToFromByteError::SliceLengthMismatch);
    assert_eq!(storage, [99; 5]);
    println!("Two elements into five slots: {error}");

    let error = bytes_into(&mut Slice(&mut storage[..1]), message).unwrap_err();
    assert_eq!(error, ToFromByteError::SliceLengthMismatch);
    assert_eq!(storage, [99; 5]);
    println!("Two elements into one slot: {error}");

    // A matching count does not guarantee a complete payload. Here the second
    // u32 is missing. Decoding is not transactional: the first slot is updated.
    let error = bytes_into(&mut Slice(&mut storage[..2]), &message[..8]).unwrap_err();
    assert_eq!(error, ToFromByteError::NotEnoughBytes);
    assert_eq!(storage, [10, 99, 99, 99, 99]);
    println!("Truncated payload: {error}; storage: {storage:?}");

    // bytes_into expects exactly one value. Extra bytes are rejected after
    // the elements have been decoded. Use read_into for incremental parsing.
    let mut extra = [0u8; 13];
    extra[..written].copy_from_slice(message);
    let error = bytes_into(&mut Slice(&mut storage[..2]), &extra).unwrap_err();
    assert_eq!(error, ToFromByteError::TrailingBytes);
    assert_eq!(storage, [10, 20, 99, 99, 99]);
    println!("Extra byte after the payload: {error}; storage: {storage:?}");

    // Element validation errors also propagate. A bool must be encoded as 0 or 1.
    let mut flags = [false];
    let error = bytes_into(&mut Slice(&mut flags), &[0, 0, 0, 1, 2]).unwrap_err();
    assert_eq!(error, ToFromByteError::InvalidValue);
    println!("Invalid bool element: {error}");

    Ok(())
}
