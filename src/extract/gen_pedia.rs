use super::pedia::*;
use crate::gpu::*;
use crate::mesh::*;
use crate::msg::*;
use crate::pak::PakReader;
use crate::pfb::Pfb;
use crate::rcol::Rcol;
use crate::rsz::*;
use crate::tex::*;
use crate::user::User;
use anyhow::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::fs::*;
use std::io::{Cursor, Read, Seek};
use std::path::*;

fn exactly_one<T>(mut iterator: impl Iterator<Item = T>) -> Result<T> {
    let next = iterator.next().context("No element found")?;
    if iterator.next().is_some() {
        bail!("Multiple elements found");
    }
    Ok(next)
}

fn gen_em_collider_path(id: u32) -> String {
    format!("enemy/em{0:03}/00/collision/em{0:03}_00_colliders.rcol", id)
}

fn gen_ems_collider_path(id: u32) -> String {
    format!(
        "enemy/ems{0:03}/00/collision/ems{0:03}_00_colliders.rcol",
        id
    )
}

pub fn gen_collider_mapping(rcol: Rcol) -> Result<ColliderMapping> {
    let mut meat_map: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
    let mut part_map: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();

    let filter = rcol.get_monster_ride_filter();

    for attachment in rcol.group_attachments {
        if rcol.collider_groups[attachment.collider_group_index]
            .colliders
            .iter()
            .all(|collider| collider.ignore_tag_bits & filter != 0)
        {
            continue;
        }
        if let Some(data) = attachment.user_data.downcast::<EmHitDamageRsData>() {
            let entry = part_map.entry(data.parts_group.try_into()?).or_default();
            entry.insert(data.base.name.clone());
            entry.insert(attachment.name);
            entry.insert(
                rcol.collider_groups[attachment.collider_group_index]
                    .name
                    .clone(),
            );
        }
    }

    for group in rcol.collider_groups {
        for collider in group.colliders {
            if collider.ignore_tag_bits & filter != 0 {
                continue;
            }
            if let Some(data) = collider.user_data.downcast::<EmHitDamageShapeData>() {
                let entry = meat_map.entry(data.meat.try_into()?).or_default();
                entry.insert(data.base.name.clone());
            }
        }
    }

    Ok(ColliderMapping { meat_map, part_map })
}

pub fn gen_monsters(
    pak: &mut PakReader<impl Read + Seek>,
    pfb_path_gen: fn(u32) -> String,
    boss_init_path_gen: fn(u32) -> Option<String>,
    collider_path_gen: fn(u32) -> String,
    data_tune_path_gen: fn(u32) -> String,
) -> Result<Vec<Monster>> {
    let mut monsters = vec![];

    fn sub_file<T: FromRsz + 'static>(
        pak: &mut PakReader<impl Read + Seek>,
        pfb: &Pfb,
    ) -> Result<T> {
        let path = &exactly_one(
            pfb.children
                .iter()
                .filter(|child| child.hash == T::type_hash()),
        )?
        .name;
        let (index, _) = pak.find_file(path)?;
        let data = User::new(Cursor::new(pak.read_file(index)?))?;
        data.rsz.deserialize_single().context(path.clone())
    }

    for id in 0..1000 {
        let main_pfb_path = pfb_path_gen(id);
        let main_pfb_index = if let Ok((index, _)) = pak.find_file(&main_pfb_path) {
            index
        } else {
            continue;
        };
        let main_pfb = Pfb::new(Cursor::new(pak.read_file(main_pfb_index)?))?;

        let data_base = sub_file(pak, &main_pfb).context("data_base")?;
        let data_tune = {
            // not using sub_file here because some pfb also somehow reference the variantion file
            let path = data_tune_path_gen(id);
            let (index, _) = pak.find_file(&path)?;
            User::new(Cursor::new(pak.read_file(index)?))?
                .rsz
                .deserialize_single()
                .context("data_tune")?
        };
        let meat_data = sub_file(pak, &main_pfb).context("meat_data")?;
        let condition_damage_data = sub_file(pak, &main_pfb).context("condition_damage_data")?;
        let anger_data = sub_file(pak, &main_pfb).context("anger_data")?;
        let parts_break_data = sub_file(pak, &main_pfb).context("parts_break_data")?;

        let boss_init_set_data = if let Some(path) = boss_init_path_gen(id) {
            let (index, _) = pak.find_file(&path)?;
            let data = User::new(Cursor::new(pak.read_file(index)?))?;
            Some(
                data.rsz
                    .deserialize_single()
                    .context("boss_init_set_data")?,
            )
        } else {
            None
        };

        let rcol_path = collider_path_gen(id);
        let (rcol_index, _) = pak.find_file(&rcol_path)?;
        let rcol = Rcol::new(Cursor::new(pak.read_file(rcol_index)?), true).context(rcol_path)?;
        let collider_mapping = gen_collider_mapping(rcol)?;

        monsters.push(Monster {
            id,
            data_base,
            data_tune,
            meat_data,
            condition_damage_data,
            anger_data,
            parts_break_data,
            boss_init_set_data,
            collider_mapping,
        })
    }

    Ok(monsters)
}

pub fn gen_pedia(pak: &mut PakReader<impl Read + Seek>) -> Result<Pedia> {
    let monsters = gen_monsters(
        pak,
        |id| format!("enemy/em{0:03}/00/prefab/em{0:03}_00.pfb", id),
        |id| {
            Some(format!(
                "enemy/em{0:03}/00/user_data/em{0:03}_00_boss_init_set_data.user",
                id
            ))
        },
        gen_em_collider_path,
        |id| format!("enemy/em{0:03}/00/user_data/em{0:03}_00_datatune.user", id),
    )
    .context("Generating large monsters")?;

    let small_monsters = gen_monsters(
        pak,
        |id| format!("enemy/ems{0:03}/00/prefab/ems{0:03}_00.pfb", id),
        |_| None,
        gen_ems_collider_path,
        |id| {
            format!(
                "enemy/ems{0:03}/00/user_data/ems{0:03}_00_datatune.user",
                id
            )
        },
    )
    .context("Generating small monsters")?;

    let (monster_names, _) = pak.find_file("Message/Tag/Tag_EM_Name.msg")?;
    let monster_names = Msg::new(Cursor::new(pak.read_file(monster_names)?))?;

    let (monster_aliases, _) = pak.find_file("Message/Tag/Tag_EM_Name_Alias.msg")?;
    let monster_aliases = Msg::new(Cursor::new(pak.read_file(monster_aliases)?))?;

    let (index, _) = pak.find_file("enemy/user_data/system_condition_damage_preset_data.user")?;
    let condition_preset: EnemyConditionPresetData = User::new(Cursor::new(pak.read_file(index)?))?
        .rsz
        .deserialize_single()
        .context("system_condition_damage_preset_data")?;

    condition_preset.verify()?;

    Ok(Pedia {
        monsters,
        small_monsters,
        monster_names,
        monster_aliases,
        condition_preset,
    })
}

fn gen_monster_hitzones(
    pak: &mut PakReader<impl Read + Seek>,
    output: &Path,
    collider_path_gen: fn(u32) -> String,
    mesh_path_gen: fn(u32) -> String,
    meat_file_name_gen: fn(u32) -> String,
    parts_group_file_name_gen: fn(u32) -> String,
) -> Result<()> {
    let mut monsters = vec![];
    for index in 0..1000 {
        let mesh_path = mesh_path_gen(index);
        let collider_path = collider_path_gen(index);
        let mesh = if let Ok((mesh, _)) = pak.find_file(&mesh_path) {
            mesh
        } else {
            continue;
        };
        let (collider, _) = pak
            .find_file(&collider_path)
            .context("Found mesh but not collider")?;
        let mesh = pak.read_file(mesh)?;
        let collider = pak.read_file(collider)?;
        monsters.push((index, mesh, collider));
    }

    monsters
        .into_par_iter()
        .map(|(index, mesh, collider)| {
            let mesh = Mesh::new(Cursor::new(mesh))?;
            let mut collider = Rcol::new(Cursor::new(collider), true)?;
            let meat_path = output.join(meat_file_name_gen(index));
            let parts_group_path = output.join(parts_group_file_name_gen(index));
            collider.apply_skeleton(&mesh)?;
            let (vertexs, indexs) = collider.color_monster_model(&mesh)?;
            let HitzoneDiagram { meat, parts_group } = gen_hitzone_diagram(vertexs, indexs)?;
            meat.save_png(&meat_path)?;
            parts_group.save_png(&parts_group_path)?;
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;

    Ok(())
}

pub fn gen_resources(pak: &mut PakReader<impl Read + Seek>, output: &Path) -> Result<()> {
    let root = PathBuf::from(output);
    if root.exists() {
        remove_dir_all(&root)?;
    }
    create_dir(&root)?;

    gen_monster_hitzones(
        pak,
        &root,
        gen_em_collider_path,
        |id| format!("enemy/em{0:03}/00/mod/em{0:03}_00.mesh", id),
        |id| format!("em{0:03}_meat.png", id),
        |id| format!("em{0:03}_parts_group.png", id),
    )?;

    gen_monster_hitzones(
        pak,
        &root,
        gen_ems_collider_path,
        |id| format!("enemy/ems{0:03}/00/mod/ems{0:03}_00.mesh", id),
        |id| format!("ems{0:03}_meat.png", id),
        |id| format!("ems{0:03}_parts_group.png", id),
    )?;

    for index in 0..1000 {
        let icon_path = format!("gui/80_Texture/boss_icon/em{:03}_00_IAM.tex", index);
        let icon = if let Ok((icon, _)) = pak.find_file(&icon_path) {
            icon
        } else {
            continue;
        };
        let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?;
        icon.save_png(0, 0, &root.join(format!("em{0:03}_icon.png", index)))?;
    }

    for index in 0..1000 {
        let icon_path = format!("gui/80_Texture/boss_icon/ems{:03}_00_IAM.tex", index);
        let icon = if let Ok((icon, _)) = pak.find_file(&icon_path) {
            icon
        } else {
            continue;
        };
        let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?;
        icon.save_png(0, 0, &root.join(format!("ems{0:03}_icon.png", index)))?;
    }
    Ok(())
}
