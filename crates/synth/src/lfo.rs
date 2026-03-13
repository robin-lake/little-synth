//! Programmable LFOs: retrigger on key, repeat, envelope mode; waveshaping with nodes; duration in ms or BPM sync.

/// LFO run mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LfoMode {
    /// Restart phase from 0 on each key down.
    RetriggerOnKey,
    /// Continuous loop.
    Repeat,
    /// One-shot: run for duration then hold end value (like an envelope).
    Envelope,
}

/// Single node for waveshaping: x in [0, 1] (phase), y in [-1, 1] (output).
#[derive(Debug, Clone, Copy)]
pub struct LfoNode {
    pub x: f32,
    pub y: f32,
}

/// Duration specification: either milliseconds or BPM-synced (e.g. 1/4 note).
#[derive(Debug, Clone, Copy)]
pub enum LfoDuration {
    Milliseconds(f32),
    /// Beat division of a whole note (e.g. 4 = quarter note). Pass current BPM to `to_seconds`.
    SyncBpm { beat_division: f32 },
}

impl LfoDuration {
    /// Duration in seconds. For SyncBpm, pass the current BPM.
    pub fn to_seconds(&self, bpm: f32) -> f32 {
        match self {
            LfoDuration::Milliseconds(ms) => ms / 1000.0,
            LfoDuration::SyncBpm { beat_division } => (60.0 / bpm) * (4.0 / beat_division),
        }
    }
}

/// Max number of nodes for one LFO (no heap).
pub const LFO_MAX_NODES: usize = 16;

/// Programmable LFO with multi-node waveshape and configurable duration.
#[derive(Debug, Clone)]
pub struct Lfo {
    pub mode: LfoMode,
    pub duration: LfoDuration,
    /// Nodes sorted by x; linear interpolation between nodes.
    pub nodes: [Option<LfoNode>; LFO_MAX_NODES],
    phase: f32,
    running: bool,
    #[allow(dead_code)]
    sample_rate: f32,
    current_bpm: f32,
}

impl Lfo {
    pub fn new(sample_rate: f32) -> Self {
        let mut nodes = [None; LFO_MAX_NODES];
        nodes[0] = Some(LfoNode { x: 0.0, y: 0.0 });
        nodes[1] = Some(LfoNode { x: 1.0, y: 1.0 });
        Self {
            mode: LfoMode::Repeat,
            duration: LfoDuration::Milliseconds(1000.0),
            nodes,
            phase: 0.0,
            running: true,
            sample_rate,
            current_bpm: 120.0,
        }
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.current_bpm = bpm;
    }

    /// Retrigger (e.g. on key down when mode is RetriggerOnKey).
    pub fn retrigger(&mut self) {
        self.phase = 0.0;
        if self.mode == LfoMode::Envelope {
            self.running = true;
        }
    }

    /// Advance by dt seconds; returns value in approximately [-1, 1] based on nodes.
    pub fn advance(&mut self, dt: f32) -> f32 {
        let period_s = self.duration.to_seconds(self.current_bpm);
        if period_s <= 0.0 {
            return 0.0;
        }
        if self.mode == LfoMode::Envelope && !self.running {
            return self.output_at_phase(1.0);
        }
        self.phase += dt / period_s;
        if self.phase >= 1.0 {
            match self.mode {
                LfoMode::RetriggerOnKey | LfoMode::Repeat => self.phase -= 1.0,
                LfoMode::Envelope => {
                    self.phase = 1.0;
                    self.running = false;
                }
            }
        }
        self.output_at_phase(self.phase)
    }

    /// Get output for a given phase in [0, 1] using node interpolation.
    fn output_at_phase(&self, phase: f32) -> f32 {
        let mut prev = LfoNode { x: 0.0, y: 0.0 };
        for n in self.nodes.iter().flatten() {
            if n.x > phase {
                let t = if n.x <= prev.x {
                    0.0
                } else {
                    (phase - prev.x) / (n.x - prev.x)
                };
                return prev.y + t * (n.y - prev.y);
            }
            prev = *n;
        }
        prev.y
    }
}
