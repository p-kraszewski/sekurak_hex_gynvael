use rand::{Rng, RngCore};
use std::{
    fs::File,
    io::{Read, Result, Write},
};

fn main() -> Result<()> {
    // Uchwyt do generatora danych losowych
    let mut rng = rand::thread_rng();

    // Otwarcie pliku do odczytu
    let mut ifile = File::open("data/sing_scape.bmp")?;

    // Wczytanie całego pliku do wektora
    let mut data = Vec::new();
    ifile.read_to_end(&mut data)?;

    // Potnij dane na modyfikowalne bloczki po 256B
    for mut d in data.chunks_mut(256) {
        // Losowy rozmiar
        let siz = rng.gen_range(1 ..= 8);

        // Losowy wskaźnik tak, aby rozmiar się zmieścił
        let offs = rng.gen_range(0 .. (d.len() - siz));

        // Wypełnij ten kawałek danymi losowymi
        rng.fill_bytes(&mut d[offs .. offs + siz]);
    }

    // Stwórz plik wyjściowy
    let mut ofile = File::create("data/sing_scape.broken")?;

    // Zapisz zmodyfikowane dane na dysku
    ofile.write_all(&data)?;

    Ok(())
}
