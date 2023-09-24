fn main() {
    println!("Maski");
    maska(0xab, 0x81);
    maska(0x23, 0x0f);
    maska(0x0f, 0x23);
    maska(0xcd, 0xff);
    maska(0x78, 0x00);
}

fn maska(maska: u8, pole: u8) {
    let m = maska & pole;
    println!("0x{maska:02x} & 0x{pole:02x} = 0x{m:02x}\n  {maska:08b}\n& {pole:08b}\n= {m:08b}\n");
}
