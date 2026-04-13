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
use teensy4_panic as _;

use little_synth_firmware::{audio, display::DummyDisplay};
use teensy4_bsp::board;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = audio::init_audio();
    let resources = board::t41(board::instances());

    // USB serial logging via imxrt-log. Appears as /dev/ttyACM0 on the host.
    // Using polling mode (Interrupts::Disabled); poller.poll() is called each loop iteration.
    // Switch to Interrupts::Enabled + unmask USB_OTG1 interrupt for lower-latency output.
    let mut poller = imxrt_log::log::usbd(resources.usb, imxrt_log::Interrupts::Disabled)
        .expect("USB logger init failed");

    log::set_max_level(log::LevelFilter::Debug);
    log::info!("little-synth booting");

    audio::init_audio();
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
        poller.poll();
        // cortex_m::asm::wfe();
    }
}
