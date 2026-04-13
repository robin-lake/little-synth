# little-synth: Tasks to make it functional

## Tasks

### P0 — Critical path (nothing makes sound without these)

**Task 1 — I2S/SAI driver** (`crates/firmware/src/audio.rs`, `main.rs`)
- Configure SAI peripheral in I2S mode via imxrt-hal/teensy4-bsp on pins BCLK=4, LRCK=3, DIN=2 at 48 kHz
- Set up DMA double-buffer or interrupt-driven refill that calls `fill_stereo_block()` every 128 samples
- Wire into `main.rs` so the stream runs after `init_audio()`
- See `docs/AGENT_TASKS.md` Task 1 for full spec

**Task 2 — MIDI UART init** (`crates/firmware/src/main.rs`, new `midi_rx.rs`)
- Initialize an LPUART at 31250 baud on the correct RX pin
- In main loop or RX interrupt, feed bytes to `parse_midi_byte()` with persistent state
- Store `MidiMessage` values in a small queue/ring buffer for the engine to drain
- See `docs/AGENT_TASKS.md` Task 2 for full spec

**Task 3 — Voice engine + integration** (new `crates/synth/src/engine.rs`, `firmware/src/audio.rs`)
- Add `Engine` to the synth crate: owns 1–N voices (each: oscillator + ADSR envelope + filter), exposes `on_midi(msg)` and `fill_block(left, right)`
- MIDI note → frequency (A4 = 440 Hz, 2^((n-69)/12) formula)
- Wire `fill_stereo_block()` to call the engine
- Wire main loop MIDI drain to call `engine.on_midi(msg)`
- See `docs/AGENT_TASKS.md` Task 3 for full spec

### P1 — Polish / completeness

**Task 4 — Reverb implementation** (`crates/synth/src/effects.rs`)
- `Reverb::tick()` currently returns input unchanged
- Implement a basic Schroeder reverb (comb + all-pass) or Freeverb-style algo

**Task 5 — CI workflow** (`.github/workflows/ci.yml`)
- `cargo test -p little-synth` — host DSP tests
- `cargo build -p little-synth-firmware --release --target thumbv7em-none-eabihf` — firmware compile check (no hardware needed)

---

## Architecture decisions needed before starting

These should be resolved and documented in `ARCHITECTURE.md` before diving into the tasks above to avoid rework:

**Decision A — Audio interrupt vs. polled** (before Task 1)
- DMA with interrupt-based double-buffer swap: better real-time behavior, harder to implement
- Polling in main loop: simpler, fine for first bring-up
- Make the choice explicit before implementing so Task 1 doesn't get revised

**Decision B — Polyphony strategy** (before Task 3)
- Monophonic (1 voice): simplest, recommended for v1
- Last-note-wins mono: slightly more playable
- 4/8-voice polyphonic: needs a voice allocator, all sizes must be compile-time constants (no heap, no_std)
- Polyphony affects memory layout; decide upfront

---

## Housekeeping / one-off fixes

- **Remove `little-synth-firmware.hex` from git** — it's a build artifact that will drift out of sync; add to `.gitignore` and document the build command in README
- **Clarify controller crate status** — `crates/controller/` is not wired into the firmware's MIDI input path; decide if brkbx hardware is near-term (wire it in) or future work (add a clear note)
- **Add `heapless` for MIDI queue** — Task 2 needs a ring buffer for `MidiMessage`; use the `heapless` crate rather than a one-off in `main.rs` so it stays testable

---

## Summary table

| Priority | Task | Location |
|---|---|---|
| Prereq | Decide interrupt vs. polled audio | `ARCHITECTURE.md` |
| Prereq | Decide polyphony strategy | `ARCHITECTURE.md` |
| P0 | I2S/SAI driver | `firmware/src/audio.rs` |
| P0 | MIDI UART init | `firmware/src/main.rs` + new `midi_rx.rs` |
| P0 | Voice engine | new `synth/src/engine.rs` + firmware wiring |
| P1 | Reverb implementation | `synth/src/effects.rs` |
| P1 | CI workflow | `.github/workflows/ci.yml` |
| Cleanup | Remove `.hex` from git | `.gitignore` |
| Cleanup | Controller crate status | decision or wiring |
| Cleanup | Add `heapless` MIDI queue | `synth/src/` or `firmware/src/midi_rx.rs` |
