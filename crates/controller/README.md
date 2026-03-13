# little-synth-controller

Hardware abstraction for synth controllers so you can switch between:

- **Brkbx-style hardware** – key matrix, knobs, faders, joysticks, rotary encoders (same pinout as [brkbx](https://github.com/allieum/brkbx))
- **Arturia Keystep Pro** (or other MIDI controllers) – by implementing `Controller` from MIDI input
- Other backends – implement the `Controller` trait for any input source

## Usage

Enable the `brkbx` feature and use the trait + state types:

```rust
use little_synth_controller::{Controller, ControllerState, BrkbxHal};
use little_synth_controller::brkbx::BrkbxHardware;

// With a concrete hardware driver (e.g. from firmware):
let mut hal = BrkbxHal::new(my_brkbx_hardware);
let state = hal.poll();
// state.keys, state.knobs, state.sliders, state.joystick1, etc.
hal.set_leds(LedsState { slow: false, flip: true, hold: false, play: true });
```

The key matrix is 4 rows × 5 columns (20 buttons). Use `key_index(row, col)` for the flat index (row-major), or index `state.keys` directly. Check a button with `state.key_at(row, col)`.

## Key press logging

To log every key press and release from the brkbx key matrix, use `BrkbxHal::new_with_key_log` and pass a type that implements `brkbx::KeyPressLog`. With the **`key_log_std`** feature you can log to the terminal or to a file:

```rust
use little_synth_controller::brkbx::{BrkbxHal, KeyPressLog};
use little_synth_controller::std_key_log::StdKeyLog;

// Log to terminal (stdout)
let logger = StdKeyLog::terminal();
let mut hal = BrkbxHal::new_with_key_log(my_brkbx_hardware, logger);

// Or log to a file
let logger = StdKeyLog::to_file(std::path::Path::new("keylog.txt")).unwrap();
let mut hal = BrkbxHal::new_with_key_log(my_brkbx_hardware, logger);

// Poll as usual; each key press/release is logged automatically
let state = hal.poll();
```

On embedded (no std), implement `KeyPressLog` yourself (e.g. write to UART) and pass it to `new_with_key_log`. Without a logger, use `BrkbxHal::new(hardware)` for the default no-op.

## Brkbx pinout

See the table in `src/brkbx.rs` (and `firmware/src/brkbx_teensy41.rs` for Teensy 4.1). The firmware crate provides a stub `BrkbxStub`; replace it with a real `BrkbxHardware` implementation that owns the board’s GPIO and ADC to read the hardware.
