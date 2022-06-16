use crate::file_ext::*;
use crate::gpu::ColoredVertex;
use crate::hash::hash_as_utf16;
use crate::mesh::*;
use crate::rsz::*;
use anyhow::{bail, Context, Result};
use nalgebra_glm::*;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub enum UserData {
    RszRootIndex(usize),
    Data(AnyRsz),
}

impl UserData {
    fn accept(&mut self, roots: &mut [Option<AnyRsz>]) -> Result<()> {
        if let UserData::RszRootIndex(index) = *self {
            *self = UserData::Data(
                roots
                    .get_mut(index)
                    .context("RSZ root index out of bound")?
                    .take()
                    .context("Data already taken")?,
            );
            Ok(())
        } else {
            panic!();
        }
    }

    pub fn downcast<T: 'static>(self) -> Option<Box<T>> {
        if let UserData::Data(data) = self {
            data.downcast()
        } else {
            panic!();
        }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if let UserData::Data(data) = self {
            data.downcast_ref()
        } else {
            panic!();
        }
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere { p: Vec3, r: f32 },
    Capsule { p0: Vec3, p1: Vec3, r: f32 },
    Unknown,
}

impl Shape {
    pub fn distance(&self, point: &Vec3) -> Result<f32> {
        match self {
            Shape::Sphere { p, r } => Ok(distance(p, point) / r),
            Shape::Capsule { p0, p1, r } => {
                let l2 = distance2(p0, p1);
                let t = clamp_scalar(dot(&(point - p0), &(p1 - p0)) / l2, 0.0, 1.0);
                let projection = p0 + t * (p1 - p0);
                Ok(distance(point, &projection) / r)
            }
            Shape::Unknown => bail!("Unknown shape"),
        }
    }
}

pub struct Collider {
    pub name: String,
    pub bone_a: String,
    pub bone_b: String,
    pub shape: Shape,
    pub user_data: UserData,
    pub ignore_tag_bits: u32,
}
pub struct ColliderGroup {
    pub name: String,
    pub colliders: Vec<Collider>,
}

pub struct GroupAttachment {
    pub user_data: UserData,
    pub name: String,
    pub name_b: String,
    pub p: u32,
    pub collider_group_index: usize,
    pub r: u64,
}

pub struct E {
    pub name: String,
}

pub struct Rcol {
    pub rsz: Rsz,
    pub collider_groups: Vec<ColliderGroup>,
    pub ignore_tags: Vec<String>,
    pub group_attachments: Vec<GroupAttachment>,
    pub es: Vec<E>,
}

impl Rcol {
    pub fn new<F: Read + Seek>(mut file: F, deserialize_user_data: bool) -> Result<Rcol> {
        if &file.read_magic()? != b"RCOL" {
            bail!("Wrong magic for RCOL");
        }

        let collider_group_count = file.read_u32()?;
        let total_collider_count = file.read_u32()?;
        file.read_u32()?;

        let group_attachment_count = file.read_u32()?;
        file.read_u32()?;
        let ignore_tag_count = file.read_u32()?;
        let e_count = file.read_u32()?;

        let rsz_len = file.read_u32()?;
        file.read_u32()?;

        let mut collider_group_offset = file.read_u64()?;
        let mut new_version = false;
        if collider_group_offset != 0x50 {
            new_version = true;
            let _ = collider_group_offset;
            collider_group_offset = file.read_u64()?;
        }

        let rsz_offset = file.read_u64()?;
        let group_attachment_offset = file.read_u64()?;

        let ignore_tag_offset = file.read_u64()?;
        let e_offset = file.read_u64()?;

        if new_version {
            // TODO: what these are and where string_table_offset is
            let _u_offset = file.read_u64()?;
            let _v_offset = file.read_u64()?;
            let _ = file.read_u64()?;
        }

        // only for verification
        let string_table_offset = e_offset + u64::from(e_count) * 0x40;

        file.seek_noop(collider_group_offset)?;

        let mut collider_groups = (0..collider_group_count)
            .map(|_| {
                file.seek(SeekFrom::Current(0x10))?;
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }

                let hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                let collider_count = file.read_u32()?;
                let m_count = file.read_u32()?;
                let collider_offset = file.read_u64()?;
                if !(collider_offset
                    >= collider_group_offset + 0x50 * u64::from(collider_group_count)
                    && collider_offset <= rsz_offset)
                {
                    bail!("j offset out of bound")
                }

                let x = file.read_u64()?;
                if x != 0 {
                    bail!("Expected zero");
                }
                let m_offset = file.read_u64()?;
                if !(m_offset >= group_attachment_offset + 0x30 * u64::from(group_attachment_count)
                    && m_offset <= ignore_tag_offset)
                {
                    bail!("m offset out of bound")
                }

                file.seek(SeekFrom::Current(0x10))?;

                let old = file.tell()?;

                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;

                let hash_calc = hash_as_utf16(&name);
                if hash_calc != hash {
                    bail!("hash mismatch")
                }

                file.seek(SeekFrom::Start(collider_offset))?;
                let colliders = (0..collider_count)
                    .map(|_| {
                        file.seek(SeekFrom::Current(0x10))?;

                        let name_offset = file.read_u64()?;
                        if name_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }
                        let hash = file.read_u32()?;
                        let rsz_root_index = file.read_u32()?;

                        let x = file.read_u32()?;
                        if x != 0 {
                            bail!("Expected zero");
                        }
                        let x = file.read_u32()?;
                        if x != 0xFFFFFFFF {
                            bail!("Expected FFFFFFFF");
                        }
                        let _y = file.read_u32()?;
                        let ignore_tag_bits = file.read_u32()?;
                        if ignore_tag_bits >= 1 << ignore_tag_count {
                            bail!("ignore_tag out of bound")
                        }

                        let bone_a_offset = file.read_u64()?;
                        let bone_b_offset = file.read_u64()?;
                        if bone_a_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }
                        if bone_b_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }

                        let bone_a_hash = file.read_u32()?;
                        let bone_b_hash = file.read_u32()?;
                        let shape_type = file.read_u32()?;
                        let x = file.read_u32()?;
                        if x != 0 {
                            bail!("Expected zero");
                        }

                        let old = file.tell()?;
                        file.seek(SeekFrom::Start(name_offset))?;
                        let name = file.read_u16str()?;
                        file.seek(SeekFrom::Start(bone_a_offset))?;
                        let bone_a = file.read_u16str()?;
                        file.seek(SeekFrom::Start(bone_b_offset))?;
                        let bone_b = file.read_u16str()?;
                        file.seek(SeekFrom::Start(old))?;

                        let hash_calc = hash_as_utf16(&name);
                        if hash_calc != hash {
                            bail!("hash mismatch")
                        }

                        let bone_a_hash_calc = hash_as_utf16(&bone_a);
                        if bone_a_hash_calc != bone_a_hash {
                            bail!("bone_a_hash mismatch")
                        }

                        let bone_b_hash_calc = hash_as_utf16(&bone_b);
                        if bone_b_hash_calc != bone_b_hash {
                            bail!("bone_b_hash mismatch")
                        }

                        let shape = match shape_type {
                            1 => {
                                let p = file.read_f32vec4()?;
                                let mut padding = [0; 0x40];
                                file.read_exact(&mut padding)?;
                                Shape::Sphere { p: p.xyz(), r: p.w }
                            }
                            3 => {
                                let p0 = file.read_f32vec4()?;
                                let p1 = file.read_f32vec4()?;
                                let r = file.read_f32vec4()?;
                                let mut padding = [0; 0x20];
                                file.read_exact(&mut padding)?;
                                if p0.w != 0.0 {
                                    bail!("p0.w != 0")
                                }
                                if p1.w != 0.0 {
                                    bail!("p1.w != 0")
                                }
                                #[allow(clippy::float_cmp)] // I mean it
                                if r.x != r.y || r.x != r.z || r.x != r.w {
                                    bail!("r not all same")
                                }
                                if padding.iter().any(|&p| p != 0) {
                                    bail!("Padding not zero")
                                }
                                Shape::Capsule {
                                    p0: p0.xyz(),
                                    p1: p1.xyz(),
                                    r: r.x,
                                }
                            }
                            _ => {
                                file.seek(SeekFrom::Current(0x50))?;
                                Shape::Unknown
                            }
                        };

                        Ok(Collider {
                            name,
                            bone_a,
                            bone_b,
                            shape,
                            user_data: UserData::RszRootIndex(rsz_root_index.try_into()?),
                            ignore_tag_bits,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                // This is fishy. It seems that they tried to deduplicate
                // so multiple group might point to the same m, but on the other hand,
                // the duplicate elements are still stored and ultimatedly
                // go unused.
                file.seek(SeekFrom::Start(m_offset))?;
                (0..m_count)
                    .map(|_| {
                        file.read_u64()?;
                        file.read_u64()?;
                        Ok(())
                    })
                    .collect::<Result<Vec<_>>>()?;

                file.seek(SeekFrom::Start(old))?;

                Ok(ColliderGroup { name, colliders })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek(SeekFrom::Current(0xA0 * i64::from(total_collider_count)))?;

        file.seek_noop(rsz_offset)?;
        let mut rsz_buf = vec![0; usize::try_from(rsz_len)?];
        file.read_exact(&mut rsz_buf)?;
        let rsz = Rsz::new(Cursor::new(&rsz_buf), 0)?;

        file.seek_assert_align_up(group_attachment_offset, 16)?;

        let mut group_attachments = (0..group_attachment_count)
            .map(|i| {
                let p = file.read_u32()?;
                let collider_group_index = file.read_u32()?;
                if collider_group_index >= collider_group_count {
                    bail!("collider group index out of bound");
                }
                let r = file.read_u64()?;

                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let name_hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    //bail!("Expected zero");
                    // new version start to have non-zero id here
                }

                let name_b_offset = file.read_u64()?;
                if name_b_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let name_b_hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                let old = file.tell()?;
                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(name_b_offset))?;
                let name_b = file.read_u16str()?;
                file.seek(SeekFrom::Start(old))?;

                let hash_calc = hash_as_utf16(&name);
                if hash_calc != name_hash {
                    bail!("hash mismatch")
                }

                let hash_b_calc = hash_as_utf16(&name_b);
                if hash_b_calc != name_b_hash {
                    bail!("hash_b mismatch")
                }

                Ok(GroupAttachment {
                    user_data: UserData::RszRootIndex(i.try_into()?),
                    name,
                    name_b,
                    p,
                    collider_group_index: collider_group_index.try_into()?,
                    r,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // Some data pointed by A is in between here

        file.seek(SeekFrom::Start(ignore_tag_offset))?;
        let ignore_tags = (0..ignore_tag_count)
            .map(|_| {
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let old = file.tell()?;
                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(old))?;
                let hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                let hash_calc = hash_as_utf16(&name);
                if hash_calc != hash {
                    bail!("hash mismatch")
                }

                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(e_offset)?;
        let es = (0..e_count)
            .map(|_| {
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let old = file.tell()?;
                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(old))?;

                file.seek(SeekFrom::Current(0x38))?;

                Ok(E { name })
            })
            .collect::<Result<Vec<_>>>()?;

        // followed by string table

        if deserialize_user_data {
            let mut roots: Vec<_> = rsz.deserialize()?.into_iter().map(Option::Some).collect();
            for group_attachment in &mut group_attachments {
                group_attachment.user_data.accept(&mut roots)?;
            }
            for group in &mut collider_groups {
                for collider in &mut group.colliders {
                    collider.user_data.accept(&mut roots)?;
                }
            }
            if roots.iter().any(Option::is_some) {
                bail!("Left over user data")
            }
        }

        Ok(Rcol {
            rsz,
            collider_groups,
            ignore_tags,
            group_attachments,
            es,
        })
    }

    pub fn dump(&self) -> Result<()> {
        fn print_user_data(user_data: &AnyRsz) -> Result<()> {
            println!("{}", user_data.to_json()?);
            Ok(())
        }
        for (i, collider_group) in self.collider_groups.iter().enumerate() {
            println!("[{}] {}", i, collider_group.name);
            for collider in &collider_group.colliders {
                println!(
                    " - {}, {}, {}, /** {} **/",
                    collider.name, collider.bone_a, collider.bone_b, collider.ignore_tag_bits
                );
                if let UserData::Data(data) = &collider.user_data {
                    print_user_data(data)?;
                }
                println!("{:#?}", collider.shape);
            }
        }

        for ignore_tag in &self.ignore_tags {
            println!("* {}", ignore_tag);
        }

        for c in &self.group_attachments {
            println!(
                ">>>->[{}] {}, {}, {}, {}",
                c.collider_group_index, c.name, c.name_b, c.p, c.r
            );
            if let UserData::Data(data) = &c.user_data {
                print_user_data(data)?;
            }
        }

        for e in &self.es {
            println!("##> {}", e.name);
        }

        Ok(())
    }

    pub fn apply_skeleton(&mut self, mesh: &Mesh) -> Result<()> {
        for group in &mut self.collider_groups {
            for collider in &mut group.colliders {
                let bone_a = mesh
                    .bone_names
                    .get(&collider.bone_a)
                    .map(|i| &mesh.bones[*i]);
                let bone_b = mesh
                    .bone_names
                    .get(&collider.bone_b)
                    .map(|i| &mesh.bones[*i]);

                let bone_a = if let Some(bone_a) = bone_a {
                    bone_a
                } else {
                    eprintln!("Unknown bone a {}", collider.bone_a);
                    continue;
                };

                collider.shape = match collider.shape {
                    Shape::Capsule { p0, p1, r } => {
                        let bone_b = if let Some(bone_b) = bone_b {
                            bone_b
                        } else {
                            eprintln!("Unknown bone b {}", collider.bone_b);
                            continue;
                        };
                        Shape::Capsule {
                            p0: (bone_a.absolute_transform * vec4(p0.x, p0.y, p0.z, 1.0)).xyz(),
                            p1: (bone_b.absolute_transform * vec4(p1.x, p1.y, p1.z, 1.0)).xyz(),
                            r,
                        }
                    }
                    Shape::Sphere { p, r } => Shape::Sphere {
                        p: (bone_a.absolute_transform * vec4(p.x, p.y, p.z, 1.0)).xyz(),
                        r,
                    },
                    Shape::Unknown => Shape::Unknown,
                }
            }
        }
        Ok(())
    }

    pub fn get_monster_ride_filter(&self) -> u32 {
        if let Some((i, _)) = self
            .ignore_tags
            .iter()
            .enumerate()
            .find(|(_, s)| *s == "操獣受付中の追加アタリ")
        {
            1 << i
        } else {
            0
        }
    }

    pub fn color_monster_model(&self, mesh: &Mesh) -> Result<(Vec<ColoredVertex>, Vec<u32>)> {
        let position = mesh
            .vertex_layouts
            .iter()
            .find(|layout| layout.usage == 0)
            .context("No position data")?;

        let vertex_count = (mesh.vertex_layouts[1].offset - mesh.vertex_layouts[0].offset)
            / u32::from(mesh.vertex_layouts[0].width);

        let mut buffer = mesh
            .vertex_buffer
            .get(usize::try_from(position.offset)?..)
            .context("Vertex out of bound")?;

        let ignore_tag_filter = self.get_monster_ride_filter();
        if ignore_tag_filter == 0 {
            bail!("Didn't find monster ride filter")
        }

        let vertexs = (0..vertex_count)
            .map(|_| {
                let position = buffer.read_f32vec3()?;

                let mut meat_dist = f32::MAX;
                let mut meat = None;
                let mut parts_group = None;
                let mut meat_set = HashSet::new();
                let mut parts_group_set = HashSet::new();
                for (i, group) in self.collider_groups.iter().enumerate() {
                    let mut new_parts_group = None;
                    for attachment in &self.group_attachments {
                        if attachment.collider_group_index == i {
                            if let Some(data) =
                                attachment.user_data.downcast_ref::<EmHitDamageRsData>()
                            {
                                new_parts_group = Some(usize::try_from(data.parts_group)?);
                            }
                        }
                    }

                    for collider in &group.colliders {
                        if collider.ignore_tag_bits & ignore_tag_filter != 0 {
                            continue;
                        }
                        if let Some(data) =
                            collider.user_data.downcast_ref::<EmHitDamageShapeData>()
                        {
                            let new_dist = collider.shape.distance(&position)?;
                            let new_meat = Some(usize::try_from(data.meat)?);
                            if new_dist < meat_dist {
                                meat_dist = new_dist;
                                meat = new_meat;
                                parts_group = new_parts_group;
                            }
                            if new_dist < 1.0 {
                                if let Some(new_meat) = new_meat {
                                    meat_set.insert(new_meat);
                                }
                                if let Some(new_parts_group) = new_parts_group {
                                    parts_group_set.insert(new_parts_group);
                                }
                            }
                        }
                    }
                }

                if meat_dist < 1.5 {
                    if meat_set.is_empty() {
                        if let Some(new_meat) = meat {
                            meat_set.insert(new_meat);
                        }
                    }
                    if parts_group_set.is_empty() {
                        if let Some(new_parts_group) = parts_group {
                            parts_group_set.insert(new_parts_group);
                        }
                    }
                }

                Ok(ColoredVertex {
                    position,
                    meat: meat_set,
                    parts_group: parts_group_set,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut indexs = vec![];

        for model_group in &mesh.main_model_lods[0].model_groups {
            for model in &model_group.models {
                let mut index_buffer = mesh
                    .index_buffer
                    .get(model.index_buffer_start as usize * 2..)
                    .context("index out of bound")?;
                for _ in 0..usize::try_from(model.vertex_count)? {
                    indexs.push(u32::from(index_buffer.read_u16()?) + (model.vertex_buffer_start))
                }
            }
        }

        Ok((vertexs, indexs))
    }
}
