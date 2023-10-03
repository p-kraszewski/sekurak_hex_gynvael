use clap::Parser;
use env_logger::Env;
use eyre::Result;
use log::debug;
use rand::{Rng, RngCore};
use std::{
    fs::File,
    io::{Read, Write},
};

use sekurak_hex_gynvael::image_codec as ic;

#[derive(Parser)]
#[command(name = "bmp2png")]
#[command(author = "Pyth0n")]
#[command(version = "1.0")]
#[command(about = "Simple UNZIP implementation", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "data/sing_scape.bmp")]
    /// Input ZIP file
    in_file: String,

    /// Directory to unpack to
    #[arg(short, long, default_value = "data/sing_scape.broken.bmp")]
    out_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Uchwyt do generatora danych losowych
    let mut rng = rand::thread_rng();

    // Otwarcie pliku do odczytu
    let mut ifile = File::open(&args.in_file)?;

    // Wczytanie całego pliku do wektora
    let mut data = Vec::new();
    ifile.read_to_end(&mut data)?;

    // Potnij dane na modyfikowalne bloczki po 256B
    for mut d in data.chunks_mut(256) {
        // Losowy rozmiar
        let siz = rng.gen_range(1..=8);

        // Losowy wskaźnik tak, aby rozmiar się zmieścił
        let offs = rng.gen_range(0..(d.len() - siz));

        // Wypełnij ten kawałek danymi losowymi
        rng.fill_bytes(&mut d[offs..offs + siz]);
    }

    // Stwórz plik wyjściowy
    let mut ofile = File::create(&args.out_file)?;

    // Zapisz zmodyfikowane dane na dysku
    ofile.write_all(&data)?;

    Ok(())
}
