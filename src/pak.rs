use crate::file_ext::*;
use crate::suffix::SUFFIX_MAP;
use anyhow::*;
use murmur3::murmur3_32;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct PakFileIndex {
    version: usize,
    index: u32,
}

#[derive(Debug)]
pub struct PakReader<F> {
    files: Vec<F>,
    counts: Vec<u32>,
    hash_map: HashMap<u64, PakFileIndex>,
}

impl<F: Read + Seek> PakReader<F> {
    pub fn new(mut files: Vec<F>) -> Result<PakReader<F>> {
        let mut hash_map = HashMap::new();
        let mut counts = vec![];
        for (version, file) in files.iter_mut().enumerate() {
            let magic = file.read_magic()?;
            if &magic != b"KPKA" {
                bail!("Wrong magic for PAK file");
            }
            let pak_version = file.read_u32()?;
            if pak_version != 4 {
                bail!("Wrong version for PAK file");
            }
            let count = file.read_u32()?;
            counts.push(count);
            file.seek(SeekFrom::Current(4))?;

            for index in 0..count {
                let hash = file.read_u64()?;
                file.seek(SeekFrom::Current(0x28))?;
                hash_map.insert(hash, PakFileIndex { version, index });
            }
        }

        Ok(PakReader {
            files,
            counts,
            hash_map,
        })
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

    pub fn read_file(&mut self, file_index: PakFileIndex) -> Result<Vec<u8>> {
        let info_offset = 0x10 + 0x30 * u64::from(file_index.index) + 8;
        let file = &mut self.files[file_index.version];
        file.seek(SeekFrom::Start(info_offset))?;
        let offset = file.read_u64()?;
        let len_compressed = file.read_u64()?;
        let len = file.read_u64()?;
        let format = file.read_u8()?;
        let _ /*? */ = file.read_u8()?;

        file.seek(SeekFrom::Start(offset))?;
        match format {
            0 => {
                if len != len_compressed {
                    bail!("Uncompressed file should have len == len_compressed")
                }
                let mut data = vec![0; len.try_into()?];
                file.read_exact(&mut data)?;
                Ok(data)
            }
            2 => {
                let decoded = zstd::decode_all(file.by_ref().take(len_compressed))?;
                if u64::try_from(decoded.len()).unwrap() != len {
                    bail!("Expected size {}, actual size {}", len, decoded.len());
                }
                Ok(decoded)
            }
            _ => bail!("Unsupported format: {}", format),
        }
    }

    pub fn read_file_at(&mut self, version: usize, index: u32) -> Result<Vec<u8>> {
        if version > self.files.len() {
            bail!("Version out of bound")
        }
        if index >= self.counts[version] {
            bail!("Index out of bound");
        }
        self.read_file(PakFileIndex { version, index })
    }

    pub fn all_file_indexs(&self) -> Vec<PakFileIndex> {
        let mut v: Vec<_> = self.hash_map.values().cloned().collect();
        v.sort();
        v
    }
}
