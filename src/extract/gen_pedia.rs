use super::pedia::*;
use crate::msg::*;
use crate::pak::PakReader;
use crate::pfb::Pfb;
use crate::rsz::*;
use crate::user::User;
use anyhow::*;
use std::fs::File;
use std::io::{Cursor, Read, Seek};

fn exactly_one<T>(mut iterator: impl Iterator<Item = T>) -> Result<T> {
    let next = iterator.next().context("No element found")?;
    if iterator.next().is_some() {
        bail!("Multiple elements found");
    }
    Ok(next)
}

pub fn gen_monsters(
    pak: &mut PakReader<impl Read + Seek>,
    pfb_path_gen: fn(u32) -> String,
    boss_init_path_gen: fn(u32) -> Option<String>,
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

        let data_base = sub_file(pak, &main_pfb)?;
        let data_tune = sub_file(pak, &main_pfb)?;
        let meat_data = sub_file(pak, &main_pfb)?;
        let condition_damage_data = sub_file(pak, &main_pfb)?;
        let anger_data = sub_file(pak, &main_pfb)?;
        let parts_break_data = sub_file(pak, &main_pfb)?;

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

        monsters.push(Monster {
            id,
            data_base,
            data_tune,
            meat_data,
            condition_damage_data,
            anger_data,
            parts_break_data,
            boss_init_set_data,
        })
    }

    Ok(monsters)
}

pub fn gen_pedia(pak: String) -> Result<Pedia> {
    let mut pak = PakReader::new(File::open(pak)?)?;

    let monsters = gen_monsters(
        &mut pak,
        |id| format!("enemy/em{0:03}/00/prefab/em{0:03}_00.pfb", id),
        |id| {
            Some(format!(
                "enemy/em{0:03}/00/user_data/em{0:03}_00_boss_init_set_data.user",
                id
            ))
        },
    )
    .context("Generating large monsters")?;

    let small_monsters = gen_monsters(
        &mut pak,
        |id| format!("enemy/ems{0:03}/00/prefab/ems{0:03}_00.pfb", id),
        |_| None,
    )
    .context("Generating small monsters")?;

    let (monster_names, _) = pak.find_file("Message/Tag/Tag_EM_Name.msg")?;
    let monster_names = Msg::new(Cursor::new(pak.read_file(monster_names)?))?;

    let (monster_aliases, _) = pak.find_file("Message/Tag/Tag_EM_Name_Alias.msg")?;
    let monster_aliases = Msg::new(Cursor::new(pak.read_file(monster_aliases)?))?;

    Ok(Pedia {
        monsters,
        small_monsters,
        monster_names,
        monster_aliases,
    })
}
