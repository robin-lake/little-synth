//! Audio output to PCM5102 via I2S/SAI. Teensy 4.1 SAI pins and clock setup.
//!
//! PCM5102 expects I2S: BCK, LRCK (WS), DIN (data). No MCLK required for PCM5102.
//! Teensy 4.1 I2S2 pins (from PJRC/teensy4-rs docs):
//!   - BCLK: 4, LRCK: 3, DIN (TX): 2 (DOUT from Teensy perspective = data to DAC).

use teensy4_bsp::board;

/// Audio sample rate for the synth (PCM5102 supports 44.1 kHz, 48 kHz, etc.).
pub const SAMPLE_RATE_HZ: u32 = 48_000;

/// Buffer size in samples per channel (block sent to DAC).
pub const BLOCK_SIZE: usize = 128;

/// Board resources type (Teensy 4.1).
pub type BoardResources = board::T41Resources;

/// Initialize board and return peripherals needed for I2S output.
/// Actual SAI/I2S driver setup will use imxrt-hal (via BSP) when we add the driver.
pub fn init_audio() -> BoardResources {
    board::t41(board::instances())
}

/// Fill a stereo block for the DAC. Left and right are interleaved in I2S order.
/// Implementations should call the synth engine to fill `left` and `right`.
#[inline]
pub fn fill_stereo_block(left: &mut [f32], right: &mut [f32], _board: &BoardResources) {
    debug_assert_eq!(left.len(), right.len());
    // Placeholder: silence. Replace with synth callback.
    left.fill(0.0);
    right.fill(0.0);
}
