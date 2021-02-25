use crate::file_ext::*;
use crate::rsz::*;
use anyhow::*;
use std::any::Any;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub enum UserData {
    RszRootIndex(usize),
    Data(Box<dyn Any>),
}

impl UserData {
    fn accept(&mut self, roots: &mut [Option<Box<dyn Any>>]) -> Result<()> {
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
            data.downcast().ok()
        } else {
            panic!();
        }
    }
}

pub enum Shape {
    Capsule {
        p0: Vec4<f32>,
        p1: Vec4<f32>,
        r: f32,
    },
    Unknown,
}

pub struct Collider {
    pub name: String,
    pub bone_a: String,
    pub bone_b: String,
    pub shape: Shape,
    pub user_data: UserData,
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

pub struct Rcol {
    pub rsz: Rsz,
    pub collider_groups: Vec<ColliderGroup>,
    pub enablers: Vec<String>,
    pub group_attachments: Vec<GroupAttachment>,
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
        let enabler_count = file.read_u32()?;
        let e_count = file.read_u32()?;

        let rsz_len = file.read_u32()?;
        file.read_u32()?;
        let a_offset = file.read_u64()?;

        let rsz_offset = file.read_u64()?;
        let group_attachment_offset = file.read_u64()?;

        let enabler_offset = file.read_u64()?;
        let e_offset = file.read_u64()?;
        let string_table_offset = e_offset + u64::from(e_count) * 0x40;

        file.seek_noop(a_offset)?;

        let mut collider_groups = (0..collider_group_count)
            .map(|_| {
                file.seek(SeekFrom::Current(0x10))?;
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }

                let _hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                let collider_count = file.read_u32()?;
                let m_count = file.read_u32()?;
                let collider_offset = file.read_u64()?;
                if !(collider_offset >= a_offset + 0x50 * u64::from(collider_group_count)
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
                    && m_offset <= enabler_offset)
                {
                    bail!("m offset out of bound")
                }

                file.seek(SeekFrom::Current(0x10))?;

                let old = file.tell()?;

                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;

                file.seek(SeekFrom::Start(collider_offset))?;
                let colliders = (0..collider_count)
                    .map(|_| {
                        file.seek(SeekFrom::Current(0x10))?;

                        let name_offset = file.read_u64()?;
                        if name_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }
                        let _hash = file.read_u32()?;
                        let rsz_root_index = file.read_u32()?;

                        let x = file.read_u32()?;
                        if x != 0 {
                            bail!("Expected zero");
                        }
                        let x = file.read_u32()?;
                        if x != 0xFFFFFFFF {
                            bail!("Expected FFFFFFFF");
                        }
                        let x = file.read_u32()?;
                        if x != 0 {
                            bail!("Expected zero");
                        }
                        let _ = file.read_u32()?;

                        let bone_a_offset = file.read_u64()?;
                        let bone_b_offset = file.read_u64()?;
                        if bone_a_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }
                        if bone_b_offset < string_table_offset {
                            bail!("name offset out of bound");
                        }

                        let _bone_a_hash = file.read_u32()?;
                        let _bone_b_hash = file.read_u32()?;
                        let shape_type = file.read_u32()?;
                        let x = file.read_u32()?;
                        if x != 0 {
                            bail!("Expected zero");
                        }

                        let shape = match shape_type {
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
                                Shape::Capsule { p0, p1, r: r.x }
                            }
                            _ => {
                                file.seek(SeekFrom::Current(0x50))?;
                                Shape::Unknown
                            }
                        };

                        let old = file.tell()?;
                        file.seek(SeekFrom::Start(name_offset))?;
                        let name = file.read_u16str()?;
                        file.seek(SeekFrom::Start(bone_a_offset))?;
                        let bone_a = file.read_u16str()?;
                        file.seek(SeekFrom::Start(bone_b_offset))?;
                        let bone_b = file.read_u16str()?;
                        file.seek(SeekFrom::Start(old))?;

                        Ok(Collider {
                            name,
                            bone_a,
                            bone_b,
                            shape,
                            user_data: UserData::RszRootIndex(rsz_root_index.try_into()?),
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
                let _name_hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                let name_b_offset = file.read_u64()?;
                if name_b_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let _name_b_hash = file.read_u32()?;
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

        file.seek(SeekFrom::Start(enabler_offset))?;
        let enablers = (0..enabler_count)
            .map(|_| {
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let old = file.tell()?;
                file.seek(SeekFrom::Start(name_offset))?;
                let name = file.read_u16str()?;
                file.seek(SeekFrom::Start(old))?;
                let _hash = file.read_u32()?;
                let x = file.read_u32()?;
                if x != 0 {
                    bail!("Expected zero");
                }

                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(e_offset)?;
        (0..e_count)
            .map(|_| {
                let name_offset = file.read_u64()?;
                if name_offset < string_table_offset {
                    bail!("name offset out of bound");
                }
                let old = file.tell()?;
                file.seek(SeekFrom::Start(name_offset))?;
                let _name = file.read_u16str()?;
                file.seek(SeekFrom::Start(old))?;

                file.seek(SeekFrom::Current(0x38))?;

                Ok(())
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
            enablers,
            group_attachments,
        })
    }

    pub fn dump(&self) -> Result<()> {
        for (i, collider_group) in self.collider_groups.iter().enumerate() {
            println!("[{}] {}", i, collider_group.name);
            for collider in &collider_group.colliders {
                println!(
                    " - {}, {}, {}",
                    collider.name, collider.bone_a, collider.bone_b
                );
            }
        }

        for enabler in &self.enablers {
            println!("* {}", enabler);
        }

        for c in &self.group_attachments {
            println!(
                ">>>->[{}] {}, {}, {}, {}",
                c.collider_group_index, c.name, c.name_b, c.p, c.r
            )
        }

        Ok(())
    }
}
