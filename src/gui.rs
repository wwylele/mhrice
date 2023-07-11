#![allow(unused_variables)]

use crate::file_ext::*;
use anyhow::{bail, Context, Result};
use serde::*;
use std::io::{Read, Seek, SeekFrom};

use serde::Serializer;

pub fn ser_hash<S>(bytes: &[u8; 0x10], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{bytes:?}"))
}

#[derive(Debug, Serialize)]
pub enum FieldValue {
    Bool(bool),
    F64(f64),
    String(String),
    Size(f32, f32),
    Texture(String),
    Unknown(u32, u64),
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub b: u32,
    pub name: String,
    pub value: FieldValue,
}

#[derive(Debug, Serialize)]
pub struct PlayObject {
    #[serde(serialize_with = "ser_hash")]
    pub hash: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    pub child_control_hash: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    pub hash2: [u8; 0x10],
    pub name: String,
    pub type_name: String,
    pub properties: Vec<Field>,
    pub variables: Vec<Field>,
}

#[derive(Debug, Serialize)]
pub struct ObjectPathComponent {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct VariableRef {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct VariableValue {
    pub value: u64,
}

#[derive(Debug, Serialize)]
pub struct Clip {
    pub name: String,
    pub object_path: Vec<ObjectPathComponent>,
    pub variable_refs: Vec<VariableRef>,
    pub variable_values: Vec<VariableValue>,
}

#[derive(Debug, Serialize)]
pub struct Control {
    #[serde(serialize_with = "ser_hash")]
    pub hash: [u8; 0x10],
    pub name: String,
    pub type_name: String,
    pub play_objects: Vec<PlayObject>,
    pub clips: Vec<Clip>,
    pub q_what: u32,
}

#[derive(Debug, Serialize)]
pub struct Gui {
    pub root: PlayObject,
    pub controls: Vec<Control>,
}

impl Gui {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Gui> {
        const VERSION_A: u32 = 0x061A96;
        const VERSION_B: u32 = 0x068FD0;
        const VERSION_C: u32 = 0x068FD1;

        const CLIP_VERSION_A: u32 = 0x28;
        const CLIP_VERSION_B: u32 = 0x2B;

        let version = file.read_u32()?;
        if !matches!(version, VERSION_A | VERSION_B | VERSION_C) {
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
        // TODO: new version has a new offset here, but it is unclear whether it is the last one
        let f_offset = (version >= VERSION_B)
            .then(|| file.read_u64())
            .transpose()?;

        file.seek_noop(a_offset).context("a_offset")?;

        let control_start_offset = file.read_u64()?;
        let root_offset = file.read_u64()?;
        let control_count = file.read_u64()?;
        let control_offset_list = (0..control_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(control_start_offset)
            .context("control_start_offset")?;

        struct ControlInfo {
            hash: [u8; 0x10],
            name: String,
            type_name: String,
            play_object_list_offset: u64,
            clip_list_offset: u64,
        }

        let controls = control_offset_list
            .into_iter()
            .map(|x_offset| {
                file.seek_noop(x_offset).context("x_offset")?;
                let mut hash = [0; 0x10];
                file.read_exact(&mut hash)?;
                let name = file.read_u64()?;
                let type_name = file.read_u64()?;
                let play_object_list_offset = file.read_u64()?;
                let clip_list_offset = file.read_u64()?;

                let old = file.tell()?;
                file.seek(SeekFrom::Start(name))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(type_name))?;
                let type_name = file.read_u8str()?;
                file.seek(SeekFrom::Start(old))?;

                Ok(ControlInfo {
                    hash,
                    name,
                    type_name,
                    play_object_list_offset,
                    clip_list_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let read_field = |file: &mut F| -> Result<Field> {
            let value_type = file.read_u32()?;
            let b = file.read_u32()?;
            let name = file.read_u64()?;
            let value = file.read_u64()?;
            let zinogre = (version >= VERSION_B)
                .then(|| file.read_u64())
                .transpose()?;

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
                31 => {
                    file.seek(SeekFrom::Start(value))?;
                    let a = file.read_f32()?;
                    let b = file.read_f32()?;
                    FieldValue::Size(a, b)
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

            Ok(Field { b, name, value })
        };
        let read_play_object = |file: &mut F| -> Result<PlayObject> {
            let mut hash = [0; 0x10];
            file.read_exact(&mut hash)?;
            let mut child_control_hash = [0; 0x10];
            file.read_exact(&mut child_control_hash)?;
            let mut hash2 = [0; 0x10];
            file.read_exact(&mut hash2)?;

            let name = file.read_u64()?;
            let type_name = file.read_u64()?;
            let property_offset = file.read_u64()?;
            let sc_offset = (version >= VERSION_C)
                .then(|| file.read_u64())
                .transpose()?;
            let variable_offset = file.read_u64()?;
            let mtx_offset = file.read_u64()?;
            let sa_offset = (version >= VERSION_B)
                .then(|| file.read_u64())
                .transpose()?;
            let sb_offset = (version >= VERSION_B)
                .then(|| file.read_u64())
                .transpose()?;

            file.seek_noop(property_offset).context("property_offset")?;
            let property_count = file.read_u64()?;
            let properties = (0..property_count)
                .map(|_| read_field(file))
                .collect::<Result<Vec<_>>>()?;

            if let Some(sc_offset) = sc_offset {
                file.seek_noop(sc_offset).context("sc_offset")?;
                let property_whats = (0..property_count)
                    .map(|_| file.read_u16())
                    .collect::<Result<Vec<_>>>()?;
            }

            file.seek_assert_align_up(variable_offset, 8)
                .context("variable_offset")?;
            let variable_count = file.read_u64()?;
            let variables = (0..variable_count)
                .map(|_| read_field(file))
                .collect::<Result<Vec<_>>>()?;

            let old = file.tell()?;
            file.seek(SeekFrom::Start(name))?;
            let name = file.read_u16str()?;
            file.seek(SeekFrom::Start(type_name))?;
            let type_name = file.read_u8str()?;
            file.seek(SeekFrom::Start(old))?;

            Ok(PlayObject {
                hash,
                child_control_hash,
                hash2,
                name,
                type_name,
                properties,
                variables,
            })
        };

        let controls = controls
            .into_iter()
            .map(|control| {
                file.seek_noop(control.play_object_list_offset)
                    .context("play_object_list_offset")?;
                let play_object_count = file.read_u64()?;
                let play_object_offsets = (0..play_object_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;
                file.seek_noop(control.clip_list_offset)
                    .context("clip_list_offset")?;
                let clip_count = file.read_u32()?;
                let q_what = file.read_u32()?; //?
                let clip_offsets = (0..clip_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;

                let old = file.tell()?;

                let play_objects = play_object_offsets
                    .into_iter()
                    .map(|offset| {
                        file.seek(SeekFrom::Start(offset))?;
                        read_play_object(&mut file)
                    })
                    .collect::<Result<Vec<_>>>()?;

                let clips = clip_offsets
                    .into_iter()
                    .map(|clip_offset| {
                        file.seek(SeekFrom::Start(clip_offset))?;
                        let mut hash = [0; 0x10];
                        file.read_exact(&mut hash)?;

                        let x = file.read_u64()?;
                        let name = file.read_u64()?;
                        let y = file.read_u64()?; // ref to other CLIP if not zero?!

                        let base_offset = clip_offset + 0x28;
                        if file.read_magic()? != *b"CLIP" {
                            bail!("Wrong magic for CLIP")
                        }

                        let version = file.read_u32()?;
                        if !matches!(version, CLIP_VERSION_A | CLIP_VERSION_B) {
                            bail!("Wrong version for CLIP")
                        }

                        let k = file.read_f32()?;
                        let u_count = file.read_u32()?;
                        let v_count = file.read_u32()?;
                        let w_count = file.read_u32()?;

                        let r0_offset = file.read_u64()?;
                        let r1_offset = file.read_u64()?;
                        let r2_offset = file.read_u64()?;
                        let r3_offset = file.read_u64()?;
                        let r4_offset = file.read_u64()?;
                        let r5_offset = file.read_u64()?;
                        let r6_offset = file.read_u64()?;
                        let str8_offset = file.read_u64()?;
                        let str16_offset = file.read_u64()?;
                        let r9_offset = file.read_u64()?;

                        let z = file.read_u64()?;
                        if z != 0 {
                            bail!("Expected 0")
                        }

                        file.seek_noop(base_offset + r0_offset)
                            .context("r0_offset")?;

                        let object_path = (0..u_count)
                            .map(|_| {
                                let alatreon = file.read_u64()?;
                                let hash = file.read_u64()?;
                                let name_offset = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;

                                let old = file.tell()?;
                                file.seek(SeekFrom::Start(
                                    name_offset * 2 + str16_offset + base_offset,
                                ))?;
                                let name = file.read_u16str()?;
                                file.seek(SeekFrom::Start(old))?;

                                Ok(ObjectPathComponent { name })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r1_offset)
                            .context("r1_offset")?;

                        let variable_refs = (0..v_count)
                            .map(|_| {
                                let _ = file.read_u64()?;
                                let hash = file.read_u64()?;
                                let name_offset = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;

                                let old = file.tell()?;
                                file.seek(SeekFrom::Start(
                                    name_offset + str8_offset + base_offset,
                                ))?;
                                let name = file.read_u8str()?;
                                file.seek(SeekFrom::Start(old))?;

                                Ok(VariableRef { name })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r2_offset)
                            .context("r2_offset")?;

                        let variable_values = (0..w_count)
                            .map(|_| {
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let value = file.read_u64()?;
                                let _ = file.read_u64()?;
                                Ok(VariableValue { value })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r3_offset)
                            .context("r3_offset")?;
                        //..

                        let old = file.tell()?;
                        file.seek(SeekFrom::Start(name))?;
                        let name = file.read_u16str()?;
                        file.seek(SeekFrom::Start(old))?;

                        Ok(Clip {
                            name,
                            object_path,
                            variable_refs,
                            variable_values,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                file.seek(SeekFrom::Start(old))?;

                Ok(Control {
                    hash: control.hash,
                    name: control.name,
                    type_name: control.type_name,
                    play_objects,
                    clips,
                    q_what,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(root_offset).context("root_offset")?;
        let root = read_play_object(&mut file)?;

        Ok(Gui { root, controls })
    }
}
