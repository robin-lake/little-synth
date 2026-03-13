//! little-synth firmware: Teensy 4.1, PCM5102 I2S out, UART MIDI, display interface.

#![no_std]
#![no_main]

use little_synth_firmware::{audio, display::DummyDisplay};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let _board = audio::init_audio();

    // Display: use DummyDisplay until a real screen driver is added
    let _screen = DummyDisplay::new(320, 240);

    loop {
        cortex_m::asm::wfe();
    }
}
