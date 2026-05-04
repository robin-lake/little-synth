//! Simplified SAI (Serial Audio Interface) implementation for Teensy 4.1
//! This provides a working SAI configuration without the complex DMA setup.

// teensy4-bsp imports not needed in simplified RAL implementation
use imxrt_ral as ral;

pub const SAMPLE_RATE: u32 = 48_000;
pub const BLOCK_SIZE: usize = 128;

pub struct SimpleSai {
    // SAI peripheral instance for register access
}

impl SimpleSai {
    pub fn new() -> Result<Self, &'static str> {
        // Clock configuration is handled by the BSP for now
        // In a complete implementation, this would configure:
        // - SAI1 clock gate enable
        // - Clock source selection (PLL3 PFD3)
        // - Clock dividers for 48kHz sample rate
        
        Ok(Self {})
    }

    pub fn init_sai(&self) -> Result<(), &'static str> {
        let sai1 = unsafe { ral::sai::SAI1::instance() };

        // Disable SAI transmitter first
        ral::modify_reg!(ral::sai, sai1, TCSR, TE: 0);

        // Reset SAI transmitter
        ral::modify_reg!(ral::sai, sai1, TCSR, SR: 1);
        
        // Small delay for reset
        for _ in 0..1000 {
            cortex_m::asm::nop();
        }
        
        ral::modify_reg!(ral::sai, sai1, TCSR, SR: 0);

        // Configure transmitter control register 2
        ral::write_reg!(ral::sai, sai1, TCR2,
            SYNC: 0,    // Asynchronous mode
            BCS: 0,     // Bit clock generated externally (by MCLK)
            BCI: 0,     // Bit clock not inverted
            MSEL: 1,    // MCLK selected  
            BCP: 0,     // Bit clock active high
            BCD: 1      // Bit clock generated internally
        );

        // Configure transmitter control register 3
        ral::write_reg!(ral::sai, sai1, TCR3,
            WDFL: 0,    // Word flag configuration
            TCE: 1      // Transmit channel 0 enable
        );

        // Configure transmitter control register 4 (frame configuration)
        ral::write_reg!(ral::sai, sai1, TCR4,
            FRSZ: 1,    // Frame size = 2 words (stereo)
            SYWD: 15,   // Sync width = 16 clocks
            MF: 1,      // MSB first
            FSE: 1,     // Frame sync asserted one bit early
            FSP: 0,     // Frame sync active high
            FSD: 1      // Frame sync generated internally
        );

        // Configure transmitter control register 5 (word configuration)
        ral::write_reg!(ral::sai, sai1, TCR5,
            WNW: 15,    // Word N width = 16 bits - 1
            W0W: 15,    // Word 0 width = 16 bits - 1
            FBT: 15     // First bit shifted = 16 - 1
        );

        // Configure transmitter mask register
        ral::write_reg!(ral::sai, sai1, TMR, TWM: 0); // No words masked

        // Set FIFO watermark
        ral::write_reg!(ral::sai, sai1, TCR1, TFW: 2); // FIFO watermark = 2

        Ok(())
    }

    pub fn start_transmission(&self) -> Result<(), &'static str> {
        let sai1 = unsafe { ral::sai::SAI1::instance() };

        // Enable transmitter
        ral::modify_reg!(ral::sai, sai1, TCSR, TE: 1);

        Ok(())
    }

    pub fn send_sample(&self, sample: i16) -> Result<(), &'static str> {
        let sai1 = unsafe { ral::sai::SAI1::instance() };

        // Check if FIFO has space (FWF = FIFO warning flag)
        let status = ral::read_reg!(ral::sai, sai1, TCSR);
        if (status & (1 << 17)) == 0 {  // FWF bit
            return Err("FIFO full");
        }

        // Write sample to transmit data register
        sai1.TDR[0].write(sample as u32);

        Ok(())
    }

    pub fn send_stereo_sample(&self, left: i16, right: i16) -> Result<(), &'static str> {
        // Send left channel
        self.send_sample(left)?;
        // Send right channel  
        self.send_sample(right)?;
        Ok(())
    }

    pub fn is_fifo_ready(&self) -> bool {
        let sai1 = unsafe { ral::sai::SAI1::instance() };
        let status = ral::read_reg!(ral::sai, sai1, TCSR);
        (status & (1 << 17)) != 0  // FWF bit - FIFO warning flag
    }

    pub fn send_audio_block(&self, samples: &[f32]) -> Result<(), &'static str> {
        // Convert f32 samples to i16 and send
        for chunk in samples.chunks(2) {
            if chunk.len() == 2 {
                let left = (chunk[0].clamp(-1.0, 1.0) * 32767.0) as i16;
                let right = (chunk[1].clamp(-1.0, 1.0) * 32767.0) as i16;
                
                // Wait for FIFO space if needed
                let mut timeout = 10000;
                while !self.is_fifo_ready() && timeout > 0 {
                    cortex_m::asm::nop();
                    timeout -= 1;
                }
                
                if timeout == 0 {
                    return Err("FIFO timeout");
                }
                
                self.send_stereo_sample(left, right)?;
            }
        }
        Ok(())
    }
}