// Linter CLIPPY jest dość wnikliwy. Musimy powyłączać parę warningów, które absolutnie mają sens w
// kodzie produkcyjnym

// Clippy widzi, że obie stałe liczbowe w assert_eq! są sobie równe.
#![allow(clippy::eq_op)]
// Clippy marudzi, że w przykładzie z "kładką" grupy cyfr dwójkowych nie mają jednakowych rozmiarów
#![allow(clippy::unusual_byte_groupings)]
use sekurak_hex_gynvael::pokaz_liczbe;

fn main() {
    // Jeżeli programista nie powie inaczej, a kompilator nie ma innych wskazówek (inferencja
    // typów), przyjmowany jest i32 (32 bitowa liczba ze znakiem)

    // Konwersje na wejściu
    assert_eq!(0b111, 7, "Konwersja z liczby dwójkowej");
    assert_eq!(0o111, 73, "Konwersja z liczby ósemkowej");
    assert_eq!(0x111, 273, "Konwersja z liczby szesnastkowej");

    // Rust umożliwia "kładki" do formatowania liczb
    assert_eq!(
        0b_1_0000_0000, 256,
        "Konwersja z liczby dwójkowej z grupowaniem cyfr"
    );

    // Formatowanie wyników

    let liczba = 0x1234;
    pokaz_liczbe(liczba, "Liczba 0x1234");

    pokaz_liczbe(u8::MAX, "Maksymalne 8 bitów bez znaku");
    pokaz_liczbe(i8::MIN, "Minimalne 8 bitów ze znakiem");
    pokaz_liczbe(i8::MAX, "Maksymalne 8 bitów ze znakiem");

    pokaz_liczbe(u16::MAX, "Maksymalne 16 bitów bez znaku");
    pokaz_liczbe(i16::MIN, "Minimalne 16 bitów ze znakiem");
    pokaz_liczbe(i16::MAX, "Maksymalne 16 bitów ze znakiem");

    pokaz_liczbe(u32::MAX, "Maksymalne 32 bity bez znaku");
    pokaz_liczbe(i32::MIN, "Minimalne 32 bity ze znakiem");
    pokaz_liczbe(i32::MAX, "Maksymalne 32 bity ze znakiem");

    pokaz_liczbe(u64::MAX, "Maksymalne 64 bity bez znaku");
    pokaz_liczbe(i64::MIN, "Minimalne 64 bity ze znakiem");
    pokaz_liczbe(i64::MAX, "Maksymalne 64 bity ze znakiem");

    pokaz_liczbe(u128::MAX, "Maksymalne 128 bitów bez znaku");
    pokaz_liczbe(i128::MIN, "Minimalne 128 bitów ze znakiem");
    pokaz_liczbe(i128::MAX, "Maksymalne 128 bitów ze znakiem");
}
