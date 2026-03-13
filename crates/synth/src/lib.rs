//! # little-synth
//!
//! Core DSP and synth logic for the little-synth firmware. No_std, designed for
//! extensive unit testing on host and deployment on Teensy 4.1.

#![no_std]

pub mod envelope;
pub mod lfo;
pub mod midi;
pub mod oscillator;
pub mod effects;
pub mod filter;
