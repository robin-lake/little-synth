# little-synth

A live-oriented synthesizer firmware for **Teensy 4.1** in Rust.  
**For architecture, module boundaries, and diagrams, see [ARCHITECTURE.md](ARCHITECTURE.md)** (single source of truth; LLM-friendly).: waveform oscillators, effects, ADSR, LFOs, MIDI I/O, and a display interface. Designed for use with the **Arturia Keystep Pro** and **PCM5102** I2S DAC.

## Features

- **Teensy 4.1** with modern embedded Rust (`thumbv7em-none-eabihf`, no_std)
- **Extensive unit testing**: core DSP lives in the `little-synth` library and is tested on the host
- **MIDI**: Input and output over UART (31250 baud); first controller target is Arturia Keystep Pro
- **Oscillator**: Wavetable + additive harmonics, programmable waves (Serum-style)
- **Effects**: Reverb, delay, tube distortion, soft clipping, wave-folding
- **Filter & envelope**: ADSR filter and envelope
- **LFOs**: Multiple programmable LFOs; modes (retrigger on key, repeat, envelope); waveshaping with x/y nodes; duration in ms or BPM sync
- **Audio out**: PCM5102 DAC via I2S (BCK, LRCK, DIN)
- **Display**: Trait-based interface for a screen (UI not implemented yet)

## Hardware

### PCM5102 DAC (e.g. [HiLetgo PCM5102](https://www.amazon.com/dp/B09C5QX228))

| PCM5102 | Teensy 4.1 (I2S2) |
|---------|-------------------|
| BCK     | Pin 4 (BCLK)      |
| LRCK/WS | Pin 3 (LRCLK)     |
| DIN     | Pin 2 (TX data)   |
| GND     | GND               |
| VIN     | 3.3 V             |

PCM5102 does not require MCLK. Use 44.1 kHz or 48 kHz sample rate as desired.

### MIDI (UART)

- **MIDI IN**: Connect to a UART RX pin (e.g. LPUART with 31250 baud) via optocoupler/level shifting as per MIDI spec.
- **MIDI OUT**: Connect from a UART TX pin through a MIDI output driver circuit to the Keystep Pro MIDI IN.

Choose specific LPUART pins according to [Teensy 4.1 pinout](https://www.pjrc.com/teensy/schematic.html) and your carrier board.

### Display

Implement the `little_synth_firmware::display::Display` trait for your panel (SPI TFT, I2C OLED, etc.). A `DummyDisplay` is provided for bring-up without a screen.

## Project layout

```
little-synth/
├── crates/
│   ├── synth/          # Core DSP (no_std, unit-tested on host)
│   │   ├── src/
│   │   │   ├── oscillator.rs
│   │   │   ├── envelope.rs
│   │   │   ├── lfo.rs
│   │   │   ├── filter.rs
│   │   │   ├── effects.rs
│   │   │   └── midi.rs
│   │   └── tests/
│   └── firmware/       # Teensy 4.1 binary
│       ├── src/
│       │   ├── main.rs
│       │   ├── audio.rs   # I2S/SAI → PCM5102
│       │   ├── midi_uart.rs
│       │   └── display.rs # Display trait + DummyDisplay
│       └── .cargo/config.toml
├── Cargo.toml
└── README.md
```

## Prerequisites

- Rust (stable) and `thumbv7em-none-eabihf` target:
  ```bash
  rustup target add thumbv7em-none-eabihf
  ```
- For building the firmware hex: LLVM `objcopy` (e.g. via `cargo-binutils`):
  ```bash
  cargo install cargo-binutils
  ```
- **teensy_loader_cli** for flashing: [https://github.com/PaulStoffregen/teensy_loader_cli](https://github.com/PaulStoffregen/teensy_loader_cli). Install it and ensure `teensy_loader_cli` is on your PATH. Alternatively you can use the Teensy Loader GUI.

## Build and test

```bash
# Run all library tests (host)
cargo test -p little-synth

# Add Teensy target once (if not already installed)
rustup target add thumbv7em-none-eabihf

# Build firmware (release) for Teensy 4.1
cargo build -p little-synth-firmware --release --target thumbv7em-none-eabihf

# Produce Intel HEX for Teensy Loader (requires cargo-binutils)
cargo objcopy -p little-synth-firmware --release --target thumbv7em-none-eabihf -- -O ihex little-synth-firmware.hex
```

Then load `little-synth-firmware.hex` onto the Teensy 4.1 with Teensy Loader or:

```bash
teensy_loader_cli -w -v little-synth-firmware.hex
```

## License

MIT OR Apache-2.0.
