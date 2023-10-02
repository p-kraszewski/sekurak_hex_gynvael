use color_eyre::eyre::Result;
use rand::{Rng, RngCore};
use sekurak_hex_gynvael::files::File;

fn main() -> Result<()> {
    // Uchwyt do generatora danych losowych
    let mut rng = rand::thread_rng();

    // Otwarcie pliku do odczytu
    let mut ifile = File::open("data/spec1_1.bin")?;

    println!("a={}/", ifile.read_u8()?);
    println!("b={}/", ifile.read_i8()?);
    println!("c={}/", ifile.read_u16le()?);
    println!("d={}/", ifile.read_i16le()?);
    println!("e={}/", ifile.read_u32le()?);
    println!("f={}/", ifile.read_i32le()?);
    println!("g={}/", ifile.read_u16be()?);
    println!("h={}/", ifile.read_i16be()?);
    println!("i={}/", ifile.read_u32be()?);
    println!("j={}/", ifile.read_i32be()?);
    Ok(())
}
