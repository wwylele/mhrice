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
    index: usize,
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
struct PakEntry {
    offset: u64,
    len_compressed: u64,
    len: u64,
    format: u8,
    flag: u8,
}

#[derive(Debug)]
struct PakFile<F> {
    file: F,
    entries: Vec<PakEntry>,
}

#[derive(Debug)]
pub struct PakReader<F> {
    files: Vec<PakFile<F>>,
    hash_map: HashMap<u64, PakFileIndex>,
}

impl<F: Read + Seek> PakReader<F> {
    pub fn new(raw_files: Vec<F>) -> Result<PakReader<F>> {
        let mut hash_map = HashMap::new();
        let files = raw_files
            .into_iter()
            .enumerate()
            .map(|(version, mut file)| {
                let magic = file.read_magic()?;
                if &magic != b"KPKA" {
                    bail!("Wrong magic for PAK file");
                }
                let pak_version = file.read_u16()?;
                if pak_version != 4 {
                    bail!("Wrong version for PAK file");
                }
                let flag = file.read_u16()?;
                if flag & 1 != 0 {
                    bail!("Unimplemented flag 1")
                }
                let count = file.read_u32()?;
                file.seek(SeekFrom::Current(4))?;

                let mut entries_buffer = vec![0; count as usize * 0x30];
                file.read_exact(&mut entries_buffer)?;

                if flag & 8 != 0 {
                    let key = guess_key(&entries_buffer)?;
                    decrypt_pak_entry_table(&mut entries_buffer, &key);
                }

                let entries: Vec<PakEntry> = entries_buffer
                    .chunks(0x30)
                    .enumerate()
                    .map(|(index, mut entry)| {
                        let hash = entry.read_u64()?;
                        hash_map.insert(hash, PakFileIndex { version, index });
                        let offset = entry.read_u64()?;
                        let len_compressed = entry.read_u64()?;
                        let len = entry.read_u64()?;
                        let format = entry.read_u8()?;
                        let flag = entry.read_u8()?;
                        Ok(PakEntry {
                            offset,
                            len_compressed,
                            len,
                            format,
                            flag,
                        })
                    })
                    .collect::<Result<Vec<PakEntry>>>()?;

                Ok(PakFile { file, entries })
            })
            .collect::<Result<Vec<PakFile<F>>>>()?;

        Ok(PakReader { files, hash_map })
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
        for suffix in suffix {
            let full_path = format!("natives/STM/{}.{}", path, suffix);
            let full_path_platform = format!("{}.x64", &full_path);

            let mut result = vec![];

            for &language in LANGUAGE_LIST {
                let (path_l, path_nsw_l) = if language.is_empty() {
                    (full_path.clone(), full_path_platform.clone())
                } else {
                    let path_l = format!("{}.{}", &full_path, language);
                    let path_nsw_l = format!("{}.{}", &full_path_platform, language);
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
            if !result.is_empty() {
                return Ok(result);
            }
        }
        Ok(vec![])
    }

    pub fn find_file(&mut self, path: &str) -> Result<PakFileIndex> {
        Ok(self
            .find_file_i18n(path)?
            .first()
            .with_context(|| format!("No matching hash for {}", path))?
            .index)
    }

    pub fn read_file(&mut self, file_index: PakFileIndex) -> Result<Vec<u8>> {
        let PakFile { file, entries } = &mut self.files[file_index.version];
        let PakEntry {
            offset,
            len_compressed,
            len,
            format,
            flag,
        } = entries[file_index.index];

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

    pub fn read_file_at(&mut self, version: usize, index: usize) -> Result<Vec<u8>> {
        if version > self.files.len() {
            bail!("Version out of bound")
        }
        if index >= self.files[version].entries.len() {
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

fn guess_key(bytes: &[u8]) -> Result<[u8; 0x20]> {
    const P0: usize = 32;
    const P1: usize = 29;
    const MUL_TABLE_LEN: usize = P0 * (P0 + 1) / 2;
    fn index(x: usize, y: usize) -> usize {
        let (x, y) = (std::cmp::max(x, y), std::cmp::min(x, y));
        x * (x + 1) / 2 + y
    }

    let mut xorpads: [HashMap<u8, u32>; MUL_TABLE_LEN] =
        [(); MUL_TABLE_LEN].map(|_| HashMap::new());

    // find the common xorpad
    for (i, b) in bytes.iter().enumerate() {
        if i % 8 == 0 {
            // bytes at this position is rarely zero in plaintext, so we skip them
            continue;
        }
        let x = i % P0;
        let y = i % P1;
        let xorpad = b.wrapping_sub(i as u8);
        *xorpads[index(x, y)].entry(xorpad).or_default() += 1;
    }

    // find the most likely ones
    let xorpads = xorpads.map(|list| -> Option<(u8, u32)> {
        let (xorpad, freq) = list.into_iter().max_by_key(|(_, freq)| *freq)?;
        (freq > 100).then(|| (xorpad, freq))
    });

    // on the diagonal, find the two odd xorpads with the top confidence as pivots
    // we also need to make sure their connecting xorpad has a guess
    // Technically we only need one odd pivot to derive the rest of the key,
    // but we use two to verify we are not hit by occasional wrong xorpad guess.
    let odd_diagonal_xorpads = || {
        (0..P0).filter_map(|i| {
            let (xorpad, freq) = xorpads[index(i, i)]?;
            (xorpad % 2 != 0).then(|| (i, freq))
        })
    };

    let pivot_a = odd_diagonal_xorpads()
        .max_by_key(|&(_, freq)| freq)
        .context("Can't find a pivot")?
        .0;
    let pivot_b = odd_diagonal_xorpads()
        .filter(|&(i, _)| i != pivot_a && xorpads[index(i, pivot_a)].is_some())
        .max_by_key(|&(_, freq)| freq)
        .context("Can't find another pivot")?
        .0;

    // compute the two pivots
    let mut key = [None; 0x20];
    'outer: for a in 0..=255u8 {
        for b in 0..=255u8 {
            if a.wrapping_mul(a) == xorpads[index(pivot_a, pivot_a)].unwrap().0
                && b.wrapping_mul(b) == xorpads[index(pivot_b, pivot_b)].unwrap().0
                && a.wrapping_mul(b) == xorpads[index(pivot_a, pivot_b)].unwrap().0
            {
                key[pivot_a] = Some(a);
                key[pivot_b] = Some(b);
                break 'outer;
            }
        }
    }
    let a = key[pivot_a].unwrap();
    let b = key[pivot_b].unwrap();

    // compute the rest of the key using the pivots
    for (i, key_slot) in key.iter_mut().enumerate() {
        if key_slot.is_some() {
            continue;
        }
        for c in 0..=255u8 {
            if a.wrapping_mul(c) == xorpads[index(pivot_a as usize, i as usize)].unwrap().0
                && b.wrapping_mul(c) == xorpads[index(pivot_b as usize, i as usize)].unwrap().0
            {
                *key_slot = Some(c);
                break;
            }
        }
    }

    let key = key.map(|slot| slot.unwrap());

    // Verify the matching rate
    for x in 0..P0 {
        let mut matched = 0;
        for y in 0..P0 {
            if let Some((xorpad, _)) = xorpads[index(x, y)] {
                if xorpad == key[x].wrapping_mul(key[y]) {
                    matched += 1;
                }
            }
        }
        let rate = matched as f32 / P0 as f32;
        if rate < 0.8 {
            bail!("Failed to guess the key")
        }
    }

    Ok(key)
}

fn decrypt_pak_entry_table(bytes: &mut [u8], key: &[u8; 0x20]) {
    for (i, b) in bytes.iter_mut().enumerate() {
        *b ^= (key[i % 32] as usize * key[i % 29] as usize + i) as u8
    }
}
