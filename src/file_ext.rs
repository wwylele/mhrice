use crate::align::*;
use anyhow::*;
use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};

pub trait ReadExt {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_magic(&mut self) -> Result<[u8; 4]>;
    fn read_u16str(&mut self) -> Result<String>;
    fn read_f32(&mut self) -> Result<f32>;
}

pub trait SeekExt {
    fn seek_noop(&mut self, from_start: u64) -> Result<u64>;
    fn seek_assert_align_up(&mut self, from_start: u64, align: u64) -> Result<u64>;
    fn seek_align_up(&mut self, align: u64) -> Result<u64>;
}

impl<T: Read + ?Sized> ReadExt for T {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    fn read_magic(&mut self) -> Result<[u8; 4]> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn read_u16str(&mut self) -> Result<String> {
        let mut u16str = vec![];
        loop {
            let c = self.read_u16()?;
            if c == 0 {
                break;
            }
            if !(0x20..0x7F).contains(&c) {
                bail!("non-ASCII string")
            }
            u16str.push(c);
        }
        Ok(String::from_utf16(&u16str)?)
    }
    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }
}

impl<T: Seek + Read + ?Sized> SeekExt for T {
    fn seek_noop(&mut self, from_start: u64) -> Result<u64> {
        let pos = self.seek(SeekFrom::Current(0))?;
        if pos != from_start {
            bail!("This seek is expected to be no-op");
        }
        Ok(pos)
    }

    fn seek_assert_align_up(&mut self, from_start: u64, align: u64) -> Result<u64> {
        let pos = self.seek(SeekFrom::Current(0))?;
        if align_up(pos, align) != from_start {
            bail!("This seek is expected to only align up {}", align);
        }
        if pos != from_start {
            let mut buf = vec![0; (from_start - pos).try_into()?];
            self.read_exact(&mut buf)?;
            if buf.into_iter().any(|x| x != 0) {
                bail!("Non zero padding");
            }
        }

        Ok(from_start)
    }

    fn seek_align_up(&mut self, align: u64) -> Result<u64> {
        let pos = self.seek(SeekFrom::Current(0))?;
        let aligned = align_up(pos, align);
        if aligned != pos {
            let mut buf = vec![0; (aligned - pos).try_into()?];
            self.read_exact(&mut buf)?;
            if buf.into_iter().any(|x| x != 0) {
                bail!("Non zero padding");
            }
        }

        Ok(aligned)
    }
}
