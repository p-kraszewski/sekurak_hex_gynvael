# Przykłady w języku Rust do szkolenia Gyndvaela z HEX-ów

* Działają na wersji Rust `stable`
* Mają skrajnie uproszczoną obsługę błędów
* Generyczne funkcje do pracy z maskami (`sekurak_hex_gynvael::masks`) korzytstają z biblioteki `num`, z interfejsu (traita) `PrimInt`.

## Uruchamianie

### Testy funkcji na maskach

```cargo test```

### Przykłady

Część przykładów nic nie drukuje (albo niewiele) -- sprawdzają tylko porawność operacji (są bardziej jak testy :/ )

```
cargo run --example d1_01_kowersje_typow_prostych
cargo run --example d1_02_maski_pola
cargo run --example d1_03_indianie
cargo run --example d2_01_znak_bez_znaku
cargo run --example d2_02_leb128_vlq
```
