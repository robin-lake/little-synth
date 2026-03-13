//! MIDI over UART. Teensy 4.1 has multiple LPUARTs; pick pins for 31250 baud MIDI in/out.
//!
//! Arturia Keystep Pro: connect MIDI OUT (Keystep) -> MIDI IN (Teensy), MIDI IN (Keystep) <- MIDI OUT (Teensy).
//! Use optocoupler on MIDI IN and driver on MIDI OUT per spec; UART pins are 3.3V so level shift/optos as needed.

use little_synth::midi::MidiMessage;

/// MIDI standard baud rate.
pub const MIDI_BAUD: u32 = 31250;

/// Parse a single status + data bytes from UART. Call with bytes as they arrive;
/// returns Some when a complete 1 or 2 data-byte message is parsed.
/// Real-time bytes (0xF8–0xFF) can be handled separately and don't break running status.
pub fn parse_midi_byte(
    status: &mut Option<u8>,
    data: &mut [u8; 2],
    data_len: &mut u8,
    byte: u8,
) -> Option<MidiMessage> {
    if byte >= 0x80 {
        *status = Some(byte);
        *data_len = 0;
        if MidiMessage::data_bytes(byte) == 0 {
            return None; // SysEx or real-time
        }
        return None;
    }
    let s = match *status {
        Some(x) => x,
        None => return None,
    };
    let need = MidiMessage::data_bytes(s);
    data[*data_len as usize] = byte;
    *data_len += 1;
    if *data_len >= need {
        *status = None;
        *data_len = 0;
        return MidiMessage::from_bytes([s, data[0], data[1]]);
    }
    None
}
