//! Simplified audio system for initial testing.
//! This provides a basic framework for audio generation without complex SAI configuration.

use little_synth::oscillator::Oscillator;
use embedded_hal::digital::v2::OutputPin;
use crate::sai_simple::{SimpleSai, SAMPLE_RATE, BLOCK_SIZE};

// Constants imported from sai_simple

pub struct SimpleAudioSystem<PIN: OutputPin> {
    oscillator: Oscillator,
    sai: SimpleSai,
    debug_pin: PIN, // Use built-in LED for debug
    audio_buffer: [f32; BLOCK_SIZE * 2], // Stereo buffer
}

impl<PIN: OutputPin> SimpleAudioSystem<PIN> {
    pub fn new(debug_pin: PIN, sai: SimpleSai) -> Self {
        let oscillator = Oscillator::new(SAMPLE_RATE as f32);
        let audio_buffer = [0.0f32; BLOCK_SIZE * 2];
        
        Self {
            oscillator,
            sai,
            debug_pin,
            audio_buffer,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // Initialize SAI peripheral
        self.sai.init_sai()?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        // Start SAI transmission
        self.sai.start_transmission()?;
        Ok(())
    }

    pub fn generate_test_tone(&mut self, frequency: f32) -> [f32; BLOCK_SIZE * 2] {
        let mut buffer = [0.0f32; BLOCK_SIZE * 2];
        
        // Generate stereo samples (interleaved L/R)
        for i in (0..BLOCK_SIZE * 2).step_by(2) {
            let sample = self.oscillator.tick(frequency);
            buffer[i] = sample;     // Left channel
            buffer[i + 1] = sample; // Right channel
        }
        
        buffer
    }

    pub fn process_audio_block(&mut self, frequency: f32) -> Result<(), &'static str> {
        // Generate audio samples using oscillator
        for i in (0..BLOCK_SIZE * 2).step_by(2) {
            let sample = self.oscillator.tick(frequency);
            self.audio_buffer[i] = sample;     // Left channel
            self.audio_buffer[i + 1] = sample; // Right channel (mono to stereo)
        }
        
        // Send buffer to SAI for audio output
        self.sai.send_audio_block(&self.audio_buffer)?;
        
        // Toggle debug LED to show audio is being processed
        let _ = self.debug_pin.set_high();
        
        Ok(())
    }
}