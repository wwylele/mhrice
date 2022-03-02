use crate::file_ext::*;
use crate::pak::*;
use crate::rsz;
use crate::rsz::Rsz;
use crate::user::UserChild;
use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek};
use std::rc::Rc;

#[derive(Debug)]
pub struct ScnGameObject {
    guid: [u8; 16],
    object_index: u32,
    parent_index: Option<u32>, // could be a game object or a folder
    component_count: u32,
    prefab_index: Option<u32>,
}

#[derive(Debug)]
pub struct ScnFolder {
    folder_object_index: u32,
    parent_index: Option<u32>,
}

#[derive(Debug)]
pub struct Scn {
    pub game_objects: Vec<ScnGameObject>,
    pub folders: Vec<ScnFolder>,
    pub resource_names: Vec<String>,
    pub prefab_paths: Vec<String>,
    pub children: Vec<UserChild>,
    pub rsz: Rsz,
}

fn scn_option(v: u32) -> Option<u32> {
    if v == 0xFFFFFFFF {
        None
    } else {
        Some(v)
    }
}

impl Scn {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Scn> {
        let magic = file.read_magic()?;
        if &magic != b"SCN\0" {
            bail!("Wrong magic for SCN file");
        }
        let game_object_count = file.read_u32()?;
        let resource_count = file.read_u32()?;
        let folder_count = file.read_u32()?;
        let prefab_count = file.read_u32()?;
        let child_count = file.read_u32()?;

        let folder_list_offset = file.read_u64()?;
        let resource_list_offset = file.read_u64()?;
        let prefab_list = file.read_u64()?;
        let child_list_offset = file.read_u64()?;
        let rsz_offset = file.read_u64()?;

        let mut index_footprint = BTreeSet::new();

        let game_objects = (0..game_object_count)
            .map(|_| {
                let mut guid = [0; 16];
                file.read_exact(&mut guid)?;
                let object_index = file.read_u32()?;
                let parent_index = scn_option(file.read_u32()?);
                let component_count = file.read_u32()?;
                let prefab_index = scn_option(file.read_u32()?);
                if let Some(prefab_index) = prefab_index {
                    if prefab_index >= prefab_count {
                        bail!("Prefab index out of bound")
                    }
                }
                for index in object_index..=object_index + component_count {
                    if !index_footprint.insert(index) {
                        bail!("Multiple object referring to the same index")
                    }
                }
                Ok(ScnGameObject {
                    guid,
                    object_index,
                    parent_index,
                    component_count,
                    prefab_index,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(folder_list_offset)
            .context("Undisconvered data before velkhana list")?;
        let folders = (0..folder_count)
            .map(|_| {
                let folder_object_index = file.read_u32()?;
                let parent_index = scn_option(file.read_u32()?);
                if !index_footprint.insert(folder_object_index) {
                    bail!("Multiple object referring to the same index")
                }
                Ok(ScnFolder {
                    folder_object_index,
                    parent_index,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(resource_list_offset, 16)
            .context("Undisconvered data before resource A list")?;
        let resource_name_offsets = (0..resource_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(prefab_list, 16)
            .context("Undisconvered data before resource B list")?;
        let prefab_offsets = (0..prefab_count)
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

        let prefab_paths = prefab_offsets
            .into_iter()
            .map(|prefab_name_offset| {
                file.seek_noop(prefab_name_offset)
                    .context("Undiscovered data in prefab paths")?;
                let name = file.read_u16str()?;
                if !name.ends_with(".pfb") {
                    bail!("Expected prefab file");
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

        if rsz.root_count() != index_footprint.len() {
            bail!("Incorrect rsz root count")
        }

        if u32::try_from(index_footprint.len())?
            != index_footprint
                .iter()
                .next_back()
                .map(|x| *x + 1)
                .unwrap_or(0)
        {
            bail!("index foot print has gaps")
        }

        Ok(Scn {
            game_objects,
            folders,
            resource_names,
            prefab_paths,
            children,
            rsz,
        })
    }

    pub fn dump(&self) {
        println!("Game objects:");
        for n in &self.game_objects {
            println!(
                "object={}, parent={:?}, sub={}, {:?}",
                n.object_index, n.parent_index, n.component_count, n.prefab_index
            )
        }
        println!();

        println!("Folders:");
        for v in &self.folders {
            println!(
                "object={}, parent={:?}",
                v.folder_object_index, v.parent_index
            )
        }
        println!();

        println!("Resource:");
        for r in &self.resource_names {
            println!(" - {}", r)
        }
        println!();

        println!("Prefab:");
        for r in &self.prefab_paths {
            println!(" - {}", r)
        }
        println!();

        println!("Children:");
        for c in &self.children {
            println!(" - [{:08X}] {}", c.hash, c.name)
        }
        println!();

        println!("RSZ:");
        match self.rsz.deserialize() {
            Ok(objects) => {
                for (i, o) in objects.into_iter().enumerate() {
                    println!("== {i} ==");
                    println!("{}", o.to_json().unwrap());
                }
            }
            Err(e) => {
                println!("Failed to serialize because {}", e);
                for (i, root) in self.rsz.roots.iter().enumerate() {
                    let type_descriptor = self.rsz.type_descriptors[*root as usize];
                    let hash = (type_descriptor & 0xFFFFFFFF) as u32;
                    let symbol = rsz::RSZ_TYPE_MAP
                        .get(&hash)
                        .map(|t| t.symbol)
                        .unwrap_or_default();
                    println!(" [{}] - {:016X} - {}", i, type_descriptor, symbol)
                }
                Result::<()>::Err(e).unwrap()
            }
        }
    }
}

#[derive(Debug)]
struct GameObject {
    object: rsz::GameObject,
    components: Vec<rsz::AnyRsz>,
    prefab: Option<Rc<String>>,
    children: Vec<GameObject>,
}

#[derive(Debug)]
struct Folder {
    folder: rsz::Folder,
    subscene: Result<Scene>,
    children: Vec<GameObject>,
    subfolders: Vec<Folder>,
}

#[derive(Debug)]
pub struct Scene {
    objects: Vec<GameObject>,
    folders: Vec<Folder>,
}

impl Scene {
    pub fn new<F: Read + Seek>(pak: &mut PakReader<F>, path: &str) -> Result<Scene> {
        let index = pak.find_file(path)?;
        let content = pak.read_file(index)?;
        let scn = Scn::new(Cursor::new(content))?;
        let mut data: Vec<Option<rsz::AnyRsz>> =
            scn.rsz.deserialize()?.into_iter().map(Some).collect();
        let prefabs: Vec<Rc<String>> = scn.prefab_paths.into_iter().map(|s| Rc::new(s)).collect();

        let mut orphans: HashMap<Option<u32>, Vec<GameObject>> = HashMap::new();
        let mut orphan_folders: HashMap<Option<u32>, Vec<Folder>> = HashMap::new();

        for go in scn.game_objects.into_iter().rev() {
            let object: rsz::GameObject = data
                .get_mut(usize::try_from(go.object_index)?)
                .context("game object index out of bound")?
                .take()
                .context("game object data already taken")?
                .downcast()
                .context("GameObject type mismatch")?;
            let components: Vec<rsz::AnyRsz> = (go.object_index + 1
                ..=go.object_index + go.component_count)
                .map(|i| {
                    data.get_mut(usize::try_from(i)?)
                        .context("component index out of bound")?
                        .take()
                        .context("component data already taken")
                })
                .collect::<Result<_>>()?;
            let prefab: Option<Rc<String>> = go
                .prefab_index
                .map(|i| -> Result<Rc<String>> {
                    Ok(prefabs
                        .get(usize::try_from(i)?)
                        .context("prefab index out of bound")?
                        .clone())
                })
                .transpose()?;
            let children = orphans.remove(&Some(go.object_index)).map_or_else(
                Vec::new,
                |mut children: Vec<GameObject>| {
                    children.reverse();
                    children
                },
            );

            let game_object = GameObject {
                object,
                components,
                prefab,
                children,
            };

            orphans
                .entry(go.parent_index)
                .or_default()
                .push(game_object);
        }

        for f in scn.folders.into_iter().rev() {
            let folder: rsz::Folder = data
                .get_mut(usize::try_from(f.folder_object_index)?)
                .context("folder index out of bound")?
                .take()
                .context("folder data already taken")?
                .downcast()
                .context("Folder type mismatch")?;
            let subscene = Scene::new(pak, &folder.path);
            let children = orphans.remove(&Some(f.folder_object_index)).map_or_else(
                Vec::new,
                |mut children: Vec<GameObject>| {
                    children.reverse();
                    children
                },
            );
            let subfolders = orphan_folders
                .remove(&Some(f.folder_object_index))
                .map_or_else(Vec::new, |mut children: Vec<Folder>| {
                    children.reverse();
                    children
                });
            let folder = Folder {
                folder,
                subscene,
                children,
                subfolders,
            };

            orphan_folders
                .entry(f.parent_index)
                .or_default()
                .push(folder);
        }

        if data.into_iter().any(|d| d.is_some()) {
            bail!("Left over data")
        }

        let objects =
            orphans
                .remove(&None)
                .map_or_else(Vec::new, |mut children: Vec<GameObject>| {
                    children.reverse();
                    children
                });

        let folders =
            orphan_folders
                .remove(&None)
                .map_or_else(Vec::new, |mut children: Vec<Folder>| {
                    children.reverse();
                    children
                });

        if !orphans.is_empty() {
            bail!("Found orphan game object")
        }

        if !orphan_folders.is_empty() {
            bail!("Found orphan folder")
        }

        Ok(Scene { objects, folders })
    }
}
