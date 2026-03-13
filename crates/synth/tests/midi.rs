use little_synth::midi::MidiMessage;

#[test]
fn parse_note_on() {
    let m = MidiMessage::from_bytes([0x90, 60, 100]);
    assert_eq!(m, Some(MidiMessage::NoteOn(0, 60, 100)));
}

#[test]
fn parse_note_off() {
    let m = MidiMessage::from_bytes([0x80, 60, 0]);
    assert_eq!(m, Some(MidiMessage::NoteOff(0, 60, 0)));
}

#[test]
fn parse_cc() {
    let m = MidiMessage::from_bytes([0xB1, 7, 64]);
    assert_eq!(m, Some(MidiMessage::ControlChange(1, 7, 64)));
}

#[test]
fn parse_pitch_bend() {
    // LSB=0, MSB=64 => 14-bit value = (64<<7)|0 = 8192
    let m = MidiMessage::from_bytes([0xE0, 0, 64]);
    assert_eq!(m, Some(MidiMessage::PitchBend(0, 8192)));
}

#[test]
fn data_bytes() {
    assert_eq!(MidiMessage::data_bytes(0x90), 2);
    assert_eq!(MidiMessage::data_bytes(0xC0), 1);
}
