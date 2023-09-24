fn main() {
    // Rust oczywiście robi komplet konwersji ze znakiem

    println!("a) {}", i8::from_le_bytes([0xfe]));
    println!("b) {}", i8::from_le_bytes([0x70]));
    println!("c) {}", i16::from_be_bytes([0x82, 0x53]));
    println!("d) {}", i16::from_le_bytes([0x82, 0x53]));
    println!(
        "e) {}",
        i64::from_le_bytes([0xA2, 0x34, 0x12, 0x34, 0x77, 0x77, 0x66, 0x7F])
    );
    println!(
        "f) {}",
        i64::from_be_bytes([0xB0, 0xC2, 0x13, 0x1A, 0x00, 0x9C, 0x00, 0x7F])
    );
    parsuj_szereg_i16_le(&[0xf6, 0x5f, 0x9f, 0xbe, 0x3c, 0x54, 0xd3, 0xd0, 0xc5, 0xcb]);
}

fn parsuj_szereg_i16_le(dane: &[u8]) {
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
            let liczba = i16::from_le_bytes(bajty);

            // Wynik
            println!("{dwubajt:02x?} -> 0x{liczba:04x}={liczba}");
            //         ^^^^^^^^^^^^
            //         Wyświetlenie tablicy jako ciąg hex (trait `std::fmt::Debug`).
        }
    }
}
