//! Waveform-based oscillator: harmonics shaping and programmable wavetables (Serum-style).

/// Maximum wavetable length (power of two for fast indexing).
pub const WAVETABLE_LEN: usize = 2048;

/// Oscillator that can be driven by a wavetable and/or additive harmonics.
#[derive(Debug, Clone)]
pub struct Oscillator {
    /// Single-cycle wavetable [0, WAVETABLE_LEN). Values in [-1, 1].
    pub wavetable: [f32; WAVETABLE_LEN],
    phase: f32,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        let mut wavetable = [0.0f32; WAVETABLE_LEN];
        // Default: sine
        for (i, s) in wavetable.iter_mut().enumerate() {
            let t = (i as f32 / WAVETABLE_LEN as f32) * 2.0 * core::f32::consts::PI;
            *s = libm::sinf(t);
        }
        Self {
            wavetable,
            phase: 0.0,
            sample_rate,
        }
    }

    /// Set frequency and advance one sample; returns sample in [-1, 1].
    #[inline]
    pub fn tick(&mut self, frequency_hz: f32) -> f32 {
        let phase_step = frequency_hz / self.sample_rate;
        self.phase += phase_step;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let idx = (self.phase * WAVETABLE_LEN as f32) as usize;
        let idx = idx.min(WAVETABLE_LEN - 1);
        self.wavetable[idx]
    }

    /// Reset phase to 0 (e.g. for retrigger).
    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }

    /// Set phase (0..1).
    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase - libm::floorf(phase);
    }

    /// Fill wavetable from additive harmonics: sum of sin(k*phase) * amplitude[k-1].
    pub fn set_harmonics(&mut self, amplitudes: &[f32]) {
        for (i, s) in self.wavetable.iter_mut().enumerate() {
            let phase = (i as f32 / WAVETABLE_LEN as f32) * 2.0 * core::f32::consts::PI;
            *s = amplitudes
                .iter()
                .enumerate()
                .map(|(k, &a)| libm::sinf(phase * (k + 1) as f32) * a)
                .sum();
        }
        self.normalize_wavetable();
    }

    /// Normalize so max absolute value is 1.0.
    pub fn normalize_wavetable(&mut self) {
        let max = self
            .wavetable
            .iter()
            .map(|x| x.abs())
            .fold(0.0f32, |a, b| a.max(b));
        if max > 0.0 {
            for s in self.wavetable.iter_mut() {
                *s /= max;
            }
        }
    }
}
