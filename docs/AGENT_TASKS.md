# Agent tasks: I2S, MIDI UART, and voice engine

These tasks are designed to be run **one agent per task**, **in order**. Each task is self-contained with explicit file paths and success criteria so it can be completed without loading the whole codebase.

**Prerequisites:** Read `ARCHITECTURE.md` (sections 1–2 and 5–6) for crate boundaries and data flow. Synth crate never touches hardware; firmware never implements DSP.

---

## Task 1: I2S/SAI driver and buffer callback

**Goal:** Use imxrt-hal (via teensy4-bsp) to drive SAI in I2S mode on the documented pins and call `fill_stereo_block()` from the audio callback so the DAC receives real samples (even if silence or a test tone).

**Scope:**

- **Files to touch:** `crates/firmware/src/audio.rs`, `crates/firmware/src/main.rs`, optionally `crates/firmware/Cargo.toml` if new deps are needed.
- **Do not change:** `crates/synth/*`, `crates/firmware/src/midi_uart.rs`, `crates/firmware/src/display.rs`.

**Context to load:**

- `crates/firmware/src/audio.rs` — `BoardResources`, `init_audio()`, `fill_stereo_block()`, `SAMPLE_RATE_HZ`, `BLOCK_SIZE`.
- `crates/firmware/src/main.rs` — current `main()` and how it gets `BoardResources`.
- ARCHITECTURE.md §6 (Hardware map): BCLK 4, LRCK 3, DIN 2; PCM5102, no MCLK.

**Requirements:**

1. In `audio.rs`: configure SAI in I2S mode (teensy4-bsp / imxrt-hal) on pins BCLK 4, LRCK 3, DIN (TX) 2, at `SAMPLE_RATE_HZ` (48 kHz).
2. Set up a block-based callback or DMA double-buffer that, when a block is due, calls `fill_stereo_block(left, right, &board)` with slices of length `BLOCK_SIZE`, then sends the interleaved/stereo data to the SAI TX.
3. From `main.rs`: call into the audio layer so that after `init_audio()`, the I2S stream runs (e.g. main loop or interrupt drives refill and SAI feed).
4. Keep `fill_stereo_block` signature as-is: `(left: &mut [f32], right: &mut [f32], _board: &BoardResources)`. The implementation inside may remain filling silence until Task 3.

**Success criteria:**

- Firmware builds for `thumbv7em-none-eabihf`.
- At runtime, SAI TX is active and `fill_stereo_block()` is invoked periodically with correct block size; no panic or deadlock. (Verification can be scope/led or by measuring DAC output.)

**Output for next task:** No API change. Task 2 will assume `fill_stereo_block()` is already driven by I2S.

---

## Task 2: MIDI UART read and feed to parser

**Goal:** In `main` (or an interrupt), read LPUART at 31250 baud and feed received bytes to `parse_midi_byte()`. Store or forward parsed `MidiMessage` values so a future voice engine can consume them (e.g. a simple queue or callback; no need to implement the synth yet).

**Scope:**

- **Files to touch:** `crates/firmware/src/main.rs`, `crates/firmware/src/midi_uart.rs` (only if you need to expose or tweak the parser API), and one new file or section for “MIDI RX + parser state” (e.g. `crates/firmware/src/midi_rx.rs` or logic in `main.rs`).
- **Do not change:** `crates/synth/*` (except you may add a re-export or type alias in synth if the task spec says so; otherwise leave synth as-is), `crates/firmware/src/audio.rs` (I2S is already wired by Task 1), `crates/firmware/src/display.rs`.

**Context to load:**

- `crates/firmware/src/midi_uart.rs` — `parse_midi_byte(status, data, data_len, byte) -> Option<MidiMessage>`, `MIDI_BAUD`.
- `crates/synth/src/midi.rs` — `MidiMessage` enum (NoteOn, NoteOff, ControlChange, etc.).
- `crates/firmware/src/main.rs` — current loop and where to plug in UART read.

**Requirements:**

1. Configure one LPUART at 31250 baud (use `MIDI_BAUD`) on the chosen RX (and optionally TX) pins per ARCHITECTURE.md §6.
2. In the main loop or in an interrupt: read bytes from the UART and call `parse_midi_byte()` with persistent state (`status`, `data`, `data_len`). When it returns `Some(msg)`, store or hand off the `MidiMessage` (e.g. circular buffer of messages, or a single “last message” for the next task).
3. Document where the parsed messages go (e.g. “messages are stored in `midi_rx::messages()`” or “passed to `engine::on_midi(msg)`”) so Task 3 can consume them.

**Success criteria:**

- Firmware builds. When MIDI bytes are sent to the Teensy at 31250 baud, `parse_midi_byte()` is called and completes; Note On/Off and CCs are parsed and available where you documented. No need to drive the synth yet.

**Output for next task:** Clear contract: “The voice engine can obtain MIDI messages from [X]” (function, static queue, or callback). Task 3 will implement the engine and call that contract.

---

## Task 3: Voice engine in synth and wire to MIDI and I2S

**Goal:** In the synth crate, add a voice/engine that uses the existing oscillator, envelope, filter, and LFOs, and drive it from MIDI. In firmware, connect the MIDI source from Task 2 to this engine and call the engine from `fill_stereo_block()` so that MIDI note on/off and CCs produce sound.

**Scope:**

- **Files to touch:**  
  - **Synth:** New module e.g. `crates/synth/src/engine.rs` (or `voice.rs`) and export from `crates/synth/src/lib.rs`; use existing `oscillator`, `envelope`, `filter`, `lfo`, `midi`.  
  - **Firmware:** `crates/firmware/src/audio.rs` (inside `fill_stereo_block`, call the engine to fill `left`/`right`); `crates/firmware/src/main.rs` (pass MIDI messages from Task 2 into the engine, e.g. each time a `MidiMessage` is available).
- **Do not change:** I2S/SAI setup in `audio.rs` (only the body of `fill_stereo_block` and how it gets the engine reference); MIDI UART and parser implementation (only how main passes messages to the engine).

**Context to load:**

- `crates/synth/src/oscillator.rs` — `Oscillator::new(sample_rate)`, `tick(frequency_hz)`, `reset_phase()`.
- `crates/synth/src/envelope.rs` — `AdsrEnvelope`, `AdsrParams`, `trigger()`, `release()`, `advance(dt)`, `level()`.
- `crates/synth/src/filter.rs` — `OnePoleLowpass` or `AdsrFilter`, `tick(input)`.
- `crates/synth/src/lfo.rs` — `Lfo`, mode, `tick()`, etc.
- `crates/synth/src/midi.rs` — `MidiMessage::NoteOn`, `NoteOff`, `ControlChange`; midi note number to Hz (e.g. A4 = 440).
- Task 2 output: where to get `MidiMessage` in firmware (e.g. `midi_rx::drain()` or callback).

**Requirements:**

1. **Synth crate:** Add a small voice/engine that:
   - Holds one or more voices (e.g. one oscillator + envelope + filter per voice; LFO can modulate filter or level).
   - Exposes something like `engine.tick() -> (f32, f32)` (left, right) or `engine.fill_block(left, right)`.
   - Has an API to feed MIDI: e.g. `engine.on_midi(msg)` that handles `NoteOn`/`NoteOff` (note number, velocity) and optionally `ControlChange` (e.g. CC 74 cutoff, CC 71 resonance). Map note number to frequency (e.g. 69 → 440 Hz).
2. **Firmware:** In `fill_stereo_block()`, obtain the engine (or a shared reference) and fill `left` and `right` by calling the engine. In the path where Task 2 delivers MIDI (main loop or interrupt), call the engine’s `on_midi(msg)` (or equivalent) for each parsed message.
3. Keep the synth crate no_std and testable; no hardware in the synth. The engine may take `sample_rate: f32` and use the existing DSP types.

**Success criteria:**

- Synth tests still pass. Firmware builds. Playing notes on the MIDI controller produces audible sound from the PCM5102; note off stops the voice; optional CCs affect the sound (e.g. filter cutoff).

**Output:** End-to-end path: MIDI IN → UART → parser → engine → `fill_stereo_block()` → I2S → DAC.

---

## Summary

| Order | Task | Main deliverable |
|-------|------|------------------|
| 1 | I2S/SAI driver | SAI in I2S mode on BCLK 4, LRCK 3, DIN 2; `fill_stereo_block()` called every block. |
| 2 | MIDI UART → parser | LPUART at 31250 baud; bytes fed to `parse_midi_byte()`; `MidiMessage`s available for engine. |
| 3 | Voice engine | Synth engine (osc + env + filter + LFO) driven by MIDI; `fill_stereo_block()` fills buffers from engine. |

Each agent should only need the files listed under **Context to load** and **Files to touch** for its task, plus ARCHITECTURE.md for boundaries.
