use std::fmt;

pub mod conversions;
pub mod masks;

pub fn pokaz_liczbe<N>(liczba: N, nazwa: &str)
where
    N: fmt::Display + fmt::LowerHex + fmt::Binary + fmt::Octal,
{
    // UWAGA! Rust wyświetla ze znakiem TYLKO liczby dziesiętne. Wszystkie inne
    // wyświetla ignorując znak - traktując typy iXX jak uXX.
    println!("{nazwa} = \n\tHEX:{liczba:x}\n\tDEC:{liczba}\n\tOCT:{liczba:o}\n\tBIN:{liczba:b}");
}
