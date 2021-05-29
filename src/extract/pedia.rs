use crate::msg::*;
use crate::rsz::*;
use serde::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ColliderMapping {
    pub meat_map: BTreeMap<usize, BTreeSet<String>>,
    pub part_map: BTreeMap<usize, BTreeSet<String>>,
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub sub_id: u32,
    pub enemy_type: Option<i32>,
    pub data_base: EnemyDataBase,
    pub data_tune: EnemyDataTune,
    pub meat_data: EnemyMeatData,
    pub condition_damage_data: EnemyConditionDamageData,
    pub anger_data: EnemyAngerData,
    pub parts_break_data: EnemyPartsBreakData,
    pub boss_init_set_data: Option<EnemyBossInitSetData>,
    pub collider_mapping: ColliderMapping,
    pub drop_item: EnemyDropItemInfoData,
    pub parts_break_reward: Option<EnemyPartsBreakRewardData>,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
    pub small_monsters: Vec<Monster>,
    pub monster_names: Msg,
    pub monster_aliases: Msg,
    pub condition_preset: EnemyConditionPresetData,
    pub monster_list: MonsterListBossData,
    pub hunter_note_msg: Msg,

    pub monster_lot: MonsterLotTableUserData,
    pub parts_type: PartsTypeTextUserData,

    pub normal_quest_data: NormalQuestData,
    pub normal_quest_data_for_enemy: NormalQuestDataForEnemy,
    pub difficulty_rate: SystemDifficultyRateData,
    pub random_scale: EnemyBossRandomScaleData,
    pub size_list: EnemySizeListData,
    pub discover_em_set_data: DiscoverEmSetData,
    pub quest_hall_msg: Msg,
    pub quest_village_msg: Msg,
    pub quest_tutorial_msg: Msg,
    pub quest_arena_msg: Msg,

    pub armor: ArmorBaseUserData,
    pub armor_series: ArmorSeriesUserData,
    pub armor_product: ArmorProductUserData,
    pub overwear: PlOverwearBaseUserData,
    pub overwear_product: PlOverwearProductUserData,
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

    pub hyakuryu_skill: PlHyakuryuSkillBaseUserData,
    pub hyakuryu_skill_recipe: PlHyakuryuSkillRecipeUserData,

    pub alchemy_pattern: AlchemyPatturnUserData,
    pub alchemy_pl_skill: AlchemyPlSkillTableUserData,
    pub alchemy_grade_worth: GradeWorthTableUserData,
    pub alchemy_rare_type: RareTypeTableUserData,
    pub alchemy_second_skill_lot: SecondSkillLotRateTableUserData,
    pub alchemy_skill_grade_lot: SkillGradeLotRateTableUserData,
    pub alchemy_slot_num: SlotNumTableUserData,
    pub alchemy_slot_worth: SlotWorthTableUserData,

    pub items: ItemUserData,
    pub items_name_msg: Msg,
    pub material_category_msg: Msg,
}

pub struct Quest<'a> {
    pub param: &'a NormalQuestDataParam,
    pub enemy_param: Option<&'a NormalQuestDataForEnemyParam>,
    pub name: Option<MsgEntry>,
    pub target: Option<MsgEntry>,
    pub condition: Option<MsgEntry>,
}

pub struct Skill {
    pub name: MsgEntry,
    pub explain: MsgEntry,
    pub levels: Vec<MsgEntry>,
    pub icon_color: i32,
}

pub struct Armor<'a> {
    pub name: MsgEntry,
    pub data: &'a ArmorBaseUserDataParam,
    pub product: Option<&'a ArmorProductUserDataParam>,
    pub overwear: Option<&'a PlOverwearBaseUserDataParam>,
    pub overwear_product: Option<&'a PlOverwearProductUserDataParam>,
}

pub struct ArmorSeries<'a> {
    pub name: Option<MsgEntry>,
    pub series: &'a ArmorSeriesUserDataParam,
    pub pieces: [Option<Armor<'a>>; 10],
}

#[derive(Hash, PartialEq, Eq)]
pub struct MeatKey {
    pub em_type: EmTypes,
    pub part: usize,
    pub phase: usize,
}

pub struct Item<'a> {
    pub name: MsgEntry,
    pub param: &'a ItemUserDataParam,
    pub multiple_def: bool,
}

pub struct PediaEx<'a> {
    pub sizes: HashMap<EmTypes, &'a SizeInfo>,
    pub size_dists: HashMap<i32, &'a [ScaleAndRateData]>,
    pub quests: Vec<Quest<'a>>,
    pub discoveries: HashMap<EmTypes, &'a DiscoverEmSetDataParam>,
    pub skills: BTreeMap<PlEquipSkillId, Skill>,
    pub armors: Vec<ArmorSeries<'a>>,
    pub meat_names: HashMap<MeatKey, MsgEntry>,
    pub items: BTreeMap<ItemId, Item<'a>>,
    pub material_categories: HashMap<MaterialCategory, MsgEntry>,
    pub monster_lot: HashMap<(EmTypes, QuestRank), &'a MonsterLotTableUserDataParam>,
    pub parts_dictionary: HashMap<(EmTypes, BrokenPartsTypes), MsgEntry>,
}
