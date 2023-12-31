use eyre::Result;
use std::path::Path;
use std::{
    fs, io,
    io::{Read, Seek, Write},
};

pub struct File {
    f: fs::File,
}

impl File {
    pub fn open<P>(name: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let f = fs::File::open(name)?;
        Ok(File { f })
    }

    pub fn create<P>(name: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let f = fs::File::create(name)?;
        Ok(File { f })
    }

    pub fn seek(&mut self, d: io::SeekFrom) -> Result<u64> {
        let pos = self.f.seek(d)?;
        Ok(pos)
    }

    pub fn tell(&mut self) -> Result<u64> {
        let pos = self.f.stream_position()?;
        Ok(pos)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.f.read_exact(buf.as_mut())?;
        Ok(u8::from_be_bytes(buf))
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0u8; 1];
        self.f.read_exact(buf.as_mut())?;
        Ok(i8::from_be_bytes(buf))
    }

    pub fn read_u16be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.f.read_exact(buf.as_mut())?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_u16le(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.f.read_exact(buf.as_mut())?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_i16be(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.f.read_exact(buf.as_mut())?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_i16le(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.f.read_exact(buf.as_mut())?;
        Ok(i16::from_le_bytes(buf))
    }

    pub fn read_u32be(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.f.read_exact(buf.as_mut())?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_u32le(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.f.read_exact(buf.as_mut())?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_i32be(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.f.read_exact(buf.as_mut())?;
        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_i32le(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.f.read_exact(buf.as_mut())?;
        Ok(i32::from_le_bytes(buf))
    }

    pub fn read_u64be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.f.read_exact(buf.as_mut())?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_u64le(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.f.read_exact(buf.as_mut())?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_i64be(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.f.read_exact(buf.as_mut())?;
        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_i64le(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.f.read_exact(buf.as_mut())?;
        Ok(i64::from_le_bytes(buf))
    }

    pub fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(1500);
        self.f.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.f.read_exact(buf)?;
        Ok(())
    }

    pub fn read_as_arr<const SIZE: usize>(&mut self) -> Result<[u8; SIZE]> {
        let mut buf = [0u8; SIZE];
        self.f.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_as_vec(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.f.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn write_exact(&mut self, data: &[u8]) -> Result<()> {
        self.f.write_all(data)?;
        Ok(())
    }
}
pub mod LE {
    use super::File;
    use eyre::Result;
    pub trait BinReader<T> {
        fn binread(&mut self) -> Result<T>;
    }

    impl BinReader<u8> for File {
        fn binread(&mut self) -> Result<u8> {
            self.read_u8()
        }
    }

    impl BinReader<u16> for File {
        fn binread(&mut self) -> Result<u16> {
            self.read_u16le()
        }
    }

    impl BinReader<u32> for File {
        fn binread(&mut self) -> Result<u32> {
            self.read_u32le()
        }
    }

    impl BinReader<u64> for File {
        fn binread(&mut self) -> Result<u64> {
            self.read_u64le()
        }
    }

    impl BinReader<i8> for File {
        fn binread(&mut self) -> Result<i8> {
            self.read_i8()
        }
    }

    impl BinReader<i16> for File {
        fn binread(&mut self) -> Result<i16> {
            self.read_i16le()
        }
    }

    impl BinReader<i32> for File {
        fn binread(&mut self) -> Result<i32> {
            self.read_i32le()
        }
    }

    impl BinReader<i64> for File {
        fn binread(&mut self) -> Result<i64> {
            self.read_i64le()
        }
    }
}

pub mod BE {
    use super::File;
    use eyre::Result;
    pub trait BinReader<T> {
        fn binread(&mut self) -> Result<T>;
    }

    impl BinReader<u8> for File {
        fn binread(&mut self) -> Result<u8> {
            self.read_u8()
        }
    }

    impl BinReader<u16> for File {
        fn binread(&mut self) -> Result<u16> {
            self.read_u16be()
        }
    }

    impl BinReader<u32> for File {
        fn binread(&mut self) -> Result<u32> {
            self.read_u32be()
        }
    }

    impl BinReader<u64> for File {
        fn binread(&mut self) -> Result<u64> {
            self.read_u64be()
        }
    }

    impl BinReader<i8> for File {
        fn binread(&mut self) -> Result<i8> {
            self.read_i8()
        }
    }

    impl BinReader<i16> for File {
        fn binread(&mut self) -> Result<i16> {
            self.read_i16be()
        }
    }

    impl BinReader<i32> for File {
        fn binread(&mut self) -> Result<i32> {
            self.read_i32be()
        }
    }

    impl BinReader<i64> for File {
        fn binread(&mut self) -> Result<i64> {
            self.read_i64be()
        }
    }
}
