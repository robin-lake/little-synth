//! little-synth firmware: Teensy 4.1, PCM5102 I2S out, UART MIDI, display interface.

#![no_std]
#![no_main]

use little_synth_firmware::{
    audio, 
    display::DummyDisplay, 
    simple_audio::SimpleAudioSystem,
    sai_simple::SimpleSai
};
// embedded_hal traits are used by SimpleAudioSystem generics
use teensy4_bsp as bsp;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = audio::init_audio();

    // Display: use DummyDisplay until a real screen driver is added
    let _screen = DummyDisplay::new(320, 240);

    // Create debug LED pin (built-in LED on Teensy 4.1)
    let debug_pin = bsp::board::led(&mut board.gpio2, board.pins.p13);

    // Initialize SAI peripheral
    let sai = match SimpleSai::new() {
        Ok(sai) => sai,
        Err(_) => {
            // Failed to initialize SAI
            loop {
                cortex_m::asm::bkpt();
            }
        }
    };

    // Create simplified audio system
    let mut audio_system = SimpleAudioSystem::new(debug_pin, sai);

    // Initialize audio system
    if let Err(_) = audio_system.init() {
        // Failed to initialize audio
        loop {
            cortex_m::asm::bkpt();
        }
    }

    // Start audio system
    if let Err(_) = audio_system.start() {
        // Failed to start audio
        loop {
            cortex_m::asm::bkpt();
        }
    }

    // Main audio loop - generate 440Hz A4 note
    let frequency = 440.0; // A4
    
    loop {
        // Process audio block (generates samples and toggles LED)
        if let Err(_) = audio_system.process_audio_block(frequency) {
            // Audio processing failed
            cortex_m::asm::bkpt();
        }
        
        // Simulate audio sample rate timing
        // 128 samples at 48kHz = ~2.67ms per block
        cortex_m::asm::delay(2_670 * 600); // Roughly 2.67ms at 600MHz
    }
}
