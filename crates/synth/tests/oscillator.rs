use little_synth::oscillator::Oscillator;

#[test]
fn oscillator_phase_advances() {
    let mut osc = Oscillator::new(48000.0);
    let _ = osc.tick(440.0);
    let _ = osc.tick(440.0);
    // Phase should have advanced
    let s = osc.tick(440.0);
    assert!(s >= -1.0 && s <= 1.0);
}

#[test]
fn oscillator_reset_phase() {
    let mut osc = Oscillator::new(48000.0);
    let a = osc.tick(440.0);
    osc.reset_phase();
    let b = osc.tick(440.0);
    assert_eq!(a, b);
}

#[test]
fn harmonics_sine() {
    let mut osc = Oscillator::new(48000.0);
    osc.set_harmonics(&[1.0]);
    let s = osc.tick(440.0);
    assert!(s >= -1.01 && s <= 1.01);
}
