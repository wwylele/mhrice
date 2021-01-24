use crate::file_ext::FileExt;
use anyhow::*;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct SlotString {
    pub slot: u32,
    pub hash: u32,
    pub string: String,
}

#[derive(Debug)]
pub struct Rsz {
    pub nargacugas: Vec<u32>,
    pub slot_strings: Vec<SlotString>,
    pub type_descriptors: Vec<u64>,
}

impl Rsz {
    pub fn new<F: Read + Seek>(mut file: F, base: u64) -> Result<Rsz> {
        file.seek(SeekFrom::Start(base))?;
        let magic = file.read_magic()?;
        if &magic != b"RSZ\0" {
            bail!("Wrong magic for RSZ block");
        }

        let version = file.read_u32()?;
        if version != 0x10 {
            bail!("Unexpected RSZ version {}", version);
        }

        let nargacuga_count = file.read_u32()?;
        let type_descriptor_count = file.read_u32()?;
        let string_count = file.read_u32()?;
        let padding = file.read_u32()?;
        if padding != 0 {
            bail!("Unexpected non-zero padding C: {}", padding);
        }
        let type_descriptor_offset = file.read_u64()?;
        let data_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;

        let nargacugas = (0..nargacuga_count)
            .map(|_| file.read_u32())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(base + type_descriptor_offset)
            .context("Undiscovered data before type descriptor")?;

        let type_descriptors = (0..type_descriptor_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        if type_descriptors.get(0) != Some(&0) {
            bail!("The first type descriptor should be 0")
        }

        file.seek_align_up(base + string_table_offset, 16)
            .context("Undiscovered data before string table")?;

        let string_info = (0..string_count)
            .map(|_| {
                let slot = file.read_u32()?;
                let hash = file.read_u32()?;
                let offset = file.read_u64()?;
                Ok((slot, hash, offset))
            })
            .collect::<Result<Vec<_>>>()?;

        let slot_strings = string_info
            .into_iter()
            .map(|(slot, hash, offset)| {
                file.seek_noop(base + offset)
                    .context("Undiscovered data in string table")?;
                let string = file.read_u16str()?;
                Ok(SlotString { slot, hash, string })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_align_up(base + data_offset, 16)
            .context("Undiscovered data before data")?;

        Ok(Rsz {
            nargacugas,
            slot_strings,
            type_descriptors,
        })
    }
}
