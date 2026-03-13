//! MIDI message types and parsing. Transport (UART) is handled in firmware.

/// Standard MIDI channel (0–15).
pub type Channel = u8;

/// Note number (0–127).
pub type NoteNumber = u8;

/// Velocity (0–127).
pub type Velocity = u8;

/// Control change number (0–127).
pub type Controller = u8;

/// Control change value (0–127).
pub type ControlValue = u8;

/// Parsed MIDI message (no_std friendly, no heap).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MidiMessage {
    /// Note off: channel, note, velocity
    NoteOff(Channel, NoteNumber, Velocity),
    /// Note on: channel, note, velocity (0 = note off)
    NoteOn(Channel, NoteNumber, Velocity),
    /// Polyphonic key pressure
    PolyPressure(Channel, NoteNumber, u8),
    /// Control change
    ControlChange(Channel, Controller, ControlValue),
    /// Program change
    ProgramChange(Channel, u8),
    /// Channel pressure
    ChannelPressure(Channel, u8),
    /// Pitch bend (14-bit, LSB then MSB in raw; here as 0..16383)
    PitchBend(Channel, u16),
    /// System real-time / common handled by transport
    SysExStart,
    /// Quarter frame, Song Select, etc. (payload in transport)
    SysExEnd,
    /// Raw bytes for sys ex (firmware can buffer and pass)
    Other(u8, u8, u8),
}

impl MidiMessage {
    /// Parse three bytes (status + data1 + data2) into a message.
    /// Returns None if not a known 3-byte message.
    #[inline]
    pub fn from_bytes(m: [u8; 3]) -> Option<MidiMessage> {
        let status = m[0];
        let channel = status & 0x0F;
        let cmd = status & 0xF0;
        let d1 = m[1];
        let d2 = m[2];

        match cmd {
            0x80 => Some(MidiMessage::NoteOff(channel, d1, d2)),
            0x90 => Some(MidiMessage::NoteOn(channel, d1, d2)),
            0xA0 => Some(MidiMessage::PolyPressure(channel, d1, d2)),
            0xB0 => Some(MidiMessage::ControlChange(channel, d1, d2)),
            0xC0 => Some(MidiMessage::ProgramChange(channel, d1)),
            0xD0 => Some(MidiMessage::ChannelPressure(channel, d1)),
            0xE0 => Some(MidiMessage::PitchBend(channel, ((d2 as u16) << 7) | d1 as u16)),
            _ => None,
        }
    }

    /// Number of data bytes following status for this message type (1 or 2).
    pub fn data_bytes(status: u8) -> u8 {
        match status & 0xF0 {
            0xC0 | 0xD0 => 1,
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => 2,
            _ => 0,
        }
    }
}
