use crate::file_ext::*;
use crate::rsz;
use crate::rsz::Rsz;
use crate::scn::scn_option;
use crate::user::UserChild;
use anyhow::{bail, Context, Result};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct PfbGameObject {
    object_index: u32,
    parent_index: Option<u32>,
    component_count: u32,
}

#[derive(Debug)]
pub struct RefLink {
    // this refers to a "root" in RSZ. However, this
    // root isn't a real root object, but a reference to a leaf object
    node_index: u32,

    // This refers to a member to populate in the leaf object.
    // It has type GameObjectRef, or an rray of it.
    // Unclear how members are indexed. Roughly it seems to index property
    // first, then fields.
    member_index: u16,

    b: u16,

    // If the field is an rray, this is the index in to the array
    array_index: u32,

    // The game object the member should reference to
    object_index: u32,
}

#[derive(Debug)]
pub struct Pfb {
    pub game_objects: Vec<PfbGameObject>,
    pub ref_links: Vec<RefLink>,
    pub resource_names: Vec<String>,
    pub children: Vec<UserChild>,
    pub rsz: Rsz,
}

impl Pfb {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Pfb> {
        let magic = file.read_magic()?;
        if &magic != b"PFB\0" {
            bail!("Wrong magic for PFB file");
        }
        let game_object_count = file.read_u32()?;
        let resource_count = file.read_u32()?;
        let ref_link_count = file.read_u32()?;
        let child_count = file.read_u32()?;
        let padding = file.read_u32()?;
        if padding != 0 {
            bail!("Unexpected non-zero padding A: {}", padding);
        }
        let rathalos_offset = file.read_u64()?;
        let resource_list_offset = file.read_u64()?;
        let child_list_offset = file.read_u64()?;
        let rsz_offset = file.read_u64()?;

        let game_objects = (0..game_object_count)
            .map(|_| {
                let object_index = file.read_u32()?;
                let parent_index = scn_option(file.read_u32()?);
                let component_count = file.read_u32()?;
                Ok(PfbGameObject {
                    object_index,
                    parent_index,
                    component_count,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        file.seek_noop(rathalos_offset)
            .context("Undisconvered data before rathalos list")?;

        let ref_links = (0..ref_link_count)
            .map(|_| {
                let node_index = file.read_u32()?;
                let member_index = file.read_u16()?;
                let b = file.read_u16()?;
                let array_index = file.read_u32()?;
                let object_index = file.read_u32()?;
                Ok(RefLink {
                    node_index,
                    member_index,
                    b,
                    array_index,
                    object_index,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(resource_list_offset, 16)
            .context("Undisconvered data before resource list")?;
        let resource_name_offsets = (0..resource_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(child_list_offset, 16)
            .context("Undisconvered data before child list")?;
        let child_info = (0..child_count)
            .map(|_| {
                let hash = file.read_u32()?;
                let padding = file.read_u32()?;
                if padding != 0 {
                    bail!("ChildInfo non-zero padding {}", padding);
                }
                let name_offset = file.read_u64()?;
                Ok((hash, name_offset))
            })
            .collect::<Result<Vec<_>>>()?;

        let resource_names = resource_name_offsets
            .into_iter()
            .map(|resource_name_offset| {
                file.seek_noop(resource_name_offset)
                    .context("Undiscovered data in resource names")?;
                let name = file.read_u16str()?;
                if name.ends_with(".user") {
                    bail!("USER resource");
                }
                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;

        let children = child_info
            .into_iter()
            .map(|(hash, name_offset)| {
                file.seek_noop(name_offset)
                    .context("Undiscovered data in child info")?;
                let name = file.read_u16str()?;
                if !name.ends_with(".user") {
                    bail!("Non-USER child");
                }
                Ok(UserChild { hash, name })
            })
            .collect::<Result<Vec<_>>>()?;

        let rsz = Rsz::new(file, rsz_offset)?;

        /*if !ref_links.is_empty() {
            println!("------------");
            for ref_link in &ref_links {
                let t = rsz.type_descriptors[rsz.roots[ref_link.node_index as usize] as usize].hash;
                println!(
                    "node={}, [{}, {}, {}], type={:08X}",
                    ref_link.node_index, ref_link.member_index, ref_link.b, ref_link.array_index, t
                )
            }
        }*/

        Ok(Pfb {
            game_objects,
            ref_links,
            resource_names,
            children,
            rsz,
        })
    }

    pub fn dump(&self) {
        println!("Game objects:");
        for game_object in &self.game_objects {
            println!(
                "object={}, parent={:?}, components={}",
                game_object.object_index, game_object.parent_index, game_object.component_count
            )
        }
        println!();

        println!("Ref link:");
        for ref_link in &self.ref_links {
            println!(
                "node={}, parent={}, [{}, {}, {}]",
                ref_link.node_index,
                ref_link.object_index,
                ref_link.member_index,
                ref_link.b,
                ref_link.array_index,
            )
        }
        println!();

        println!("Resource:");
        for r in &self.resource_names {
            println!(" - {r}")
        }
        println!();

        println!("Children:");
        for c in &self.children {
            println!(" - [{:08X}] {}", c.hash, c.name)
        }
        println!();

        println!("RSZ:");
        for (i, root) in self.rsz.roots.iter().enumerate() {
            println!("{i:4} -> {root:4}")
        }
        println!();
        match self.rsz.deserialize(None) {
            Ok(objects) => {
                for (i, o) in objects.into_iter().enumerate() {
                    println!("== {i} ==");
                    println!("{}", o.to_json().unwrap());
                }
            }
            Err(e) => {
                println!("Failed to serialize because {e}");
                for (i, type_descriptor) in self.rsz.type_descriptors.iter().enumerate() {
                    // let type_descriptor = self.rsz.type_descriptors[*root as usize];
                    let hash = type_descriptor.hash;
                    let symbol = rsz::RSZ_TYPE_MAP
                        .get(&hash)
                        .map(|t| t.symbol)
                        .unwrap_or_default();
                    println!(
                        " [{}] - {:08X}, {:08X} - {}",
                        i, hash, type_descriptor.crc, symbol
                    )
                }
                Result::<()>::Err(e).unwrap()
            }
        }
    }
}
