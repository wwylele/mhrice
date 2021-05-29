use crate::file_ext::*;
use crate::hash::hash_as_utf16;
use crate::suffix::SUFFIX_MAP;
use anyhow::*;
use compress::flate;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

const LANGUAGE_LIST: &[&str] = &[
    "", "Ja", "En", "Fr", "It", "De", "Es", "Ru", "Pl", "Nl", "Pt", "PtBR", "Ko", "ZhTW", "ZhCN",
    "Fi", "Sv", "Da", "No", "Cs", "Hu", "Sk", "Ar", "Tr", "Bu", "Gr", "Ro", "Th", "Uk", "Vi", "Id",
    "Fc", "Hi",
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct PakFileIndex {
    version: usize,
    index: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct I18nPakFileIndex {
    pub language: &'static str,
    pub index: PakFileIndex,
}

impl PakFileIndex {
    pub fn short_string(&self) -> String {
        format!("{:02}-{:06}", self.version, self.index)
    }
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

    fn find_file_internal(&mut self, full_path: String) -> Option<PakFileIndex> {
        let hash: u64 = u64::from(hash_as_utf16(&full_path.to_lowercase()))
            | (u64::from(hash_as_utf16(&full_path.to_uppercase())) << 32);
        self.hash_map.get(&hash).cloned()
    }

    pub fn find_file_i18n(&mut self, mut path: &str) -> Result<Vec<I18nPakFileIndex>> {
        if path.starts_with('@') {
            path = &path[1..];
        }
        let dot = path.rfind('.').context("Path missing extension")?;
        let suffix = *SUFFIX_MAP
            .get(&path[dot + 1..])
            .context("Unknown extension")?;
        let full_path = format!("natives/NSW/{}.{}", path, suffix);
        let full_path_nsw = format!("{}.NSW", &full_path);

        let mut result = vec![];

        for &language in LANGUAGE_LIST {
            let (path_l, path_nsw_l) = if language.is_empty() {
                (full_path.clone(), full_path_nsw.clone())
            } else {
                let path_l = format!("{}.{}", &full_path, language);
                let path_nsw_l = format!("{}.{}", &full_path_nsw, language);
                (path_l, path_nsw_l)
            };
            if let Some(index) = self.find_file_internal(path_l) {
                result.push(I18nPakFileIndex { language, index });
                continue;
            }

            if let Some(index) = self.find_file_internal(path_nsw_l) {
                result.push(I18nPakFileIndex { language, index });
            }
        }

        Ok(result)
    }

    pub fn find_file(&mut self, path: &str) -> Result<PakFileIndex> {
        Ok(self
            .find_file_i18n(path)?
            .first()
            .with_context(|| format!("No matching hash for {}", path))?
            .index)
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
            1 => {
                let stream = file.by_ref().take(len_compressed);
                let mut decompressed = Vec::new();
                flate::Decoder::new(stream).read_to_end(&mut decompressed)?;
                if u64::try_from(decompressed.len()).unwrap() != len {
                    bail!("Expected size {}, actual size {}", len, decompressed.len());
                }
                Ok(decompressed)
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
