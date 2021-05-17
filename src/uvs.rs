use crate::file_ext::*;
use anyhow::*;
use nalgebra_glm::*;
use std::io::{Read, Seek};

pub struct TextureRef {
    pub id: u64,
    pub path: String,
}

pub struct Spriter {
    pub id: u64,
    pub p0: Vec2,
    pub p1: Vec2,
    pub anchors: Option<Vec<Vec2>>,
}

pub struct SpriterGroup {
    pub spriters: Vec<Spriter>,
}

pub struct Uvs {
    pub textures: Vec<TextureRef>,
    pub spriter_groups: Vec<SpriterGroup>,
}

impl Uvs {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Uvs> {
        if &file.read_magic()? != b".SVU" {
            bail!("Wrong magic for .SVU");
        }
        let texture_count = file.read_u32()?;
        let group_count = file.read_u32()?;
        let spriter_count = file.read_u32()?;
        let enable_anchor = file.read_u32()?;
        if enable_anchor != 0 && enable_anchor != 1 {
            bail!("Expected 0 or 1");
        }
        let x = file.read_u32()?;
        if x != 0 {
            bail!("Expected 0");
        }

        let texture_offset = file.read_u64()?;
        let group_offset = file.read_u64()?;
        let spriter_offset = file.read_u64()?;
        let string_offset = file.read_u64()?;

        file.seek_noop(texture_offset)?;

        struct TextureDesc {
            id: u64,
            path_offset: u64,
            path_b_offset: u64,
        }

        let textures = (0..texture_count)
            .map(|_| {
                let id = file.read_u64()?;
                let path_offset = file.read_u64()?;
                let path_b_offset = file.read_u64()?;
                let _ = file.read_u64()?; // more paths? currently all 0xFFFFF..
                let _ = file.read_u64()?;
                Ok(TextureDesc {
                    id,
                    path_offset,
                    path_b_offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(group_offset)?;

        let mut current = 0;
        let groups = (0..group_count)
            .map(|_| {
                let count = file.read_u32()?;
                let start = file.read_u32()?;
                if start != current {
                    bail!("Unexpected group start");
                }
                current += count;
                Ok(count)
            })
            .collect::<Result<Vec<_>>>()?;
        if current != spriter_count {
            bail!("Groups not cover all spriters");
        }

        file.seek_noop(spriter_offset)?;

        let spriter_groups = groups
            .into_iter()
            .map(|group| {
                Ok(SpriterGroup {
                    spriters: (0..group)
                        .map(|_| {
                            let id = file.read_u64()?;
                            let x0 = file.read_f32()?;
                            let y0 = file.read_f32()?;
                            let x1 = file.read_f32()?;
                            let y1 = file.read_f32()?;
                            let texture_index = file.read_u32()?;
                            if texture_index >= texture_count {
                                bail!("Texture index out of bound");
                            }
                            let anchor_count = file.read_u32()?;
                            if (anchor_count != 0xFFFFFFFF) != (enable_anchor == 1) {
                                bail!("Mismatch anchor param");
                            }
                            let anchors = if enable_anchor == 1 {
                                Some(
                                    (0..anchor_count)
                                        .map(|_| {
                                            let x = file.read_f32()?;
                                            let y = file.read_f32()?;
                                            Ok(vec2(x, y))
                                        })
                                        .collect::<Result<Vec<_>>>()?,
                                )
                            } else {
                                None
                            };

                            Ok(Spriter {
                                id,
                                p0: vec2(x0, y0),
                                p1: vec2(x1, y1),
                                anchors,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let textures = textures
            .into_iter()
            .map(|texture| {
                file.seek_noop(string_offset + texture.path_offset * 2)?;
                let path = file.read_u16str()?;

                let _ = if texture.path_b_offset != u64::MAX {
                    file.seek_noop(string_offset + texture.path_b_offset * 2)?;
                    Some(file.read_u16str()?)
                } else {
                    None
                };

                Ok(TextureRef {
                    id: texture.id,
                    path,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Uvs {
            textures,
            spriter_groups,
        })
    }
}
