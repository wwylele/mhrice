use crate::rsz::*;
use serde::*;

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub meat_data: EnemyMeatData,
    pub condition_damage_data: EnemyConditionDamageData,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
}
