use color_eyre::eyre::Result;
use sekurak_hex_gynvael::files::File;
use std::io::SeekFrom;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut ofile = File::create("data/flaga.png")?;
    let mut buf10 = [0u8; 10];
    let mut buf5 = [0u8; 5];

    for n in 0 ..= 317 {
        let in_file_name = format!("data/splitflag/{n:03}.bin");
        let mut ifile = File::open(&in_file_name)?;

        ifile.seek(SeekFrom::Start(0x123))?;
        ifile.read_into(buf10.as_mut())?;
        ofile.write_exact(&buf10)?;

        ifile.seek(SeekFrom::Start(0xabc))?;
        ifile.read_into(buf10.as_mut())?;
        ofile.write_exact(&buf10)?;

        ifile.seek(SeekFrom::End(-10))?;
        ifile.read_into(buf5.as_mut())?;
        ofile.write_exact(&buf5)?;
    }

    Ok(())
}
