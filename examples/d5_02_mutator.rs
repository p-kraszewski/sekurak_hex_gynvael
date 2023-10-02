use rand::{Rng, RngCore};
use std::{
    fs::File,
    io::{Read, Result, Write},
};

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut ifile = File::open("data/sing_scape.bmp")?;
    let mut data = Vec::new();
    ifile.read_to_end(&mut data)?;

    for mut d in data.chunks_mut(256) {
        let offs = rng.gen_range(0 .. (d.len() - 8));
        let siz = rng.gen_range(1 ..= 8);
        rng.fill_bytes(&mut d[offs .. offs + siz]);
    }

    let mut ofile = File::create("data/sing_scape.broken")?;
    ofile.write_all(&data)?;

    Ok(())
}
