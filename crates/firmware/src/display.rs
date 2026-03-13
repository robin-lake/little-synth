//! Display interface for little-synth. No UI implementation yet; this module
//! defines the trait and types so firmware can be written against the abstraction.
//!
//! Implement this trait for your chosen display (e.g. SPI TFT, I2C OLED) in the
//! same crate or a feature-gated module.

/// Pixel format for the framebuffer or draw operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelFormat {
    /// 16-bit RGB565
    Rgb565,
    /// 8-bit grayscale
    Grayscale8,
    /// 1-bit monochrome
    Monochrome,
}

/// Rectangle for dirty regions or clipping.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// Display interface: implement for your hardware (SPI/I2C screen).
/// All methods are optional to allow minimal implementations; UI will use what's available.
pub trait Display {
    /// Width and height in pixels.
    fn size(&self) -> (u16, u16);

    /// Pixel format.
    fn format(&self) -> PixelFormat;

    /// Clear entire display to color (format-dependent).
    fn clear(&mut self, color: u32);

    /// Flush a region to the display (e.g. after drawing).
    fn flush_rect(&mut self, rect: Rect);

    /// Flush entire buffer.
    fn flush(&mut self) {
        let (w, h) = self.size();
        self.flush_rect(Rect {
            x: 0,
            y: 0,
            w,
            h,
        });
    }
}

/// Dummy display for bring-up when no screen is connected.
#[derive(Debug)]
pub struct DummyDisplay {
    width: u16,
    height: u16,
}

impl DummyDisplay {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Display for DummyDisplay {
    fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn format(&self) -> PixelFormat {
        PixelFormat::Rgb565
    }

    fn clear(&mut self, _color: u32) {}

    fn flush_rect(&mut self, _rect: Rect) {}
}
