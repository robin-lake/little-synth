//! Audio output to PCM5122 via I2S/SAI. Teensy 4.1 SAI pins and clock setup.
//!
//! PCM5122 expects I2S: BCK, LRCK (WS), DIN (data). MCLK optional but recommended.
//! Teensy 4.1 I2S pins for PCM5122:
//!   - BCLK: 21, LRCK: 23, DIN: 7 (data to DAC).

use teensy4_bsp::board;

/// Audio sample rate for the synth (PCM5102 supports 44.1 kHz, 48 kHz, etc.).
pub const SAMPLE_RATE_HZ: u32 = 48_000;

/// Buffer size in samples per channel (block sent to DAC).
pub const BLOCK_SIZE: usize = 128;

/// Board resources type (Teensy 4.1).
pub type BoardResources = board::T41Resources;

/// Initialize audio hardware.
/// Board resources are owned by main; this is a stub until Task 1 wires up the SAI/I2S driver.
/// TODO Task 1: accept the SAI peripheral from BoardResources and configure I2S output.
pub fn init_audio() {}

/// Fill a stereo block for the DAC. Left and right are interleaved in I2S order.
/// Implementations should call the synth engine to fill `left` and `right`.
#[inline]
pub fn fill_stereo_block(left: &mut [f32], right: &mut [f32], _board: &BoardResources) {
    debug_assert_eq!(left.len(), right.len());
    // Placeholder: silence. Replace with synth callback.
    left.fill(0.0);
    right.fill(0.0);
}
