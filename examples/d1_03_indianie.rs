fn main() {
    // Rust ma wbudowane konwersje int <-> ciąg bajtów BE/LE

    // Z bajtów Little Endian
    // Uwaga! argument X::from_le_bytes to rzadko (w sumie) używana w Rust sztywna tablica `[u8;2]`,
    // `[u8;4]`, itd (zależnie od typu docelowego), *NIE* tzw. slice. `&[u8]` albo wektor
    // `Vec::<u8>`!
    assert_eq!(0xF6AE, u16::from_le_bytes([0xAE, 0xF6]));

    // Z little/big endian na bajty. Tak samo, zwracana jest sztywna tablica.
    assert_eq!([0x78, 0x56, 0x34, 0x12], 0x1234_5678_u32.to_le_bytes());
    assert_eq!([0x23, 0x01], 0x123_u16.to_le_bytes());
    assert_eq!(
        [0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0x34, 0x12],
        0x1234_5678_9ABC_DEF0_u64.to_le_bytes()
    );

    parsuj_szereg_u16_le(&[
        0x4a, 0x9c, 0x28, 0x81, 0x2f, 0xa6, 0xfb, 0xb6, 0x64, 0x6d, 0xb0, 0x99, 0x4a, 0x40, 0xbb,
        0x0e,
    ]);
}

fn parsuj_szereg_u16_le(dane: &[u8]) {
    // Potnij tablicę na kawałki po 2 bajty.
    for dwubajt in dane.chunks(2) {
        if dwubajt.len() != 2 {
            println!("Dane mają nieparzystą długość!");
        } else {
            // Rust nie ma ładnej składni konwersji slice'a `&[u8]` na tablicę `[u8;N]`. Nie chcę
            // psuć krajobrazu transmutacją.
            let mut bajty = [0u8; 2];

            // Rozmiar tablicy i slice'a *musi* być jednakowy, inaczej stdlib panikuje.
            bajty.copy_from_slice(dwubajt);

            // Konwersja
            let liczba = u16::from_le_bytes(bajty);

            // Wynik
            println!("{dwubajt:02x?} -> 0x{liczba:04x}={liczba}");
            //         ^^^^^^^^^^^^
            //         Wyświetlenie tablicy jako ciąg hex (trait `std::fmt::Debug`).
        }
    }
}
