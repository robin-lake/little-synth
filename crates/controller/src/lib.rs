//! Hardware abstraction for synth controllers.
//!
//! Lets you switch between physical hardware (e.g. brkbx-style key matrix,
//! knobs, sliders, joysticks) and other controllers (e.g. Arturia Keystep Pro over MIDI)
//! by implementing the [`Controller`] trait.

#![no_std]

mod state;

pub use state::{
    ControllerState, JoystickState, LedsState, KeyIndex, KEY_COLS, KEY_COUNT, KEY_ROWS,
    key_index,
};

/// Controller input abstraction: poll current state and drive LEDs.
///
/// Implement this for brkbx hardware, Keystep Pro (MIDI), or other backends.
pub trait Controller {
    /// Poll the controller and return the current state.
    fn poll(&mut self) -> ControllerState;

    /// Update LED outputs (e.g. SLOW, FLIP, HOLD, PLAY).
    fn set_leds(&mut self, leds: LedsState);
}

#[cfg(feature = "brkbx")]
pub mod brkbx;
