use crate::file_ext::*;
use crate::suffix::SUFFIX_MAP;
use anyhow::*;
use murmur3::murmur3_32;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Copy, Clone)]
pub struct PakFileIndex(u32);

impl PakFileIndex {
    pub fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct PakReader<F> {
    file: F,
    hash_map: HashMap<u64, PakFileIndex>,
}

impl<F: Read + Seek> PakReader<F> {
    pub fn new(mut file: F) -> Result<PakReader<F>> {
        let magic = file.read_magic()?;
        if &magic != b"KPKA" {
            bail!("Wrong magic for PAK file");
        }
        let version = file.read_u32()?;
        if version != 4 {
            bail!("Wrong version for PAK file");
        }
        let count = file.read_u32()?;
        file.seek(SeekFrom::Current(4))?;

        let mut hash_map = HashMap::new();
        for index in 0..count {
            let hash = file.read_u64()?;
            file.seek(SeekFrom::Current(0x28))?;
            if hash_map.insert(hash, PakFileIndex(index)).is_some() {
                bail!("Duplicate hash");
            }
        }

        Ok(PakReader { file, hash_map })
    }

    fn find_file_internal(&mut self, full_path: String) -> Option<(PakFileIndex, String)> {
        fn iter(a: [u8; 2]) -> impl Iterator<Item = u8> {
            std::iter::once(a[0]).chain(std::iter::once(a[1]))
        }
        let upper: Vec<u8> = full_path
            .to_uppercase()
            .encode_utf16()
            .flat_map(|u| iter(u16::to_le_bytes(u)))
            .collect();
        let lower: Vec<u8> = full_path
            .to_lowercase()
            .encode_utf16()
            .flat_map(|u| iter(u16::to_le_bytes(u)))
            .collect();
        let seed = 0xFFFF_FFFF;
        let hash: u64 = u64::from(murmur3_32(&mut &lower[..], seed).unwrap())
            | (u64::from(murmur3_32(&mut &upper[..], seed).unwrap()) << 32);
        let index = *self.hash_map.get(&hash)?;
        Some((index, full_path))
    }

    pub fn find_file(&mut self, mut path: &str) -> Result<(PakFileIndex, String)> {
        if path.starts_with('@') {
            path = &path[1..];
        }
        let dot = path.rfind('.').context("Path missing extension")?;
        let suffix = *SUFFIX_MAP
            .get(&path[dot + 1..])
            .context("Unknown extension")?;
        let full_path = format!("natives/NSW/{}.{}", path, suffix);
        let full_path_nsw = format!("{}.NSW", &full_path);
        let full_path_nsw_l = format!("{}.NSW.Ja", &full_path);
        let full_path_l = format!("{}.Ja", &full_path);

        if let Some(result) = self.find_file_internal(full_path) {
            return Ok(result);
        }

        if let Some(result) = self.find_file_internal(full_path_nsw) {
            return Ok(result);
        }

        if let Some(result) = self.find_file_internal(full_path_nsw_l) {
            return Ok(result);
        }

        if let Some(result) = self.find_file_internal(full_path_l) {
            return Ok(result);
        }

        Err(anyhow!("No matching hash"))
    }

    pub fn read_file(&mut self, PakFileIndex(index): PakFileIndex) -> Result<Vec<u8>> {
        let info_offset = 0x10 + 0x30 * u64::from(index) + 8;
        self.file.seek(SeekFrom::Start(info_offset))?;
        let offset = self.file.read_u64()?;
        let len_compressed = self.file.read_u64()?;
        let len = self.file.read_u64()?;
        let format = self.file.read_u8()?;
        let _ /*? */ = self.file.read_u8()?;

        self.file.seek(SeekFrom::Start(offset))?;
        match format {
            0 => {
                if len != len_compressed {
                    bail!("Uncompressed file should have len == len_compressed")
                }
                let mut data = vec![0; len.try_into()?];
                self.file.read_exact(&mut data)?;
                Ok(data)
            }
            2 => {
                let decoded = zstd::decode_all(self.file.by_ref().take(len_compressed))?;
                if u64::try_from(decoded.len()).unwrap() != len {
                    bail!("Expected size {}, actual size {}", len, decoded.len());
                }
                Ok(decoded)
            }
            _ => bail!("Unsupported format: {}", format),
        }
    }

    pub fn get_file_count(&self) -> u32 {
        self.hash_map.len().try_into().unwrap()
    }

    pub fn read_file_at(&mut self, index: u32) -> Result<Vec<u8>> {
        if index >= self.get_file_count() {
            bail!("Index out of bound");
        }
        self.read_file(PakFileIndex(index))
    }
}
