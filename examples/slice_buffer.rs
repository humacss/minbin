//! Apply incoming updates to different regions of a reusable sensor buffer.
//!
//! Run with: cargo run --example slice_buffer --no-default-features
//! The executable uses std for printing; the Minbin calls need no alloc.

use minbin::core::containers::slice::Slice;
use minbin::{bytes_into, write_bytes, ToFromByteError};

fn main() -> Result<(), ToFromByteError> {
    let mut sensors = [0u32; 16];
    let mut buffer = [0u8; 64];
    println!("Initial readings: {sensors:?}");

    // Each input supplies new readings and the destination region to update.
    // One input deliberately selects three slots for only two readings.
    let inputs: [(core::ops::Range<usize>, &mut [u32]); 4] =
        [(0..3, &mut [21, 22, 23]), (8..10, &mut [31, 32]), (4..7, &mut [41, 42]), (12..16, &mut [51, 52, 53, 54])];

    for (region, readings) in inputs {
        // Encode each incoming message without allocating an output buffer.
        let written = write_bytes(&Slice(readings), &mut buffer)?;
        let message = &buffer[..written];

        match bytes_into(&mut Slice(&mut sensors[region.clone()]), message) {
            Ok(()) => println!("Updated {region:?}: {sensors:?}"),
            Err(ToFromByteError::SliceLengthMismatch) => {
                println!("Skipped {region:?}: incoming count does not match the destination length");
            }
            Err(error) => {
                // Other decoding errors can leave the selected region partially updated.
                println!("Could not decode update for {region:?}: {error}");
            }
        }
    }

    println!("Final readings:   {sensors:?}");
    Ok(())
}
