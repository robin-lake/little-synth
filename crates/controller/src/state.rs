//! Controller state and key matrix layout.
//!
//! Key matrix is 4 rows × 5 cols = 20 buttons, indexable by (row, col) or by flat index.
//! Storage is row-major: index = row * KEY_COLS + col.

/// Index into the key matrix (0..KEY_COUNT). Row-major: row * KEY_COLS + col.
pub type KeyIndex = u8;

/// Number of key matrix rows.
pub const KEY_ROWS: usize = 4;

/// Number of key matrix columns.
pub const KEY_COLS: usize = 5;

/// Number of keys in the matrix (KEY_ROWS * KEY_COLS).
pub const KEY_COUNT: usize = KEY_ROWS * KEY_COLS;

/// Flat index for the button at (row, col). Panics if row >= KEY_ROWS or col >= KEY_COLS.
#[inline]
pub fn key_index(row: u8, col: u8) -> KeyIndex {
    assert!(row < KEY_ROWS as u8 && col < KEY_COLS as u8);
    (row as KeyIndex) * KEY_COLS as KeyIndex + col
}

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
    /// Key matrix: key down (true) or up (false). Row-major: index = row * KEY_COLS + col.
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

impl ControllerState {
    /// True if the button at (row, col) is pressed.
    #[inline]
    pub fn key_at(&self, row: u8, col: u8) -> bool {
        self.keys[key_index(row, col) as usize]
    }
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
