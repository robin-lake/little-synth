//! Brkbx hardware driver for Teensy 4.1.
//!
//! Implements [`little_synth_controller::brkbx::BrkbxHardware`] for the pinout from
//! [brkbx control.py](https://github.com/allieum/brkbx/blob/main/src/control.py).
//!
//! ## Pinout (Teensy 4.1 physical pin numbers)
//!
//! | Function         | Pin(s)              | Notes                         |
//! |------------------|---------------------|-------------------------------|
//! | Key matrix rows  | 1, 2, 3, 4          | GPIO input, pull-down         |
//! | Key matrix cols  | 9, 10, 11, 12, 0    | GPIO output (high when scan)  |
//! | Knobs 1–4        | 14, 15, 16, 17 (A0–A3) | ADC1/ADC2                    |
//! | Faders 1–2       | 26, 27 (A12, A13)   | ADC                           |
//! | Faders 3–4       | 38, 39              | ADC                           |
//! | Joystick 1 X,Y   | 24, 25 (A10, A11)   | ADC                           |
//! | Joystick 1 sel   | 30                  | GPIO input, pull-up (invert)  |
//! | Joystick 2 X,Y   | 40, 41              | ADC                           |
//! | Joystick 2 sel   | 35                  | GPIO input, pull-up (invert)  |
//! | LEDs             | 5, 6, 23, 22        | SLOW, FLIP, HOLD, PLAY        |
//! | Rotary 1 (sample)| 32 (CLK), 31 (DT)   | Quadrature encoder            |
//! | Rotary 1 button  | 36                  | GPIO input, pull-up (invert) |
//! | Rotary 2 (BPM)   | 33 (CLK), 34 (DT)   | Quadrature encoder            |
//! | Rotary 2 button  | 37                  | GPIO input, pull-up (invert) |
//!
//! This module currently provides a **stub** implementation that returns default
//! values (no keys pressed, knobs/sliders at 0, no rotary delta). Replace
//! [`BrkbxStub`] with a real implementation that takes ownership of the board's
//! pins and ADC (e.g. by destructuring `board::Resources` and configuring
//! the pins above) to read the actual hardware.

use little_synth_controller::brkbx::BrkbxHardware;

/// Stub implementation: returns default/zero for all inputs, no-ops for LEDs.
///
/// Use this until a real hardware driver is wired from the board's pins and ADC.
#[derive(Default)]
pub struct BrkbxStub;

impl BrkbxHardware for BrkbxStub {
    fn read_key(&mut self, _row: u8, _col: u8) -> bool {
        false
    }

    fn read_adc_raw(&mut self, _channel: u8) -> u16 {
        0
    }

    fn read_joystick1_sel(&mut self) -> bool {
        false
    }

    fn read_joystick2_sel(&mut self) -> bool {
        false
    }

    fn read_rotary1_button(&mut self) -> bool {
        false
    }

    fn read_rotary2_button(&mut self) -> bool {
        false
    }

    fn rotary1_delta(&mut self) -> i32 {
        0
    }

    fn rotary2_delta(&mut self) -> i32 {
        0
    }

    fn set_led_slow(&mut self, _on: bool) {}
    fn set_led_flip(&mut self, _on: bool) {}
    fn set_led_hold(&mut self, _on: bool) {}
    fn set_led_play(&mut self, _on: bool) {}
}
