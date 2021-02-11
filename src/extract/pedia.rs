use crate::msg::*;
use crate::rsz::*;
use serde::*;

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub data_base: EnemyDataBase,
    pub data_tune: EnemyDataTune,
    pub meat_data: EnemyMeatData,
    pub condition_damage_data: EnemyConditionDamageData,
    pub anger_data: EnemyAngerData,
    pub parts_break_data: EnemyPartsBreakData,
    pub boss_init_set_data: Option<EnemyBossInitSetData>,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
    pub small_monsters: Vec<Monster>,
    pub monster_names: Msg,
    pub monster_aliases: Msg,
}
