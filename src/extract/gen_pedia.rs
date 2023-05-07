use super::pedia::*;
use super::prepare_map::*;
use super::sink::*;
use crate::gpu::*;
use crate::gui::*;
use crate::mesh::*;
use crate::msg::*;
use crate::pak::PakReader;
use crate::pfb::Pfb;
use crate::rcol::Rcol;
use crate::rsz::*;
use crate::tex::*;
use crate::user::User;
use crate::uvs::*;
use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read, Seek, Write};
use std::ops::Deref;

pub static ITEM_ICON_SPECIAL_COLOR: [i32; 7] = [93, 115, 121, 123, 178, 179, 189];

// This map is hardcoded in the game code, unfortunatley
// class snow.enemy.EnemyManager {.cctor}
pub static EMS_ID_LIST: &[(u32, i32)] = &[
    (0x1003, 0x2f),
    (0x1503, 0x30),
    (0x1007, 0x31),
    (0x1008, 0x32),
    (0x100d, 0x33),
    (0x100e, 0x34),
    (0x1010, 0x35),
    (0x1013, 0x36),
    (0x1019, 0x37),
    (0x101a, 0x38),
    (0x101b, 0x39),
    (0x101d, 0x3a),
    (0x1022, 0x3b),
    (0x1023, 0x3c),
    (0x1024, 0x3d),
    (0x1026, 0x3e),
    (0x1027, 0x3f),
    (0x1028, 0x40),
    (0x1029, 0x41),
    (0x102a, 0x42),
    (0x102b, 0x43),
    (0x102c, 0x44),
    (0x1031, 0x45),
    (0x1033, 0x46),
    (0x1533, 0x47),
    (0x105a, 0x48),
    (0x105b, 0x49),
    (0x155b, 0x4a),
    (0x105c, 0x4b),
    (0x1005, 0x63),
    (0x1006, 0x64),
    (0x1009, 0x65),
    (0x1014, 0x66),
    (0x1015, 0x67),
    (0x115c, 0x68),
    (0x105d, 0x69),
    (0x105e, 0x6a),
];

pub static EMS_ID_MAP: Lazy<HashMap<u32, i32>> =
    Lazy::new(|| EMS_ID_LIST.iter().cloned().collect());

fn exactly_one<T>(mut iterator: impl Iterator<Item = T>) -> Result<T> {
    let next = iterator.next().context("No element found")?;
    if iterator.next().is_some() {
        bail!("Multiple elements found");
    }
    Ok(next)
}

fn atmost_one<T>(mut iterator: impl Iterator<Item = T>) -> Result<Option<T>> {
    let Some(next) = iterator.next() else {return Ok(None)};
    if iterator.next().is_some() {
        bail!("Multiple elements found");
    }
    Ok(Some(next))
}

fn gen_em_collider_path(id: u32, sub_id: u32) -> String {
    format!("enemy/em{id:03}/{sub_id:02}/collision/em{id:03}_{sub_id:02}_colliders.rcol")
}

fn gen_ems_collider_path(id: u32, sub_id: u32) -> String {
    format!("enemy/ems{id:03}/{sub_id:02}/collision/ems{id:03}_{sub_id:02}_colliders.rcol")
}

fn gen_em_atk_collider_path(id: u32, sub_id: u32) -> String {
    format!("enemy/em{id:03}/{sub_id:02}/collision/em{id:03}_{sub_id:02}_atk_colliders.rcol")
}

fn gen_ems_atk_collider_path(id: u32, sub_id: u32) -> String {
    format!("enemy/ems{id:03}/{sub_id:02}/collision/ems{id:03}_{sub_id:02}_atk_colliders.rcol")
}

fn gen_em_shell_collider_path(id: u32, sub_id: u32) -> Vec<String> {
    let folder = format!("enemy/em{id:03}/{sub_id:02}/shell/collision/");
    let mut paths = vec![format!("{folder}em{id:03}_{sub_id:02}_shell_collider.rcol",)];

    if id == 27 && sub_id == 0 {
        paths.push(format!("{folder}em027_00_shell_collider_id_0.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_1.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_10.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_11.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_12.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_13.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_14.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_15.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_16.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_17.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_18.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_19.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_22_30.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_24.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_25.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_26.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_27.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_28.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_2_29.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_4.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_5.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_6.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_7.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_8.rcol"));
        paths.push(format!("{folder}em027_00_shell_collider_id_9.rcol"));
    }

    if id == 27 && sub_id == 8 {
        paths.push(format!("{folder}em027_08_shell_colider_nova.rcol"));
        paths.push(format!("{folder}em027_08_shell_colider_plus.rcol"));
        paths.push(format!("{folder}em027_08_shell_colider_setfire.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_0.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_1.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_10.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_11.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_12.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_13.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_14.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_15.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_16.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_17.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_18.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_19.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_22_30.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_24.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_25.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_26.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_27.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_28.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_2_29.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_4.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_5.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_6.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_7.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_8.rcol"));
        paths.push(format!("{folder}em027_08_shell_collider_id_9.rcol"));
    }

    if id == 94 && sub_id == 1 {
        paths.push(format!("{folder}em094_01_shell_collider_id_0.rcol"));
        paths.push(format!("{folder}em094_01_shell_collider_id_500.rcol"));
        paths.push(format!("{folder}em094_01_shell_collider_id_501.rcol"));
        paths.push(format!("{folder}em094_01_shell_collider_id_502.rcol"));
        paths.push(format!("{folder}em094_01_shell_collider_id_503.rcol"));
    }

    if id == 118 && sub_id == 0 {
        paths.push(format!("{folder}em118_00_shell_collider_id_0.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_1.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_10.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_100.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_2_10.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_3_10.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_5.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_6.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_7.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_8.rcol"));
        paths.push(format!("{folder}em118_00_shell_collider_id_9.rcol"));
    }

    if id == 118 && sub_id == 5 {
        paths.push(format!("{folder}em118_05_shell_collider_id_0.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_1.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_10.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_100.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_2_10.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_3_10.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_5.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_500.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_510.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_520.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_530.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_6.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_7.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_8.rcol"));
        paths.push(format!("{folder}em118_05_shell_collider_id_9.rcol"));
    }

    paths
}

fn gen_ems_shell_collider_path(id: u32, sub_id: u32) -> Vec<String> {
    vec![format!(
        "enemy/ems{id:03}/{sub_id:02}/shell/collision/ems{id:03}_{sub_id:02}_shell_collider.rcol"
    )]
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
            entry.insert(data.name.clone());
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
                entry.insert(data.name.clone());
            }
        }
    }

    Ok(ColliderMapping { meat_map, part_map })
}

#[allow(clippy::too_many_arguments)]
pub fn gen_monsters(
    pak: &mut PakReader<impl Read + Seek>,
    pfb_path_gen: fn(u32, u32) -> String,
    boss_init_path_gen: fn(u32, u32) -> Option<String>,
    collider_path_gen: fn(u32, u32) -> String,
    data_tune_path_gen: fn(u32, u32) -> String,
    atk_collider_path_gen: fn(u32, u32) -> String,
    shell_collider_path_gen: fn(u32, u32) -> Vec<String>,
    is_large: bool,
    version_hint: Option<u32>,
) -> Result<Vec<Monster>> {
    let mut monsters = vec![];

    fn sub_file<T: FromRsz + 'static>(
        pak: &mut PakReader<impl Read + Seek>,
        pfb: &Pfb,
        version_hint: Option<u32>,
    ) -> Result<T> {
        let path = &exactly_one(
            pfb.children
                .iter()
                .filter(|child| child.hash == T::type_hash()),
        )?
        .name;
        let index = pak.find_file(path)?;
        let data = User::new(Cursor::new(pak.read_file(index)?))?;
        data.rsz
            .deserialize_single(version_hint)
            .context(path.clone())
    }

    for id in 0..1000 {
        for sub_id in 0..10 {
            let main_pfb_path = pfb_path_gen(id, sub_id);
            let main_pfb_index = if let Ok(index) = pak.find_file(&main_pfb_path) {
                index
            } else {
                continue;
            };
            let main_pfb = Pfb::new(Cursor::new(pak.read_file(main_pfb_index)?))?;

            let data_base = sub_file(pak, &main_pfb, version_hint).context("data_base")?;
            let data_tune = {
                // not using sub_file here because some pfb also somehow reference the variantion file
                let path = data_tune_path_gen(id, sub_id);
                let index = pak.find_file(&path)?;
                User::new(Cursor::new(pak.read_file(index)?))?
                    .rsz
                    .deserialize_single(version_hint)
                    .context("data_tune")?
            };
            let meat_data = sub_file(pak, &main_pfb, version_hint).context("meat_data")?;
            let condition_damage_data =
                sub_file(pak, &main_pfb, version_hint).context("condition_damage_data")?;
            let anger_data = sub_file(pak, &main_pfb, version_hint).context("anger_data")?;
            let stamina_data = sub_file(pak, &main_pfb, version_hint).context("stamina_data")?;
            let parts_break_data =
                sub_file(pak, &main_pfb, version_hint).context("parts_break_data")?;

            let boss_init_set_data = if let Some(path) = boss_init_path_gen(id, sub_id) {
                if let Ok(index) = pak.find_file(&path) {
                    let data = User::new(Cursor::new(pak.read_file(index)?))?;
                    Some(
                        data.rsz
                            .deserialize_single(version_hint)
                            .context("boss_init_set_data")?,
                    )
                } else {
                    None
                }
            } else {
                None
            };

            let enemy_type = boss_init_set_data
                .as_ref()
                .map(|b: &EnemyBossInitSetData| b.enemy_type)
                .or_else(|| EMS_ID_MAP.get(&(id + (sub_id << 8) + 0x1000)).cloned());

            let rcol_path = collider_path_gen(id, sub_id);
            let rcol_index = pak.find_file(&rcol_path)?;
            let rcol =
                Rcol::new(Cursor::new(pak.read_file(rcol_index)?), true).context(rcol_path)?;
            let collider_mapping = gen_collider_mapping(rcol)?;

            let drop_item = sub_file(pak, &main_pfb, version_hint).context("drop_item")?;
            let parts_break_reward = is_large
                .then(|| sub_file(pak, &main_pfb, version_hint).context("parts_break_reward"))
                .transpose()?;

            let em_type = if is_large { EmTypes::Em } else { EmTypes::Ems }(id | (sub_id << 8));

            let mut atk_colliders = vec![];

            let mut add_atk_colliders = |rcol: Rcol| {
                let mut dedup = HashSet::new();
                for group in &rcol.group_attachments {
                    let (data, is_shell) =
                        if let Some(data) = group.user_data.downcast_ref::<EmHitAttackRsData>() {
                            (&data.base.0, false)
                        } else if let Some(data) =
                            group.user_data.downcast_ref::<EmShellHitAttackRsData>()
                        {
                            (&data.base.0, true)
                        } else {
                            continue;
                        };
                    if !dedup.insert(data) {
                        continue;
                    }
                    atk_colliders.push(AttackCollider {
                        is_shell,
                        data: data.clone(),
                    })
                }
            };

            let atk_collider_path = atk_collider_path_gen(id, sub_id);
            if let Ok(index) = pak.find_file(&atk_collider_path) {
                let rcol = Rcol::new(Cursor::new(pak.read_file(index)?), true)
                    .context(atk_collider_path)?;
                add_atk_colliders(rcol);
            } else {
                eprintln!("Attack collider file not found {atk_collider_path}")
            }

            for shell_collider_path in shell_collider_path_gen(id, sub_id) {
                if let Ok(index) = pak.find_file(&shell_collider_path) {
                    let rcol = Rcol::new(Cursor::new(pak.read_file(index)?), true)
                        .context(shell_collider_path)?;
                    add_atk_colliders(rcol);
                } else {
                    eprintln!("Shell collider file not found {shell_collider_path}")
                }
            }

            let pop_parameter = sub_file(pak, &main_pfb, version_hint).context("pop_parameter")?;

            let unique_mystery = if let Some((loader, path)) =
                atmost_one(main_pfb.children.iter().filter_map(|child| {
                    UNIQUE_MYSTERY_TYPE_MAP
                        .get(&child.hash)
                        .map(|loader| (loader, &child.name))
                }))? {
                let index = pak.find_file(path)?;
                let data = User::new(Cursor::new(pak.read_file(index)?))?;
                let data = loader(
                    data.rsz
                        .deserialize_single_any(version_hint)
                        .context(path.clone())?,
                )?;
                if data.base.condition_damage_data.len() > 1 {
                    bail!("Multiple condition damage data for mystery {main_pfb_path}")
                }
                Some(data)
            } else {
                eprintln!("Unique mystery file not found for {main_pfb_path}");
                None
            };

            let unique_over_mystery = if let Some(child) = atmost_one(
                main_pfb
                    .children
                    .iter()
                    .filter(|child| child.hash == EnemyUniqueOverMysteryData::type_hash()),
            )? {
                let path = &child.name;
                let index = pak.find_file(path)?;
                let data = User::new(Cursor::new(pak.read_file(index)?))?;
                Some(
                    data.rsz
                        .deserialize_single(version_hint)
                        .context(path.clone())?,
                )
            } else {
                None
            };

            monsters.push(Monster {
                id,
                sub_id,
                enemy_type,
                em_type,
                data_base,
                data_tune,
                meat_data,
                condition_damage_data,
                anger_data,
                stamina_data,
                parts_break_data,
                boss_init_set_data,
                collider_mapping,
                drop_item,
                parts_break_reward,
                atk_colliders,
                pop_parameter,
                unique_mystery,
                unique_over_mystery,
            })
        }
    }

    Ok(monsters)
}

fn get_msg(pak: &mut PakReader<impl Read + Seek>, path: &str) -> Result<Msg> {
    let index = pak.find_file(path)?;
    Msg::new(Cursor::new(pak.read_file(index)?))
}

fn get_user<T: 'static>(
    pak: &mut PakReader<impl Read + Seek>,
    path: &str,
    version_hint: Option<u32>,
) -> Result<T> {
    let index = pak.find_file(path)?;
    User::new(Cursor::new(pak.read_file(index)?))?
        .rsz
        .deserialize_single(version_hint)
        .with_context(|| path.to_string())
}

fn get_user_opt<T: 'static>(
    pak: &mut PakReader<impl Read + Seek>,
    path: &str,
    version_hint: Option<u32>,
) -> Result<Option<T>> {
    let index = if let Ok(index) = pak.find_file(path) {
        index
    } else {
        return Ok(None);
    };

    let user = User::new(Cursor::new(pak.read_file(index)?))?
        .rsz
        .deserialize_single(version_hint)
        .with_context(|| path.to_string())?;
    Ok(Some(user))
}

fn get_singleton<T: 'static + SingletonUser>(
    pak: &mut PakReader<impl Read + Seek>,
    version_hint: Option<u32>,
) -> Result<T> {
    Ok(T::from_rsz(get_user(pak, T::PATH, version_hint)?))
}

fn get_singleton_opt<T: 'static + SingletonUser>(
    pak: &mut PakReader<impl Read + Seek>,
    version_hint: Option<u32>,
) -> Result<Option<T>> {
    if let Some(user) = get_user_opt(pak, T::PATH, version_hint)? {
        Ok(Some(T::from_rsz(user)))
    } else {
        Ok(None)
    }
}

fn get_weapon_list<BaseData: 'static>(
    pak: &mut PakReader<impl Read + Seek>,
    weapon_class: &str,
    version_hint: Option<u32>,
) -> Result<WeaponList<BaseData>> {
    Ok(WeaponList {
        base_data: get_user(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}BaseData.user"),
            version_hint,
        )?,
        product: get_user(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}ProductData.user"),
            version_hint,
        )?,
        change: get_user(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}ChangeData.user"),
            version_hint,
        )?,
        process: get_user(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}ProcessData.user"),
            version_hint,
        )?,
        tree: get_user(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}UpdateTreeData.user"),
            version_hint,
        )?,
        overwear: get_user_opt(
            pak,
            &format!(
                "data/Define/Player/Weapon/{weapon_class}/{weapon_class}OverwearBaseData.user"
            ),
            version_hint,
        )?,
        overwear_product: get_user_opt(
            pak,
            &format!(
                "data/Define/Player/Weapon/{weapon_class}/{weapon_class}OverwearProductData.user"
            ),
            version_hint,
        )?,
        name: get_msg(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}_Name.msg"),
        )?,
        explain: get_msg(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}_Explain.msg"),
        )?,
        name_mr: get_msg(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}_Name_MR.msg"),
        )?,
        explain_mr: get_msg(
            pak,
            &format!("data/Define/Player/Weapon/{weapon_class}/{weapon_class}_Explain_MR.msg"),
        )?,
    })
}

fn get_version_hint<T: 'static + SingletonUser, U: FromRsz>(
    pak: &mut PakReader<impl Read + Seek>,
) -> Result<u32> {
    let index = pak.find_file(T::PATH)?;
    let user = User::new(Cursor::new(pak.read_file(index)?))?;
    for td in &user.rsz.type_descriptors {
        if td.hash == U::type_hash() {
            return Ok(U::VERSIONS
                .iter()
                .find(|&&(crc, _)| crc == td.crc)
                .context("Type hint CRC not found")?
                .1);
        }
    }
    bail!("Type not found for version hint")
}

pub fn gen_pedia(pak: &mut PakReader<impl Read + Seek>) -> Result<Pedia> {
    let version_hint = Some(get_version_hint::<MonsterListBossData, BossMonsterData>(
        pak,
    )?);

    fn boss_init_set_path(id: u32, sub_id: u32) -> Option<String> {
        if id == 99 && sub_id == 5 {
            // wow
            return Some(format!(
                "enemy/em{id:03}/00/user_data/em{id:03}_{sub_id:02}_boss_init_set_data.user"
            ));
        }
        Some(format!(
            "enemy/em{id:03}/{sub_id:02}/user_data/em{id:03}_{sub_id:02}_boss_init_set_data.user"
        ))
    }

    let monsters = gen_monsters(
        pak,
        |id, sub_id| format!("enemy/em{id:03}/{sub_id:02}/prefab/em{id:03}_{sub_id:02}.pfb"),
        boss_init_set_path,
        gen_em_collider_path,
        |id, sub_id| {
            format!("enemy/em{id:03}/{sub_id:02}/user_data/em{id:03}_{sub_id:02}_datatune.user")
        },
        gen_em_atk_collider_path,
        gen_em_shell_collider_path,
        true,
        version_hint,
    )
    .context("Generating large monsters")?;

    let small_monsters = gen_monsters(
        pak,
        |id, sub_id| format!("enemy/ems{id:03}/{sub_id:02}/prefab/ems{id:03}_{sub_id:02}.pfb"),
        |_, _| None,
        gen_ems_collider_path,
        |id, sub_id| {
            format!("enemy/ems{id:03}/{sub_id:02}/user_data/ems{id:03}_{sub_id:02}_datatune.user")
        },
        gen_ems_atk_collider_path,
        gen_ems_shell_collider_path,
        false,
        version_hint,
    )
    .context("Generating small monsters")?;

    let monster_names = get_msg(pak, "Message/Tag/Tag_EM_Name.msg")?;
    let monster_aliases = get_msg(pak, "Message/Tag/Tag_EM_Name_Alias.msg")?;
    let monster_explains = get_msg(pak, "Message/HunterNote/HN_MonsterListMsg.msg")?;

    let monster_names_mr = get_msg(pak, "Message/Tag_MR/Tag_EM_Name_MR.msg")?;
    let monster_aliases_mr = get_msg(pak, "Message/Tag_MR/Tag_EM_Name_Alias_MR.msg")?;
    let monster_explains_mr = get_msg(pak, "Message/HunterNote_MR/HN_MonsterListMsg_MR.msg")?;

    let condition_preset: EnemyConditionPresetData = get_singleton(pak, version_hint)?;
    condition_preset.verify()?;

    let hunter_note_msg = get_msg(pak, "Message/HunterNote/HN_Hunternote_Menu.msg")?;
    let hunter_note_msg_mr = get_msg(pak, "Message/HunterNote_MR/HN_Hunternote_Menu_MR.msg")?;

    let quest_hall_msg = get_msg(pak, "Message/Quest/QuestData_Hall.msg")?;
    let quest_hall_msg_mr = get_msg(pak, "Message/Quest/QuestData_Hall_MR.msg")?;
    let quest_hall_msg_mr2 = get_msg(pak, "Message/Quest/QuestData_Hall2_MR.msg")?;
    let quest_village_msg = get_msg(pak, "Message/Quest/QuestData_Village.msg")?;
    let quest_village_msg_mr = get_msg(pak, "Message/Quest/QuestData_Village_MR.msg")?;
    let quest_tutorial_msg = get_msg(pak, "Message/Quest/QuestData_Tutorial.msg")?;
    let quest_arena_msg = get_msg(pak, "Message/Quest/QuestData_Arena.msg")?;
    let quest_dlc_msg = get_msg(pak, "Message/Quest/QuestData_Dlc.msg")?;
    let npc_mission_msg = get_msg(pak, "Message/Quest/QuestData_NpcMission.msg")?;
    let npc_mission_msg_mr = get_msg(pak, "Message/Quest/QuestData_NpcMission_MR.msg")?;

    let armor_head_name_msg = get_msg(pak, "data/Define/Player/Armor/Head/A_Head_Name.msg")?;
    let armor_chest_name_msg = get_msg(pak, "data/Define/Player/Armor/Chest/A_Chest_Name.msg")?;
    let armor_arm_name_msg = get_msg(pak, "data/Define/Player/Armor/Arm/A_Arm_Name.msg")?;
    let armor_waist_name_msg = get_msg(pak, "data/Define/Player/Armor/Waist/A_Waist_Name.msg")?;
    let armor_leg_name_msg = get_msg(pak, "data/Define/Player/Armor/Leg/A_Leg_Name.msg")?;
    let armor_head_explain_msg = get_msg(pak, "data/Define/Player/Armor/Head/A_Head_Explain.msg")?;
    let armor_chest_explain_msg =
        get_msg(pak, "data/Define/Player/Armor/Chest/A_Chest_Explain.msg")?;
    let armor_arm_explain_msg = get_msg(pak, "data/Define/Player/Armor/Arm/A_Arm_Explain.msg")?;
    let armor_waist_explain_msg =
        get_msg(pak, "data/Define/Player/Armor/Waist/A_Waist_Explain.msg")?;
    let armor_leg_explain_msg = get_msg(pak, "data/Define/Player/Armor/Leg/A_Leg_Explain.msg")?;
    let armor_series_name_msg =
        get_msg(pak, "data/Define/Player/Armor/ArmorSeries_Hunter_Name.msg")?;

    let armor_head_name_msg_mr = get_msg(pak, "data/Define/Player/Armor/Head/A_Head_Name_MR.msg")?;
    let armor_chest_name_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Chest/A_Chest_Name_MR.msg")?;
    let armor_arm_name_msg_mr = get_msg(pak, "data/Define/Player/Armor/Arm/A_Arm_Name_MR.msg")?;
    let armor_waist_name_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Waist/A_Waist_Name_MR.msg")?;
    let armor_leg_name_msg_mr = get_msg(pak, "data/Define/Player/Armor/Leg/A_Leg_Name_MR.msg")?;
    let armor_head_explain_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Head/A_Head_Explain_MR.msg")?;
    let armor_chest_explain_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Chest/A_Chest_Explain_MR.msg")?;
    let armor_arm_explain_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Arm/A_Arm_Explain_MR.msg")?;
    let armor_waist_explain_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Waist/A_Waist_Explain_MR.msg")?;
    let armor_leg_explain_msg_mr =
        get_msg(pak, "data/Define/Player/Armor/Leg/A_Leg_Explain_MR.msg")?;
    let armor_series_name_msg_mr = get_msg(
        pak,
        "data/Define/Player/Armor/ArmorSeries_Hunter_Name_MR.msg",
    )?;

    let player_skill_detail_msg = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Detail.msg",
    )?;
    let player_skill_explain_msg = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Explain.msg",
    )?;
    let player_skill_name_msg = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Name.msg",
    )?;
    let player_skill_detail_msg_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Detail_MR.msg",
    )?;
    let player_skill_explain_msg_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Explain_MR.msg",
    )?;
    let player_skill_name_msg_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlEquipSkill/PlayerSkill_Name_MR.msg",
    )?;

    let hyakuryu_skill_name_msg = get_msg(
        pak,
        "data/Define/Player/Skill/PlHyakuryuSkill/HyakuryuSkill_Name.msg",
    )?;
    let hyakuryu_skill_explain_msg = get_msg(
        pak,
        "data/Define/Player/Skill/PlHyakuryuSkill/HyakuryuSkill_Explain.msg",
    )?;

    let hyakuryu_skill_name_msg_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlHyakuryuSkill/HyakuryuSkill_Name_MR.msg",
    )?;
    let hyakuryu_skill_explain_msg_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlHyakuryuSkill/HyakuryuSkill_Explain_MR.msg",
    )?;

    let decorations_name_msg = get_msg(
        pak,
        "data/Define/Player/Equip/Decorations/Decorations_Name.msg",
    )?;
    let decorations_name_msg_mr = get_msg(
        pak,
        "data/Define/Player/Equip/Decorations/Decorations_Name_MR.msg",
    )?;
    let hyakuryu_decos_name_msg = get_msg(
        pak,
        "data/Define/Player/Equip/HyakuryuDeco/HyakuryuDeco_Name_MR.msg",
    )?;

    let items_name_msg = get_msg(pak, "data/System/ContentsIdSystem/Item/Normal/ItemName.msg")?;
    let items_explain_msg = get_msg(
        pak,
        "data/System/ContentsIdSystem/Item/Normal/ItemExplain.msg",
    )?;
    let items_name_msg_mr = get_msg(
        pak,
        "data/System/ContentsIdSystem/Item/Normal/ItemName_MR.msg",
    )?;
    let items_explain_msg_mr = get_msg(
        pak,
        "data/System/ContentsIdSystem/Item/Normal/ItemExplain_MR.msg",
    )?;
    let material_category_msg = get_msg(
        pak,
        "data/System/ContentsIdSystem/Common/ItemCategoryType_Name.msg",
    )?;
    let material_category_msg_mr = get_msg(
        pak,
        "data/System/ContentsIdSystem/Common/ItemCategoryType_Name_MR.msg",
    )?;

    let great_sword = get_weapon_list(pak, "GreatSword", version_hint)?;
    let short_sword = get_weapon_list(pak, "ShortSword", version_hint)?;
    let hammer = get_weapon_list(pak, "Hammer", version_hint)?;
    let lance = get_weapon_list(pak, "Lance", version_hint)?;
    let long_sword = get_weapon_list(pak, "LongSword", version_hint)?;
    let slash_axe = get_weapon_list(pak, "SlashAxe", version_hint)?;
    let gun_lance = get_weapon_list(pak, "GunLance", version_hint)?;
    let dual_blades = get_weapon_list(pak, "DualBlades", version_hint)?;
    let horn = get_weapon_list(pak, "Horn", version_hint)?;
    let insect_glaive = get_weapon_list(pak, "InsectGlaive", version_hint)?;
    let charge_axe = get_weapon_list(pak, "ChargeAxe", version_hint)?;
    let light_bowgun = get_weapon_list(pak, "LightBowgun", version_hint)?;
    let heavy_bowgun = get_weapon_list(pak, "HeavyBowgun", version_hint)?;
    let bow = get_weapon_list(pak, "Bow", version_hint)?;

    let horn_melody = get_msg(pak, "data/Define/Player/Weapon/Horn/Horn_UniqueParam.msg")?;
    let horn_melody_mr = get_msg(
        pak,
        "data/Define/Player/Weapon/Horn/Horn_UniqueParam_MR.msg",
    )?;
    let weapon_series = get_msg(
        pak,
        "data/Define/Player/Weapon/WeaponSeries_Hunter_Name.msg",
    )?;
    let weapon_series_mr = get_msg(
        pak,
        "data/Define/Player/Weapon/WeaponSeries_Hunter_Name_MR.msg",
    )?;

    let maps = prepare_maps(pak)?;
    let map_name = get_msg(pak, "Message/Common_Msg/Stage_Name.msg")?;
    let map_name_mr = get_msg(pak, "Message/Common_Msg_MR/Stage_Name_MR.msg")?;

    let airou_armor_head_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Head_Name.msg",
    )?;
    let airou_armor_head_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Head_Explain.msg",
    )?;
    let airou_armor_chest_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Chest_Name.msg",
    )?;
    let airou_armor_chest_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Chest_Explain.msg",
    )?;
    let dog_armor_head_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Head_Name.msg",
    )?;
    let dog_armor_head_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Head_Explain.msg",
    )?;
    let dog_armor_chest_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Chest_Name.msg",
    )?;
    let dog_armor_chest_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Chest_Explain.msg",
    )?;
    let airou_weapon_name = get_msg(pak, "data/Define/Otomo/Equip/Weapon/OtAirouWeapon_Name.msg")?;
    let airou_weapon_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtAirouWeapon_Explain.msg",
    )?;
    let dog_weapon_name = get_msg(pak, "data/Define/Otomo/Equip/Weapon/OtDogWeapon_Name.msg")?;
    let dog_weapon_explain = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtDogWeapon_Explain.msg",
    )?;
    let airou_series_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/ArmorSeries_OtAirou_Name.msg",
    )?;
    let dog_series_name = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/ArmorSeries_OtDog_Name.msg",
    )?;

    let airou_armor_head_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Head_Name_MR.msg",
    )?;
    let airou_armor_head_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Head_Explain_MR.msg",
    )?;
    let airou_armor_chest_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Chest_Name_MR.msg",
    )?;
    let airou_armor_chest_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtAirouArmor_Chest_Explain_MR.msg",
    )?;
    let dog_armor_head_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Head_Name_MR.msg",
    )?;
    let dog_armor_head_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Head_Explain_MR.msg",
    )?;
    let dog_armor_chest_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Chest_Name_MR.msg",
    )?;
    let dog_armor_chest_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/OtDogArmor_Chest_Explain_MR.msg",
    )?;
    let airou_weapon_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtAirouWeapon_Name_MR.msg",
    )?;
    let airou_weapon_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtAirouWeapon_Explain_MR.msg",
    )?;
    let dog_weapon_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtDogWeapon_Name_MR.msg",
    )?;
    let dog_weapon_explain_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Weapon/OtDogWeapon_Explain_MR.msg",
    )?;
    let airou_series_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/ArmorSeries_OtAirou_Name_MR.msg",
    )?;
    let dog_series_name_mr = get_msg(
        pak,
        "data/Define/Otomo/Equip/Armor/ArmorSeries_OtDog_Name_MR.msg",
    )?;

    let servant_profile = get_msg(pak, "Message/Servant/ServantProfile_MR.msg")?;

    let mut random_mystery_difficulty: Option<RandomMysteryDifficultyRateListData> =
        get_singleton_opt(pak, version_hint)?;

    if let Some(rmd) = &mut random_mystery_difficulty {
        for nand_data in &mut rmd.nand_data {
            for nand_kinds_data in &mut nand_data.nand_kinds_data {
                nand_kinds_data.nando_ref_table.load(pak, version_hint)?;
            }
        }
    }

    let switch_action_name = get_msg(
        pak,
        "data/Define/Player/Skill/PlSwitchAction/PlayerSwitchAction_Name.msg",
    )?;

    let switch_action_name_mr = get_msg(
        pak,
        "data/Define/Player/Skill/PlSwitchAction/PlayerSwitchAction_Name_MR.msg",
    )?;

    let weapon_control = get_msg(pak, "Message/HunterNote/HN_WeaponControlsMsg.msg")?;

    let weapon_control_mr = get_msg(pak, "Message/HunterNote_MR/HN_WeaponControlsMsg_MR.msg")?;

    let buff_cage_name = get_msg(
        pak,
        "data/System/ContentsIdSystem/LvBuffCage/Normal/LvBuffCage_Name.msg",
    )?;

    let buff_cage_explain = get_msg(
        pak,
        "data/System/ContentsIdSystem/LvBuffCage/Normal/LvBuffCage_Explain.msg",
    )?;

    let dlc_name = get_msg(pak, "Message/DLC/DLC_Name.msg")?;
    let dlc_name_mr = get_msg(pak, "Message/DLC/DLC_Name_MR.msg")?;
    let dlc_explain = get_msg(pak, "Message/DLC/DLC_Explain.msg")?;
    let dlc_explain_mr = get_msg(pak, "Message/DLC/DLC_Explain_MR.msg")?;

    let award_name = get_msg(pak, "Message/GuildCard/GC_Award_Name.msg")?;
    let award_name_mr = get_msg(pak, "Message/GuildCard/GC_Award_Name_MR.msg")?;
    let award_explain = get_msg(pak, "Message/GuildCard/GC_Award_Explain.msg")?;
    let award_explain_mr = get_msg(pak, "Message/GuildCard/GC_Award_Explain_MR.msg")?;

    let achievement_name = get_msg(pak, "Message/GuildCard/GC_Achievement_Name.msg")?;
    let achievement_name_mr = get_msg(pak, "Message/GuildCard/GC_Achievement_Name_MR.msg")?;
    let achievement_explain = get_msg(pak, "Message/GuildCard/GC_Achievement_Explain.msg")?;
    let achievement_explain_mr = get_msg(pak, "Message/GuildCard/GC_Achievement_Explain_MR.msg")?;

    Ok(Pedia {
        monsters,
        small_monsters,
        monster_names,
        monster_aliases,
        monster_explains,
        monster_names_mr,
        monster_aliases_mr,
        monster_explains_mr,
        condition_preset,
        monster_list: get_singleton(pak, version_hint)?,
        hunter_note_msg,
        hunter_note_msg_mr,
        material_category_msg_mr,
        monster_lot: get_singleton(pak, version_hint)?,
        monster_lot_mr: get_singleton(pak, version_hint)?,
        parts_type: get_singleton(pak, version_hint)?,
        normal_quest_data: get_singleton(pak, version_hint)?,
        normal_quest_data_mr: get_singleton(pak, version_hint)?,
        normal_quest_data_for_enemy: get_singleton(pak, version_hint)?,
        normal_quest_data_for_enemy_mr: get_singleton(pak, version_hint)?,
        dl_quest_data: get_singleton(pak, version_hint)?,
        dl_quest_data_for_enemy: get_singleton(pak, version_hint)?,
        dl_quest_data_mr: get_singleton_opt(pak, version_hint)?,
        dl_quest_data_for_enemy_mr: get_singleton_opt(pak, version_hint)?,
        difficulty_rate: get_singleton(pak, version_hint)?,
        difficulty_rate_anomaly: get_singleton_opt(pak, version_hint)?,
        random_scale: get_singleton(pak, version_hint)?,
        size_list: get_singleton(pak, version_hint)?,
        discover_em_set_data: get_singleton(pak, version_hint)?,
        quest_data_for_reward: get_singleton(pak, version_hint)?,
        quest_data_for_reward_mr: get_singleton(pak, version_hint)?,
        reward_id_lot_table: get_singleton(pak, version_hint)?,
        reward_id_lot_table_mr: get_singleton(pak, version_hint)?,
        main_target_reward_lot_num: get_singleton(pak, version_hint)?,
        fixed_hyakuryu_quest: get_singleton(pak, version_hint)?,
        mystery_reward_item: get_singleton(pak, version_hint)?,
        quest_servant: get_singleton(pak, version_hint)?,
        supply_data: get_singleton(pak, version_hint)?,
        supply_data_mr: get_singleton(pak, version_hint)?,
        arena_quest: get_singleton(pak, version_hint)?,
        quest_unlock: get_singleton(pak, version_hint)?,
        time_attack_reward: get_singleton(pak, version_hint)?,
        talk_condition_quest_list: get_singleton(pak, version_hint)?,
        npc_mission: get_singleton(pak, version_hint)?,
        npc_mission_mr: get_singleton(pak, version_hint)?,
        quest_hall_msg,
        quest_hall_msg_mr,
        quest_hall_msg_mr2,
        quest_village_msg,
        quest_village_msg_mr,
        quest_tutorial_msg,
        quest_arena_msg,
        quest_dlc_msg,
        npc_mission_msg,
        npc_mission_msg_mr,
        armor: get_singleton(pak, version_hint)?,
        armor_series: get_singleton(pak, version_hint)?,
        armor_product: get_singleton(pak, version_hint)?,
        overwear: get_singleton(pak, version_hint)?,
        overwear_product: get_singleton(pak, version_hint)?,
        armor_buildup: get_singleton(pak, version_hint)?,
        armor_pair: get_singleton(pak, version_hint)?,
        armor_head_name_msg,
        armor_chest_name_msg,
        armor_arm_name_msg,
        armor_waist_name_msg,
        armor_leg_name_msg,
        armor_head_explain_msg,
        armor_chest_explain_msg,
        armor_arm_explain_msg,
        armor_waist_explain_msg,
        armor_leg_explain_msg,
        armor_series_name_msg,
        armor_head_name_msg_mr,
        armor_chest_name_msg_mr,
        armor_arm_name_msg_mr,
        armor_waist_name_msg_mr,
        armor_leg_name_msg_mr,
        armor_head_explain_msg_mr,
        armor_chest_explain_msg_mr,
        armor_arm_explain_msg_mr,
        armor_waist_explain_msg_mr,
        armor_leg_explain_msg_mr,
        armor_series_name_msg_mr,
        equip_skill: get_singleton(pak, version_hint)?,
        player_skill_detail_msg,
        player_skill_explain_msg,
        player_skill_name_msg,
        player_skill_detail_msg_mr,
        player_skill_explain_msg_mr,
        player_skill_name_msg_mr,
        hyakuryu_skill: get_singleton(pak, version_hint)?,
        hyakuryu_skill_recipe: get_singleton(pak, version_hint)?,
        hyakuryu_skill_name_msg,
        hyakuryu_skill_explain_msg,
        hyakuryu_skill_name_msg_mr,
        hyakuryu_skill_explain_msg_mr,
        decorations: get_singleton(pak, version_hint)?,
        decorations_product: get_singleton(pak, version_hint)?,
        decorations_name_msg,
        decorations_name_msg_mr,
        hyakuryu_decos: get_singleton(pak, version_hint)?,
        hyakuryu_decos_product: get_singleton(pak, version_hint)?,
        hyakuryu_decos_name_msg,
        //alchemy_pattern: get_singleton(pak,version_hint)?,
        alchemy_pl_skill: get_singleton(pak, version_hint)?,
        /*alchemy_grade_worth: get_singleton(pak,version_hint)?,
        alchemy_rare_type: get_singleton(pak,version_hint)?,
        alchemy_second_skill_lot: get_singleton(pak,version_hint)?,
        alchemy_skill_grade_lot: get_singleton(pak,version_hint)?,
        alchemy_slot_num: get_singleton(pak,version_hint)?,
        alchemy_slot_worth: get_singleton(pak,version_hint)?,*/
        items: get_singleton(pak, version_hint)?,
        items_name_msg,
        items_explain_msg,
        items_name_msg_mr,
        items_explain_msg_mr,
        material_category_msg,
        great_sword,
        short_sword,
        hammer,
        lance,
        long_sword,
        slash_axe,
        gun_lance,
        dual_blades,
        horn,
        insect_glaive,
        charge_axe,
        light_bowgun,
        heavy_bowgun,
        bow,
        horn_melody,
        horn_melody_mr,
        hyakuryu_weapon_buildup: get_singleton(pak, version_hint)?,
        weapon_chaos_critical: get_singleton_opt(pak, version_hint)?,
        weapon_series,
        weapon_series_mr,
        maps,
        map_name,
        map_name_mr,
        item_pop_lot: get_singleton(pak, version_hint)?,
        airou_armor: get_singleton(pak, version_hint)?,
        airou_armor_product: get_singleton(pak, version_hint)?,
        dog_armor: get_singleton(pak, version_hint)?,
        dog_armor_product: get_singleton(pak, version_hint)?,
        airou_weapon: get_singleton(pak, version_hint)?,
        airou_weapon_product: get_singleton(pak, version_hint)?,
        dog_weapon: get_singleton(pak, version_hint)?,
        dog_weapon_product: get_singleton(pak, version_hint)?,
        ot_equip_series: get_singleton(pak, version_hint)?,
        airou_overwear: get_singleton(pak, version_hint)?,
        dog_overwear: get_singleton(pak, version_hint)?,
        ot_overwear_recipe: get_singleton(pak, version_hint)?,
        airou_armor_head_name,
        airou_armor_head_explain,
        airou_armor_chest_name,
        airou_armor_chest_explain,
        dog_armor_head_name,
        dog_armor_head_explain,
        dog_armor_chest_name,
        dog_armor_chest_explain,
        airou_weapon_name,
        airou_weapon_explain,
        dog_weapon_name,
        dog_weapon_explain,
        airou_series_name,
        dog_series_name,
        airou_armor_head_name_mr,
        airou_armor_head_explain_mr,
        airou_armor_chest_name_mr,
        airou_armor_chest_explain_mr,
        dog_armor_head_name_mr,
        dog_armor_head_explain_mr,
        dog_armor_chest_name_mr,
        dog_armor_chest_explain_mr,
        airou_weapon_name_mr,
        airou_weapon_explain_mr,
        dog_weapon_name_mr,
        dog_weapon_explain_mr,
        airou_series_name_mr,
        dog_series_name_mr,
        servant_profile,
        custom_buildup_base: get_singleton_opt(pak, version_hint)?,
        custom_buildup_armor_open: get_singleton_opt(pak, version_hint)?,
        custom_buildup_weapon_open: get_singleton_opt(pak, version_hint)?,
        custom_buildup_armor_material: get_singleton_opt(pak, version_hint)?,
        custom_buildup_weapon_material: get_singleton_opt(pak, version_hint)?,
        custom_buildup_armor_lot: get_singleton_opt(pak, version_hint)?,
        custom_buildup_armor_category_lot: get_singleton_opt(pak, version_hint)?,
        custom_buildup_equip_skill_detail: get_singleton_opt(pak, version_hint)?,
        custom_buildup_wep_table: get_singleton_opt(pak, version_hint)?,
        custom_buildup_slot_bonus: get_singleton_opt(pak, version_hint)?,
        random_mystery_difficulty,
        random_mystery_enemy: get_singleton_opt(pak, version_hint)?,
        random_mystery_rank_release: get_singleton_opt(pak, version_hint)?,
        random_mystery_reward_base: get_singleton_opt(pak, version_hint)?,
        random_mystery_reward_subtarget: get_singleton_opt(pak, version_hint)?,
        progress: get_singleton(pak, version_hint)?,
        enemy_rank: get_singleton(pak, version_hint)?,
        species: get_singleton(pak, version_hint)?,
        switch_action_name,
        switch_action_name_mr,
        weapon_control,
        weapon_control_mr,
        buff_cage: get_singleton(pak, version_hint)?,
        buff_cage_name,
        buff_cage_explain,
        item_shop: get_singleton(pak, version_hint)?,
        item_shop_lot: get_singleton(pak, version_hint)?,
        fukudama: get_singleton(pak, version_hint)?,
        mystery_labo_trade_item: get_singleton_opt(pak, version_hint)?,
        item_mix: get_singleton(pak, version_hint)?,
        bbq: get_singleton(pak, version_hint)?,
        exchange_item: get_singleton(pak, version_hint)?,
        trade_dust: get_singleton(pak, version_hint)?,
        trade_feature: get_singleton(pak, version_hint)?,
        trade_rare: get_singleton(pak, version_hint)?,
        trade: get_singleton(pak, version_hint)?,
        spy: get_singleton(pak, version_hint)?,
        offcut_convert: get_singleton(pak, version_hint)?,

        dlc: get_singleton(pak, version_hint)?,
        dlc_add: get_singleton(pak, version_hint)?,
        item_pack: get_singleton(pak, version_hint)?,
        slc_item_pack: get_singleton(pak, version_hint)?,
        dlc_name,
        dlc_name_mr,
        dlc_explain,
        dlc_explain_mr,

        system_mystery: get_singleton(pak, version_hint)?,
        system_mario: get_singleton(pak, version_hint)?,

        award: get_singleton(pak, version_hint)?,
        award_name,
        award_name_mr,
        award_explain,
        award_explain_mr,

        achievement: get_singleton(pak, version_hint)?,
        achievement_name,
        achievement_name_mr,
        achievement_explain,
        achievement_explain_mr,
    })
}

fn gen_monster_hitzones(
    pak: &mut PakReader<impl Read + Seek>,
    output: &impl Sink,
    collider_path_gen: fn(u32, u32) -> String,
    mesh_path_gen: fn(u32, u32) -> String,
    meat_file_name_gen: fn(u32, u32) -> String,
    parts_group_file_name_gen: fn(u32, u32) -> String,
) -> Result<()> {
    let mut monsters = vec![];
    for index in 0..1000 {
        for sub_id in 0..10 {
            let mesh_path = mesh_path_gen(index, sub_id);
            let collider_path = collider_path_gen(index, sub_id);
            let mesh = if let Ok(mesh) = pak.find_file(&mesh_path) {
                mesh
            } else {
                continue;
            };
            let collider = if let Ok(collider) = pak.find_file(&collider_path) {
                collider
            } else {
                continue;
            };
            let mesh = pak.read_file(mesh)?;
            let collider = pak.read_file(collider)?;
            monsters.push((index, sub_id, mesh, collider));
        }
    }

    monsters
        .into_par_iter()
        .map(|(index, sub_id, mesh, collider)| {
            let mesh = Mesh::new(Cursor::new(mesh))?;
            let mut collider = Rcol::new(Cursor::new(collider), true)?;

            // pre-check a glitchy thing
            for attachment in &collider.group_attachments {
                if let Some(data) = attachment.user_data.downcast_ref::<EmHitDamageRsData>() {
                    if data.parent_user_data.is_none() {
                        eprintln!(
                            "Found glitch collider '{}' for em{}_{}",
                            data.name, index, sub_id
                        );
                    }
                }
            }

            if collider.get_special_ammo_filter() != 0 {
                eprintln!("Found special ammo collider for em{index}_{sub_id}");
            }

            let meat_path = output.create(&meat_file_name_gen(index, sub_id))?;
            let parts_group_path = output.create(&parts_group_file_name_gen(index, sub_id))?;
            collider.apply_skeleton(&mesh)?;
            let (vertexs, indexs) = collider.color_monster_model(&mesh)?;
            let HitzoneDiagram { meat, parts_group } = gen_hitzone_diagram(vertexs, indexs)?;
            meat.save_png(meat_path)?;
            parts_group.save_png(parts_group_path)?;
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;

    Ok(())
}

// Monsters in title updates have icon files with special names. It is hard to
// get the name mapping without hard-coding it here.
// Icon file names, including normal ones and special ones, are referred in
// gui/01_Common/boss_icon.gui, but they have their own order in the frame
// sequence there. The mapping from EM ID to frame ID is probably done by
// snow.gui.SnowGuiCommonUtility.Icon.getEnemyIconFrame, which would be
// hard-coded in game code.
static EM_ICON_MAP: Lazy<HashMap<(i32, i32), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // This is not applicable in PC update anymore
    /*
    m.insert((24, 0), "A0");
    m.insert((25, 0), "B1");
    m.insert((27, 0), "C2");
    m.insert((86, 5), "D3");
    m.insert((118, 0), "E4");
    // F5?
    m.insert((2, 7), "G6");
    m.insert((7, 7), "H7");
    m.insert((57, 7), "I8");
    // J9?
    // KA?
    m.insert((99, 5), "LB");
    */

    // Except... they did a oopsie on pc
    m.insert((86, 5), "em086_00");
    m
});

fn save_spriter(
    tex: &RgbaImage,
    group: &SpriterGroup,
    index: usize,
    path: &str,
    output: &impl Sink,
) -> Result<()> {
    let spriter = group.spriters.get(index).context("Broken UV group")?;
    if spriter.p0.x == spriter.p1.x || spriter.p0.y == spriter.p1.y {
        return Ok(());
    }
    tex.sub_image_f(spriter.p0, spriter.p1)?
        .save_png(output.create(path)?)?;
    Ok(())
}

pub fn gen_resources(pak: &mut PakReader<impl Read + Seek>, output: &impl Sink) -> Result<()> {
    let mesh_path_gen = |id, mut sub_id| {
        if id == 99 && sub_id == 5 {
            sub_id = 0;
        }
        format!("enemy/em{id:03}/{sub_id:02}/mod/em{id:03}_{sub_id:02}.mesh")
    };

    gen_monster_hitzones(
        pak,
        output,
        gen_em_collider_path,
        mesh_path_gen,
        |id, sub_id| format!("em{id:03}_{sub_id:02}_meat.png"),
        |id, sub_id| format!("em{id:03}_{sub_id:02}_parts_group.png"),
    )?;

    gen_monster_hitzones(
        pak,
        output,
        gen_ems_collider_path,
        |id, sub_id| format!("enemy/ems{id:03}/{sub_id:02}/mod/ems{id:03}_{sub_id:02}.mesh"),
        |id, sub_id| format!("ems{id:03}_{sub_id:02}_meat.png"),
        |id, sub_id| format!("ems{id:03}_{sub_id:02}_parts_group.png"),
    )?;

    for index in 0..1000 {
        for sub_id in 0..10 {
            let icon_path = if let Some(name) = EM_ICON_MAP.get(&(index, sub_id)) {
                format!("gui/80_Texture/boss_icon/{name}_IAM.tex")
            } else {
                format!("gui/80_Texture/boss_icon/em{index:03}_{sub_id:02}_IAM.tex")
            };
            let icon = if let Ok(icon) = pak.find_file(&icon_path) {
                icon
            } else {
                continue;
            };
            let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?;
            icon.save_png(
                0,
                0,
                output.create(&format!("em{index:03}_{sub_id:02}_icon.png"))?,
            )?;
        }
    }

    for index in 0..1000 {
        for sub_id in 0..10 {
            let icon_path = format!("gui/80_Texture/boss_icon/ems{index:03}_{sub_id:02}_IAM.tex");
            let icon = if let Ok(icon) = pak.find_file(&icon_path) {
                icon
            } else {
                continue;
            };
            let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?;
            icon.save_png(
                0,
                0,
                output.create(&format!("ems{index:03}_{sub_id:02}_icon.png"))?,
            )?;
        }
    }

    let icon_uvs = pak.find_file("gui/70_UVSequence/boss_icon_mini.uvs")?;
    let icon_uvs = Uvs::new(Cursor::new(pak.read_file(icon_uvs)?))?;
    if icon_uvs.textures.is_empty() || icon_uvs.spriter_groups.is_empty() {
        bail!("Broken boss_icon_mini.uvs");
    }
    let icon = pak.find_file(&icon_uvs.textures[0].path)?;
    let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?.to_rgba(0, 0)?;
    let spriters = &icon_uvs.spriter_groups[0];
    save_spriter(&icon, spriters, 40, "em131_00_icon.png", output)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_ore_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy0.png")?)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_bone_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy1.png")?)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_herbs_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy2.png")?)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_mushroom_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy3.png")?)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_fish_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy4.png")?)?;

    let path = pak.find_file("gui/80_Texture/boss_icon/em_spy_insect_IAM.tex")?;
    Tex::new(Cursor::new(pak.read_file(path)?))?.save_png(0, 0, output.create("spy5.png")?)?;

    let guild_card = pak.find_file("gui/80_Texture/GuildCard_IAM.tex")?;
    let guild_card = Tex::new(Cursor::new(pak.read_file(guild_card)?))?.to_rgba(0, 0)?;

    guild_card
        .sub_image(302, 397, 24, 24)?
        .save_png(output.create("king_crown.png")?)?;

    guild_card
        .sub_image(302, 424, 24, 24)?
        .save_png(output.create("large_crown.png")?)?;

    guild_card
        .sub_image(302, 453, 24, 24)?
        .save_png(output.create("small_crown.png")?)?;

    let map_icon = pak.find_file("gui/80_Texture/map/map_icon_IAM.tex")?;
    let map_icon = Tex::new(Cursor::new(pak.read_file(map_icon)?))?.to_rgba(0, 0)?;
    map_icon
        .sub_image(0, 31, 31, 33)?
        .save_png(output.create("main_camp.png")?)?;
    map_icon
        .sub_image(0, 64, 31, 30)?
        .save_png(output.create("sub_camp.png")?)?;

    let map_icon = pak.find_file("gui/80_Texture/map/map_icon02_MR_IAM.tex")?;
    let map_icon = Tex::new(Cursor::new(pak.read_file(map_icon)?))?.to_rgba(0, 0)?;
    map_icon
        .sub_image(35, 68, 32, 32)?
        .save_png(output.create("recon.png")?)?;

    let item_icon_path = output.sub_sink("item")?;

    let item_icon_files = [
        ("gui/70_UVSequence/cmn_icon.uvs", 0, 200),
        ("gui/70_UVSequence/cmn_icon_MR.uvs", 200, usize::MAX),
    ];

    for (file, offset, max_i) in item_icon_files {
        let item_icon_uvs = pak.find_file(file)?;
        let item_icon_uvs = Uvs::new(Cursor::new(pak.read_file(item_icon_uvs)?))?;
        if item_icon_uvs.textures.len() != 1 || item_icon_uvs.spriter_groups.len() != 1 {
            bail!("Broken {file}");
        }
        let item_icon = pak.find_file(&item_icon_uvs.textures[0].path)?;
        let item_icon = Tex::new(Cursor::new(pak.read_file(item_icon)?))?.to_rgba(0, 0)?;
        for (i, spriter) in item_icon_uvs.spriter_groups[0].spriters.iter().enumerate() {
            if i >= max_i {
                break;
            }
            let i = i + offset;
            let item_icon = item_icon.sub_image_f(spriter.p0, spriter.p1)?;

            if ITEM_ICON_SPECIAL_COLOR.contains(&(i as i32)) {
                item_icon.save_png(item_icon_path.create(&format!("{i:03}.png"))?)?;
            } else {
                let (item_icon_r, item_icon_a) = item_icon.gen_double_mask();
                item_icon_r.save_png(item_icon_path.create(&format!("{i:03}.r.png"))?)?;
                item_icon_a.save_png(item_icon_path.create(&format!("{i:03}.a.png"))?)?;
            }
        }
    }

    let item_addon_uvs = pak.find_file("gui/70_UVSequence/Item_addonicon.uvs")?;
    let item_addon_uvs = Uvs::new(Cursor::new(pak.read_file(item_addon_uvs)?))?;
    if item_addon_uvs.textures.len() != 1 || item_addon_uvs.spriter_groups.len() != 1 {
        bail!("Broken item_addon.uvs");
    }
    let item_addon = pak.find_file(&item_addon_uvs.textures[0].path)?;
    let item_addon = Tex::new(Cursor::new(pak.read_file(item_addon)?))?.to_rgba(0, 0)?;
    for (i, spriter) in item_addon_uvs.spriter_groups[0].spriters.iter().enumerate() {
        item_addon
            .sub_image_f(spriter.p0, spriter.p1)?
            .save_png(output.create(&format!("item_addon_{i}.png"))?)?;
    }

    let message_window_uvs = pak.find_file("gui/70_UVSequence/message_window.uvs")?;
    let message_window_uvs = Uvs::new(Cursor::new(pak.read_file(message_window_uvs)?))?;
    if message_window_uvs.textures.len() != 1 || message_window_uvs.spriter_groups.len() != 1 {
        bail!("Broken message_window.uvs");
    }
    let message_window = pak.find_file(&message_window_uvs.textures[0].path)?;
    let message_window = Tex::new(Cursor::new(pak.read_file(message_window)?))?.to_rgba(0, 0)?;
    let skill_icon = message_window_uvs.spriter_groups[0]
        .spriters
        .get(170)
        .context("Skill icon not found")?;
    let (skill_r, skill_a) = message_window
        .sub_image_f(skill_icon.p0, skill_icon.p1)?
        .gen_double_mask();
    skill_r.save_png(output.create("skill.r.png")?)?;
    skill_a.save_png(output.create("skill.a.png")?)?;

    let rskill_icon = message_window_uvs.spriter_groups[0]
        .spriters
        .get(172)
        .context("Rampage skill icon not found")?;
    let (rskill_r, rskill_a) = message_window
        .sub_image_f(rskill_icon.p0, rskill_icon.p1)?
        .gen_double_mask();
    rskill_r.save_png(output.create("rskill.r.png")?)?;
    rskill_a.save_png(output.create("rskill.a.png")?)?;

    let equip_icon_path = output.sub_sink("equip")?;
    let equip_icon_uvs = pak.find_file("gui/70_UVSequence/EquipIcon.uvs")?;
    let equip_icon_uvs = Uvs::new(Cursor::new(pak.read_file(equip_icon_uvs)?))?;
    if equip_icon_uvs.textures.len() != 2 || equip_icon_uvs.spriter_groups.len() != 2 {
        bail!("Broken EquipIcon.uvs");
    }
    let equip_icon = pak.find_file(&equip_icon_uvs.textures[0].path)?;
    let equip_icon = Tex::new(Cursor::new(pak.read_file(equip_icon)?))?.to_rgba(0, 0)?;
    for (i, spriter) in equip_icon_uvs.spriter_groups[0].spriters.iter().enumerate() {
        let (equip_icon_r, equip_icon_a) = equip_icon
            .sub_image_f(spriter.p0, spriter.p1)?
            .gen_double_mask();
        equip_icon_r.save_png(equip_icon_path.create(&format!("{i:03}.r.png"))?)?;
        equip_icon_a.save_png(equip_icon_path.create(&format!("{i:03}.a.png"))?)?;
    }

    let icon_uvs = pak.find_file("gui/70_UVSequence/state_icon.uvs")?;
    let icon_uvs = Uvs::new(Cursor::new(pak.read_file(icon_uvs)?))?;
    if icon_uvs.textures.is_empty() || icon_uvs.spriter_groups.is_empty() {
        bail!("Broken state_icon.uvs");
    }
    let icon = pak.find_file(&icon_uvs.textures[0].path)?;
    let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?.to_rgba(0, 0)?;
    let spriters = &icon_uvs.spriter_groups[0];
    save_spriter(&icon, spriters, 3, "fire.png", output)?;
    save_spriter(&icon, spriters, 4, "water.png", output)?;
    save_spriter(&icon, spriters, 5, "thunder.png", output)?;
    save_spriter(&icon, spriters, 6, "ice.png", output)?;
    save_spriter(&icon, spriters, 7, "dragon.png", output)?;
    save_spriter(&icon, spriters, 8, "poison.png", output)?;
    save_spriter(&icon, spriters, 9, "noxious.png", output)?;
    save_spriter(&icon, spriters, 10, "para.png", output)?;
    save_spriter(&icon, spriters, 11, "stun.png", output)?;
    save_spriter(&icon, spriters, 12, "sleep.png", output)?;
    save_spriter(&icon, spriters, 13, "blast.png", output)?;
    save_spriter(&icon, spriters, 14, "bubble.png", output)?;
    save_spriter(&icon, spriters, 15, "bubblel.png", output)?;
    save_spriter(&icon, spriters, 16, "attackup.png", output)?;
    save_spriter(&icon, spriters, 18, "defencedown.png", output)?;
    save_spriter(&icon, spriters, 20, "resdown.png", output)?;
    save_spriter(&icon, spriters, 24, "oni.png", output)?;
    save_spriter(&icon, spriters, 25, "dung.png", output)?;
    save_spriter(&icon, spriters, 30, "capture.png", output)?; //?
    save_spriter(&icon, spriters, 52, "heal.png", output)?;
    save_spriter(&icon, spriters, 70, "exhaust.png", output)?;
    save_spriter(&icon, spriters, 76, "steelfang.png", output)?;

    let icon_uvs = pak.find_file("gui/70_UVSequence/state_icon_MR.uvs")?;
    let icon_uvs = Uvs::new(Cursor::new(pak.read_file(icon_uvs)?))?;
    if icon_uvs.textures.is_empty() || equip_icon_uvs.spriter_groups.is_empty() {
        bail!("Broken state_icon_MR.uvs");
    }
    let icon = pak.find_file(&icon_uvs.textures[0].path)?;
    let icon = Tex::new(Cursor::new(pak.read_file(icon)?))?.to_rgba(0, 0)?;
    let spriters = &icon_uvs.spriter_groups[0];
    save_spriter(&icon, spriters, 0, "bleed.png", output)?;
    save_spriter(&icon, spriters, 9, "blood.png", output)?;
    save_spriter(&icon, spriters, 22, "frenzy.png", output)?;

    let icon_uvs = pak.find_file("gui/70_UVSequence/Arms_addonicon_MR.uvs")?;
    let icon_uvs = Uvs::new(Cursor::new(pak.read_file(icon_uvs)?))?;
    if icon_uvs.textures.is_empty() || equip_icon_uvs.spriter_groups.is_empty() {
        bail!("Broken Arms_addonicon_MR.uvs");
    }
    let equip_icon = pak.find_file(&icon_uvs.textures[0].path)?;
    let icon = Tex::new(Cursor::new(pak.read_file(equip_icon)?))?.to_rgba(0, 0)?;
    let spriter = icon_uvs.spriter_groups[0]
        .spriters
        .get(0)
        .context("Broken Arms_addonicon_MR.uvs")?;
    icon.sub_image_f(spriter.p0, spriter.p1)?
        .save_png(output.create("afflicted.png")?)?;

    let common_uvs = pak.find_file("gui/70_UVSequence/common.uvs")?;
    let common_uvs = Uvs::new(Cursor::new(pak.read_file(common_uvs)?))?;
    if common_uvs.textures.len() != 1 || common_uvs.spriter_groups.len() != 2 {
        bail!("Broken common.uvs");
    }
    let common = pak.find_file(&common_uvs.textures[0].path)?;
    let common = Tex::new(Cursor::new(pak.read_file(common)?))?.to_rgba(0, 0)?;
    for (i, spriter) in common_uvs.spriter_groups[1].spriters.iter().enumerate() {
        common
            .sub_image_f(spriter.p0, spriter.p1)?
            .save_png(output.create(&format!("questtype_{i}.png"))?)?;
    }

    let common_uvs = pak.find_file("gui/70_UVSequence/Slot_Icon.uvs")?;
    let common_uvs = Uvs::new(Cursor::new(pak.read_file(common_uvs)?))?;
    if common_uvs.textures.len() != 1 || common_uvs.spriter_groups.len() != 1 {
        bail!("Broken Slot_Icon.uvs");
    }
    let common = pak.find_file(&common_uvs.textures[0].path)?;
    let common = Tex::new(Cursor::new(pak.read_file(common)?))?.to_rgba(0, 0)?;
    for (i, spriter) in common_uvs.spriter_groups[0]
        .spriters
        .iter()
        .enumerate()
        .take(3)
    {
        common
            .sub_image_f(spriter.p0, spriter.p1)?
            .save_png(output.create(&format!("slot_{i}.png"))?)?;
    }

    let common_uvs = pak.find_file("gui/70_UVSequence/Slot_Icon_MR.uvs")?;
    let common_uvs = Uvs::new(Cursor::new(pak.read_file(common_uvs)?))?;
    if common_uvs.textures.len() != 1 || common_uvs.spriter_groups.len() != 1 {
        bail!("Broken Slot_Icon_MR.uvs");
    }
    let common = pak.find_file(&common_uvs.textures[0].path)?;
    let common = Tex::new(Cursor::new(pak.read_file(common)?))?.to_rgba(0, 0)?;
    let spriter = common_uvs.spriter_groups[0]
        .spriters
        .get(1)
        .context("Broken Slot_Icon_MR.uvs: no 4-slot")?;
    common
        .sub_image_f(spriter.p0, spriter.p1)?
        .save_png(output.create("slot_3.png")?)?;
    let spriter = common_uvs.spriter_groups[0]
        .spriters
        .get(9)
        .context("Broken Slot_Icon_MR.uvs: no rampage slot icon")?;
    common
        .sub_image_f(spriter.p0, spriter.p1)?
        .save_png(output.create("slot_rampage.png")?)?;

    let award_file = [
        "gui/70_UVSequence/Award_Icon.uvs",
        "gui/70_UVSequence/Award_Icon_MR.uvs",
    ];
    let mut award_i = 0;
    for file in award_file {
        let award = pak.find_file(file)?;
        let award = Uvs::new(Cursor::new(pak.read_file(award)?))?;
        if award.textures.len() != 1 || award.spriter_groups.len() != 1 {
            bail!("Broken {}", file);
        }
        let tex = pak.find_file(&award.textures[0].path)?;
        let tex = Tex::new(Cursor::new(pak.read_file(tex)?))?.to_rgba(0, 0)?;
        for spriter in &award.spriter_groups[0].spriters {
            tex.sub_image_f(spriter.p0, spriter.p1)?
                .save_png(output.create(&format!("award_{award_i}.png"))?)?;
            award_i += 1;
        }
    }

    let item_colors_path = output.create("item_color.css")?;
    gen_item_colors(pak, item_colors_path)?;

    let item_colors_path = output.create("rarity_color.css")?;
    gen_rarity_colors(pak, item_colors_path)?;

    gen_map_resource(pak, output)?;

    Ok(())
}

fn gen_gui_colors(
    pak: &mut PakReader<impl Read + Seek>,
    mut file: impl Write,
    gui: &str,
    control_name: &str,
    prefix: &str,
    css_prefix: &str,
) -> Result<()> {
    let item_icon_gui = pak.find_file(gui)?;
    let item_icon_gui = Gui::new(Cursor::new(pak.read_file(item_icon_gui)?))?;
    let item_icon_color = item_icon_gui
        .controls
        .iter()
        .find(|control| control.name == control_name)
        .context("Control not found")?;

    fn color_tran(value: u64) -> Result<u8> {
        let value = f64::from_bits(value);
        if !(0.0..=1.0).contains(&value) {
            bail!("Bad color value");
        }
        // linear RGB to sRGB
        Ok((value.powf(1.0 / 2.2) * 255.0).round() as u8)
    }

    for clips in &item_icon_color.clips {
        if !clips.name.starts_with(prefix) {
            continue;
        }
        let id: u32 = clips.name[prefix.len()..].parse()?;
        if clips.variable_values.len() != 3 && clips.variable_values.len() != 7 {
            bail!(
                "Unexpected variable values len {} for {}",
                clips.variable_values.len(),
                clips.name
            );
        }

        // Alternative color? the only difference happened in item color 6.
        // The second one looks correct
        let offset = if clips.variable_values.len() == 7 {
            4
        } else {
            0
        };
        let r = color_tran(clips.variable_values[offset].value)?;
        let g = color_tran(clips.variable_values[offset + 1].value)?;
        let b = color_tran(clips.variable_values[offset + 2].value)?;

        writeln!(
            file,
            ".{css_prefix}{id} {{background-color: #{r:02X}{g:02X}{b:02X}}}",
        )?;
    }

    Ok(())
}

fn gen_item_colors(pak: &mut PakReader<impl Read + Seek>, mut output: impl Write) -> Result<()> {
    gen_gui_colors(
        pak,
        &mut output,
        "gui/01_Common/ItemIcon.gui",
        "pnl_ItemIcon_Color",
        "ITEM_ICON_COLOR_",
        "mh-item-color-",
    )?;

    // I can't find this in game so just add it hered
    output.write_all(b".mh-item-color-51 {background-color: #FF5687}\n")?;

    Ok(())
}

fn gen_rarity_colors(pak: &mut PakReader<impl Read + Seek>, output: impl Write) -> Result<()> {
    gen_gui_colors(
        pak,
        output,
        "gui/01_Common/EquipIcon.gui",
        "pnl_EquipIcon_Color",
        "rare_col",
        "mh-rarity-color-",
    )
}

fn hash_map_unique<T, K: Eq + std::hash::Hash + std::fmt::Debug, V>(
    iter: impl IntoIterator<Item = T>,
    kv: impl Fn(T) -> (K, V),
    ignore_duplicate: bool,
) -> Result<HashMap<K, V>> {
    use std::collections::hash_map::Entry;
    let mut result = HashMap::new();
    for t in iter {
        let (k, v) = kv(t);
        match result.entry(k) {
            Entry::Occupied(slot) => {
                let message = format!(
                    "Duplicate {} record with key {:?}",
                    std::any::type_name::<V>(),
                    slot.key()
                );
                if ignore_duplicate {
                    eprintln!("{message}");
                } else {
                    bail!("{}", message);
                }
            }
            Entry::Vacant(slot) => {
                slot.insert(v);
            }
        }
    }
    Ok(result)
}

fn prepare_size_map(size_data: &EnemySizeListData) -> Result<HashMap<EmTypes, &SizeInfo>> {
    hash_map_unique(
        size_data
            .size_info_list
            .iter()
            .filter(|e| e.em_type != EmTypes::Em(0)),
        |e| (e.em_type, e),
        false,
    )
}

fn prepare_size_dist_map(
    size_dist_data: &EnemyBossRandomScaleData,
) -> Result<HashMap<i32, &[ScaleAndRateData]>> {
    let mut result = hash_map_unique(
        &size_dist_data.random_scale_table_data_list,
        |e| (e.type_, &e.scale_and_rate_data[..]),
        false,
    )?;
    if result.contains_key(&0) {
        bail!("Defined size dist for 0");
    }
    result.insert(
        0,
        &[ScaleAndRateData {
            scale: 1.0,
            rate: 100,
        }],
    );
    Ok(result)
}

fn prepare_quests<'a>(
    pedia: &'a Pedia,
    reward_lot: &'_ HashMap<u32, &'a RewardIdLotTableUserDataParam>,
) -> Result<BTreeMap<i32, Quest<'a>>> {
    let all_msg = pedia
        .quest_hall_msg
        .entries
        .iter()
        .chain(&pedia.quest_village_msg.entries)
        .chain(&pedia.quest_village_msg_mr.entries)
        .chain(&pedia.quest_tutorial_msg.entries)
        .chain(&pedia.quest_arena_msg.entries)
        .chain(&pedia.quest_dlc_msg.entries)
        .chain(&pedia.quest_hall_msg_mr.entries)
        .chain(&pedia.quest_hall_msg_mr2.entries);

    let all_msg = hash_map_unique(all_msg, |e| (&e.name, e), false)?;

    let enemy_params = pedia
        .normal_quest_data_for_enemy
        .param
        .iter()
        .chain(&pedia.dl_quest_data_for_enemy.param)
        .chain(&pedia.normal_quest_data_for_enemy_mr.param)
        .chain(
            pedia
                .dl_quest_data_for_enemy_mr
                .iter()
                .flat_map(|p| &p.param),
        )
        .filter(|e| e.quest_no != 0);

    let enemy_params = hash_map_unique(enemy_params, |param| (param.quest_no, param), false)?;

    let reward_params = hash_map_unique(
        pedia
            .quest_data_for_reward
            .param
            .iter()
            .chain(&pedia.quest_data_for_reward_mr.param)
            .filter(|e| e.quest_numer != 0),
        |param| (param.quest_numer, param),
        false,
    )?;

    let hyakuryu_list = pedia
        .fixed_hyakuryu_quest
        .data_list
        .iter()
        .chain(pedia.fixed_hyakuryu_quest.data_list_310.iter())
        .chain(pedia.fixed_hyakuryu_quest.data_list_320.iter())
        .chain(pedia.fixed_hyakuryu_quest.data_list_350.iter())
        .chain(pedia.fixed_hyakuryu_quest.data_list_370.iter())
        .chain(pedia.fixed_hyakuryu_quest.data_list_380.iter())
        .chain(pedia.fixed_hyakuryu_quest.data_list_390.iter());

    let hyakuryus = hash_map_unique(
        hyakuryu_list,
        |hyakuryu| (hyakuryu.quest_no, hyakuryu),
        false,
    )?;

    let servant = hash_map_unique(
        &pedia.quest_servant.quest_servant_data_list,
        |s| (s.quest_no, s),
        false,
    )?;

    let arena = hash_map_unique(
        pedia
            .arena_quest
            .param
            .iter()
            .chain(&pedia.arena_quest.param1)
            .chain(&pedia.arena_quest.param2)
            .chain(&pedia.arena_quest.param3)
            .chain(&pedia.arena_quest.param_mr)
            .chain(&pedia.arena_quest.param_mr1),
        |p| (p.quest_no, p),
        false,
    )?;

    let time_attack = hash_map_unique(
        &pedia.time_attack_reward.data_list,
        |p| (p.quest_no, p),
        false,
    )?;

    let mr_all_clear_quest: HashSet<_> = pedia.talk_condition_quest_list.quest_group[0]
        .quest_no_array
        .iter()
        .copied()
        .collect();

    let mr_all_clear_follower_quest: HashSet<_> = pedia.talk_condition_quest_list.quest_group[1]
        .quest_no_array
        .iter()
        .copied()
        .collect();

    let mut result = pedia
        .normal_quest_data
        .param
        .iter()
        .filter(|param| param.quest_no != 0)
        .map(|param| (param, false))
        .chain(
            pedia
                .normal_quest_data_mr
                .param
                .iter()
                .filter(|param| param.quest_no != 0)
                .map(|param| (param, false)),
        )
        .chain(
            pedia
                .dl_quest_data
                .param
                .iter()
                .filter(|param| param.quest_no != 0)
                .map(|param| (param, true)),
        )
        .chain(
            pedia
                .dl_quest_data_mr
                .iter()
                .flat_map(|p| &p.param)
                .filter(|param| param.quest_no != 0)
                .map(|param| (param, true)),
        )
        .map(|(param, is_dl)| {
            let name_msg_name = format!("QN{:06}_01", param.quest_no);
            let requester_msg_name = format!("QN{:06}_02", param.quest_no);
            let detail_msg_name = format!("QN{:06}_03", param.quest_no);
            let target_msg_name = format!("QN{:06}_04", param.quest_no);
            let condition_msg_name = format!("QN{:06}_05", param.quest_no);

            let reward = if let Some(&reward) = reward_params.get(&param.quest_no) {
                let additional_target_reward = if reward.additional_target_reward_table_index != 0 {
                    Some(
                        *reward_lot
                            .get(&reward.additional_target_reward_table_index)
                            .with_context(|| {
                                format!(
                                    "Can't find additional_target_reward for quest {}, id {}",
                                    param.quest_no, reward.additional_target_reward_table_index
                                )
                            })?,
                    )
                } else {
                    None
                };

                let common_material_reward = if reward.common_material_reward_table_index != 0 {
                    Some(
                        *reward_lot
                            .get(&reward.common_material_reward_table_index)
                            .with_context(|| {
                                format!(
                                    "Can't find common_material_reward for quest {}, id {}",
                                    param.quest_no, reward.common_material_reward_table_index
                                )
                            })?,
                    )
                } else {
                    None
                };

                let additional_quest_reward = reward
                    .additional_quest_reward_table_index
                    .iter()
                    .filter(|&&i| i != 0)
                    .map(|i| {
                        Ok(*reward_lot.get(i).with_context(|| {
                            format!(
                                "Can't find additional_quest_reward for quest {}, id {}",
                                param.quest_no, i
                            )
                        })?)
                    })
                    .collect::<Result<Vec<_>>>()?;

                let cloth_ticket = if reward.cloth_ticket_index != 0 {
                    Some(
                        *reward_lot
                            .get(&reward.cloth_ticket_index)
                            .with_context(|| {
                                format!(
                                    "Can't find cloth_ticket for quest {}, id {}",
                                    param.quest_no, reward.cloth_ticket_index
                                )
                            })?,
                    )
                } else {
                    None
                };

                Some(QuestReward {
                    param: reward,
                    additional_target_reward,
                    common_material_reward,
                    additional_quest_reward,
                    cloth_ticket,
                })
            } else {
                None
            };

            let time_attack_reward = if let Some(ta) = time_attack.get(&param.quest_no) {
                ta.rank_data_list
                    .iter()
                    .map(|rank| {
                        let reward = *reward_lot.get(&rank.reward_table_id).with_context(|| {
                            format!(
                                "Can't find time attack reward for quest {}, id {}",
                                param.quest_no, rank.reward_table_id
                            )
                        })?;
                        Ok(TimeAttackReward { reward, rank })
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                vec![]
            };

            Ok((
                param.quest_no,
                Quest {
                    param,
                    enemy_param: enemy_params.get(&param.quest_no).cloned(),
                    name: all_msg.get(&name_msg_name).cloned(),
                    requester: all_msg.get(&requester_msg_name).cloned(),
                    detail: all_msg.get(&detail_msg_name).cloned(),
                    target: all_msg.get(&target_msg_name).cloned(),
                    condition: all_msg.get(&condition_msg_name).cloned(),
                    is_dl,
                    reward,
                    hyakuryu: hyakuryus.get(&param.quest_no).cloned(),
                    servant: servant.get(&param.quest_no).cloned(),
                    arena: arena.get(&param.quest_no).cloned(),
                    unlock: vec![],
                    random_group: None,
                    time_attack_reward,
                    is_mr_all_clear_quest: mr_all_clear_quest.contains(&param.quest_no),
                    is_mr_all_clear_follower_quest: mr_all_clear_follower_quest
                        .contains(&param.quest_no),
                },
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;

    for p in &pedia.quest_unlock.relation {
        for &release in &p.release_group_idx {
            let release_group = pedia
                .quest_unlock
                .quest_group
                .get(usize::try_from(release)?)
                .with_context(|| format!("Release group index {release} out of bound"))?;
            for &quest_no in &release_group.quest_no_array {
                result
                    .get_mut(&quest_no)
                    .with_context(|| format!("Unknown quest {quest_no} for group unlock"))?
                    .unlock
                    .push(QuestUnlock::Group(p))
            }
        }
    }

    for p in &pedia.quest_unlock.quest_unlock_by_talk_flag {
        result
            .get_mut(&p.quest_no)
            .with_context(|| format!("Unknown quest {} for talk flag unlock", p.quest_no))?
            .unlock
            .push(QuestUnlock::Talk(p))
    }

    for p in &pedia.quest_unlock.quest_unlock_by_quest_clear {
        for quest in &p.unlock_quest_no_list {
            result
                .get_mut(&quest.unlock_quest)
                .with_context(|| {
                    format!(
                        "Unknown quest {} for clear quest unlock",
                        quest.unlock_quest
                    )
                })?
                .unlock
                .push(QuestUnlock::Clear(p))
        }
    }

    for p in &pedia.quest_unlock.random_quest_unlock_by_quest_clear {
        for quest in &p.random_group {
            let random_group = &mut result
                .get_mut(&quest.random_quest)
                .with_context(|| {
                    format!(
                        "Unknown quest {} for random clear quest unlock",
                        quest.random_quest
                    )
                })?
                .random_group;
            if random_group.is_some() {
                bail!(
                    "Multiple random group specified for quest {}. Previous {:?}. Current {:?}",
                    quest.random_quest,
                    random_group,
                    p
                )
            }
            *random_group = Some(p)
        }
    }

    for p in &pedia.quest_unlock.quest_unlock_by_hunt_enemy {
        result
            .get_mut(&p.unlock_quest_no)
            .with_context(|| format!("Unknown quest {} for enemy unlock", p.unlock_quest_no))?
            .unlock
            .push(QuestUnlock::Enemy(p))
    }

    Ok(result)
}

fn prepare_skills(pedia: &Pedia) -> Result<BTreeMap<PlEquipSkillId, Skill<'_>>> {
    let mut result = BTreeMap::new();

    let mut name_msg: HashMap<&String, &MsgEntry> = pedia.player_skill_name_msg.get_name_map();

    let mut explain_msg: HashMap<&String, &MsgEntry> =
        pedia.player_skill_explain_msg.get_name_map();

    let mut detail_msg: HashMap<&String, &MsgEntry> = pedia.player_skill_detail_msg.get_name_map();

    let mut name_msg_mr: HashMap<&String, &MsgEntry> =
        pedia.player_skill_name_msg_mr.get_name_map();

    let mut explain_msg_mr: HashMap<&String, &MsgEntry> =
        pedia.player_skill_explain_msg_mr.get_name_map();

    let mut detail_msg_mr: HashMap<&String, &MsgEntry> =
        pedia.player_skill_detail_msg_mr.get_name_map();

    let custom_buildup_costs: HashMap<PlEquipSkillId, u32> = hash_map_unique(
        pedia
            .custom_buildup_equip_skill_detail
            .iter()
            .flat_map(|p| &p.param),
        |p| (p.skill_id, p.cost),
        false,
    )?;

    for skill in &pedia.equip_skill.param {
        if skill.id == PlEquipSkillId::None {
            continue;
        }
        if result.contains_key(&skill.id) {
            bail!("Multiple definition for skill {:?}", skill.id);
        }

        let msg_tag = skill.id.to_msg_tag();

        let name_tag = format!("{msg_tag}_Name");
        let name_tag_mr = format!("{msg_tag}_Name_MR");
        let explain_tag = format!("{msg_tag}_Explain");
        let explain_tag_mr = format!("{msg_tag}_Explain_MR");

        let name = name_msg_mr
            .remove(&name_tag_mr)
            .or_else(|| name_msg_mr.remove(&name_tag))
            .or_else(|| name_msg.remove(&name_tag))
            .with_context(|| format!("No name for skill {:?}", skill.id))?;

        let explain = explain_msg_mr
            .remove(&explain_tag_mr)
            .or_else(|| explain_msg_mr.remove(&explain_tag))
            .or_else(|| explain_msg.remove(&explain_tag))
            .with_context(|| format!("No explain for skill {:?}", skill.id))?;

        let levels = (0..(skill.max_level + 1))
            .map(|level| {
                let detail_tag = format!("{msg_tag}_{level:02}_Detail");
                let detail_tag_mr = format!("{msg_tag}_{level:02}_Detail_MR");

                detail_msg_mr
                    .remove(&detail_tag_mr)
                    .or_else(|| detail_msg_mr.remove(&detail_tag))
                    .or_else(|| detail_msg.remove(&detail_tag))
                    .with_context(|| format!("No detail for skill {:?} level {}", skill.id, level))
            })
            .collect::<Result<Vec<_>>>()?;

        result.insert(
            skill.id,
            Skill {
                name,
                explain,
                levels,
                icon_color: skill.icon_color,
                decos: vec![],
                custom_buildup_cost: custom_buildup_costs.get(&skill.id).copied(),
                alchemy: BTreeMap::new(),
                alchemy_grade: None,
            },
        );
    }

    let deco_name_msg = pedia.decorations_name_msg.get_name_map();
    let deco_name_msg_mr = pedia.decorations_name_msg_mr.get_name_map();
    let mut deco_products = hash_map_unique(
        pedia
            .decorations_product
            .param
            .iter()
            .filter(|product| product.id != DecorationsId::None),
        |product| (product.id, product),
        false,
    )?;

    let mut deco_dedup: HashSet<DecorationsId> = HashSet::new();
    for deco in &pedia.decorations.param {
        if deco.id == DecorationsId::None {
            continue;
        }
        if !deco_dedup.insert(deco.id) {
            bail!("Duplicate deco definition for {:?}", deco.id)
        }
        let product = deco_products
            .remove(&deco.id)
            .with_context(|| format!("No product for deco {:?}", deco.id))?;

        let name_tag = format!("{}_Name", deco.id.to_msg_tag());
        let name = *deco_name_msg
            .get(&name_tag)
            .or_else(|| deco_name_msg_mr.get(&name_tag))
            .with_context(|| format!("no name for deco {:?}", deco.id))?;

        if deco.skill_id_list[1] != PlEquipSkillId::None {
            bail!("Combo deco {:?}", deco.id);
        }

        result
            .get_mut(&deco.skill_id_list[0])
            .with_context(|| {
                format!(
                    "Deco {:?} is for unknown skill {:?}",
                    deco.id, deco.skill_id_list[0]
                )
            })?
            .decos
            .push(Deco {
                data: deco,
                product,
                name,
            });
    }

    if !deco_products.is_empty() {
        bail!("Leftover deco product")
    }

    for al_skill in &pedia.alchemy_pl_skill.param {
        if al_skill.skill_id == PlEquipSkillId::None {
            continue;
        }
        if al_skill.pick_rate == 0
            && al_skill.skill1_rate_list.iter().all(|&x| x == 0)
            && al_skill.skill2_rate_list.iter().all(|&x| x == 0)
            && al_skill.miss_rate_list.iter().all(|&x| x == 0)
        {
            // Crapcom: dummy data?
            continue;
        }

        let skill = result
            .get_mut(&al_skill.skill_id)
            .with_context(|| format!("Alchemy skill {:?} not found", al_skill.skill_id))?;
        if skill.alchemy.insert(al_skill.patturn, al_skill).is_some() {
            bail!(
                "Multiple alchemy data defined for skill {:?} pattern {:?}",
                al_skill.skill_id,
                al_skill.patturn
            )
        }
    }

    for skill in result.values_mut() {
        if skill.alchemy.is_empty() {
            continue;
        }
        skill.alchemy_grade = Some(skill.alchemy.values().next().unwrap().grade);
        if skill
            .alchemy
            .values()
            .any(|a| a.grade != skill.alchemy_grade.unwrap())
        {
            bail!("Inconsistent skill grade {:?}", skill.alchemy)
        }
    }

    Ok(result)
}

fn prepare_hyakuryu_skills(
    pedia: &Pedia,
) -> Result<BTreeMap<PlHyakuryuSkillId, HyakuryuSkill<'_>>> {
    let names = pedia.hyakuryu_skill_name_msg.get_name_map();
    let explains = pedia.hyakuryu_skill_explain_msg.get_name_map();
    let names_mr = pedia.hyakuryu_skill_name_msg_mr.get_name_map();
    let explains_mr = pedia.hyakuryu_skill_explain_msg_mr.get_name_map();
    let mut recipes = hash_map_unique(
        pedia
            .hyakuryu_skill_recipe
            .param
            .iter()
            .filter(|r| r.skill_id != PlHyakuryuSkillId::None),
        |r| (r.skill_id, r),
        false,
    )?;

    let get_name_explain = |id: PlHyakuryuSkillId| -> Result<(&MsgEntry, &MsgEntry)> {
        let raw_id = if let PlHyakuryuSkillId::Skill(id) = id {
            id
        } else {
            bail!("None Hyakuryu skill ID")
        };

        let name_tag = format!("HyakuryuSkill_{raw_id:03}_Name");
        let explain_tag = format!("HyakuryuSkill_{raw_id:03}_Explain");
        let name = *names
            .get(&name_tag)
            .or_else(|| names_mr.get(&name_tag))
            .with_context(|| format!("No name found for hyakuryu skill {id:?}"))?;
        let explain = *explains
            .get(&explain_tag)
            .or_else(|| explains_mr.get(&explain_tag))
            .with_context(|| format!("No explain found for hyakuryu skill {id:?}"))?;

        Ok((name, explain))
    };

    let mut result = BTreeMap::new();
    for skill in &pedia.hyakuryu_skill.param {
        if skill.id == PlHyakuryuSkillId::None {
            continue;
        }

        if result.contains_key(&skill.id) {
            bail!("Multiple definition for hyakuryu skill {:?}", skill.id);
        }

        let recipe = recipes.remove(&skill.id);

        let (name, explain) = get_name_explain(skill.id)?;
        let skill_package = HyakuryuSkill {
            data: Some(skill),
            recipe,
            name,
            explain,
            deco: None,
        };
        result.insert(skill.id, skill_package);
    }

    let deco_name_msg = pedia.hyakuryu_decos_name_msg.get_name_map();
    let mut deco_products = hash_map_unique(
        pedia
            .hyakuryu_decos_product
            .param
            .iter()
            .filter(|product| product.id != HyakuryuDecoId::None),
        |product| (product.id, product),
        false,
    )?;

    let mut deco_dedup: HashSet<HyakuryuDecoId> = HashSet::new();
    for deco in &pedia.hyakuryu_decos.param {
        let name_tag = match deco.id {
            HyakuryuDecoId::None => continue,
            HyakuryuDecoId::Deco(id) => format!("HyakuryuDeco_{id:03}_Name"),
        };
        if deco.id == HyakuryuDecoId::None {
            continue;
        }
        if !deco_dedup.insert(deco.id) {
            bail!("Duplicate deco definition for hyakuryu {:?}", deco.id)
        }
        let product = deco_products
            .remove(&deco.id)
            .with_context(|| format!("No product for hyakuryu deco {:?}", deco.id))?;

        let name = *deco_name_msg
            .get(&name_tag)
            .with_context(|| format!("no name for hyakuryu deco {:?}", deco.id))?;

        let deco_slot = if let Some(skill) = result.get_mut(&deco.hyakuryu_skill_id) {
            &mut skill.deco
        } else {
            // This happens for Fanged Exploit
            eprintln!(
                "Hyakuryu deco {:?} is for unknown skill {:?}. Going to make up one",
                deco.id, deco.hyakuryu_skill_id
            );

            let (skill_name, skill_explain) = get_name_explain(deco.hyakuryu_skill_id)?;
            result.insert(
                deco.hyakuryu_skill_id,
                HyakuryuSkill {
                    data: None,
                    recipe: None,
                    name: skill_name,
                    explain: skill_explain,
                    deco: Some(HyakuryuDeco {
                        data: deco,
                        product,
                        name,
                    }),
                },
            );

            continue;
        };
        if deco_slot.is_some() {
            bail!(
                "Multiple hyakuryu deco for skill {:?}",
                deco.hyakuryu_skill_id
            )
        }
        *deco_slot = Some(HyakuryuDeco {
            data: deco,
            product,
            name,
        });
    }

    if !deco_products.is_empty() {
        bail!("Leftover hyakuryu deco product")
    }

    Ok(result)
}

fn prepare_armors(pedia: &Pedia) -> Result<BTreeMap<PlArmorSeriesTypes, ArmorSeries<'_>>> {
    let mut product_map = hash_map_unique(
        &pedia.armor_product.param,
        |product| (product.id, product),
        false,
    )?;
    let mut series_map: BTreeMap<PlArmorSeriesTypes, ArmorSeries> = BTreeMap::new();

    fn get_msg<'a>(id: usize, msg: &'a Msg, msg_mr: &'a Msg) -> Option<&'a MsgEntry> {
        // Why is this index-based
        if id < 300 {
            msg.entries.get(id)
        } else {
            msg_mr.entries.get(id - 300)
        }
    }

    for armor_series in &pedia.armor_series.param {
        if series_map.contains_key(&armor_series.armor_series) {
            if armor_series.armor_series.0 == 0 {
                // Crapcom please
                eprintln!("Multiple armor series with ID 0. Ignoring");
                continue;
            }
            bail!(
                "Duplicate armor series for ID {:?}",
                armor_series.armor_series
            );
        }
        let name = get_msg(
            usize::try_from(armor_series.armor_series.0)?,
            &pedia.armor_series_name_msg,
            &pedia.armor_series_name_msg_mr,
        )
        .with_context(|| {
            format!(
                "Cannot find name for armor series {:?}",
                armor_series.armor_series
            )
        })?;
        let series = ArmorSeries {
            name,
            series: armor_series,
            pieces: [None, None, None, None, None, None, None, None, None, None],
        };
        series_map.insert(armor_series.armor_series, series);
    }

    let delegate = |pl_armor_id: PlArmorId| -> Result<(usize, &MsgEntry, &MsgEntry)> {
        let (slot, msg, explain_msg, msg_mr, explain_msg_mr, id) = match pl_armor_id {
            PlArmorId::Head(id) => (
                0,
                &pedia.armor_head_name_msg,
                &pedia.armor_head_explain_msg,
                &pedia.armor_head_name_msg_mr,
                &pedia.armor_head_explain_msg_mr,
                id,
            ),
            PlArmorId::Chest(id) => (
                1,
                &pedia.armor_chest_name_msg,
                &pedia.armor_chest_explain_msg,
                &pedia.armor_chest_name_msg_mr,
                &pedia.armor_chest_explain_msg_mr,
                id,
            ),
            PlArmorId::Arm(id) => (
                2,
                &pedia.armor_arm_name_msg,
                &pedia.armor_arm_explain_msg,
                &pedia.armor_arm_name_msg_mr,
                &pedia.armor_arm_explain_msg_mr,
                id,
            ),
            PlArmorId::Waist(id) => (
                3,
                &pedia.armor_waist_name_msg,
                &pedia.armor_waist_explain_msg,
                &pedia.armor_waist_name_msg_mr,
                &pedia.armor_waist_explain_msg_mr,
                id,
            ),
            PlArmorId::Leg(id) => (
                4,
                &pedia.armor_leg_name_msg,
                &pedia.armor_leg_explain_msg,
                &pedia.armor_leg_name_msg_mr,
                &pedia.armor_leg_explain_msg_mr,
                id,
            ),
            _ => bail!("Unknown armor ID {:?}", pl_armor_id),
        };

        let id = usize::try_from(id)?;

        let name = get_msg(id, msg, msg_mr)
            .with_context(|| format!("Cannot find name for armor {:?}", pl_armor_id))?;
        let explain = get_msg(id, explain_msg, explain_msg_mr)
            .with_context(|| format!("Cannot find name for armor {:?}", pl_armor_id))?;
        Ok((slot, name, explain))
    };

    for armor in &pedia.armor.param {
        if !armor.is_valid {
            continue;
        }

        let (mut slot, name, explain) = delegate(armor.pl_armor_id)?;

        if armor.id_after_ex_change == PlArmorId::ChangedEx {
            slot += 5;
        }

        let product = product_map.remove(&armor.pl_armor_id);

        let series = series_map.get_mut(&armor.series).with_context(|| {
            format!(
                "Cannot find series {:?} for armor {:?}",
                armor.series, armor.pl_armor_id
            )
        })?;

        if series.pieces[slot].is_some() {
            bail!(
                "Duplicated pieces for series {:?} slot {}",
                armor.series,
                slot
            );
        }

        series.pieces[slot] = Some(Armor {
            name,
            explain,
            fake_data: false,
            data: Cow::Borrowed(armor),
            product,
            overwear: None,
            overwear_product: None,
        });
    }

    let mut overwear_product_map = HashMap::new();
    for overwear_product in &pedia.overwear_product.param {
        if matches!(
            overwear_product.id,
            PlOverwearId::Head(0)
                | PlOverwearId::Chest(0)
                | PlOverwearId::Arm(0)
                | PlOverwearId::Waist(0)
                | PlOverwearId::Leg(0)
        ) {
            continue;
        }

        if overwear_product_map
            .insert(overwear_product.id, overwear_product)
            .is_some()
        {
            bail!(
                "Multiple definition for overwear product for {:?}",
                overwear_product.id
            );
        }
    }

    let mut overwear_set = HashSet::new();
    for overwear in &pedia.overwear.param {
        if !overwear.is_valid {
            continue;
        }
        if overwear_set.contains(&overwear.id) {
            bail!("Multiple definition for overwear {:?}", overwear.id);
        }
        overwear_set.insert(overwear.id);
        let series = series_map.get_mut(&overwear.series).with_context(|| {
            format!(
                "Cannot find series {:?} for overwear {:?}",
                overwear.series, overwear.id
            )
        })?;
        let slot = match overwear.id {
            PlOverwearId::Head(_) => 0,
            PlOverwearId::Chest(_) => 1,
            PlOverwearId::Arm(_) => 2,
            PlOverwearId::Waist(_) => 3,
            PlOverwearId::Leg(_) => 4,
        };

        if series.pieces[slot].is_none() {
            // Crapcom TU 5. Let's make up some entry
            eprintln!(
                "No armor slot for for overwear {:?}. Making up one",
                overwear.id
            );

            let data = ArmorBaseUserDataParam {
                pl_armor_id: overwear.relative_id,
                is_valid: true,
                series: overwear.series,
                sort_id: 0,
                model_id: overwear.model_id,
                rare: overwear.rare_type,
                value: 0,
                buy_value: 0,
                sexual_equipable: SexualEquipableFlag::Both,
                symbol_color1: false,
                symbol_color2: false,
                def_val: 0,
                fire_reg_val: 0,
                water_reg_val: 0,
                ice_reg_val: 0,
                thunder_reg_val: 0,
                dragon_reg_val: 0,
                buildup_table: 0,
                buff_formula: 0,
                decorations_num_list: [0; 4],
                skill_list: vec![],
                skill_lv_list: vec![],
                id_after_ex_change: PlArmorId::TableNone,
                custom_table_no: 0,
                custom_cost: 0,
            };

            let (_, name, explain) = delegate(overwear.relative_id)?;
            series.pieces[slot] = Some(Armor {
                name,
                explain,
                data: Cow::Owned(data),
                fake_data: true,
                product: None,
                overwear: None,
                overwear_product: None,
            })
        }

        let armor = series.pieces[slot].as_mut().unwrap();

        if armor.data.pl_armor_id != overwear.relative_id {
            bail!("Mismatch armor ID for overwear {:?}", overwear.id);
        }
        if armor.overwear.is_some() {
            bail!("Multiple definition for overwear {:?}", overwear.id);
        }
        armor.overwear = Some(overwear);
        armor.overwear_product = overwear_product_map.remove(&overwear.id);
    }

    Ok(series_map)
}

fn prepare_meat_names(pedia: &Pedia) -> Result<HashMap<MeatKey, Vec<&MsgEntry>>> {
    let msg_map: HashMap<_, _> = pedia.hunter_note_msg.get_name_map();

    let mut result: HashMap<MeatKey, Vec<&MsgEntry>> = HashMap::new();

    for boss_monster in &pedia.monster_list.data_list {
        for part_data in &boss_monster.part_table_data {
            let part = part_data.em_meat.try_into()?;
            let phase = part_data.em_meat_group_index.try_into()?;
            let key = MeatKey {
                em_type: boss_monster.em_type,
                part,
                phase,
            };

            let name = if let Some(&name) = msg_map.get(&format!(
                "HN_Hunternote_ML_Tab_02_Parts{:02}",
                part_data.part
            )) {
                name
            } else {
                // TODO: MR name
                eprintln!(
                    "Part text not found for {:?}-{part}-{phase}. part ID {}",
                    boss_monster.em_type, part_data.part
                );
                continue;
            };

            result.entry(key).or_default().push(name);
        }
    }

    Ok(result)
}

fn prepare_items<'a>(pedia: &'a Pedia) -> Result<BTreeMap<ItemId, Item<'a>>> {
    let mut result: BTreeMap<ItemId, Item<'a>> = BTreeMap::new();
    let name_map: HashMap<_, _> = pedia.items_name_msg.get_name_map();
    let explain_map: HashMap<_, _> = pedia.items_explain_msg.get_name_map();
    let name_map_mr: HashMap<_, _> = pedia.items_name_msg_mr.get_name_map();
    let explain_map_mr: HashMap<_, _> = pedia.items_explain_msg_mr.get_name_map();
    for param in &pedia.items.param {
        if let Some(existing) = result.get_mut(&param.id) {
            eprintln!("Duplicate definition for item {:?}", param.id);
            existing.multiple_def = true;
            continue;
        }

        let (name_tag, explain_tag, name_tag_mr, explain_tag_mr) = match param.id {
            ItemId::Normal(id) => (
                format!("I_{:04}_Name", id & 0xFFFF),
                format!("I_{:04}_Explain", id & 0xFFFF),
                format!("I_{:04}_Name_MR", id & 0xFFFF),
                format!("I_{:04}_Explain_MR", id & 0xFFFF),
            ),
            _ => bail!("Unexpected item type"),
        };

        let name = *name_map_mr
            .get(&name_tag_mr)
            .or_else(|| name_map_mr.get(&name_tag))
            .or_else(|| name_map.get(&name_tag))
            .with_context(|| format!("Name not found for item {:?}", param.id))?;

        let explain = *explain_map_mr
            .get(&explain_tag_mr)
            .or_else(|| explain_map_mr.get(&explain_tag))
            .or_else(|| explain_map.get(&explain_tag))
            .with_context(|| format!("Explain not found for item {:?}", param.id))?;

        let item = Item {
            name,
            explain,
            param,
            multiple_def: false,
        };
        result.insert(param.id, item);
    }

    Ok(result)
}

static AMMOR_SPHERE_CATEGORY_MSG: Lazy<MsgEntry> = Lazy::new(|| MsgEntry {
    name: "".to_string(),
    guid: Guid { bytes: [0; 16] },
    hash: 0,
    attributes: vec![],
    content: vec!["Armor sphere".to_string(); 32],
});

fn prepare_material_categories(pedia: &Pedia) -> HashMap<MaterialCategory, &MsgEntry> {
    const PREFIX: &str = "ICT_Name_";
    pedia
        .material_category_msg
        .entries
        .iter()
        .chain(pedia.material_category_msg_mr.entries.iter())
        .filter_map(|entry| {
            if !entry.name.starts_with(PREFIX) {
                return None;
            }
            Some((
                MaterialCategory::from_msg_id(entry.name[PREFIX.len()..].parse().ok()?),
                entry,
            ))
        })
        .chain(std::iter::once((
            MaterialCategory::ArmorSphere,
            &*AMMOR_SPHERE_CATEGORY_MSG,
        )))
        .collect()
}

fn prepare_weapon<'a, 'b, T, Param>(
    weapon_list: &'a WeaponList<T>,
    hyakuryu_weapon_map: &'b mut HashMap<
        WeaponId,
        BTreeMap<i32, &'a HyakuryuWeaponHyakuryuBuildupUserDataParam>,
    >,
    chaos: &'b mut HashMap<WeaponId, &'a WeaponChaosCriticalUserDataParam>,
) -> Result<WeaponTree<'a, Param>>
where
    T: Deref<Target = [Param]>,
    Param: ToBase<WeaponBaseData>,
{
    let mut product_map = hash_map_unique(
        weapon_list
            .product
            .param
            .iter()
            .filter(|p| p.base.id != WeaponId::None && p.base.id != WeaponId::Null),
        |p| (p.base.id, p),
        false,
    )?;

    let mut change_map = hash_map_unique(
        weapon_list
            .change
            .param
            .iter()
            .filter(|p| p.base.id != WeaponId::None && p.base.id != WeaponId::Null),
        |p| (p.base.id, p),
        false,
    )?;

    let mut process_map = hash_map_unique(
        weapon_list
            .process
            .param
            .iter()
            .filter(|p| p.base.id != WeaponId::None && p.base.id != WeaponId::Null),
        |p| (p.base.id, p),
        false,
    )?;

    let mut name_map = weapon_list.name.get_name_map();
    let mut explain_map = weapon_list.explain.get_name_map();
    let mut name_map_mr = weapon_list.name_mr.get_name_map();
    let mut explain_map_mr = weapon_list.explain_mr.get_name_map();

    let (overwear_map, overwear_product_map) = if let (Some(overwear), Some(product)) =
        (&weapon_list.overwear, &weapon_list.overwear_product)
    {
        (
            hash_map_unique(
                overwear.param.iter().filter(|p| p.id != 0),
                |p| (p.relative_id, p),
                true, // Crapcom made some duplicate entries
            )?,
            hash_map_unique(
                product.param.iter().filter(|p| p.id != 0),
                |p| (p.id, p),
                false,
            )?,
        )
    } else {
        (HashMap::new(), HashMap::new())
    };

    let mut weapons = BTreeMap::new();
    for param in &*weapon_list.base_data {
        let id = param.to_base().id;
        if id == WeaponId::None || id == WeaponId::Null {
            continue;
        }
        if weapons.contains_key(&id) {
            bail!("Multiple definition for weapon {:?}", param.to_base().id)
        }
        let tag = id.to_tag();
        let name_tag = format!("W_{tag}_Name");
        let explain_tag = format!("W_{tag}_Explain");

        let name = name_map
            .remove(&name_tag)
            .or_else(|| name_map_mr.remove(&name_tag))
            .with_context(|| format!("Weapon name for {tag} not found"))?;
        let explain = explain_map
            .remove(&explain_tag)
            .or_else(|| explain_map_mr.remove(&explain_tag));

        let (overwear, overwear_product) = if let Some(&overwear) = overwear_map.get(&id) {
            if let Some(&product) = overwear_product_map.get(&overwear.id) {
                (Some(overwear), Some(product))
            } else {
                // This happens for DLC layered
                (Some(overwear), None)
            }
        } else {
            (None, None)
        };

        let weapon = Weapon {
            param,
            product: product_map.remove(&id),
            change: change_map.remove(&id),
            process: process_map.remove(&id),
            overwear,
            overwear_product,
            name,
            explain,
            children: vec![],
            parent: None,
            hyakuryu_weapon_buildup: hyakuryu_weapon_map.remove(&id).unwrap_or_default(),
            update: None,
            chaos: chaos.remove(&id),
        };
        weapons.insert(param.to_base().id, weapon);
    }

    if !product_map.is_empty() {
        eprintln!("Left over product data: {product_map:?}")
    }

    if !process_map.is_empty() {
        eprintln!("Left over process data: {process_map:?}")
    }

    if !change_map.is_empty() {
        eprintln!("Left over change data: {change_map:?}")
    }

    let mut tree_map = HashMap::new();
    let mut tree_id_set = HashSet::new();
    for node in &weapon_list.tree.param {
        if node.weapon_id == WeaponId::None || node.weapon_id == WeaponId::Null {
            continue;
        }
        if tree_id_set.contains(&node.weapon_id) {
            bail!("Multiple tree node for weapon {:?}", node.weapon_id)
        }
        if let Some(weapon) = weapons.get_mut(&node.weapon_id) {
            weapon.update = Some(node);
        } else {
            bail!("Unknown weapon in tree node {:?}", node.weapon_id);
        }
        tree_id_set.insert(node.weapon_id);
        if tree_map
            .insert((node.tree_type, node.index), node)
            .is_some()
        {
            bail!(
                "Multiple weapon at position {:?}",
                (node.tree_type, node.index)
            )
        }
    }

    let mut unpositioned = vec![];

    for weapon in weapons.keys() {
        if !tree_id_set.contains(weapon) {
            unpositioned.push(*weapon);
        }
    }

    let mut roots = vec![];

    for node in &weapon_list.tree.param {
        if node.weapon_id == WeaponId::None || node.weapon_id == WeaponId::Null {
            continue;
        }
        if node.prev_weapon_type == TreeType::None {
            roots.push(node.weapon_id);
        } else {
            let prev = tree_map
                .get(&(node.prev_weapon_type, node.prev_weapon_index))
                .with_context(|| format!("Unknown previous position for {node:?}"))?;
            weapons.get_mut(&node.weapon_id).unwrap().parent = Some(prev.weapon_id);
            if !prev
                .next_weapon_type_list
                .iter()
                .cloned()
                .zip(prev.next_weapon_index_list.iter().cloned())
                .any(|ti| ti == (node.tree_type, node.index))
            {
                bail!("Previous node doesn't contain next for {:?}", node)
            }
        }
        let mut children: Vec<_> = node
            .next_weapon_type_list
            .iter()
            .cloned()
            .zip(node.next_weapon_index_list.iter().cloned())
            .filter(|&(t, _)| t != TreeType::None)
            .collect();
        children.sort_by_key(|&(t, i)| {
            if t == node.tree_type {
                (TreeType::None, -1)
            } else {
                (t, i)
            }
        });
        let mut prev_child = None;
        for (t, i) in children {
            if prev_child == Some((t, i)) {
                eprintln!(
                    "Duplicate weapon branch at {:?}, {}, for weapon {:?}",
                    t, i, node.weapon_id
                );
                continue;
            }
            prev_child = Some((t, i));
            let next = if let Some(next) = tree_map.get(&(t, i)) {
                next
            } else {
                eprintln!(
                    "Unknown children at {:?}, {}, for weapon {:?}",
                    t, i, node.weapon_id
                );
                continue;
            };
            if next.prev_weapon_type != node.tree_type || next.prev_weapon_index != node.index {
                bail!("Mismatch prev type/index")
            }
            weapons
                .get_mut(&node.weapon_id)
                .unwrap()
                .children
                .push(next.weapon_id);
        }
    }

    let result = WeaponTree {
        weapons,
        unpositioned,
        roots,
    };

    Ok(result)
}

fn prepare_monster_lot(
    pedia: &Pedia,
) -> Result<HashMap<(EmTypes, QuestRank), &MonsterLotTableUserDataParam>> {
    let iter = pedia
        .monster_lot
        .param
        .iter()
        .chain(pedia.monster_lot_mr.param.iter());

    hash_map_unique(
        iter.filter(|lot| lot.em_types != EmTypes::Em(0)),
        |lot| ((lot.em_types, lot.quest_rank), lot),
        false,
    )
}

fn prepare_parts_dictionary(
    pedia: &Pedia,
) -> Result<HashMap<(EmTypes, BrokenPartsTypes), &MsgEntry>> {
    let msgs: HashMap<_, _> = pedia.hunter_note_msg.get_guid_map();
    let msgs_mr: HashMap<_, _> = pedia.hunter_note_msg_mr.get_guid_map();

    let mut result = HashMap::new();

    for part in &pedia.parts_type.params {
        for info in &part.text_infos {
            let msg = *msgs
                .get(&info.text_for_monster_list)
                .or_else(|| msgs_mr.get(&info.text_for_monster_list))
                .with_context(|| {
                    format!("Cannot found part text for {:?}", part.broken_parts_types)
                })?;
            for &em in &info.enemy_type_list {
                if em == EmTypes::Em(0) {
                    continue;
                }
                if result.insert((em, part.broken_parts_types), msg).is_some() {
                    eprintln!(
                        "Multiple part text for {:?}, {:?}",
                        em, part.broken_parts_types
                    );
                }
            }
        }
    }

    Ok(result)
}

fn prepare_horn_melody(pedia: &Pedia) -> HashMap<i32, &'_ MsgEntry> {
    let mut res = HashMap::new();
    let map = pedia.horn_melody.get_name_map();
    let map_mr = pedia.horn_melody_mr.get_name_map();
    for id in 0..999 {
        let name = format!("Horn_UniqueParam_{id:03}_Name");
        if let Some(&name) = map.get(&name) {
            res.insert(id, name);
        } else if let Some(&name) = map_mr.get(&name) {
            res.insert(id, name);
        }
    }
    res
}

fn prepare_item_pop(
    pedia: &Pedia,
) -> Result<HashMap<(i32, i32), &'_ ItemPopLotTableUserDataParam>> {
    let mut res = HashMap::new();
    for param in &pedia.item_pop_lot.param {
        if res
            .insert((param.pop_id, param.field_type), param)
            .is_some()
        {
            bail!(
                "Multiple definition for item pop {} in map {}",
                param.pop_id,
                param.field_type
            );
        }
    }
    Ok(res)
}

fn prepeare_ot_equip(pedia: &Pedia) -> Result<BTreeMap<OtEquipSeriesId, OtEquipSeries<'_>>> {
    let mut res = BTreeMap::new();

    let airou_series_name = pedia.airou_series_name.get_name_map();
    let dog_series_name = pedia.dog_series_name.get_name_map();
    let airou_series_name_mr = pedia.airou_series_name_mr.get_name_map();
    let dog_series_name_mr = pedia.dog_series_name_mr.get_name_map();

    for series in &pedia.ot_equip_series.param {
        if res.contains_key(&series.id) {
            eprintln!("Found multiple definition for otomo series {:?}", series.id);
            if series.id == OtEquipSeriesId::Airou(0) {
                // this seems to be a placeholder. continue
                continue;
            }
            bail!("multiple otomo series definition")
        }
        let name = *match series.id {
            OtEquipSeriesId::Airou(id) => {
                let tag = format!("ArmorSeries_OtAirou_{id:03}_Name");
                airou_series_name
                    .get(&tag)
                    .or_else(|| airou_series_name_mr.get(&tag))
            }
            OtEquipSeriesId::Dog(id) => {
                let tag = format!("ArmorSeries_OtDog_{id:03}_Name");
                dog_series_name
                    .get(&tag)
                    .or_else(|| dog_series_name_mr.get(&tag))
            }
        }
        .with_context(|| format!("Cannot find name for otomo series {:?}", series.id))?;

        let entry = OtEquipSeries {
            series,
            name,
            weapon: None,
            head: None,
            chest: None,
        };

        res.insert(series.id, entry);
    }

    let weapon_products = pedia
        .airou_weapon_product
        .param
        .iter()
        .chain(&pedia.dog_weapon_product.param)
        .filter(|p| p.id != OtWeaponId::None);

    let mut weapon_products = hash_map_unique(weapon_products, |p| (p.id, p), true)?;

    let airou_weapon_name = pedia.airou_weapon_name.get_name_map();
    let dog_weapon_name = pedia.dog_weapon_name.get_name_map();
    let airou_weapon_explain = pedia.airou_weapon_explain.get_name_map();
    let dog_weapon_explain = pedia.dog_weapon_explain.get_name_map();
    let airou_weapon_name_mr = pedia.airou_weapon_name_mr.get_name_map();
    let dog_weapon_name_mr = pedia.dog_weapon_name_mr.get_name_map();
    let airou_weapon_explain_mr = pedia.airou_weapon_explain_mr.get_name_map();
    let dog_weapon_explain_mr = pedia.dog_weapon_explain_mr.get_name_map();
    let mut weapon_dedup = HashSet::new();

    for weapon in pedia
        .airou_weapon
        .param
        .iter()
        .chain(pedia.dog_weapon.param.iter())
    {
        let (name, explain) = match weapon.id {
            OtWeaponId::None => continue,
            OtWeaponId::Airou(id) => {
                let name_tag = format!("OtAirouWeapon_{id:03}_Name");
                let explain_tag = format!("OtAirouWeapon_{id:03}_Explain");
                (
                    airou_weapon_name
                        .get(&name_tag)
                        .or_else(|| airou_weapon_name_mr.get(&name_tag)),
                    airou_weapon_explain
                        .get(&explain_tag)
                        .or_else(|| airou_weapon_explain_mr.get(&explain_tag)),
                )
            }
            OtWeaponId::Dog(id) => {
                let name_tag = format!("OtDogWeapon_{id:03}_Name");
                let explain_tag = format!("OtDogWeapon_{id:03}_Explain");
                (
                    dog_weapon_name
                        .get(&name_tag)
                        .or_else(|| dog_weapon_name_mr.get(&name_tag)),
                    dog_weapon_explain
                        .get(&explain_tag)
                        .or_else(|| dog_weapon_explain_mr.get(&explain_tag)),
                )
            }
        };

        let name =
            *name.with_context(|| format!("Cannot find name for otomo weapon {:?}", weapon.id))?;
        let explain = *explain
            .with_context(|| format!("Cannot find explain for otomo weapon {:?}", weapon.id))?;

        if !weapon_dedup.insert(weapon.id) {
            eprintln!("Multiple definition for otomo weapon {:?}", weapon.id);
            continue;
        }
        let entry = OtWeapon {
            name,
            explain,
            param: weapon,
            product: weapon_products.remove(&weapon.id),
        };
        let slot = &mut res
            .get_mut(&weapon.series_id)
            .with_context(|| {
                format!(
                    "Unknown otomo series {:?} from weapon {:?}",
                    weapon.series_id, weapon.id
                )
            })?
            .weapon;

        if slot.is_some() {
            eprintln!(
                "Multiple weapon defintion for otomo series {:?}. Discarding the latest one {:?}",
                weapon.series_id, weapon.id
            );
            continue;
        }

        *slot = Some(entry);
    }

    if !weapon_products.is_empty() {
        eprintln!("Left over otomo weapon product {weapon_products:?}")
    }

    let armor_products = pedia
        .airou_armor_product
        .param
        .iter()
        .chain(&pedia.dog_armor_product.param)
        .filter(|p| p.id != OtArmorId::None);

    let mut armor_products = hash_map_unique(armor_products, |p| (p.id, p), true)?;

    let airou_armor_head_name = pedia.airou_armor_head_name.get_name_map();
    let dog_armor_head_name = pedia.dog_armor_head_name.get_name_map();
    let airou_armor_head_explain = pedia.airou_armor_head_explain.get_name_map();
    let dog_armor_head_explain = pedia.dog_armor_head_explain.get_name_map();
    let airou_armor_chest_name = pedia.airou_armor_chest_name.get_name_map();
    let dog_armor_chest_name = pedia.dog_armor_chest_name.get_name_map();
    let airou_armor_chest_explain = pedia.airou_armor_chest_explain.get_name_map();
    let dog_armor_chest_explain = pedia.dog_armor_chest_explain.get_name_map();
    let airou_armor_head_name_mr = pedia.airou_armor_head_name_mr.get_name_map();
    let dog_armor_head_name_mr = pedia.dog_armor_head_name_mr.get_name_map();
    let airou_armor_head_explain_mr = pedia.airou_armor_head_explain_mr.get_name_map();
    let dog_armor_head_explain_mr = pedia.dog_armor_head_explain_mr.get_name_map();
    let airou_armor_chest_name_mr = pedia.airou_armor_chest_name_mr.get_name_map();
    let dog_armor_chest_name_mr = pedia.dog_armor_chest_name_mr.get_name_map();
    let airou_armor_chest_explain_mr = pedia.airou_armor_chest_explain_mr.get_name_map();
    let dog_armor_chest_explain_mr = pedia.dog_armor_chest_explain_mr.get_name_map();

    let mut armor_dedup = HashSet::new();

    for armor in pedia
        .airou_armor
        .param
        .iter()
        .map(|a| &a.base)
        .chain(pedia.dog_armor.param.iter().map(|a| &a.base))
    {
        let (name, explain) = match armor.id {
            OtArmorId::AirouHead(id) => {
                let name_tag = format!("OtAirouArmor_Head_{id:03}_Name");
                let explain_tag = format!("OtAirouArmor_Head_{id:03}_Explain");
                (
                    airou_armor_head_name
                        .get(&name_tag)
                        .or_else(|| airou_armor_head_name_mr.get(&name_tag)),
                    airou_armor_head_explain
                        .get(&explain_tag)
                        .or_else(|| airou_armor_head_explain_mr.get(&explain_tag)),
                )
            }
            OtArmorId::DogHead(id) => {
                let name_tag = format!("OtDogArmor_Head_{id:03}_Name");
                let explain_tag = format!("OtDogArmor_Head_{id:03}_Explain");
                (
                    dog_armor_head_name
                        .get(&name_tag)
                        .or_else(|| dog_armor_head_name_mr.get(&name_tag)),
                    dog_armor_head_explain
                        .get(&explain_tag)
                        .or_else(|| dog_armor_head_explain_mr.get(&explain_tag)),
                )
            }
            OtArmorId::AirouChest(id) => {
                let name_tag = format!("OtAirouArmor_Chest_{id:03}_Name");
                let explain_tag = format!("OtAirouArmor_Chest_{id:03}_Explain");
                (
                    airou_armor_chest_name
                        .get(&name_tag)
                        .or_else(|| airou_armor_chest_name_mr.get(&name_tag)),
                    airou_armor_chest_explain
                        .get(&explain_tag)
                        .or_else(|| airou_armor_chest_explain_mr.get(&explain_tag)),
                )
            }
            OtArmorId::DogChest(id) => {
                let name_tag = format!("OtDogArmor_Chest_{id:03}_Name");
                let explain_tag = format!("OtDogArmor_Chest_{id:03}_Explain");
                (
                    dog_armor_chest_name
                        .get(&name_tag)
                        .or_else(|| dog_armor_chest_name_mr.get(&name_tag)),
                    dog_armor_chest_explain
                        .get(&explain_tag)
                        .or_else(|| dog_armor_chest_explain_mr.get(&explain_tag)),
                )
            }
            OtArmorId::None => continue,
        };

        let name = name.with_context(|| format!("Cannot find name for armor {:?}", armor.id))?;
        let explain =
            explain.with_context(|| format!("Cannot find explain for armor {:?}", armor.id))?;

        if !armor_dedup.insert(armor.id) {
            eprintln!("Multiple definition for otomo armor {:?}", armor.id);
            continue;
        }
        let entry = OtArmor {
            param: armor,
            product: armor_products.remove(&armor.id),
            overwear: None,
            overwear_recipe: None,
            name,
            explain,
        };
        let series = res.get_mut(&armor.series_id).with_context(|| {
            format!(
                "Unknown otomo series {:?} from armor {:?}",
                armor.series_id, armor.id
            )
        })?;

        let (slot, desc) = match armor.id {
            OtArmorId::AirouHead(_) | OtArmorId::DogHead(_) => (&mut series.head, "head"),
            OtArmorId::AirouChest(_) | OtArmorId::DogChest(_) => (&mut series.chest, "chest"),
            OtArmorId::None => unreachable!(),
        };

        if slot.is_some() {
            eprintln!(
                "Multiple {} armor defintion for otomo series {:?}. Discarding the latest one {:?}",
                desc, armor.series_id, armor.id
            );
            continue;
        }

        *slot = Some(entry);
    }

    if !armor_products.is_empty() {
        bail!("Left over otomo armor product")
    }

    let mut overwear_recipe = hash_map_unique(
        pedia
            .ot_overwear_recipe
            .param
            .iter()
            .filter(|p| p.id != OtArmorId::None),
        |p| (p.id, p),
        true, // crapcom please: multiple AirouHead(0)
    )?;

    for overwear in pedia
        .airou_overwear
        .param
        .iter()
        .chain(&pedia.dog_overwear.param)
    {
        if !overwear.is_valid || overwear.id == OtArmorId::None {
            continue;
        }
        if overwear.id != overwear.relative_id {
            bail!(
                "Overwear ID not matching itself: {:?} vs {:?}",
                overwear.id,
                overwear.relative_id
            )
        }
        let series = res.get_mut(&overwear.series_id).with_context(|| {
            format!(
                "Series {:?} not found for overwear {:?}",
                overwear.series_id, overwear.id
            )
        })?;
        let piece = match overwear.id {
            OtArmorId::None => unreachable!(),
            OtArmorId::AirouHead(_) | OtArmorId::DogHead(_) => &mut series.head,
            OtArmorId::AirouChest(_) | OtArmorId::DogChest(_) => &mut series.chest,
        };
        let piece = piece
            .as_mut()
            .with_context(|| format!("OtArmor not init for overwear {:?}", overwear.id))?;
        piece.overwear = Some(overwear);
        piece.overwear_recipe = overwear_recipe.remove(&overwear.id);
    }

    if !overwear_recipe.is_empty() {
        bail!("Leftover recipe {:?}", overwear_recipe)
    }

    Ok(res)
}

fn prepare_monsters<'a>(
    pedia: &'a Pedia,
    reward_lot: &'_ HashMap<u32, &'a RewardIdLotTableUserDataParam>,
) -> Result<BTreeMap<EmTypes, MonsterEx<'a>>> {
    let mut result = BTreeMap::new();

    let names = pedia.monster_names.get_name_map();
    let names_mr = pedia.monster_names_mr.get_name_map();
    let aliases = pedia.monster_aliases.get_name_map();
    let aliases_mr = pedia.monster_aliases_mr.get_name_map();
    let explains = pedia.monster_explains.get_name_map();
    let explains_mr = pedia.monster_explains_mr.get_name_map();

    let random_quests: HashMap<EmTypes, &LotEnemyData> = hash_map_unique(
        pedia
            .random_mystery_enemy
            .iter()
            .flat_map(|p| &p.lot_enemy_list),
        |p| (p.em_type, p),
        false,
    )?;

    let mut random_mystery_reward: HashMap<EmTypes, Vec<&RandomMysteryRewardBaseParam>> =
        HashMap::new();

    for p in pedia
        .random_mystery_reward_base
        .iter()
        .flat_map(|p| &p.param_data)
    {
        random_mystery_reward.entry(p.em_type).or_default().push(p)
    }

    let random_mystery_subtarget = hash_map_unique(
        pedia
            .random_mystery_reward_subtarget
            .iter()
            .flat_map(|p| &p.param_data),
        |p| (p.em_type, p),
        false,
    )?;

    let discoveries: HashMap<EmTypes, &DiscoverEmSetDataParam> = hash_map_unique(
        pedia
            .discover_em_set_data
            .param
            .iter()
            .filter(|p| p.em_type != EmTypes::Em(0)),
        |p| (p.em_type, p),
        false,
    )?;

    let ranks: HashMap<EmTypes, u8> = hash_map_unique(
        pedia
            .enemy_rank
            .rank_info_list
            .iter()
            .filter(|p| p.em_type != EmTypes::Em(0)),
        |p| (p.em_type, p.rank),
        false,
    )?;

    let speciess: HashMap<EmTypes, &EmSpeciesData> = hash_map_unique(
        pedia
            .species
            .em_species_list
            .iter()
            .filter(|p| p.em_type != EmTypes::Em(0)),
        |p| (p.em_type, p),
        false,
    )?;

    let mut mystery_rewards: HashMap<EmTypes, Vec<MysteryReward>> = HashMap::new();

    for mystery_reward in &pedia.mystery_reward_item.param {
        if mystery_reward.em_type == EmTypes::Em(0) {
            continue;
        }
        if mystery_reward.quest_no != -1 {
            bail!("Mystery reward with quest_no: {mystery_reward:?}")
        }

        let quest_reward = (mystery_reward.quest_reward_table_index != 0)
            .then(|| {
                reward_lot
                    .get(&mystery_reward.quest_reward_table_index)
                    .copied()
                    .with_context(|| format!("Quest reward not found for {mystery_reward:?}"))
            })
            .transpose()?;

        let additional_quest_reward = mystery_reward
            .additional_quest_reward_table_index
            .iter()
            .filter(|&&i| i != 0)
            .map(|i| -> Result<Option<&RewardIdLotTableUserDataParam>> {
                let Some(reward) = reward_lot.get(i) else {
                    // crapcom: v13 chaotic gore
                    eprintln!("Additional quest reward {i} not found for {mystery_reward:?}");
                    return Ok(None)
                };
                Ok(Some(reward))
            })
            .filter_map(|r| r.transpose())
            .collect::<Result<Vec<&RewardIdLotTableUserDataParam>>>()?;

        let special_quest_reward = mystery_reward
            .special_quest_reward_table_index
            .0
            .filter(|&i| i != 0)
            .map(|i| {
                reward_lot.get(&i).copied().with_context(|| {
                    format!("Special quest reward not found for {mystery_reward:?}")
                })
            })
            .transpose()?;

        let multiple_target_reward = mystery_reward
            .multiple_target_reward_table_index
            .0
            .filter(|&i| i != 0)
            .map(|i| {
                reward_lot.get(&i).copied().with_context(|| {
                    format!("Multiple target quest reward not found for {mystery_reward:?}")
                })
            })
            .transpose()?;

        let multiple_fix_reward = mystery_reward
            .multiple_fix_reward_table_index
            .0
            .filter(|&i| i != 0)
            .map(|i| {
                reward_lot.get(&i).copied().with_context(|| {
                    format!("Multiple fix quest reward not found for {mystery_reward:?}")
                })
            })
            .transpose()?;

        mystery_rewards
            .entry(mystery_reward.em_type)
            .or_default()
            .push(MysteryReward {
                lv_lower_limit: mystery_reward.lv_lower_limit,
                lv_upper_limit: mystery_reward.lv_upper_limit,
                is_special: mystery_reward.is_special_mystery.0 == Some(true),
                hagibui_probability: mystery_reward.hagibui_probability,
                reward_item: mystery_reward.reward_item,
                item_num: mystery_reward.item_num,
                quest_reward,
                additional_quest_reward,
                special_quest_reward,
                multiple_target_reward,
                multiple_fix_reward,
            })
    }

    let hn_msgs: HashMap<_, _> = pedia.hunter_note_msg.get_name_map();
    let hn_msgs_mr: HashMap<_, _> = pedia.hunter_note_msg_mr.get_name_map();
    let monster_list = hash_map_unique(
        pedia.monster_list.data_list.iter(),
        |p| (p.em_type, p),
        false,
    )?;

    let monsters = pedia.monsters.iter().chain(&pedia.small_monsters);
    for monster in monsters {
        let mut mystery_reward = mystery_rewards.remove(&monster.em_type).unwrap_or_default();
        mystery_reward.sort_by_key(|m| m.lv_lower_limit);
        let random_quest = random_quests.get(&monster.em_type).copied();
        let random_mystery_reward = random_mystery_reward
            .remove(&monster.em_type)
            .unwrap_or_default();
        let random_mystery_subtarget_reward =
            random_mystery_subtarget.get(&monster.em_type).copied();
        let discovery = discoveries.get(&monster.em_type).copied();
        let rank = ranks.get(&monster.em_type).copied();
        let species = speciess.get(&monster.em_type).copied();
        let family = monster_list
            .get(&monster.em_type)
            .and_then(|p| match p.family_type {
                FamilyType::Species(i) => hn_msgs
                    .get(&format!("HN_Hunternote_ML_Tab_01_Spec_M{i:02}"))
                    .copied(),
                FamilyType::MrSpecies(0) => hn_msgs_mr
                    .get(&"HN_Hunternote_ML_Tab_01_Spec_M1_MR".to_owned())
                    .copied(),
                _ => None,
            });
        let entry = if let Some(index) = monster.enemy_type {
            let name = names
                .get(&format!("EnemyIndex{index:03}"))
                .copied()
                .or_else(|| names_mr.get(&format!("EnemyIndex{index:03}_MR")).copied());

            let alias_name = format!("Alias_EnemyIndex{index:03}");
            let alias = aliases
                .get(&alias_name)
                .copied()
                .or_else(|| aliases_mr.get(&alias_name).copied());

            let explain1 = explains
                .get(&format!("HN_MonsterListMsg_EnemyIndex{index:03}_page1"))
                .copied()
                .or_else(|| {
                    explains_mr
                        .get(&format!("HN_MonsterListMsg_EnemyIndex{index:03}_MR_page1"))
                        .copied()
                });

            let explain2 = explains
                .get(&format!("HN_MonsterListMsg_EnemyIndex{index:03}_page2"))
                .copied()
                .or_else(|| {
                    explains_mr
                        .get(&format!("HN_MonsterListMsg_EnemyIndex{index:03}_MR_page2"))
                        .copied()
                });

            MonsterEx {
                data: monster,
                name,
                alias,
                explain1,
                explain2,
                mystery_reward,
                random_quest,
                random_mystery_reward,
                random_mystery_subtarget_reward,
                discovery,
                rank,
                species,
                family,
            }
        } else {
            MonsterEx {
                data: monster,
                name: None,
                alias: None,
                explain1: None,
                explain2: None,
                mystery_reward,
                random_quest,
                random_mystery_reward,
                random_mystery_subtarget_reward,
                discovery,
                rank,
                species,
                family,
            }
        };
        result.insert(monster.em_type, entry);
    }

    Ok(result)
}

pub fn prepare_servant(pedia: &Pedia) -> Result<HashMap<i32, Servant<'_>>> {
    let mut result = HashMap::new();
    for entry in &pedia.servant_profile.entries {
        if let Some(id) = entry.name.strip_prefix("Name_ServantId") {
            let id: i32 = id
                .strip_suffix("_MR")
                .and_then(|id| id.parse().ok())
                .with_context(|| format!("Unexpected servant name tag {}", entry.name))?;
            let servant = Servant { name: entry };
            result.insert(id, servant);
        }
    }
    Ok(result)
}

pub fn prepare_armor_buildup(
    pedia: &Pedia,
) -> Result<HashMap<i32, Vec<&ArmorBuildupTableUserDataParam>>> {
    let mut result: HashMap<i32, Vec<&ArmorBuildupTableUserDataParam>> = HashMap::new();
    for param in &pedia.armor_buildup.param {
        result.entry(param.table_type).or_default().push(param);
    }
    for (table_type, series) in &mut result {
        series.sort_unstable_by_key(|e| e.limit_lv);
        if series
            .windows(2)
            .any(|window| window[0].limit_lv == window[1].limit_lv)
        {
            bail!("Duplicate limit lv for armor buildup type {table_type}");
        }
    }
    Ok(result)
}

// Hardcoded in snow.data.ArmorCustomBuildupData..cctor
const ARMOR_CUSTOM_BUILDUP_CATEGORIES: [u16; 4] = [
    13, // Def
    14, // Ele res
    19, // Slot
    20, // Skill
];

pub fn prepare_armor_custom_buildup<'a>(
    pedia: &'a Pedia,
    custom_buildup_pieces: &mut HashMap<(u32, u16, u16), &'a CustomBuildupBaseUserDataParam>,
) -> Result<HashMap<u32, ArmorCustomBuildup<'a>>> {
    let mut result = HashMap::new();
    for category_lot in pedia
        .custom_buildup_armor_category_lot
        .iter()
        .flat_map(|p| &p.param)
    {
        if category_lot.table_no == 0 {
            continue;
        }
        if result.contains_key(&category_lot.table_no) {
            bail!(
                "Duplicate armor custom buildup category entry for table {}",
                category_lot.table_no
            )
        }
        let mut categories: BTreeMap<u16, ArmorCustomBuildupCategory<'a>> =
            ARMOR_CUSTOM_BUILDUP_CATEGORIES
                .into_iter()
                .zip(category_lot.lot_num.iter().copied())
                .filter(|(_, l)| *l != 0)
                .map(|(c, lot)| {
                    (
                        c,
                        ArmorCustomBuildupCategory {
                            lot,
                            pieces: BTreeMap::new(),
                        },
                    )
                })
                .collect();
        categories.insert(
            15, // TODO: what is this? Added in v13
            ArmorCustomBuildupCategory {
                lot: 100, // fake lot
                pieces: BTreeMap::new(),
            },
        );
        result.insert(category_lot.table_no, ArmorCustomBuildup { categories });
    }

    for piece_lot in pedia.custom_buildup_armor_lot.iter().flat_map(|p| &p.param) {
        if piece_lot.table_no == 0 {
            continue;
        }
        let data = custom_buildup_pieces
            .remove(&(piece_lot.table_no, piece_lot.category_id, piece_lot.id))
            .with_context(|| format!("No data found for custom buildup {piece_lot:?}"))?;
        let category = result
            .get_mut(&piece_lot.table_no)
            .with_context(|| format!("Armor customer buildup table not found for {piece_lot:?}"))?
            .categories
            .get_mut(&piece_lot.category_id)
            .with_context(|| {
                format!("Armor customer buildup category not found for {piece_lot:?}")
            })?;
        if category.pieces.contains_key(&piece_lot.id) {
            bail!("Duplicate armor custom buildup piece entry {:?}", piece_lot)
        }
        category.pieces.insert(
            piece_lot.id,
            ArmorCustomBuildupPiece {
                lot: piece_lot.lot_num,
                data,
            },
        );
    }
    Ok(result)
}

pub fn prepare_weapon_custom_buildup<'a>(
    pedia: &'a Pedia,
    custom_buildup_pieces: &mut HashMap<(u32, u16, u16), &'a CustomBuildupBaseUserDataParam>,
) -> Result<HashMap<u32, WeaponCustomBuildup<'a>>> {
    let mut result = HashMap::<u32, WeaponCustomBuildup>::new();
    let material = hash_map_unique(
        pedia
            .custom_buildup_weapon_material
            .iter()
            .flat_map(|p| &p.param)
            .filter(|p| p.id != 0),
        |p| (p.id, p),
        false,
    )?;
    for category in pedia.custom_buildup_wep_table.iter().flat_map(|p| &p.param) {
        if category.table_no == 0 {
            continue;
        }
        let pieces = category
            .id
            .iter()
            .filter(|&&id| id != 0)
            .filter_map(|&id| {
                let data = if let Some(data) =
                    custom_buildup_pieces.remove(&(category.table_no, category.category_id, id))
                {
                    data
                } else {
                    // Crapcom: some is missing in v12.0.0, likely copy-paste error
                    eprintln!(
                        "Weapon custom buildup data not found for table {} category {} id {}",
                        category.table_no, category.category_id, id
                    );
                    return None;
                };

                let material = if let Some(material) = material.get(&id) {
                    *material
                } else {
                    return Some(Err(anyhow!(
                        "Weapon custom buildup material not found for {id}"
                    )));
                };
                Some(Ok((id, WeaponCustomBuildupPiece { data, material })))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let categories = &mut result.entry(category.table_no).or_default().categories;
        if categories
            .insert(category.category_id, WeaponCustomBuildupCategory { pieces })
            .is_some()
        {
            bail!(
                "Multiple weapon buildup definition for table {}, category {}",
                category.table_no,
                category.category_id
            );
        }
    }

    for slot_bonus in pedia
        .custom_buildup_slot_bonus
        .iter()
        .flat_map(|p| &p.param)
    {
        if result
            .get_mut(&slot_bonus.table_no)
            .with_context(|| format!("Table {} not found for slot bonus", slot_bonus.table_no))?
            .slot_bonus
            .insert(slot_bonus.id, slot_bonus)
            .is_some()
        {
            bail!(
                "Duplicate slot bonus for table {}, id {}",
                slot_bonus.table_no,
                slot_bonus.id
            )
        }
    }

    Ok(result)
}

pub fn prepare_supply(pedia: &Pedia) -> Result<HashMap<i32, &SupplyDataParam>> {
    hash_map_unique(
        pedia
            .supply_data
            .param
            .iter()
            .chain(&pedia.supply_data_mr.param)
            .filter(|p| p.id != 0),
        |p| (p.id, p),
        false,
    )
}

pub fn prepare_progress(pedia: &Pedia) -> Result<HashMap<i32, &ProgressCheckerUserDataParam>> {
    hash_map_unique(
        pedia
            .progress
            .param_list
            .iter()
            .filter(|p| p.progress_flag != 0),
        |p| (p.progress_flag, p),
        false,
    )
}

pub fn prepare_switch_skills(pedia: &Pedia) -> Result<HashMap<i32, SwitchSkill<'_>>> {
    let mut result = HashMap::new();
    for entry in pedia
        .switch_action_name
        .entries
        .iter()
        .chain(&pedia.switch_action_name_mr.entries)
    {
        if let Some(s) = entry.name.strip_prefix("PlayerSwitchAction_") {
            if let Some(s) = s.strip_suffix("_Name") {
                if let Ok(id) = s.parse::<i32>() {
                    result.insert(id, SwitchSkill { name: entry });
                }
            }
        }
    }
    Ok(result)
}

pub fn prepare_buff_cage(pedia: &Pedia) -> Result<BTreeMap<LvBuffCageId, BuffCage<'_>>> {
    let name = pedia.buff_cage_name.get_name_map();
    let explain = pedia.buff_cage_explain.get_name_map();
    let mut result = BTreeMap::new();
    for param in &pedia.buff_cage.param {
        if result.contains_key(&param.id) {
            bail!("Duplicate buff cage {:?}", param.id)
        }
        let (name_tag, explain_tag) = if let LvBuffCageId::Normal(id) = param.id {
            (
                format!("LvC_Normal_{id:03}_Name"),
                format!("LvC_Normal_{id:03}_Explain"),
            )
        } else {
            bail!("Unknown buff cage ID {:?}", param.id)
        };
        let name = name
            .get(&name_tag)
            .with_context(|| format!("Name not found for {:?}", param.id))?;
        let explain = explain
            .get(&explain_tag)
            .with_context(|| format!("Explain not found for {:?}", param.id))?;
        result.insert(
            param.id,
            BuffCage {
                name,
                explain,
                data: param,
            },
        );
    }
    Ok(result)
}

fn prepare_item_shop_lot<'a>(
    pedia: &'a Pedia,
    reward_lot: &'_ HashMap<u32, &'a RewardIdLotTableUserDataParam>,
) -> Result<Vec<ItemShopLot<'a>>> {
    let mut result: Vec<ItemShopLot> = vec![];
    for lot in &pedia.item_shop_lot.param {
        let reward_tables = lot
            .table_id_list
            .iter()
            .map(|id| {
                reward_lot
                    .get(id)
                    .copied()
                    .with_context(|| format!("Reward table {id} not found for item shop lot"))
            })
            .collect::<Result<Vec<_>>>()?;
        result.push(ItemShopLot {
            data: lot,
            reward_tables,
        })
    }

    result.sort_by_key(|r| (r.data.rank_type, r.data.lot_type));
    Ok(result)
}

fn prepare_bbq<'a>(
    pedia: &'a Pedia,
    reward_lot: &'_ HashMap<u32, &'a RewardIdLotTableUserDataParam>,
) -> Result<Vec<BbqData<'a>>> {
    let mut result = pedia
        .bbq
        .param
        .iter()
        .map(|param| {
            let table = (param.table_id != 0)
                .then(|| {
                    reward_lot
                        .get(&param.table_id)
                        .copied()
                        .with_context(|| format!("BBQ reward not found for {}", param.table_id))
                })
                .transpose()?;
            Ok(BbqData { param, table })
        })
        .collect::<Result<Vec<BbqData>>>()?;
    result.sort_by_key(|p| p.param.sort_id);
    Ok(result)
}

fn prepare_npc_mission(pedia: &'_ Pedia) -> Result<BTreeMap<i32, NpcMission<'_>>> {
    let mut result: BTreeMap<i32, NpcMission> = BTreeMap::new();

    let all_msg = pedia
        .npc_mission_msg
        .entries
        .iter()
        .chain(&pedia.npc_mission_msg_mr.entries);

    let all_msg = hash_map_unique(all_msg, |e| (&e.name, e), false)?;

    for param in pedia
        .npc_mission
        .param
        .iter()
        .chain(&pedia.npc_mission_mr.param)
    {
        let tag = format!("NSQ{:03}", param.id);
        let name_tag = format!("{tag}_01");
        let requester_tag = format!("{tag}_02");
        let detail_tag = format!("{tag}_03");
        let reward_tag = format!("{tag}_04");
        let target_tag = format!("{tag}_09");
        let name = *all_msg
            .get(&name_tag)
            .with_context(|| format!("{name_tag} not found"))?;
        let requester = *all_msg
            .get(&requester_tag)
            .with_context(|| format!("{requester_tag} not found"))?;
        let detail = *all_msg
            .get(&detail_tag)
            .with_context(|| format!("{detail_tag} not found"))?;
        let reward = all_msg.get(&reward_tag).copied();
        let target = all_msg.get(&target_tag).copied();
        let mission = NpcMission {
            param,
            name,
            requester,
            detail,
            target,
            reward,
        };
        if result.insert(param.id, mission).is_some() {
            bail!("duplicate NPC mission {}", param.id)
        }
    }

    Ok(result)
}

pub fn prepare_dlc(pedia: &'_ Pedia) -> Result<BTreeMap<i32, Dlc<'_>>> {
    let mut result = BTreeMap::new();

    let mut dlc_adds = hash_map_unique(
        pedia
            .dlc_add
            .add_data_info_list
            .iter()
            .filter(|p| p.dlc_id != 0 && p.slc_id == SaveLinkContents::Invalid),
        |p| (p.dlc_id, p),
        true, // crapcom: there seems to be duplicated identical stuff...
    )?;

    let mut item_pack = hash_map_unique(&pedia.item_pack.param, |p| (p.dlc_id, p), false)?;

    let names = pedia.dlc_name.get_name_map();
    let names_mr = pedia.dlc_name_mr.get_name_map();
    let explains = pedia.dlc_explain.get_name_map();
    let explains_mr = pedia.dlc_explain_mr.get_name_map();

    for dlc in &pedia.dlc.data_list {
        let add = dlc_adds.remove(&dlc.dlc_id);
        let item_pack = item_pack.remove(&dlc.dlc_id);
        let name = names
            .get(&dlc.title_msg_id)
            .or_else(|| names_mr.get(&dlc.title_msg_id))
            .cloned();
        let explain = explains
            .get(&dlc.explain_msg_id)
            .or_else(|| explains_mr.get(&dlc.explain_msg_id))
            .cloned();
        let entry = Dlc {
            data: dlc,
            add,
            item_pack,
            name,
            explain,
        };
        if result.insert(dlc.dlc_id, entry).is_some() {
            bail!("duplicate dlc {}", dlc.dlc_id)
        }
    }

    if !dlc_adds.is_empty() {
        bail!("Leftover dlc add: {:?}", dlc_adds)
    }

    if !item_pack.is_empty() {
        bail!("Left over item pack: {:?}", item_pack)
    }

    Ok(result)
}

pub fn prepare_slc(pedia: &'_ Pedia) -> Result<BTreeMap<SaveLinkContents, Slc<'_>>> {
    let mut result = BTreeMap::new();

    for add in pedia
        .dlc_add
        .add_data_info_list
        .iter()
        .filter(|add| add.slc_id != SaveLinkContents::Invalid)
    {
        if result
            .insert(
                add.slc_id,
                Slc {
                    add: Some(add),
                    item_pack: None,
                },
            )
            .is_some()
        {
            bail!("Duplicate SLC add for {:?}", add.slc_id)
        }
    }

    for item_pack in &pedia.slc_item_pack.param {
        let entry = result.entry(item_pack.save_link_id).or_default();
        if entry.item_pack.is_some() {
            bail!("Duplicate SLC item pack for {:?}", item_pack.save_link_id);
        }
        entry.item_pack = Some(item_pack);
    }

    Ok(result)
}

pub fn gen_pedia_ex(pedia: &Pedia) -> Result<PediaEx<'_>> {
    let monster_order = pedia
        .monster_list
        .data_list
        .iter()
        .enumerate()
        .map(|(i, monster)| (monster.em_type, i))
        .collect();

    let mut hyakuryu_weapon_map: HashMap<
        WeaponId,
        BTreeMap<i32, &HyakuryuWeaponHyakuryuBuildupUserDataParam>,
    > = HashMap::new();
    for param in &pedia.hyakuryu_weapon_buildup.param {
        let sub_map = hyakuryu_weapon_map.entry(param.weapon_id).or_default();
        if sub_map.insert(param.slot_type, param).is_some() {
            bail!(
                "Multiple hyakuryu weapon buildup for weapon/slot {:?}/{}",
                param.weapon_id,
                param.slot_type
            );
        }
    }

    let reward_lot = hash_map_unique(
        pedia
            .reward_id_lot_table
            .param
            .iter()
            .chain(&pedia.reward_id_lot_table_mr.param),
        |param| (param.id, param),
        false,
    )?;

    let mut custom_buildup_pieces = hash_map_unique(
        pedia
            .custom_buildup_base
            .iter()
            .flat_map(|p| &p.param)
            .filter(|p| p.table_no != 0),
        |p| ((p.table_no, p.category_id, p.id), p),
        true, // there seems to be a small bug with (table8, category7, id51)
    )?;

    let armor_custom_buildup = prepare_armor_custom_buildup(pedia, &mut custom_buildup_pieces)?;
    let weapon_custom_buildup = prepare_weapon_custom_buildup(pedia, &mut custom_buildup_pieces)?;

    if !custom_buildup_pieces.is_empty() {
        bail!("Leftover custom buildup pieces {custom_buildup_pieces:?}");
    }

    let mut chaos = if let Some(chaos) = &pedia.weapon_chaos_critical {
        hash_map_unique(&chaos.param, |p| (p.weapon, p), false)?
    } else {
        HashMap::new()
    };

    Ok(PediaEx {
        monsters: prepare_monsters(pedia, &reward_lot)?,
        sizes: prepare_size_map(&pedia.size_list)?,
        size_dists: prepare_size_dist_map(&pedia.random_scale)?,
        quests: prepare_quests(pedia, &reward_lot)?,
        npc_missions: prepare_npc_mission(pedia)?,
        skills: prepare_skills(pedia)?,
        hyakuryu_skills: prepare_hyakuryu_skills(pedia)?,
        armors: prepare_armors(pedia)?,
        armor_buildup: prepare_armor_buildup(pedia)?,
        meat_names: prepare_meat_names(pedia)?,
        items: prepare_items(pedia)?,
        material_categories: prepare_material_categories(pedia),
        monster_lot: prepare_monster_lot(pedia)?,
        parts_dictionary: prepare_parts_dictionary(pedia)?,

        great_sword: prepare_weapon(&pedia.great_sword, &mut hyakuryu_weapon_map, &mut chaos)?,
        short_sword: prepare_weapon(&pedia.short_sword, &mut hyakuryu_weapon_map, &mut chaos)?,
        hammer: prepare_weapon(&pedia.hammer, &mut hyakuryu_weapon_map, &mut chaos)?,
        lance: prepare_weapon(&pedia.lance, &mut hyakuryu_weapon_map, &mut chaos)?,
        long_sword: prepare_weapon(&pedia.long_sword, &mut hyakuryu_weapon_map, &mut chaos)?,
        slash_axe: prepare_weapon(&pedia.slash_axe, &mut hyakuryu_weapon_map, &mut chaos)?,
        gun_lance: prepare_weapon(&pedia.gun_lance, &mut hyakuryu_weapon_map, &mut chaos)?,
        dual_blades: prepare_weapon(&pedia.dual_blades, &mut hyakuryu_weapon_map, &mut chaos)?,
        horn: prepare_weapon(&pedia.horn, &mut hyakuryu_weapon_map, &mut chaos)?,
        insect_glaive: prepare_weapon(&pedia.insect_glaive, &mut hyakuryu_weapon_map, &mut chaos)?,
        charge_axe: prepare_weapon(&pedia.charge_axe, &mut hyakuryu_weapon_map, &mut chaos)?,
        light_bowgun: prepare_weapon(&pedia.light_bowgun, &mut hyakuryu_weapon_map, &mut chaos)?,
        heavy_bowgun: prepare_weapon(&pedia.heavy_bowgun, &mut hyakuryu_weapon_map, &mut chaos)?,
        bow: prepare_weapon(&pedia.bow, &mut hyakuryu_weapon_map, &mut chaos)?,
        horn_melody: prepare_horn_melody(pedia),
        monster_order,
        item_pop: prepare_item_pop(pedia)?,
        ot_equip: prepeare_ot_equip(pedia)?,
        servant: prepare_servant(pedia)?,

        armor_custom_buildup,
        weapon_custom_buildup,

        supply: prepare_supply(pedia)?,
        progress: prepare_progress(pedia)?,

        switch_skills: prepare_switch_skills(pedia)?,
        buff_cage: prepare_buff_cage(pedia)?,
        item_shop_lot: prepare_item_shop_lot(pedia, &reward_lot)?,
        bbq: prepare_bbq(pedia, &reward_lot)?,

        dlc: prepare_dlc(pedia)?,
        slc: prepare_slc(pedia)?,
    })
}
