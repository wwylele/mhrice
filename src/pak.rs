use anyhow::*;
use murmur3::murmur3_32;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
struct FileInfo {
    offset: u64,
    len_compressed: u64,
    len: u64,
    format: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct PakFileIndex(u32);

#[derive(Debug)]
pub struct PakReader<F> {
    file: F,
    hash_map: HashMap<u64, PakFileIndex>,
}

impl<F: Read + Seek> PakReader<F> {
    pub fn new(mut file: F) -> Result<PakReader<F>> {
        let mut magic = [0; 4];
        file.read_exact(&mut magic)?;
        if &magic != b"KPKA" {
            bail!("Wrong magic for PAK file");
        }
        let mut version = [0; 4];
        file.read_exact(&mut version)?;
        if u32::from_le_bytes(version) != 4 {
            bail!("Wrong version for PAK file");
        }
        let mut count_buf = [0; 4];
        file.read_exact(&mut count_buf)?;
        let count = u32::from_le_bytes(count_buf);
        file.seek(SeekFrom::Current(4))?;

        let mut hash_map = HashMap::new();
        for index in 0..count {
            let mut hash = [0; 8];
            file.read_exact(&mut hash)?;
            file.seek(SeekFrom::Current(0x28))?;
            hash_map.insert(u64::from_le_bytes(hash), PakFileIndex(index));
        }

        Ok(PakReader { file, hash_map })
    }

    pub fn find_file(&mut self, path: &str) -> Option<(PakFileIndex, String)> {
        fn iter(a: [u8; 2]) -> impl Iterator<Item = u8> {
            std::iter::once(a[0]).chain(std::iter::once(a[1]))
        }

        // It is still unknown how the suffix is determined, so let's just try
        // all numbers under 100
        for suffix in 0..100 {
            let full_path = format!("natives/NSW/{}.{}", path, suffix);
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
            if let Some(&index) = self.hash_map.get(&hash) {
                return Some((index, full_path));
            }
        }

        None
    }
}
