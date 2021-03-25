use crate::file_ext::*;
use anyhow::*;
use serde::*;
use std::collections::BTreeMap;
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
struct Field {
    b: u32,
    name: String,
    value: FieldValue,
}

#[derive(Debug, Serialize)]
struct PlayObject {
    #[serde(serialize_with = "ser_hash")]
    hash: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    child_control_hash: [u8; 0x10],
    #[serde(serialize_with = "ser_hash")]
    hash2: [u8; 0x10],
    name: String,
    type_name: String,
    properties: Vec<Field>,
    variables: Vec<Field>,
}

#[derive(Debug, Serialize)]
struct Rathalos {
    name: String,
}

#[derive(Debug, Serialize)]
struct Rathian {
    name: String,
}

#[derive(Debug, Serialize)]
struct Clip {
    name: String,
    rathalos: Vec<Rathalos>,
    rathian: Vec<Rathian>,
}

#[derive(Debug, Serialize)]
struct Control {
    #[serde(serialize_with = "ser_hash")]
    hash: [u8; 0x10],
    name: String,
    type_name: String,
    play_objects: Vec<PlayObject>,
    clips: Vec<Clip>,
    q_what: u32,
}

#[derive(Debug, Serialize)]
pub struct Gui {
    root: PlayObject,
    controls: Vec<Control>,
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

        let control_start_offset = file.read_u64()?;
        let root_offset = file.read_u64()?;
        let control_count = file.read_u64()?;
        let control_offset_list = (0..control_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(control_start_offset)?;

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
                file.seek_noop(x_offset)?;
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
            let variable_offset = file.read_u64()?;
            let mtx_offset = file.read_u64()?;

            file.seek_noop(property_offset)?;
            let property_count = file.read_u64()?;
            let properties = (0..property_count)
                .map(|_| read_field(file))
                .collect::<Result<Vec<_>>>()?;
            file.seek_noop(variable_offset)?;
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
                file.seek_noop(control.play_object_list_offset)?;
                let play_object_count = file.read_u64()?;
                let play_object_offsets = (0..play_object_count)
                    .map(|_| file.read_u64())
                    .collect::<Result<Vec<_>>>()?;
                file.seek_noop(control.clip_list_offset)?;
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

                        if file.read_u32()? != 0x28 {
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

                        file.seek_noop(base_offset + r0_offset)?;

                        let us = (0..u_count)
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

                                Ok(Rathalos { name })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r1_offset)?;

                        let vs = (0..v_count)
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

                                Ok(Rathian { name })
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r2_offset)?;

                        let ws = (0..w_count)
                            .map(|_| {
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                let _ = file.read_u64()?;
                                Ok(())
                            })
                            .collect::<Result<Vec<_>>>()?;

                        file.seek_noop(base_offset + r3_offset)?;
                        //..

                        let old = file.tell()?;
                        file.seek(SeekFrom::Start(name))?;
                        let name = file.read_u16str()?;
                        file.seek(SeekFrom::Start(old))?;

                        Ok(Clip {
                            name,
                            rathalos: us,
                            rathian: vs,
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

        file.seek_noop(root_offset)?;
        let root = read_play_object(&mut file)?;

        Ok(Gui { root, controls })
    }
}
