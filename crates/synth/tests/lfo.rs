use little_synth::lfo::{Lfo, LfoDuration, LfoMode};

#[test]
fn lfo_repeat_advances() {
    let mut lfo = Lfo::new(1000.0);
    lfo.duration = LfoDuration::Milliseconds(100.0);
    lfo.mode = LfoMode::Repeat;
    let a = lfo.advance(0.05);
    let b = lfo.advance(0.05);
    assert!(a >= -1.01 && a <= 1.01);
    assert!(b >= -1.01 && b <= 1.01);
}

#[test]
fn lfo_duration_ms() {
    let d = LfoDuration::Milliseconds(500.0);
    assert!((d.to_seconds(120.0) - 0.5).abs() < 0.001);
}

#[test]
fn lfo_sync_bpm() {
    let d = LfoDuration::SyncBpm {
        beat_division: 4.0,
    };
    // Quarter note at 120 bpm = 0.5 s
    assert!((d.to_seconds(120.0) - 0.5).abs() < 0.001);
}
