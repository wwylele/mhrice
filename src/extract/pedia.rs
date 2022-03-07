use super::prepare_map::*;
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
pub struct WeaponList<BaseData> {
    pub base_data: BaseData,
    pub product: WeaponProductUserData,
    pub change: WeaponChangeUserData,
    pub process: WeaponProcessUserData,
    pub tree: WeaponUpdateTreeUserData,
    pub name: Msg,
    pub explain: Msg,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
    pub small_monsters: Vec<Monster>,
    pub monster_names: Msg,
    pub monster_aliases: Msg,
    pub monster_explains: Msg,
    pub condition_preset: EnemyConditionPresetData,
    pub monster_list: MonsterListBossData,
    pub hunter_note_msg: Msg,

    pub monster_lot: MonsterLotTableUserData,
    pub parts_type: PartsTypeTextUserData,

    pub normal_quest_data: NormalQuestData,
    pub normal_quest_data_for_enemy: NormalQuestDataForEnemy,
    pub dl_quest_data: NormalQuestData,
    pub dl_quest_data_for_enemy: NormalQuestDataForEnemy,
    pub difficulty_rate: SystemDifficultyRateData,
    pub random_scale: EnemyBossRandomScaleData,
    pub size_list: EnemySizeListData,
    pub discover_em_set_data: DiscoverEmSetData,
    pub quest_data_for_reward: QuestDataForRewardUserData,
    pub reward_id_lot_table: RewardIdLotTableUserData,
    pub main_target_reward_lot_num: MainTargetRewardLotNumDefineUserData,
    pub fixed_hyakuryu_quest: HyakuryuQuestDataTbl,
    pub quest_hall_msg: Msg,
    pub quest_village_msg: Msg,
    pub quest_tutorial_msg: Msg,
    pub quest_arena_msg: Msg,
    pub quest_dlc_msg: Msg,

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
    pub armor_head_explain_msg: Msg,
    pub armor_chest_explain_msg: Msg,
    pub armor_arm_explain_msg: Msg,
    pub armor_waist_explain_msg: Msg,
    pub armor_leg_explain_msg: Msg,
    pub armor_series_name_msg: Msg,

    pub equip_skill: PlEquipSkillBaseUserData,
    pub player_skill_detail_msg: Msg,
    pub player_skill_explain_msg: Msg,
    pub player_skill_name_msg: Msg,

    pub hyakuryu_skill: PlHyakuryuSkillBaseUserData,
    pub hyakuryu_skill_recipe: PlHyakuryuSkillRecipeUserData,
    pub hyakuryu_skill_name_msg: Msg,
    pub hyakuryu_skill_explain_msg: Msg,

    pub decorations: DecorationsBaseUserData,
    pub decorations_product: DecorationsProductUserData,
    pub decorations_name_msg: Msg,

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
    pub items_explain_msg: Msg,
    pub material_category_msg: Msg,

    pub great_sword: WeaponList<GreatSwordBaseUserData>,
    pub short_sword: WeaponList<ShortSwordBaseUserData>,
    pub hammer: WeaponList<HammerBaseUserData>,
    pub lance: WeaponList<LanceBaseUserData>,
    pub long_sword: WeaponList<LongSwordBaseUserData>,
    pub slash_axe: WeaponList<SlashAxeBaseUserData>,
    pub gun_lance: WeaponList<GunLanceBaseUserData>,
    pub dual_blades: WeaponList<DualBladesBaseUserData>,
    pub horn: WeaponList<HornBaseUserData>,
    pub insect_glaive: WeaponList<InsectGlaiveBaseUserData>,
    pub charge_axe: WeaponList<ChargeAxeBaseUserData>,
    pub light_bowgun: WeaponList<LightBowgunBaseUserData>,
    pub heavy_bowgun: WeaponList<HeavyBowgunBaseUserData>,
    pub bow: WeaponList<BowBaseUserData>,

    pub horn_melody: Msg,
    pub hyakuryu_weapon_buildup: HyakuryuWeaponHyakuryuBuildupUserData,

    pub maps: BTreeMap<i32, GameMap>,
    pub map_name: Msg,
    pub item_pop_lot: ItemPopLotTableUserData,
}

pub struct QuestReward<'a> {
    pub param: &'a QuestDataForRewardUserDataParam,
    pub additional_target_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub common_material_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub additional_quest_reward: Vec<&'a RewardIdLotTableUserDataParam>,
    pub cloth_ticket: Option<&'a RewardIdLotTableUserDataParam>,
}

pub struct Quest<'a> {
    pub param: &'a NormalQuestDataParam,
    pub enemy_param: Option<&'a NormalQuestDataForEnemyParam>,
    pub name: Option<&'a MsgEntry>,
    pub requester: Option<&'a MsgEntry>,
    pub detail: Option<&'a MsgEntry>,
    pub target: Option<&'a MsgEntry>,
    pub condition: Option<&'a MsgEntry>,
    pub is_dl: bool,
    pub reward: Option<QuestReward<'a>>,
    pub hyakuryu: Option<&'a HyakuryuQuestData>,
}

pub struct Deco<'a> {
    pub data: &'a DecorationsBaseUserDataParam,
    pub product: &'a DecorationsProductUserDataParam,
    pub name: &'a MsgEntry,
}

pub struct Skill<'a> {
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub levels: Vec<&'a MsgEntry>,
    pub icon_color: i32,
    pub deco: Option<Deco<'a>>,
}

pub struct HyakuryuSkill<'a> {
    pub data: &'a PlHyakuryuSkillBaseUserDataParam,
    pub recipe: Option<&'a PlHyakuryuSkillRecipeUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
}

pub struct Armor<'a> {
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub data: &'a ArmorBaseUserDataParam,
    pub product: Option<&'a ArmorProductUserDataParam>,
    pub overwear: Option<&'a PlOverwearBaseUserDataParam>,
    pub overwear_product: Option<&'a PlOverwearProductUserDataParam>,
}

pub struct ArmorSeries<'a> {
    pub name: Option<&'a MsgEntry>,
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
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub param: &'a ItemUserDataParam,
    pub multiple_def: bool,
}

pub struct Weapon<'a, Param> {
    pub param: &'a Param,
    pub product: Option<&'a WeaponProductUserDataParam>,
    pub change: Option<&'a WeaponChangeUserDataParam>,
    pub process: Option<&'a WeaponProcessUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub children: Vec<WeaponId>,
    pub parent: Option<WeaponId>,
    pub hyakuryu_weapon_buildup: BTreeMap<i32, &'a HyakuryuWeaponHyakuryuBuildupUserDataParam>,
}

pub struct WeaponTree<'a, Param> {
    pub weapons: BTreeMap<WeaponId, Weapon<'a, Param>>,
    pub roots: Vec<WeaponId>,
    pub unpositioned: Vec<WeaponId>,
}

pub struct PediaEx<'a> {
    pub sizes: HashMap<EmTypes, &'a SizeInfo>,
    pub size_dists: HashMap<i32, &'a [ScaleAndRateData]>,
    pub quests: Vec<Quest<'a>>,
    pub discoveries: HashMap<EmTypes, &'a DiscoverEmSetDataParam>,
    pub skills: BTreeMap<PlEquipSkillId, Skill<'a>>,
    pub hyakuryu_skills: BTreeMap<PlHyakuryuSkillId, HyakuryuSkill<'a>>,
    pub armors: Vec<ArmorSeries<'a>>,
    pub meat_names: HashMap<MeatKey, &'a MsgEntry>,
    pub items: BTreeMap<ItemId, Item<'a>>,
    pub material_categories: HashMap<MaterialCategory, &'a MsgEntry>,
    pub monster_lot: HashMap<(EmTypes, QuestRank), &'a MonsterLotTableUserDataParam>,
    pub parts_dictionary: HashMap<(EmTypes, BrokenPartsTypes), &'a MsgEntry>,

    pub great_sword: WeaponTree<'a, GreatSwordBaseUserDataParam>,
    pub short_sword: WeaponTree<'a, ShortSwordBaseUserDataParam>,
    pub hammer: WeaponTree<'a, HammerBaseUserDataParam>,
    pub lance: WeaponTree<'a, LanceBaseUserDataParam>,
    pub long_sword: WeaponTree<'a, LongSwordBaseUserDataParam>,
    pub slash_axe: WeaponTree<'a, SlashAxeBaseUserDataParam>,
    pub gun_lance: WeaponTree<'a, GunLanceBaseUserDataParam>,
    pub dual_blades: WeaponTree<'a, DualBladesBaseUserDataParam>,
    pub horn: WeaponTree<'a, HornBaseUserDataParam>,
    pub insect_glaive: WeaponTree<'a, InsectGlaiveBaseUserDataParam>,
    pub charge_axe: WeaponTree<'a, ChargeAxeBaseUserDataParam>,
    pub light_bowgun: WeaponTree<'a, LightBowgunBaseUserDataParam>,
    pub heavy_bowgun: WeaponTree<'a, HeavyBowgunBaseUserDataParam>,
    pub bow: WeaponTree<'a, BowBaseUserDataParam>,

    pub horn_melody: HashMap<i32, &'a MsgEntry>,

    pub monster_order: HashMap<EmTypes, usize>,
    pub item_pop: HashMap<(/*pop_id*/ i32, /*map*/ i32), &'a ItemPopLotTableUserDataParam>,
}
