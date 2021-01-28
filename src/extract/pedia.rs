use crate::file_ext::*;
use crate::rsz::FromRsz;
use anyhow::*;
use serde::*;

#[derive(Debug, Serialize)]
pub struct MeatGroupInfo {
    pub slash: u16,
    pub impact: u16,
    pub shot: u16,
    pub fire: u16,
    pub water: u16,
    pub thunder: u16,
    pub ice: u16,
    pub dragon: u16,
    pub dizzy: u16,
}

impl FromRsz for MeatGroupInfo {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatContainer.MeatGroupInfo";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(2)?;
        Ok(MeatGroupInfo {
            slash: rsz.read_u16()?,
            impact: rsz.read_u16()?,
            shot: rsz.read_u16()?,
            fire: rsz.read_u16()?,
            water: rsz.read_u16()?,
            thunder: rsz.read_u16()?,
            ice: rsz.read_u16()?,
            dragon: rsz.read_u16()?,
            dizzy: rsz.read_u16()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct EnemyMeatContainer {
    pub values: Vec<MeatGroupInfo>,
}

impl FromRsz for EnemyMeatContainer {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatContainer";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let values = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(EnemyMeatContainer { values })
    }
}

#[derive(Debug, Serialize)]
pub struct EnemyMeatData {
    pub packs: Vec<EnemyMeatContainer>,
}

impl FromRsz for EnemyMeatData {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let packs = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(EnemyMeatData { packs })
    }
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub meat_data: EnemyMeatData,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
}
