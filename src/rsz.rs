use crate::file_ext::*;
use anyhow::*;
use once_cell::sync::Lazy;
use std::any::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct SlotString {
    pub slot: u32,
    pub hash: u32,
    pub string: String,
}

#[derive(Debug)]
pub struct Rsz {
    pub roots: Vec<u32>,
    pub slot_strings: Vec<SlotString>,
    pub type_descriptors: Vec<u64>,
    pub data: Vec<u8>,
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

        let root_count = file.read_u32()?;
        let type_descriptor_count = file.read_u32()?;
        let string_count = file.read_u32()?;
        let padding = file.read_u32()?;
        if padding != 0 {
            bail!("Unexpected non-zero padding C: {}", padding);
        }
        let type_descriptor_offset = file.read_u64()?;
        let data_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;

        let nargacugas = (0..root_count)
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

        file.seek_assert_align_up(base + string_table_offset, 16)
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
                if !string.ends_with(".user") {
                    bail!("Non-USER slot string");
                }
                if u64::from(hash)
                    != 0xFFFFFFFF
                        & *type_descriptors
                            .get(usize::try_from(slot)?)
                            .context("slot out of bound")?
                {
                    bail!("slot hash mismatch")
                }
                Ok(SlotString { slot, hash, string })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(base + data_offset, 16)
            .context("Undiscovered data before data")?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        Ok(Rsz {
            roots: nargacugas,
            slot_strings,
            type_descriptors,
            data,
        })
    }

    pub fn deserialize(&self) -> Result<Vec<Box<dyn Any>>> {
        let mut node_buf: Vec<Option<Box<dyn Any>>> = vec![None];
        let mut cursor = Cursor::new(&self.data);
        for td in self.type_descriptors.iter().skip(1) {
            let deserializer = RSZ_TYPE_MAP.get(td).context("Unsupported type")?;
            let mut rsz_deserializer = RszDeserializer {
                node_buf: &mut node_buf,
                cursor: &mut cursor,
            };
            let node = deserializer(&mut rsz_deserializer)?;
            node_buf.push(Some(node));
        }

        let result = self
            .roots
            .iter()
            .map(|&root| {
                Ok(node_buf
                    .get_mut(usize::try_from(root)?)
                    .context("Root index out of bound")?
                    .take()
                    .context("Empty root")?)
            })
            .collect::<Result<Vec<_>>>()?;

        if node_buf.into_iter().any(|node| node.is_some()) {
            bail!("Left over node");
        }

        let mut leftover = vec![];
        cursor.read_to_end(&mut leftover)?;
        if !leftover.is_empty() {
            bail!("Left over data");
        }

        Ok(result)
    }

    pub fn deserialize_single<T: 'static>(&self) -> Result<T> {
        let mut result = self.deserialize()?;
        if result.len() != 1 {
            bail!("Not a single-valued RSZ");
        }
        Ok(*result
            .pop()
            .unwrap()
            .downcast()
            .map_err(|_| anyhow!("Type mismatch"))?)
    }
}

pub struct RszDeserializer<'a, 'b> {
    node_buf: &'a mut [Option<Box<dyn Any>>],
    cursor: &'a mut Cursor<&'b Vec<u8>>,
}

impl<'a, 'b> RszDeserializer<'a, 'b> {
    pub fn get_child<T: 'static>(&mut self) -> Result<T> {
        let index = self.cursor.read_u32()?;
        let node = self
            .node_buf
            .get_mut(usize::try_from(index)?)
            .context("Child index out of bound")?
            .take()
            .context("None child")?
            .downcast()
            .map_err(|_| anyhow!("Type mismatch"))?;
        Ok(*node)
    }

    pub fn align_up(&mut self, align: u64) -> Result<()> {
        self.cursor.seek_align_up(align)?;
        Ok(())
    }
}

impl<'a, 'b> Read for RszDeserializer<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

pub trait FromRsz: Sized {
    fn from_rsz(rsz: &mut RszDeserializer) -> Result<Self>;
    const HASH: u64;
}

type RszDeserializerFn = fn(&mut RszDeserializer) -> Result<Box<dyn Any>>;

static RSZ_TYPE_MAP: Lazy<HashMap<u64, RszDeserializerFn>> = Lazy::new(|| {
    let mut m = HashMap::new();

    fn register<T: 'static + FromRsz>(m: &mut HashMap<u64, RszDeserializerFn>) {
        let old = m.insert(T::HASH, |rsz| {
            Ok(Box::new(T::from_rsz(rsz)?) as Box<dyn Any>)
        });
        if old.is_some() {
            panic!("Multiple type reigstered for the same hash")
        }
    }

    use crate::extract::*;

    register::<HitzoneValue>(&mut m);
    register::<HitzoneValuePack>(&mut m);
    register::<MonsterMeatData>(&mut m);

    m
});
