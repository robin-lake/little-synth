//! Brkbx-style hardware: key matrix, knobs, faders, joysticks, rotary encoders.
//!
//! Pinout matches [brkbx control.py](https://github.com/allieum/brkbx/blob/main/src/control.py)
//! (Teensy 4.1 / Arduino-style pin names).
//!
//! ## Pinout summary (Teensy 4.1)
//!
//! | Function        | Pin(s)              | Notes                    |
//! |-----------------|---------------------|--------------------------|
//! | Key matrix rows | D1, D2, D3, D4      | Input, pull-down         |
//! | Key matrix cols| D9, D10, D11, D12, D0 | Output (drive high when scanning) |
//! | Knobs 1–4       | A0, A1, A2, A3      | ADC (pins 14–17)         |
//! | Faders 1–2      | A12, A13            | ADC (pins 26–27)         |
//! | Faders 3–4      | D38, D39            | ADC (pins 38–39)         |
//! | Joystick 1 X,Y  | A10, A11             | ADC (pins 24–25)         |
//! | Joystick 1 sel  | D30                  | Digital in, pull-up      |
//! | Joystick 2 X,Y  | D40, D41             | ADC (pins 40–41)         |
//! | Joystick 2 sel  | D35                  | Digital in, pull-up      |
//! | LEDs            | D5, D6, D23, D22    | SLOW, FLIP, HOLD, PLAY   |
//! | Rotary 1 (sample)| D32 (CLK), D31 (DT)  | Encoder                  |
//! | Rotary 1 button | D36                  | Input, pull-up (inverted)|
//! | Rotary 2 (BPM)  | D33 (CLK), D34 (DT)  | Encoder                  |
//! | Rotary 2 button | D37                  | Input, pull-up (inverted)|

use crate::{key_index, Controller, ControllerState, JoystickState, LedsState, KEY_COLS, KEY_COUNT, KEY_ROWS};

/// Low-level hardware access for brkbx pinout.
///
/// Implement this for your MCU (e.g. Teensy 4.1) so that [`BrkbxHal`] can
/// read keys, ADC, and drive LEDs.
pub trait BrkbxHardware {
    /// Read key matrix: drive col `col` (0..5), then read rows; return true if (row, col) is pressed.
    fn read_key(&mut self, row: u8, col: u8) -> bool;

    /// Read ADC channel (0–11): knobs 0–3, joystick1 x/y, faders 0–3, joystick2 x/y.
    /// Returns raw 16-bit value (0..65536).
    fn read_adc_raw(&mut self, channel: u8) -> u16;

    /// Read digital input for joystick/rotary buttons (inverted logic: true = pressed).
    fn read_joystick1_sel(&mut self) -> bool;
    fn read_joystick2_sel(&mut self) -> bool;
    fn read_rotary1_button(&mut self) -> bool;
    fn read_rotary2_button(&mut self) -> bool;

    /// Rotary encoder deltas since last poll (can be read from quadrature or maintained in driver).
    fn rotary1_delta(&mut self) -> i32;
    fn rotary2_delta(&mut self) -> i32;

    /// Set LED outputs (true = on).
    fn set_led_slow(&mut self, on: bool);
    fn set_led_flip(&mut self, on: bool);
    fn set_led_hold(&mut self, on: bool);
    fn set_led_play(&mut self, on: bool);
}

/// ADC channel indices for brkbx (for implementers of BrkbxHardware).
pub mod adc_channels {
    pub const KNOB1: u8 = 0;
    pub const KNOB2: u8 = 1;
    pub const KNOB3: u8 = 2;
    pub const KNOB4: u8 = 3;
    pub const JOY1_X: u8 = 4;
    pub const JOY1_Y: u8 = 5;
    pub const FADER1: u8 = 6;
    pub const FADER2: u8 = 7;
    pub const FADER3: u8 = 8;
    pub const FADER4: u8 = 9;
    pub const JOY2_X: u8 = 10;
    pub const JOY2_Y: u8 = 11;
}

const ADC_MAX: u16 = 65535;

/// Brkbx hardware abstraction: implements [`Controller`] by polling [`BrkbxHardware`].
pub struct BrkbxHal<H> {
    hardware: H,
}

impl<H> BrkbxHal<H> {
    pub const fn new(hardware: H) -> Self {
        Self { hardware }
    }
}

impl<H: BrkbxHardware> Controller for BrkbxHal<H> {
    fn poll(&mut self) -> ControllerState {
        use adc_channels::*;

        let h = &mut self.hardware;

        // Key matrix: row × col, stored row-major
        let mut keys = [false; KEY_COUNT];
        for row in 0u8..KEY_ROWS as u8 {
            for col in 0u8..KEY_COLS as u8 {
                keys[key_index(row, col) as usize] = h.read_key(row, col);
            }
        }

        // ADC: map raw 16-bit to [0, 1] for knobs/sliders, [-1, 1] for joystick axes
        let to_01 = |raw: u16| raw as f32 / ADC_MAX as f32;
        let to_11 = |raw: u16| (raw as f32 / ADC_MAX as f32) * 2.0 - 1.0;

        let knobs = [
            to_01(h.read_adc_raw(KNOB1)),
            to_01(h.read_adc_raw(KNOB2)),
            to_01(h.read_adc_raw(KNOB3)),
            to_01(h.read_adc_raw(KNOB4)),
        ];
        let sliders = [
            to_01(h.read_adc_raw(FADER1)),
            to_01(h.read_adc_raw(FADER2)),
            to_01(h.read_adc_raw(FADER3)),
            to_01(h.read_adc_raw(FADER4)),
        ];
        let joystick1 = JoystickState {
            x: to_11(h.read_adc_raw(JOY1_X)),
            y: to_11(h.read_adc_raw(JOY1_Y)),
            pressed: h.read_joystick1_sel(),
        };
        let joystick2 = JoystickState {
            x: to_11(h.read_adc_raw(JOY2_X)),
            y: to_11(h.read_adc_raw(JOY2_Y)),
            pressed: h.read_joystick2_sel(),
        };

        ControllerState {
            keys,
            knobs,
            sliders,
            joystick1,
            joystick2,
            rotary1_delta: h.rotary1_delta(),
            rotary2_delta: h.rotary2_delta(),
            rotary1_button: h.read_rotary1_button(),
            rotary2_button: h.read_rotary2_button(),
        }
    }

    fn set_leds(&mut self, leds: LedsState) {
        self.hardware.set_led_slow(leds.slow);
        self.hardware.set_led_flip(leds.flip);
        self.hardware.set_led_hold(leds.hold);
        self.hardware.set_led_play(leds.play);
    }
}

// Teensy 4.1 implementation lives in little-synth-firmware (brkbx_teensy41 module)
// which implements BrkbxHardware using board pins and ADC.
