use crate::msg::*;
use crate::rsz::*;
use serde::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Serialize)]
pub struct ColliderMapping {
    pub meat_map: BTreeMap<usize, BTreeSet<String>>,
    pub part_map: BTreeMap<usize, BTreeSet<String>>,
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub sub_id: u32,
    pub data_base: EnemyDataBase,
    pub data_tune: EnemyDataTune,
    pub meat_data: EnemyMeatData,
    pub condition_damage_data: EnemyConditionDamageData,
    pub anger_data: EnemyAngerData,
    pub parts_break_data: EnemyPartsBreakData,
    pub boss_init_set_data: Option<EnemyBossInitSetData>,
    pub collider_mapping: ColliderMapping,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
    pub small_monsters: Vec<Monster>,
    pub monster_names: Msg,
    pub monster_aliases: Msg,
    pub condition_preset: EnemyConditionPresetData,

    pub normal_quest_data: NormalQuestData,
    pub normal_quest_data_for_enemy: NormalQuestDataForEnemy,
    pub difficulty_rate: SystemDifficultyRateData,
    pub random_scale: EnemyBossRandomScaleData,
    pub quest_hall_msg: Msg,
    pub quest_village_msg: Msg,
    pub quest_tutorial_msg: Msg,
    pub quest_arena_msg: Msg,

    pub armor: ArmorBaseUserData,
    pub armor_series: ArmorSeriesUserData,
    pub armor_head_name_msg: Msg,
    pub armor_chest_name_msg: Msg,
    pub armor_arm_name_msg: Msg,
    pub armor_waist_name_msg: Msg,
    pub armor_leg_name_msg: Msg,
    pub armor_series_name_msg: Msg,

    pub equip_skill: PlEquipSkillBaseUserData,
    pub player_skill_detail_msg: Msg,
    pub player_skill_explain_msg: Msg,
    pub player_skill_name_msg: Msg,
}
