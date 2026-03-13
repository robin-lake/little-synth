//! ADSR-modulated filter (e.g. low-pass with cutoff/ resonance controlled by envelope).

/// Filter type placeholder; actual poles and topology TBD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
}

/// One-pole low-pass state (no heap).
#[derive(Debug, Clone)]
pub struct OnePoleLowpass {
    pub cutoff_hz: f32,
    sample_rate: f32,
    z: f32,
}

impl OnePoleLowpass {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            cutoff_hz: 1000.0,
            sample_rate,
            z: 0.0,
        }
    }

    /// Coefficient from cutoff and sample rate.
    fn coeff(&self) -> f32 {
        let rc = 1.0 / (2.0 * core::f32::consts::PI * self.cutoff_hz);
        let dt = 1.0 / self.sample_rate;
        dt / (rc + dt)
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let a = self.coeff();
        self.z = self.z + a * (input - self.z);
        self.z
    }

    pub fn set_cutoff(&mut self, cutoff_hz: f32) {
        self.cutoff_hz = cutoff_hz;
    }
}

/// ADSR filter: applies envelope to filter cutoff (or other params). Combines envelope + filter.
#[derive(Debug, Clone)]
pub struct AdsrFilter {
    pub filter: OnePoleLowpass,
    pub envelope: super::envelope::AdsrEnvelope,
    /// Cutoff when envelope is 0 and at full.
    pub cutoff_min_hz: f32,
    pub cutoff_max_hz: f32,
}

impl AdsrFilter {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            filter: OnePoleLowpass::new(sample_rate),
            envelope: super::envelope::AdsrEnvelope::new(sample_rate),
            cutoff_min_hz: 200.0,
            cutoff_max_hz: 8000.0,
        }
    }

    pub fn tick(&mut self, input: f32, dt: f32) -> f32 {
        let env = self.envelope.advance(dt);
        let cutoff =
            self.cutoff_min_hz + (self.cutoff_max_hz - self.cutoff_min_hz) * env;
        self.filter.set_cutoff(cutoff);
        self.filter.tick(input)
    }

    pub fn trigger(&mut self) {
        self.envelope.trigger();
    }

    pub fn release(&mut self) {
        self.envelope.release();
    }
}
