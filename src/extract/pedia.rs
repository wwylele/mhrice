use crate::file_ext::*;
use crate::rsz::FromRsz;
use anyhow::*;
use serde::*;

#[derive(Debug, Serialize)]
pub struct HitzoneValue {
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

impl FromRsz for HitzoneValue {
    const HASH: u64 = 0xD256A6BA7C55360E;
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(2)?;
        Ok(HitzoneValue {
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
pub struct HitzoneValuePack {
    pub values: Vec<HitzoneValue>,
}

impl FromRsz for HitzoneValuePack {
    const HASH: u64 = 0x8A1F374298E90BDE;
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let values = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(HitzoneValuePack { values })
    }
}

#[derive(Debug, Serialize)]
pub struct MonsterMeatData {
    pub packs: Vec<HitzoneValuePack>,
}

impl FromRsz for MonsterMeatData {
    const HASH: u64 = 0x6AD65290B5ABFE04;
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let packs = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(MonsterMeatData { packs })
    }
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub meat_data: MonsterMeatData,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
}
