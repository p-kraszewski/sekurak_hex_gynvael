use std::ops::RangeInclusive;

use num::PrimInt;

/// Generuje maskę dla konkretnego typu całkowitego na podstawie zakresu bitów
///
/// # Arguments
///
/// * `zakres`: zakres domknięty zawierający numer pierwszego i ostatniego bitu
///   włącznie
///
/// returns: N wynikowa maska
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::maska_z_zakresu;
/// //           7654_3210
/// assert_eq!(0b0000_1110, maska_z_zakresu::<u8>(1 ..= 3));
/// assert_eq!(0b0001_1100, maska_z_zakresu::<u8>(2 ..= 4));
/// assert_eq!(0b0000_0011, maska_z_zakresu::<u8>(0 ..= 1));
/// assert_eq!(0b1111_1111, maska_z_zakresu::<u8>(0 ..= 7));
/// // Odwrócone maski
/// assert_eq!(0b0000_1110, maska_z_zakresu::<u8>(3 ..= 1));
/// assert_eq!(0b0001_1100, maska_z_zakresu::<u8>(4 ..= 2));
/// assert_eq!(0b0000_0011, maska_z_zakresu::<u8>(1 ..= 0));
/// assert_eq!(0b1111_1111, maska_z_zakresu::<u8>(7 ..= 0));
/// ```
pub fn maska_z_zakresu<N: PrimInt>(zakres: RangeInclusive<usize>) -> N {
    // Ile bitów ma mieć wynikowa liczba
    let rozmiar = N::zero().count_zeros() as usize;

    // Pozwala traktować zakres 1..=3 tak samo jak 3..=1.
    //
    // Uwaga - odwrócony zakres stresuje lintera Clippy - według stdlib odwrócony
    // zakres jest PUSTY.
    let (numer_najmlodszego_bitu, numer_najstarszego_bitu) = parsuj_zakres(zakres);

    let dlugosc_zakresu = numer_najstarszego_bitu - numer_najmlodszego_bitu + 1;

    // Rust w wersji Debug obsługuje przepełnienia przy przesunięciach (przesunięcie
    // o więcej niż rozmiar liczby), musimy to obsłużyć
    let wynik = if dlugosc_zakresu >= (rozmiar - 1) {
        N::max_value()
    } else {
        (N::one() << dlugosc_zakresu) - N::one()
    };
    wynik << numer_najmlodszego_bitu
}

/// Generuje kopię wartości z wyczyszczonym polem bitowym
///
/// # Arguments
///
/// * `liczba`: liczba wejsciowa
/// * `zakres`: Zakres bitowy pola do czyszczenia
///
/// returns: N wyczyszczona liczba
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::czysc_pole;
/// assert_eq!(0b_1100_0011, czysc_pole(0b_1111_1111u8, 5 ..= 2));
/// ```
pub fn czysc_pole<N: PrimInt>(liczba: N, zakres: RangeInclusive<usize>) -> N {
    let maska = maska_z_zakresu::<N>(zakres.clone());

    // Maskowanie negacją (`!` w Rust) maski
    liczba & !maska
}

/// Czyści pole bitowe w miejscu
///
/// # Arguments
///
/// * `liczba`: Dane do modyfikacji
/// * `zakres`: Zakres bitowy pola do czyszczenia
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::czysc_pole_in_place;
/// let mut liczba = 0b_1111_1111_u8;
/// czysc_pole_in_place(&mut liczba, 5 ..= 2);
/// assert_eq!(0b_1100_0011, liczba);
/// ```
pub fn czysc_pole_in_place<N: PrimInt>(liczba: &mut N, zakres: RangeInclusive<usize>) {
    *liczba = czysc_pole(*liczba, zakres);
}

/// Ekstrakcja wartości z pola bitowego
///
/// # Arguments
///
/// * `liczba`: Liczba zawierająca dane wejściowe
/// * `zakres`:  Zakres bitowy pola do ekstrakcji
///
/// returns: N Pobrane dane
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::ekstrakcja;
/// assert_eq!(0b11_00, ekstrakcja(0b_1111_0000_u8, 5 ..= 2))
/// //                                 \^^^/- zakres wycięcia
/// ```
pub fn ekstrakcja<N: PrimInt>(liczba: N, zakres: RangeInclusive<usize>) -> N {
    // Wyliczenie maski
    let maska = maska_z_zakresu::<N>(zakres.clone());

    // Numer najmłodszego bitu maski
    let (najmlodszy, _) = parsuj_zakres(zakres);

    // Maskowanie i przesunięcie
    (liczba & maska) >> najmlodszy
}

/// Wstawienie wartości do pola bitowego
///
/// # Arguments
///
/// * `liczba`: dane z polem bitowym
/// * `wartosc`: dane do wstawienie
/// * `zakres`: Zakres bitowy pola do modyfikacji
///
/// returns: N Zaktualizowana kopia danych z polem bitowym
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::wstawienie;
/// assert_eq!(
///     0b_1110_1000_u8,
///     wstawienie(0b_1111_0000_u8, 0b_0_1, 4 ..= 3)
/// );
/// //               \_/                         \_/           \_/ To wstawiamy
/// //                Wynik                       Tu wstawiamy
/// ```
pub fn wstawienie<N: PrimInt>(liczba: N, wartosc: N, zakres: RangeInclusive<usize>) -> N {
    // Wyliczenie maski
    let maska = maska_z_zakresu::<N>(zakres.clone());

    // Numer najmłodszego bitu maski
    let (najmlodszy, _) = parsuj_zakres(zakres);

    // Zrobienie "dziury" na dane.  W Rust `!` to negacja bitowa
    let wymaskowane_wejscie = liczba & !maska;

    // Wstawienie nowej, przesuniętej i przyciętej wartości. Przycięcie, żeby
    // potencjalnie za duże dane nie "wylały" się poza przygotowaną dziurę.
    let dane = (wartosc << najmlodszy) & maska;

    // Suma logiczna
    wymaskowane_wejscie | dane
}

/// Aktualizacja wartości w polu bitowym bitowego
///
/// # Arguments
///
/// * `liczba`: dane do modyfikacji
/// * `wartosc`: dane do wstawienie
/// * `zakres`: Zakres bitowy pola do modyfikacji
///
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::wstawienie_in_place;
/// let mut liczba = 0b_1111_0000_u8;
/// wstawienie_in_place(&mut liczba, 0b_0_1, 4 ..= 3);
/// assert_eq!(0b_1110_1000_u8, liczba);
/// ```
pub fn wstawienie_in_place<N: PrimInt>(liczba: &mut N, wartosc: N, zakres: RangeInclusive<usize>) {
    *liczba = wstawienie(*liczba, wartosc, zakres);
}

/// Parsuje standardowe zakresy domknięte. Pozwala traktować zakres 1..=3 tak
/// samo jak 3..=1.
///
/// # Arguments
///
/// * `zakres`: zakres domknięty zawierający numer pierwszego i ostatniego bitu
///   włącznie
///
/// returns: (numer_najmłodszego_bitu: usize, numer_najstarszego_bitu: usize)
///
/// # Examples
///
/// ```
/// use sekurak_hex_gynvael::masks::parsuj_zakres;
/// assert_eq!((1, 3), parsuj_zakres(3 ..= 1));
/// assert_eq!((1, 3), parsuj_zakres(1 ..= 3));
/// ```
pub fn parsuj_zakres(zakres: RangeInclusive<usize>) -> (usize, usize) {
    // Pozwala traktować zakres 1..=3 tak samo jak 3..=1.
    //
    // Uwaga - odwrócony zakres stresuje lintera Clippy - według stdlib odwrócony
    // zakres jest PUSTY.
    if zakres.end() > zakres.start() {
        (*zakres.start(), *zakres.end())
    } else {
        (*zakres.end(), *zakres.start())
    }
}
