use eyre::Result;
use std::{
    fs, io,
    io::{Read, Seek, Write},
    mem,
};

pub struct File {
    f: fs::File,
}

impl File {
    pub fn open(name: &str) -> Result<Self> {
        let f = fs::File::open(name)?;
        Ok(File { f })
    }

    pub fn create(name: &str) -> Result<Self> {
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
