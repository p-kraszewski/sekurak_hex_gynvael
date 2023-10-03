pub mod bmp;
pub mod raw;

use eyre::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorRGB8(u8, u8, u8);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pixel {
    RGB8(ColorRGB8),
    RGBA8(ColorRGB8, u8),
    GRAY8(u8),
    INDEX8(u8),
}

pub type ScanLine = Vec<Pixel>;
pub type Frame = Vec<ScanLine>;
pub type Palette = Vec<ColorRGB8>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    pub x_size: usize,
    pub y_size: usize,
    pub palette: Option<Palette>,
    pub pixels: Frame,
}

pub trait Validate {
    fn validate(&self) -> Result<()>;
}
