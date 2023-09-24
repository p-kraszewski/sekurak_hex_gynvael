pub fn leb128(bajty: &[u8]) -> Option<u128> {
    // Pusta tablica
    if bajty.is_empty() {
        return None;
    }

    // Tablica nie kończy się niskim bajtem. Unwrap, bo wiemy że jest niepusta.
    if bajty.last().unwrap() >= &0x80 {
        return None;
    }

    // Tablica zawiera niskie bajty na pozycjach przed ostatnią.
    if bajty[..bajty.len() - 1].iter().any(|v| v < &0x80) {
        return None;
    }

    // użyj prawego złożenia (right fold, rfold), aby zagregować wektor do liczby. Prawego, bo mamy
    // little endian i musimy składać w odwrotnej kolejności.
    // Więcej szczegółów of foldach na https://pl.wikipedia.org/wiki/Fold
    Some(
        bajty
            .iter()
            .rfold(0_u128, |acc, v| (acc << 7) | (*v & 0x7F) as u128),
    )
}

pub fn vlq128(bajty: &[u8]) -> Option<u128> {
    // Pusta tablica
    if bajty.is_empty() {
        return None;
    }

    // Tablica nie kończy się niskim bajtem. Unwrap, bo wiemy że jest niepusta.
    if bajty.last().unwrap() >= &0x80 {
        return None;
    }

    // Tablica zawiera niskie bajty na pozycjach przed ostatnią.
    if bajty[..bajty.len() - 1].iter().any(|v| v < &0x80) {
        return None;
    }

    // użyj lewego złożenia (left fold, lfold, fold), aby zagregować wektor do liczby. Lewego, bo mamy
    // big endian i musimy składać naturalnej kolejności.
    Some(
        bajty
            .iter()
            .fold(0_u128, |acc, v| (acc << 7) | (*v & 0x7F) as u128),
    )
}

pub fn leb128v(data: &[u8]) -> Vec<u128> {
    // Funkcja pomija niepoprawne wartości - w tym wariancie niekompletną ostatnią liczbę.

    // Potnij wektor wejściowy na wartościach ze zgaszonym najwyższym bitem. W wyniku dostajemy
    // wektor referencji do kolejnych podwektorów (reuse danych). Wersja `_inclusive` dołącza
    // separator do ciętych danych.

    // [1,2,0,3,4].split(|x| x==0) -> [[1,2],[3,4]]
    // [1,2,0,3,4].split_inclusive(|x| x==0) -> [[1,2,0],[3,4]]

    let dane_skladowe = data
        .split_inclusive(|x| (*x & 0x80) == 0)
        .collect::<Vec<_>>();

    // Po kolei dla każdego podwektora wejściowego zrób konwersję. Funkcja `filter_map` pomija
    // konwersje, które zwróciły None (niepoprawne dane wejściowe)
    dane_skladowe
        .iter()
        .filter_map(|bajty| leb128(bajty))
        .collect::<Vec<_>>()
}

pub fn vlq128v(data: &[u8]) -> Vec<u128> {
    // Funkcja pomija niepoprawne wartości - w tym wariancie niekompletną ostatnią liczbę.

    // Potnij wektor wejściowy na wartościach ze zgaszonym najwyższym bitem. W wyniku dostajemy
    // wektor referencji do kolejnych podwektorów (reuse danych). Wersja `_inclusive` dołącza
    // separator do ciętych danych.

    // [1,2,0,3,4].split(|x| x==0) -> [[1,2],[3,4]]
    // [1,2,0,3,4].split_inclusive(|x| x==0) -> [[1,2,0],[3,4]]

    let dane_skladowe = data
        .split_inclusive(|x| (*x & 0x80) == 0)
        .collect::<Vec<_>>();

    // Po kolei dla każdego podwektora wejściowego zrób konwersję. Funkcja `filter_map` pomija
    // konwersje, które zwróciły None (niepoprawne dane wejściowe)
    dane_skladowe
        .iter()
        .filter_map(|bajty| vlq128(bajty))
        .collect::<Vec<_>>()
}
