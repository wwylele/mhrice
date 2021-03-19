use crate::file_ext::*;
use anyhow::*;
use serde::*;
use std::collections::{BTreeMap, HashMap};
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read, Seek, SeekFrom};

use serde::Serializer;

pub fn ser_hash<S>(bytes: &[u8; 0x10], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{:?}", bytes))
}

#[derive(Debug, Serialize)]
enum FieldValue {
    Bool(bool),
    F64(f64),
    String(String),
    Texture(String),
    Unknown(u32, u64),
}

#[derive(Debug, Serialize)]
struct Nargacuga {
    b: u32,
    name: String,
    value: FieldValue,
}

#[derive(Debug, Serialize)]
struct Zinogre {
    #[serde(serialize_with = "ser_hash")]
    hash0: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    hash1: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    hash2: [u8; 0x10],
    name: String,
    type_name: String,
    ms: Vec<Nargacuga>,
    ns: Vec<Nargacuga>,
}

#[derive(Debug, Serialize)]
struct X3 {
    #[serde(serialize_with = "ser_hash")]
    hash: [u8; 0x10],
    name: String,
    type_name: String,
    ps: Vec<Zinogre>,
    q_offsets: Vec<u64>,
    q_what: u32,
}

#[derive(Debug, Serialize)]
struct PlayObjectExt {
    name: String,
    children: Vec<PlayObject>,
    q_offsets: Vec<u64>,
    q_what: u32,
}

#[derive(Debug, Serialize)]
struct PlayObject {
    hash0: [u8; 0x10],
    hash1: [u8; 0x10],
    hash2: [u8; 0x10],
    name: String,
    type_name: String,
    ms: Vec<Nargacuga>,
    ns: Vec<Nargacuga>,
    ext: Option<PlayObjectExt>,
}

#[derive(Debug, Serialize)]
pub struct Gui {
    root: PlayObject,
}

impl Gui {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Gui> {
        if file.read_u32()? != 0x061A96 {
            bail!("Wrong version for GUI");
        }

        if &file.read_magic()? != b"GUIR" {
            bail!("Wrong magic for GUI");
        }

        let a_offset = file.read_u64()?;
        let b_offset = file.read_u64()?;
        let c_offset = file.read_u64()?;
        let d_offset = file.read_u64()?;
        let e_offset = file.read_u64()?;

        file.seek_noop(a_offset)?;

        let aa_offset = file.read_u64()?;
        let ab_offset = file.read_u64()?;
        let x_count = file.read_u64()?;
        let x_offsets = (0..x_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(aa_offset)?;

        struct X {
            hash: [u8; 0x10],
            name: String,
            type_name: String,
            p_offset: u64,
            q_offset: u64,
        }

        let xs = x_offsets
            .into_iter()
            .map(|x_offset| {
                file.seek_noop(x_offset)?;
                let mut hash = [0; 0x10];
                file.read_exact(&mut hash)?;
                let name = file.read_u64()?;
                let type_name = file.read_u64()?;
                let p_offset = file.read_u64()?;
                let q_offset = file.read_u64()?;

                let old = file.tell()?;
                file.seek(SeekFrom::Start(name))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(type_name))?;
                let type_name = file.read_u8str()?;
                file.seek(SeekFrom::Start(old))?;

                Ok(X {
                    hash,
                    name,
                    type_name,
                    p_offset,
                    q_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let read_nargacuga = |file: &mut F| -> Result<Nargacuga> {
            let value_type = file.read_u32()?;
            let b = file.read_u32()?;
            let name = file.read_u64()?;
            let value = file.read_u64()?;

            let old = file.tell()?;

            let value = match value_type {
                1 => FieldValue::Bool(match value {
                    0 => false,
                    1 => true,
                    _ => bail!("Invalid bool value"),
                }),
                10 => FieldValue::F64(f64::from_bits(value)),
                13 => {
                    file.seek(SeekFrom::Start(value))?;
                    FieldValue::String(file.read_u16str()?)
                }
                32 => {
                    file.seek(SeekFrom::Start(value))?;
                    FieldValue::Texture(file.read_u16str()?)
                }
                x => FieldValue::Unknown(x, value),
            };

            file.seek(SeekFrom::Start(name))?;
            let name = file.read_u8str()?;

            file.seek(SeekFrom::Start(old))?;

            Ok(Nargacuga { b, name, value })
        };
        let read_zinogre = |file: &mut F| -> Result<Zinogre> {
            let mut hash0 = [0; 0x10];
            file.read_exact(&mut hash0)?;
            let mut hash1 = [0; 0x10];
            file.read_exact(&mut hash1)?;
            let mut hash2 = [0; 0x10];
            file.read_exact(&mut hash2)?;

            let name = file.read_u64()?;
            let type_name = file.read_u64()?;
            let m_offset = file.read_u64()?;
            let n_offset = file.read_u64()?;
            let mtx_offset = file.read_u64()?;

            file.seek_noop(m_offset)?;
            let m_count = file.read_u64()?;
            let ms = (0..m_count)
                .map(|_| read_nargacuga(file))
                .collect::<Result<Vec<_>>>()?;
            file.seek_noop(n_offset)?;
            let n_count = file.read_u64()?;
            let ns = (0..n_count)
                .map(|_| read_nargacuga(file))
                .collect::<Result<Vec<_>>>()?;

            let old = file.tell()?;
            file.seek(SeekFrom::Start(name))?;
            let name = file.read_u16str()?;
            file.seek(SeekFrom::Start(type_name))?;
            let type_name = file.read_u8str()?;
            file.seek(SeekFrom::Start(old))?;

            Ok(Zinogre {
                hash0,
                hash1,
                hash2,
                name,
                type_name,
                ms,
                ns,
            })
        };

        #[derive(Debug)]
        struct X2 {
            name: String,
            type_name: String,
            p_offsets: Vec<u64>,
            q_offsets: Vec<u64>,
            q_what: u32,
        }

        let mut p_sorted = BTreeMap::new();

        let mut xs = xs
            .into_iter()
            .map(|x| {
                file.seek_noop(x.p_offset)?;
                let p_count = file.read_u64()?;
                let p_offsets = (0..p_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;
                file.seek_noop(x.q_offset)?;
                let q_count = file.read_u32()?;
                let q_what = file.read_u32()?; //?
                let q_offsets = (0..q_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;

                for &p_offset in &p_offsets {
                    if p_sorted.insert(p_offset, None).is_some() {
                        bail!("Reusing P");
                    }
                }

                Ok((
                    x.hash,
                    X2 {
                        name: x.name,
                        type_name: x.type_name,
                        p_offsets,
                        q_offsets,
                        q_what,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        file.seek_noop(ab_offset)?;
        let root = read_zinogre(&mut file)?;

        for (&p_offset, p) in &mut p_sorted {
            file.seek_noop(p_offset)?;
            *p = Some(read_zinogre(&mut file)?);
        }

        fn build_play_object(
            node: Zinogre,
            p_sorted: &mut BTreeMap<u64, Option<Zinogre>>,
            xs: &mut HashMap<[u8; 16], X2>,
        ) -> Result<PlayObject> {
            let ext = if let Some(x2) = xs.remove(&node.hash1) {
                if x2.type_name != node.type_name {
                    bail!(
                        "name or type_name mismatch, {}, {}, {}, {}",
                        x2.name,
                        node.name,
                        x2.type_name,
                        node.type_name
                    )
                }
                let children = x2
                    .p_offsets
                    .into_iter()
                    .map(|p_offset| {
                        let p = p_sorted.remove(&p_offset).context("p not found")?.unwrap();
                        build_play_object(p, p_sorted, xs)
                    })
                    .collect::<Result<Vec<_>>>()?;
                Some(PlayObjectExt {
                    name: x2.name,
                    children,
                    q_offsets: x2.q_offsets,
                    q_what: x2.q_what,
                })
            } else {
                None
            };

            Ok(PlayObject {
                hash0: node.hash0,
                hash1: node.hash1,
                hash2: node.hash2,
                name: node.name,
                type_name: node.type_name,
                ms: node.ms,
                ns: node.ns,
                ext,
            })
        }

        /*let xs = xs
        .into_iter()
        .map(|x| {
            Ok(X3 {
                hash: x.hash,
                name: x.name,
                type_name: x.type_name,
                ps: x
                    .p_offsets
                    .into_iter()
                    .map(|p_offset| p_sorted.remove(&p_offset).unwrap().unwrap())
                    .collect(),
                q_offsets: x.q_offsets,
                q_what: x.q_what,
            })
        })
        .collect::<Result<Vec<_>>>()?;*/

        let root = build_play_object(root, &mut p_sorted, &mut xs)?;

        if !xs.is_empty() {
            bail!("xs not empty, {:?}", xs)
        }

        if !p_sorted.is_empty() {
            bail!("p_sorted not empty")
        }

        Ok(Gui { root })
    }
}
