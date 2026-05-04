//! little-synth firmware: Teensy 4.1, PCM5102 I2S out, UART MIDI, display interface.

#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use imxrt_ral::Interrupt;

use little_synth_firmware::{
    audio, display::DummyDisplay, sai_simple::SimpleSai, simple_audio::SimpleAudioSystem,
};
use teensy4_bsp as bsp;
use teensy4_bsp::interrupt;
use teensy4_panic as _;

use teensy4_bsp::board;

/// imxrt-log Poller must be driven from `poll()`; Teensy USB does not reliably enumerate when
/// interrupts are disabled — see teensy4-bsp `rtic_defmt_usb_log` ("need USB interrupts").
static USB_POLLER: Mutex<RefCell<Option<imxrt_log::Poller>>> =
    Mutex::new(RefCell::new(None));

#[cortex_m_rt::interrupt]
fn USB_OTG1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(p) = USB_POLLER.borrow(cs).borrow_mut().as_mut() {
            p.poll();
        }
    });
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board_resources = board::t41(board::instances());

    // USB CDC logging (imxrt-log). Build with full-speed USB — see workspace `.cargo/config.toml`
    // (`IMXRT_LOG_USB_SPEED=FULL`, bulk MPS 64). Product string on host: "imxrt-log".
    let poller = imxrt_log::log::usbd(board_resources.usb, imxrt_log::Interrupts::Enabled)
        .expect("USB logger init failed");

    cortex_m::interrupt::free(|cs| {
        *USB_POLLER.borrow(cs).borrow_mut() = Some(poller);
    });

    // Allow ISR to service attach/enumeration (polling-only mode is insufficient on this chip).
    unsafe {
        NVIC::unmask(Interrupt::USB_OTG1);
    }
    NVIC::pend(Interrupt::USB_OTG1);

    log::set_max_level(log::LevelFilter::Debug);
    log::info!("little-synth booting");

    audio::init_audio();
    let _screen = DummyDisplay::new(320, 240);

    let debug_pin = bsp::board::led(&mut board_resources.gpio2, board_resources.pins.p13);

    let sai = match SimpleSai::new() {
        Ok(sai) => sai,
        Err(_) => loop {
            cortex_m::asm::bkpt();
        },
    };

    let mut audio_system = SimpleAudioSystem::new(debug_pin, sai);

    if audio_system.init().is_err() {
        loop {
            cortex_m::asm::bkpt();
        }
    }

    if audio_system.start().is_err() {
        loop {
            cortex_m::asm::bkpt();
        }
    }

    let frequency = 440.0_f32;

    loop {
        if audio_system.process_audio_block(frequency).is_err() {
            cortex_m::asm::bkpt();
        }

        cortex_m::asm::delay(2_670 * 600);
    }
}
