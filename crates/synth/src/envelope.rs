//! ADSR envelope generator. No heap, sample-accurate.

/// ADSR envelope stage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// ADSR parameters (time in seconds or normalized rate; level 0..1).
#[derive(Debug, Clone, Copy)]
pub struct AdsrParams {
    pub attack_s: f32,
    pub decay_s: f32,
    pub sustain_level: f32,
    pub release_s: f32,
}

impl Default for AdsrParams {
    fn default() -> Self {
        Self {
            attack_s: 0.01,
            decay_s: 0.1,
            sustain_level: 0.7,
            release_s: 0.2,
        }
    }
}

/// ADSR envelope state machine. Call advance(dt) each sample.
#[derive(Debug, Clone)]
pub struct AdsrEnvelope {
    pub params: AdsrParams,
    stage: EnvelopeStage,
    level: f32,
    sample_rate: f32,
}

impl AdsrEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            params: AdsrParams::default(),
            stage: EnvelopeStage::Idle,
            level: 0.0,
            sample_rate,
        }
    }

    pub fn with_params(sample_rate: f32, params: AdsrParams) -> Self {
        Self {
            params,
            stage: EnvelopeStage::Idle,
            level: 0.0,
            sample_rate,
        }
    }

    #[inline]
    pub fn stage(&self) -> EnvelopeStage {
        self.stage
    }

    #[inline]
    pub fn level(&self) -> f32 {
        self.level
    }

    /// Start attack (e.g. on note on).
    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        // Keep level as-is for legato; for retrigger set self.level = 0.0
    }

    /// Start release (e.g. on note off).
    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    /// Advance by dt seconds; returns current envelope level.
    pub fn advance(&mut self, dt: f32) -> f32 {
        let rate = self.sample_rate;
        match self.stage {
            EnvelopeStage::Idle => {}
            EnvelopeStage::Attack => {
                if self.params.attack_s <= 0.0 {
                    self.level = 1.0;
                    self.stage = EnvelopeStage::Decay;
                } else {
                    self.level += dt / self.params.attack_s;
                    if self.level >= 1.0 {
                        self.level = 1.0;
                        self.stage = EnvelopeStage::Decay;
                    }
                }
            }
            EnvelopeStage::Decay => {
                if self.params.decay_s <= 0.0 {
                    self.level = self.params.sustain_level;
                    self.stage = EnvelopeStage::Sustain;
                } else {
                    let decay_rate = (1.0 - self.params.sustain_level) / self.params.decay_s;
                    self.level -= decay_rate * dt;
                    if self.level <= self.params.sustain_level {
                        self.level = self.params.sustain_level;
                        self.stage = EnvelopeStage::Sustain;
                    }
                }
            }
            EnvelopeStage::Sustain => {
                self.level = self.params.sustain_level;
            }
            EnvelopeStage::Release => {
                const RELEASE_THRESHOLD: f32 = 1e-6;
                if self.params.release_s <= 0.0 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                } else {
                    self.level -= (self.level / self.params.release_s) * dt;
                    if self.level <= RELEASE_THRESHOLD {
                        self.level = 0.0;
                        self.stage = EnvelopeStage::Idle;
                    }
                }
            }
        }
        let _ = rate;
        self.level
    }
}
