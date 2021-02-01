use super::pedia::*;
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

pub fn gen_monsters(pak: &mut PakReader<impl Read + Seek>) -> Result<Vec<Monster>> {
    let mut monsters = vec![];

    for id in 0..1000 {
        let main_pfb_path = format!("enemy/em{0:03}/00/prefab/em{0:03}_00.pfb", id);
        let main_pfb_index = if let Ok((index, _)) = pak.find_file(&main_pfb_path) {
            index
        } else {
            continue;
        };
        let main_pfb = Pfb::new(Cursor::new(pak.read_file(main_pfb_index)?))?;

        let meat_data_path = &exactly_one(
            main_pfb
                .children
                .iter()
                .filter(|child| child.hash == EnemyMeatData::type_hash()),
        )?
        .name;
        let (meat_data_index, _) = pak.find_file(meat_data_path)?;
        let meat_data = User::new(Cursor::new(pak.read_file(meat_data_index)?))?;
        let meat_data = meat_data.rsz.deserialize_single()?;

        let condition_damage_path = &exactly_one(
            main_pfb
                .children
                .iter()
                .filter(|child| child.hash == EnemyConditionDamageData::type_hash()),
        )?
        .name;
        let (condition_damage_index, _) = pak.find_file(condition_damage_path)?;
        let condition_damage_data = User::new(Cursor::new(pak.read_file(condition_damage_index)?))?;
        let condition_damage_data = condition_damage_data.rsz.deserialize_single()?;

        monsters.push(Monster {
            id,
            meat_data,
            condition_damage_data,
        })
    }

    Ok(monsters)
}

pub fn gen_pedia(pak: String) -> Result<Pedia> {
    let mut pak = PakReader::new(File::open(pak)?)?;

    let monsters = gen_monsters(&mut pak)?;
    Ok(Pedia { monsters })
}
