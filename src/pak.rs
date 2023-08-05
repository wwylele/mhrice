use crate::align::align_up;
use crate::file_ext::*;
use crate::hash::hash_as_utf16;
use crate::suffix::SUFFIX_MAP;
use anyhow::{bail, Context, Result};
use base64::prelude::*;
use compress::flate;
use num_bigint::BigUint;
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

static PAK_MAIN_KEY_MOD: Lazy<Option<Vec<u8>>> = Lazy::new(|| None);

static PAK_SUB_KEY_MOD: Lazy<Vec<u8>> = Lazy::new(|| {
    BASE64_STANDARD
        .decode("E9eciYiRSBDXqniu+FnffTxDoNC7Nne18FwCr2XYdwM=")
        .unwrap()
});

static PAK_SUB_KEY_EXP: Lazy<Vec<u8>> = Lazy::new(|| {
    BASE64_STANDARD
        .decode("wMJ3H1s0agHH1NeFLkIrOxY6FxMW6oMwMN8/9CWTIAE=")
        .unwrap()
});

const LANGUAGE_LIST: &[&str] = &[
    "", "Ja", "En", "Fr", "It", "De", "Es", "Ru", "Pl", "Nl", "Pt", "PtBR", "Ko", "ZhTW", "ZhCN",
    "Fi", "Sv", "Da", "No", "Cs", "Hu", "Sk", "Ar", "Tr", "Bu", "Gr", "Ro", "Th", "Uk", "Vi", "Id",
    "Fc", "Hi", "Es419",
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
    #[allow(dead_code)]
    flag: u8,
    encryption: u8,
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
                    let key = if let Some(m) = &*PAK_MAIN_KEY_MOD {
                        let mut encrypted_key = [0; 128];
                        file.read_exact(&mut encrypted_key)?;

                        let base = BigUint::from_bytes_le(&encrypted_key);
                        let modulus = BigUint::from_bytes_le(m);
                        let exponent = BigUint::from(0x10001u32);
                        let power = base.modpow(&exponent, &modulus);
                        let key_vec = power.to_bytes_le();
                        if key_vec.len() > 32 {
                            bail!("Key too long")
                        }
                        let mut key = [0; 32];
                        key[0..key_vec.len()].copy_from_slice(&key_vec);
                        key
                    } else {
                        eprintln!("PAK_MAIN_KEY_MOD not provided. Going to guess the key...");
                        guess_key(&entries_buffer)?
                    };
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
                        let encryption = entry.read_u8()?;
                        Ok(PakEntry {
                            offset,
                            len_compressed,
                            len,
                            format,
                            flag,
                            encryption,
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
        let suffix = SUFFIX_MAP
            .get(&path[dot + 1..])
            .context("Unknown extension")?;
        for suffix in suffix.iter().rev() {
            let full_paths = [
                format!("natives/NSW/{path}.{suffix}"),
                format!("natives/NSW/{path}.{suffix}.NSW"),
                format!("natives/STM/{path}.{suffix}"),
                format!("natives/STM/{path}.{suffix}.x64"),
                format!("natives/STM/{path}.{suffix}.STM"),
                format!("natives/MSG/{path}.{suffix}"),
                format!("natives/MSG/{path}.{suffix}.x64"),
                format!("natives/MSG/{path}.{suffix}.MSG"),
            ];

            let mut result = vec![];

            for &language in LANGUAGE_LIST {
                for full_path in &full_paths {
                    let dot = if language.is_empty() { "" } else { "." };
                    let with_language = format!("{full_path}{dot}{language}");
                    if let Some(index) = self.find_file_internal(with_language) {
                        result.push(I18nPakFileIndex { language, index });
                        break;
                    }
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
            .with_context(|| format!("No matching hash for {path}"))?
            .index)
    }

    pub fn read_file(&mut self, file_index: PakFileIndex) -> Result<Vec<u8>> {
        let PakFile { file, entries } = &mut self.files[file_index.version];
        let PakEntry {
            offset,
            len_compressed,
            len,
            format,
            encryption,
            ..
        } = entries[file_index.index];

        file.seek(SeekFrom::Start(offset))?;
        let mut data = vec![0; len_compressed.try_into()?];
        file.read_exact(&mut data)?;

        match encryption {
            0 => {}
            1 => {
                let encrypted = data;
                let mut encrypted = &encrypted[..];
                let plain_len: usize = encrypted.read_u64()?.try_into()?;
                if align_up(plain_len, 8) * 0x10 != encrypted.len() {
                    bail!("Unexpected size for decryption")
                }
                data = vec![0; plain_len];
                let e = &*PAK_SUB_KEY_EXP;
                let m = &*PAK_SUB_KEY_MOD;
                let e = BigUint::from_bytes_le(e);
                let m = BigUint::from_bytes_le(m);
                for (plain, enc) in data.chunks_mut(8).zip(encrypted.chunks(0x80)) {
                    let p = BigUint::from_bytes_le(&enc[0..0x40]);
                    let q = BigUint::from_bytes_le(&enc[0x40..0x80]);
                    let r = (q / (p.modpow(&e, &m))).to_bytes_le();
                    if r.len() > plain.len() {
                        bail!("Unexpected plain text")
                    }
                    plain[0..r.len()].copy_from_slice(&r);
                }
            }
            _ => bail!("Unsupported encryption {}", encryption),
        }

        match format {
            0 => {
                if len != u64::try_from(data.len())? {
                    bail!("Uncompressed file should have len == len_compressed")
                }
                Ok(data)
            }
            1 | 0x11 => {
                let mut decompressed = Vec::new();
                flate::Decoder::new(&data[..]).read_to_end(&mut decompressed)?;
                if u64::try_from(decompressed.len()).unwrap() != len {
                    bail!("Expected size {}, actual size {}", len, decompressed.len());
                }
                Ok(decompressed)
            }
            2 => {
                let decoded = zstd::decode_all(&data[..])?;
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

    pub fn sha256(&mut self) -> Result<Vec<String>> {
        self.files
            .iter_mut()
            .map(|file| {
                let mut hasher = Sha256::new();
                file.file.rewind()?;
                std::io::copy(&mut file.file, &mut hasher)?;
                let hash = hasher.finalize();
                Ok(format!("{hash:x}"))
            })
            .collect()
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
        (freq > 100).then_some((xorpad, freq))
    });

    // on the diagonal, find the two odd xorpads with the top confidence as pivots
    // we also need to make sure their connecting xorpad has a guess
    // Technically we only need one odd pivot to derive the rest of the key,
    // but we use two to verify we are not hit by occasional wrong xorpad guess.
    let odd_diagonal_xorpads = || {
        (0..P0).filter_map(|i| {
            let (xorpad, freq) = xorpads[index(i, i)]?;
            (xorpad % 2 != 0).then_some((i, freq))
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
            if a.wrapping_mul(c) == xorpads[index(pivot_a, i)].unwrap().0
                && b.wrapping_mul(c) == xorpads[index(pivot_b, i)].unwrap().0
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
