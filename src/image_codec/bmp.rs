#![allow(non_snake_case)]

use crate::files;
use color_eyre::eyre::Result;
use eyre::{eyre, WrapErr};
use log::debug;
use std::io::SeekFrom;

// Wciągnięcie wszystkich struktur z modułu nadrzędnego
use super::*;

enum Compression {
    BI_RGB,
    BI_BITFIELDS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BitmapFileHeader {
    bfType: u16,
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    bfOffBits: u32,
}

impl Validate for BitmapFileHeader {
    fn validate(&self) -> Result<()> {
        if self.bfType != 0x424f {
            return Err(eyre!("Invalid BMP magic"));
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct BitmapInfoHeader {
    biSize: u32,
    biWidth: i32,
    biHeight: i32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: u32,
    biSizeImage: u32,
    biXPelsPerMeter: i32,
    biYPelsPerMeter: i32,
    biClrUsed: u32,
    biClrImportant: u32,
}

impl Validate for BitmapInfoHeader {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

struct BMP {
    file: files::File,
}

pub fn read_bmp(in_file_name: &str) -> Result<Image> {
    let mut bmp = BMP {
        file: files::File::open(in_file_name)
            .wrap_err_with(|| format!("Failed to open {in_file_name:?}"))?,
    };

    let bfh = bmp.read_bfh()?;
    bfh.validate()?;
    debug!("BFH={bfh:?}");

    let bih = bmp.read_bih()?;
    bih.validate()?;
    debug!("BIH={bih:?}");

    let pal = if bih.biBitCount == 8 {
        let colors = match bih.biClrUsed {
            0 => 256,
            n if n <= 256 => n,
            _ => return Err(eyre!("Invalid palette size")),
        };

        Some(bmp.read_palette(colors)?)
    } else {
        None
    };
    //debug!("PAL={pal:?}");

    let pixels = if let Some(pal) = pal {
        bmp.read_pixels_pal(bih.biWidth, bih.biHeight, &pal, bfh.bfOffBits as u64)?
    } else {
        unimplemented!()
    };

    //   debug!("PIX={pixels:?}");

    Ok(Image {
        x_size: bih.biWidth as usize,
        y_size: bih.biHeight as usize,
        palette: None,
        pixels,
    })
}

impl BMP {
    pub fn read_bfh(&mut self) -> Result<BitmapFileHeader> {
        let f = &mut self.file;
        Ok(BitmapFileHeader {
            bfType: f.read_u16le()?,
            bfSize: f.read_u32le()?,
            bfReserved1: f.read_u16le()?,
            bfReserved2: f.read_u16le()?,
            bfOffBits: f.read_u32le()?,
        })
    }

    pub fn read_bih(&mut self) -> Result<BitmapInfoHeader> {
        let f = &mut self.file;
        Ok(BitmapInfoHeader {
            biSize: f.read_u32le()?,
            biWidth: f.read_i32le()?,
            biHeight: f.read_i32le()?,
            biPlanes: f.read_u16le()?,
            biBitCount: f.read_u16le()?,
            biCompression: f.read_u32le()?,
            biSizeImage: f.read_u32le()?,
            biXPelsPerMeter: f.read_i32le()?,
            biYPelsPerMeter: f.read_i32le()?,
            biClrUsed: f.read_u32le()?,
            biClrImportant: f.read_u32le()?,
        })
    }

    pub fn read_palette(&mut self, len: u32) -> Result<Palette> {
        let f = &mut self.file;
        let data = f.read_as_vec(len as usize * 4)?;
        let mut pal = Vec::with_capacity(len as usize);

        for (i, quad) in data.chunks(4).enumerate() {
            let (R, G, B) = (quad[2], quad[1], quad[0]);
            if i < 128 {
                pal.push(ColorRGB8(R, G, B));
            } else {
                pal.push(ColorRGB8(255 - R, 255 - G, 255 - B))
            }
        }

        Ok(pal)
    }

    pub fn read_pixels_pal(&mut self, xs: i32, ys: i32, pal: &Palette, pos: u64) -> Result<Frame> {
        let f = &mut self.file;
        f.seek(SeekFrom::Start(pos))?;
        let (rev, ys) = if ys < 0 { (false, -ys) } else { (true, ys) };
        let mut bitmap = Vec::with_capacity(ys as usize);
        for y in 0..ys {
            let line = f.read_as_vec(xs as usize)?;
            if xs % 4 != 0 {
                f.read_as_vec(xs as usize % 4)?;
            }
            let scanline = line
                .iter()
                .map(|idx| Pixel::RGB8(pal[*idx as usize]))
                .collect::<Vec<_>>();

            bitmap.push(scanline);
        }

        if rev {
            bitmap.reverse()
        }

        Ok(bitmap)
    }
}
