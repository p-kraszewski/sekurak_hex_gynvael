use super::*;
use crate::files;
use color_eyre::eyre::Result;
use eyre::WrapErr;

pub fn write_raw(out_file_name: &str, img: &Image) -> Result<()> {
    let mut f = files::File::create(out_file_name)?;

    for scanlines in &img.pixels {
        for pixels in scanlines.iter() {
            match pixels {
                Pixel::RGB8(ColorRGB8(r, g, b)) => {
                    let col = [*r, *g, *b];
                    f.write_exact(&col)?;
                }
                _ => unimplemented!(),
            }
        }
    }

    Ok(())
}
