use little_synth::envelope::{AdsrEnvelope, AdsrParams, EnvelopeStage};

#[test]
fn envelope_attack_decay_sustain_release() {
    let mut env = AdsrEnvelope::with_params(
        1000.0,
        AdsrParams {
            attack_s: 0.001,
            decay_s: 0.002,
            sustain_level: 0.5,
            release_s: 0.002,
        },
    );
    env.trigger();
    let dt = 0.001;
    let mut max_level = 0.0f32;
    for _ in 0..10 {
        let l = env.advance(dt);
        max_level = max_level.max(l);
    }
    assert!(max_level >= 0.99);
    assert_eq!(env.stage(), EnvelopeStage::Sustain);
    assert!((env.level() - 0.5).abs() < 0.1);

    env.release();
    for _ in 0..50 {
        env.advance(dt);
    }
    assert_eq!(env.stage(), EnvelopeStage::Idle);
    assert!(env.level() < 0.01);
}
