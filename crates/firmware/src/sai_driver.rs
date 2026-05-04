//! SAI (Synchronous Audio Interface) driver for Teensy 4.1 I2S audio output to PCM5122.
//!
//! Pin Configuration:
//! - SAI1_BCLK: Pin 21 (GPIO_AD_B1_14)
//! - SAI1_SYNC: Pin 20 (GPIO_AD_B1_15) - LRCLK 
//! - SAI1_TX_DATA0: Pin 7 (GPIO_B1_01) - Data to DAC
//! - SAI1_MCLK: Pin 23 (GPIO_AD_B1_09) - Master clock

use teensy4_bsp::hal::{iomuxc, ccm};
use imxrt_ral::{sai, dma, dmamux};
use imxrt_ral::{modify_reg, read_reg, write_reg};
use cortex_m::interrupt;
use core::sync::atomic::{AtomicBool, Ordering};

pub const SAMPLE_RATE: u32 = 48_000;
pub const BLOCK_SIZE: usize = 128;
pub const NUM_BUFFERS: usize = 2;
pub const BUFFER_SIZE_SAMPLES: usize = BLOCK_SIZE * 2; // Stereo interleaved

static mut AUDIO_BUFFERS: [[i16; BUFFER_SIZE_SAMPLES]; NUM_BUFFERS] = [[0; BUFFER_SIZE_SAMPLES]; NUM_BUFFERS];
static mut CURRENT_BUFFER: usize = 0;
static BUFFER_READY: AtomicBool = AtomicBool::new(false);

pub struct SaiDriver {
    sai1: sai::Instance,
    _dma: dma::Instance,
    _dmamux: dmamux::Instance,
}

impl SaiDriver {
    pub fn new() -> Option<Self> {
        let sai1 = sai::SAI1::take()?;
        let dma = dma::DMA0::take()?;
        let dmamux = dmamux::DMAMUX::take()?;

        Some(Self {
            sai1,
            _dma: dma,
            _dmamux: dmamux,
        })
    }

    pub fn init(&mut self, ccm: &mut ccm::CCM, iomuxc: &mut iomuxc::Iomuxc) -> Result<(), &'static str> {
        // Configure SAI pins
        self.configure_pins(iomuxc)?;
        
        // Configure clocks
        self.configure_clocks(ccm)?;

        // Configure SAI peripheral
        self.configure_sai()?;

        // Configure DMA
        self.configure_dma()?;

        Ok(())
    }

    fn configure_pins(&self, _iomuxc: &mut iomuxc::Iomuxc) -> Result<(), &'static str> {
        // Pin configuration will be handled by teensy4-bsp
        // The actual pin muxing is complex and board-specific
        // For now, we'll rely on the BSP's default configurations
        Ok(())
    }

    fn configure_clocks(&self, _ccm: &mut ccm::CCM) -> Result<(), &'static str> {
        // Clock configuration is complex and board-specific
        // For initial implementation, we'll rely on default clocks
        // TODO: Implement proper SAI clock configuration
        Ok(())
    }

    fn configure_sai(&mut self) -> Result<(), &'static str> {
        // Reset SAI transmitter
        write_reg!(sai, &self.sai1, TCSR, SR: 1);
        write_reg!(sai, &self.sai1, TCSR, SR: 0);

        // Configure transmitter control register
        write_reg!(sai, &self.sai1, TCR1, TFW: 0); // FIFO watermark

        // Configure word width and frame sync
        write_reg!(sai, &self.sai1, TCR2, 
            SYNC: 0,    // Asynchronous mode
            BCS: 1,     // Bit clock generated internally
            BCI: 0,     // Bit clock not inverted  
            MSEL: 1,    // MCLK selected
            BCP: 0,     // Bit clock active high
            BCD: 1      // Bit clock generated internally
        );

        // Configure frame sync
        write_reg!(sai, &self.sai1, TCR3,
            WDFL: 0,    // Word flag configuration
            TCE: 1      // Transmit channel enable
        );

        // Configure frame configuration
        write_reg!(sai, &self.sai1, TCR4,
            FRSZ: 1,    // Frame size = 2 words (stereo)
            SYWD: 15,   // Sync width = 16 bits
            MF: 1,      // MSB first
            FSE: 1,     // Frame sync early
            FSP: 0,     // Frame sync active high
            FSD: 1      // Frame sync generated internally
        );

        // Configure word configuration  
        write_reg!(sai, &self.sai1, TCR5,
            WNW: 15,    // Word N width = 16 bits
            W0W: 15,    // Word 0 width = 16 bits  
            FBT: 15     // First bit shifted = 16 bits
        );

        // Configure transmit mask
        write_reg!(sai, &self.sai1, TMR, TWM: 0); // No words masked

        Ok(())
    }

    fn configure_dma(&self) -> Result<(), &'static str> {
        // DMA configuration would go here
        // For now, we'll use polling mode
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        // Enable FIFO request DMA and enable transmitter
        modify_reg!(sai, &self.sai1, TCSR, 
            FRDE: 1,    // FIFO request DMA enable
            TE: 1       // Transmitter enable
        );

        Ok(())
    }

    pub fn stop(&mut self) {
        // Disable transmitter and DMA
        modify_reg!(sai, &self.sai1, TCSR, 
            TE: 0,      // Transmitter disable
            FRDE: 0     // FIFO request DMA disable
        );
    }

    pub fn fill_buffer(&mut self, samples: &[f32]) -> Result<(), &'static str> {
        if samples.len() != BLOCK_SIZE * 2 {
            return Err("Invalid buffer size");
        }

        unsafe {
            let buffer_idx = CURRENT_BUFFER;
            let buffer = &mut AUDIO_BUFFERS[buffer_idx];
            
            // Convert f32 samples to i16
            for (i, &sample) in samples.iter().enumerate() {
                let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
                buffer[i] = sample_i16;
            }

            // Swap buffers
            CURRENT_BUFFER = (CURRENT_BUFFER + 1) % NUM_BUFFERS;
            BUFFER_READY.store(true, Ordering::Release);
        }

        // For polling mode, directly write to FIFO
        self.write_fifo_polling()?;

        Ok(())
    }

    fn write_fifo_polling(&mut self) -> Result<(), &'static str> {
        unsafe {
            let prev_buffer_idx = (CURRENT_BUFFER + NUM_BUFFERS - 1) % NUM_BUFFERS;
            let buffer = &AUDIO_BUFFERS[prev_buffer_idx];

            for &sample in buffer.iter() {
                // Wait for FIFO space
                while read_reg!(sai, &self.sai1, TCSR, FWF) == 0 {
                    // FIFO not ready
                }
                
                // Write sample to transmit data register
                write_reg!(sai, &self.sai1, TDR0, TDR: sample as u32);
            }
        }

        Ok(())
    }

    pub fn is_buffer_ready(&self) -> bool {
        BUFFER_READY.load(Ordering::Acquire)
    }

    pub fn clear_buffer_ready(&self) {
        BUFFER_READY.store(false, Ordering::Release);
    }
}