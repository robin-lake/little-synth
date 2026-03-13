//! Controller state and key indices matching brkbx layout.
//!
//! Key matrix is 4 rows × 5 cols = 20 keys. Indices match brkbx `control.py`:
//! SAMPLE_KEYS 0–7, LATCH_KEYS 8–11, GATE_KEYS 12–15, function keys 16–19.

/// Index into the 20-key matrix (4×5).
pub type KeyIndex = u8;

/// Sample pad keys (indices 0–7).
pub const SAMPLE_KEYS: core::ops::Range<KeyIndex> = 0..8;

/// Latch keys (indices 8–11).
pub const LATCH_KEYS: core::ops::Range<KeyIndex> = 8..12;

/// Gate keys (indices 12–15).
pub const GATE_KEYS: core::ops::Range<KeyIndex> = 12..16;

/// All sound-related keys (0–15).
pub const SOUND_KEYS: core::ops::Range<KeyIndex> = 0..16;

/// Keys that can be held (0–17).
pub const HOLDABLE_KEYS: core::ops::Range<KeyIndex> = 0..18;

/// Function keys (brkbx: SLOW, FLIP, HOLD, PLAY).
pub const SLOW_KEY: KeyIndex = 16;
pub const FLIP_KEY: KeyIndex = 17;
pub const HOLD_KEY: KeyIndex = 18;
pub const PLAY_KEY: KeyIndex = 19;

/// Number of keys in the matrix.
pub const KEY_COUNT: usize = 20;

/// Joystick position: x, y in [-1.0, 1.0]; button pressed.
#[derive(Clone, Copy, Debug, Default)]
pub struct JoystickState {
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
}

/// LED state for the four function LEDs (SLOW, FLIP, HOLD, PLAY).
#[derive(Clone, Copy, Debug, Default)]
pub struct LedsState {
    pub slow: bool,
    pub flip: bool,
    pub hold: bool,
    pub play: bool,
}

/// Full controller state from one poll.
#[derive(Clone, Debug)]
pub struct ControllerState {
    /// Key matrix: key down (true) or up (false). Index 0..20.
    pub keys: [bool; KEY_COUNT],

    /// Four knobs in [0.0, 1.0] (or app-specific ranges).
    pub knobs: [f32; 4],

    /// Four faders/sliders in [0.0, 1.0].
    pub sliders: [f32; 4],

    /// Joystick 1 (main).
    pub joystick1: JoystickState,

    /// Joystick 2.
    pub joystick2: JoystickState,

    /// Rotary 1 delta since last poll (e.g. sample/bank selector).
    pub rotary1_delta: i32,

    /// Rotary 2 delta since last poll (e.g. BPM).
    pub rotary2_delta: i32,

    /// Rotary 1 button pressed.
    pub rotary1_button: bool,

    /// Rotary 2 button pressed.
    pub rotary2_button: bool,
}

impl Default for ControllerState {
    fn default() -> Self {
        Self {
            keys: [false; KEY_COUNT],
            knobs: [0.0; 4],
            sliders: [0.0; 4],
            joystick1: JoystickState::default(),
            joystick2: JoystickState::default(),
            rotary1_delta: 0,
            rotary2_delta: 0,
            rotary1_button: false,
            rotary2_button: false,
        }
    }
}
