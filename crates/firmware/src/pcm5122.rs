//! PCM5122 DAC driver for Teensy 4.1 via I2C control and I2S audio data.
//!
//! Pin connections:
//! - I2C: SDA=18, SCL=19
//! - I2S: BCK=21, DIN=7, MCK=23  
//! - Control: MUTE=2, DEEM=3, FILT=4, ATT0=5, MOD1=6, MOD2=8, AGN=9

use teensy4_bsp::hal::{gpio::Output, lpi2c::Lpi2c};
use teensy4_bsp::pins::t41::*;
use crate::sai_driver::{SaiDriver, SAMPLE_RATE};
use little_synth::oscillator::Oscillator;

pub const PCM5122_I2C_ADDR: u8 = 0x4D;

pub const SAMPLE_RATE_HZ: u32 = 48_000;
const BLOCK_SIZE: usize = 128;

pub struct PCM5122 {
    i2c: Lpi2c<2>,
    mute_pin: Output<P2>,
    deem_pin: Output<P3>, 
    filt_pin: Output<P4>,
    att0_pin: Output<P5>,
    mod1_pin: Output<P6>,
    mod2_pin: Output<P8>,
    agn_pin: Output<P9>,
}

impl PCM5122 {
    pub fn new(
        i2c: Lpi2c<2>,
        mute_pin: Output<P2>,
        deem_pin: Output<P3>,
        filt_pin: Output<P4>, 
        att0_pin: Output<P5>,
        mod1_pin: Output<P6>,
        mod2_pin: Output<P8>,
        agn_pin: Output<P9>,
    ) -> Self {
        Self {
            i2c,
            mute_pin,
            deem_pin,
            filt_pin,
            att0_pin,
            mod1_pin,
            mod2_pin,
            agn_pin,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // Initialize control pins
        self.mute_pin.clear();  // Unmuted
        self.deem_pin.clear();  // No de-emphasis
        self.filt_pin.clear();  // Default filter
        self.att0_pin.clear();  // No attenuation
        self.mod1_pin.clear();  // Mode 1 low
        self.mod2_pin.clear();  // Mode 2 low  
        self.agn_pin.clear();   // Analog gain low

        // Configure PCM5122 via I2C
        self.write_register(0x02, 0x11)?; // Standby
        self.write_register(0x03, 0x00)?; // Mute control
        self.write_register(0x04, 0x00)?; // PLL control
        self.write_register(0x0D, 0x10)?; // DAC control
        self.write_register(0x25, 0x08)?; // Digital filter select
        self.write_register(0x02, 0x00)?; // Exit standby

        Ok(())
    }

    pub fn mute(&mut self, mute: bool) {
        if mute {
            self.mute_pin.set();
        } else {
            self.mute_pin.clear();
        }
    }

    pub fn set_volume(&mut self, left: u8, right: u8) -> Result<(), &'static str> {
        self.write_register(0x3D, left)?;   // Left volume
        self.write_register(0x3E, right)?;  // Right volume
        Ok(())
    }

    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), &'static str> {
        let data = [reg, value];
        self.i2c.write_read(PCM5122_I2C_ADDR, &data, &mut [])
            .map_err(|_| "I2C write failed")?;
        Ok(())
    }
}

pub struct AudioSystem {
    pcm5122: PCM5122,
    sai: SaiDriver,
    oscillator: Oscillator,
    audio_buffer: [f32; BLOCK_SIZE * 2], // Stereo buffer
}

impl AudioSystem {
    pub fn new(
        pcm5122: PCM5122,
        sai: SaiDriver,
    ) -> Self {
        let oscillator = Oscillator::new(SAMPLE_RATE as f32);
        let audio_buffer = [0.0f32; BLOCK_SIZE * 2];
        
        Self {
            pcm5122,
            sai,
            oscillator,
            audio_buffer,
        }
    }

    pub fn init(&mut self, ccm: &mut teensy4_bsp::hal::ccm::CCM, iomuxc: &mut teensy4_bsp::hal::iomuxc::Iomuxc) -> Result<(), &'static str> {
        // Initialize PCM5122 DAC
        self.pcm5122.init()?;
        
        // Initialize SAI peripheral
        self.sai.init(ccm, iomuxc)?;
        
        // Set initial volume
        self.pcm5122.set_volume(200, 200)?; // ~75% volume
        
        Ok(())
    }

    pub fn start_playback(&mut self) -> Result<(), &'static str> {
        // Unmute DAC
        self.pcm5122.mute(false);
        
        // Start SAI transmission
        self.sai.start()?;
        
        Ok(())
    }

    pub fn generate_audio(&mut self, frequency_hz: f32) -> Result<(), &'static str> {
        // Generate audio samples using oscillator
        for i in (0..BLOCK_SIZE * 2).step_by(2) {
            let sample = self.oscillator.tick(frequency_hz);
            self.audio_buffer[i] = sample;     // Left channel
            self.audio_buffer[i + 1] = sample; // Right channel (mono to stereo)
        }
        
        // Send buffer to SAI
        self.sai.fill_buffer(&self.audio_buffer)?;
        
        Ok(())
    }

    pub fn set_frequency(&mut self, _frequency_hz: f32) {
        // Frequency is passed to generate_audio method
        // Could store it as state if needed for continuous generation
    }

    pub fn is_ready_for_buffer(&self) -> bool {
        self.sai.is_buffer_ready()
    }

    pub fn clear_ready_flag(&self) {
        self.sai.clear_buffer_ready();
    }
}