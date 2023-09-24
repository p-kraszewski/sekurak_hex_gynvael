use sekurak_hex_gynvael::conversions::*;

fn main() {
    let dane_leb128 = &[
        0xFB, 0x41, 0xE4, 0x95, 0xB7, 0x05, 0xF1, 0x7B, 0x15, 0xB6, 0xE4, 0xED, 0xAF, 0xA3, 0xCC,
        0x67,
    ];
    let wynik_leb128 = leb128v(dane_leb128);
    assert_eq!(
        vec![8443_u128, 11389668, 15857, 21, 455619626365494],
        wynik_leb128
    );

    // Dane z niekompletną ostatnią liczbą, powinna zostać pominięta
    let dane_leb128_z_bledem = &[0x15, 0xB6];
    let wynik_leb128_z_bledem = leb128v(dane_leb128_z_bledem);
    assert_eq!(vec![21], wynik_leb128_z_bledem);

    // Testy VLQ na bazie https://en.wikipedia.org/wiki/Variable-length_quantity

    let dane_vlq = &[
        0b_00000000, //1
        0b_01111111, //2
        0b_10000001, //3
        0b_00000000,
        0b_11000000, //4
        0b_00000000,
        0b_11111111, //5
        0b_01111111,
        0b_10000001, //6
        0b_10000000,
        0b_00000000,
        0b_11111111, //7
        0b_11111111,
        0b_01111111,
        0b_10000001, //8
        0b_10000000,
        0b_10000000,
        0b_00000000,
        0b_11000000, //9
        0b_10000000,
        0b_10000000,
        0b_00000000,
        0b_11111111, //10
        0b_11111111,
        0b_11111111,
        0b_01111111,
    ];

    let wynik_vlq = vlq128v(dane_vlq);
    assert_eq!(
        vec![
            0x00000000, 0x0000007F, 0x00000080, 0x00002000, 0x00003FFF, 0x00004000, 0x001FFFFF,
            0x00200000, 0x08000000, 0x0FFFFFFF
        ],
        wynik_vlq
    );
}
