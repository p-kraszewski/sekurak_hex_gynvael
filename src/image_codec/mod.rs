pub mod bmp;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ColorRGB(u8, u8, u8);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pixel {
    RGB8(ColorRGB),
    RGBA(ColorRGB, u8),
    GRAY(u8),
    INDEX(u8),
}

pub type ScanLine = Vec<Pixel>;
pub type Frame = Vec<ScanLine>;
pub type Palette = Vec<ColorRGB>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    pub x_size:  usize,
    pub y_size:  usize,
    pub palette: Option<Palette>,
    pub pixels:  Frame,
}
