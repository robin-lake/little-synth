//! Common synth effects: reverb, delay, distortion (tube), soft clipping, wave-folding.

/// Soft clip (tanh-style) to keep signal in [-1, 1] with smooth saturation.
#[derive(Debug, Clone, Copy, Default)]
pub struct SoftClip {
    pub drive: f32, // > 1 = more gain into nonlinearity
}

impl SoftClip {
    #[inline]
    pub fn process(&self, x: f32) -> f32 {
        libm::tanhf(x * self.drive) / libm::tanhf(self.drive).max(0.001)
    }
}

/// Wave folder: mirrors signal above threshold for asymmetric fold.
#[derive(Debug, Clone, Copy)]
pub struct WaveFolder {
    pub threshold: f32, // e.g. 0.5
    pub gain: f32,
}

impl Default for WaveFolder {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            gain: 2.0,
        }
    }
}

impl WaveFolder {
    /// Fold: scale, then reflect when beyond ±1.
    pub fn process(&self, x: f32) -> f32 {
        fold_overflow(x * self.gain)
    }
}

#[inline]
fn fold_overflow(x: f32) -> f32 {
    let mut y = x;
    while y > 1.0 {
        y = 2.0 - y;
    }
    while y < -1.0 {
        y = -2.0 - y;
    }
    y
}

/// Tube-style distortion: asymmetric soft clipping (even harmonics).
#[derive(Debug, Clone, Copy)]
pub struct TubeAmp {
    pub drive: f32,
    pub mix: f32, // 0 = dry, 1 = full wet
}

impl Default for TubeAmp {
    fn default() -> Self {
        Self {
            drive: 2.0,
            mix: 1.0,
        }
    }
}

impl TubeAmp {
    pub fn process(&self, x: f32) -> f32 {
        let d = x * self.drive;
        let wet = d / (1.0 + d.abs());
        x * (1.0 - self.mix) + wet * self.mix
    }
}

/// Simple delay line (fixed max length, no heap). Stereo or mono; here mono for simplicity.
pub const DELAY_MAX_SAMPLES: usize = 48000 * 2; // 2 s at 48 kHz

#[derive(Debug, Clone)]
pub struct Delay {
    buffer: [f32; DELAY_MAX_SAMPLES],
    write_pos: usize,
    sample_rate: f32,
    pub delay_s: f32,
    pub feedback: f32,
    pub mix: f32,
}

impl Delay {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: [0.0; DELAY_MAX_SAMPLES],
            write_pos: 0,
            sample_rate,
            delay_s: 0.3,
            feedback: 0.5,
            mix: 0.3,
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let len = self.buffer.len();
        let delay_samples = (self.delay_s * self.sample_rate) as usize;
        let delay_samples = delay_samples.min(len - 1);
        let read_pos = (self.write_pos + len - delay_samples) % len;
        let delayed = self.buffer[read_pos];
        let out = input + delayed * self.mix;
        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) % len;
        out
    }
}

/// Placeholder for reverb (all-pass / comb structure TBD; same no-heap pattern).
#[derive(Debug, Clone)]
pub struct Reverb {
    sample_rate: f32,
    pub decay: f32,
    pub mix: f32,
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            decay: 0.7,
            mix: 0.3,
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let _ = (self.sample_rate, self.decay);
        input // passthrough until implemented
    }
}
