use crate::file_ext::*;
use crate::hash::hash_as_utf16;
use crate::rsz::Guid;
use anyhow::{bail, Result};
use serde::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Read, Seek};

#[derive(Debug, Serialize)]
pub struct MsgAttributeHeader {
    pub j: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct MsgEntry {
    pub name: String,
    pub guid: Guid,
    pub hash: u32,
    pub attributes: Vec<String>,
    pub content: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Msg {
    pub attribute_headers: Vec<MsgAttributeHeader>,
    pub entries: Vec<MsgEntry>,
}

impl Msg {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Msg> {
        let version = file.read_u32()?;
        if version != 17 && version != 539100710 {
            bail!("Wrong version {version} for MSG")
        }
        if &file.read_magic()? != b"GMSG" {
            bail!("Wrong magic for MSG")
        }
        if file.read_u64()? != 0x10 {
            bail!("Expected 0x10")
        }
        let count_a = file.read_u32()?;
        let attribute_count = file.read_u32()?;
        let language_count = file.read_u32()?;
        file.seek_align_up(8)?;

        let data_offset = file.read_u64()?;
        let p_offset = file.read_u64()?;
        let q_offset = file.read_u64()?;
        let attribute_js_offset = file.read_u64()?;
        let attribute_names_offset = file.read_u64()?;

        let entries = (0..count_a)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(p_offset)?;
        let p = file.read_u64()?;
        if p != 0 {
            bail!("Expected 0")
        }

        file.seek_noop(q_offset)?;
        let languages = (0..language_count)
            .map(|_| file.read_u32())
            .collect::<Result<Vec<_>>>()?;

        for (i, language) in languages.into_iter().enumerate() {
            if i != usize::try_from(language)? {
                bail!("Unexpected language index")
            }
        }

        file.seek_noop(attribute_js_offset)?;
        let attribute_js = (0..attribute_count)
            .map(|_| file.read_i32())
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(attribute_names_offset, 8)?;
        let attribute_names = (0..attribute_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        let entries = entries
            .into_iter()
            .map(|entry| {
                file.seek_noop(entry)?;
                let mut guid = [0; 16];
                file.read_exact(&mut guid)?;
                let _ = file.read_u32()?; //???
                let hash = file.read_u32()?;

                let name = file.read_u64()?;
                let attributes = file.read_u64()?;
                let content = (0..language_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;

                Ok((name, Guid { bytes: guid }, hash, attributes, content))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|(name, guid, hash, attributes, content)| {
                file.seek_noop(attributes)?;
                let attributes = (0..attribute_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;
                Ok((name, guid, hash, attributes, content))
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(data_offset)?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let key = [
            0xCF, 0xCE, 0xFB, 0xF8, 0xEC, 0x0A, 0x33, 0x66, 0x93, 0xA9, 0x1D, 0x93, 0x50, 0x39,
            0x5F, 0x09,
        ];

        let mut prev = 0;
        for (i, byte) in data.iter_mut().enumerate() {
            let cur = *byte;
            *byte ^= prev ^ key[i & 0xF];
            prev = cur;
        }

        let entries = entries
            .into_iter()
            .map(|(name, guid, hash, attributes, content)| {
                let name = (&data[usize::try_from(name - data_offset)?..]).read_u16str()?;
                if hash_as_utf16(&name) != hash {
                    bail!("Wrong hash")
                }
                let attributes = attributes
                    .into_iter()
                    .map(|n| Ok(format!("? {n}"))) // TODO: new version has some non-string stuff here
                    .collect::<Result<Vec<_>>>()?;
                let content = content
                    .into_iter()
                    .map(|o| (&data[usize::try_from(o - data_offset)?..]).read_u16str())
                    .collect::<Result<Vec<_>>>()?;
                Ok(MsgEntry {
                    name,
                    guid,
                    hash,
                    attributes,
                    content,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let attribute_headers = attribute_js
            .into_iter()
            .zip(attribute_names)
            .map(|(j, name)| {
                let name = (&data[usize::try_from(name - data_offset)?..]).read_u16str()?;
                Ok(MsgAttributeHeader { j, name })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Msg {
            attribute_headers,
            entries,
        })
    }

    pub fn get_entry(&self, name: &str) -> Option<&MsgEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }

    pub fn get_name_map(&self) -> HashMap<&String, &MsgEntry> {
        self.entries
            .iter()
            .map(|entry| (&entry.name, entry))
            .collect()
    }

    pub fn get_guid_map(&self) -> HashMap<Guid, &MsgEntry> {
        self.entries
            .iter()
            .map(|entry| (entry.guid, entry))
            .collect()
    }
}
