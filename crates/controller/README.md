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

## Brkbx pinout

See the table in `src/brkbx.rs` (and `firmware/src/brkbx_teensy41.rs` for Teensy 4.1). The firmware crate provides a stub `BrkbxStub`; replace it with a real `BrkbxHardware` implementation that owns the board’s GPIO and ADC to read the hardware.
