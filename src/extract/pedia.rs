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
pub struct AttackCollider {
    pub is_shell: bool,
    pub data: EmBaseHitAttackRSData,
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub sub_id: u32,
    pub enemy_type: Option<i32>,
    pub em_type: EmTypes,
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
    pub atk_colliders: Vec<AttackCollider>,
}

#[derive(Debug, Serialize)]
pub struct WeaponList<BaseData> {
    pub base_data: BaseData,
    pub product: WeaponProductUserData,
    pub change: WeaponChangeUserData,
    pub process: WeaponProcessUserData,
    pub tree: WeaponUpdateTreeUserData,
    pub overwear: Option<OverwearWeaponBaseUserData>,
    pub overwear_product: Option<OverwearWeaponProductUserData>,
    pub name: Msg,
    pub explain: Msg,
    pub name_mr: Msg,
    pub explain_mr: Msg,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
    pub small_monsters: Vec<Monster>,
    pub monster_names: Msg,
    pub monster_aliases: Msg,
    pub monster_explains: Msg,
    pub monster_names_mr: Msg,
    pub monster_aliases_mr: Msg,
    pub monster_explains_mr: Msg,
    pub condition_preset: EnemyConditionPresetData,
    pub monster_list: MonsterListBossData,
    pub hunter_note_msg: Msg,
    pub hunter_note_msg_mr: Msg,
    pub monster_lot: MonsterLotTableUserDataLrHr,
    pub monster_lot_mr: MonsterLotTableUserDataMr,
    pub parts_type: PartsTypeTextUserData,
    pub normal_quest_data: BaseNormalQuestDataLrHr,
    pub normal_quest_data_mr: BaseNormalQuestDataMr,
    pub normal_quest_data_for_enemy: BaseNormalQuestDataForEnemyLrHr,
    pub normal_quest_data_for_enemy_mr: BaseNormalQuestDataForEnemyMr,
    pub dl_quest_data: DlNormalQuestDataLrHr,
    pub dl_quest_data_for_enemy: DlNormalQuestDataForEnemyLrHr,
    pub dl_quest_data_mr: Option<DlNormalQuestDataMr>,
    pub dl_quest_data_for_enemy_mr: Option<DlNormalQuestDataForEnemyMr>,
    pub difficulty_rate: SystemDifficultyRateDataNormal,
    pub difficulty_rate_anomaly: Option<SystemDifficultyRateDataAnomaly>,
    pub random_scale: EnemyBossRandomScaleData,
    pub size_list: EnemySizeListData,
    pub discover_em_set_data: DiscoverEmSetData,
    pub quest_data_for_reward: QuestDataForRewardUserDataLrHr,
    pub quest_data_for_reward_mr: QuestDataForRewardUserDataMr,
    pub reward_id_lot_table: RewardIdLotTableUserDataLrHr,
    pub reward_id_lot_table_mr: RewardIdLotTableUserDataMr,
    pub main_target_reward_lot_num: MainTargetRewardLotNumDefineUserData,
    pub fixed_hyakuryu_quest: HyakuryuQuestDataTbl,
    pub mystery_reward_item: MysteryRewardItemUserData,
    pub quest_servant: QuestServantDataList,
    pub supply_data: SupplyDataLrHr,
    pub supply_data_mr: SupplyDataMr,
    pub arena_quest: ArenaQuestData,
    pub quest_unlock: QuestUnlockRequestListUserData,
    pub time_attack_reward: TimeAttackRewardUserData,
    pub quest_hall_msg: Msg,
    pub quest_hall_msg_mr: Msg,
    pub quest_hall_msg_mr2: Msg,
    pub quest_village_msg: Msg,
    pub quest_village_msg_mr: Msg,
    pub quest_tutorial_msg: Msg,
    pub quest_arena_msg: Msg,
    pub quest_dlc_msg: Msg,

    pub armor: ArmorBaseUserData,
    pub armor_series: ArmorSeriesUserData,
    pub armor_product: ArmorProductUserData,
    pub overwear: PlOverwearBaseUserData,
    pub overwear_product: PlOverwearProductUserData,
    pub armor_buildup: ArmorBuildupTableUserData,
    pub armor_pair: ArmorSeriesPairUserData,
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
    pub armor_head_name_msg_mr: Msg,
    pub armor_chest_name_msg_mr: Msg,
    pub armor_arm_name_msg_mr: Msg,
    pub armor_waist_name_msg_mr: Msg,
    pub armor_leg_name_msg_mr: Msg,
    pub armor_head_explain_msg_mr: Msg,
    pub armor_chest_explain_msg_mr: Msg,
    pub armor_arm_explain_msg_mr: Msg,
    pub armor_waist_explain_msg_mr: Msg,
    pub armor_leg_explain_msg_mr: Msg,
    pub armor_series_name_msg_mr: Msg,

    pub equip_skill: PlEquipSkillBaseUserData,
    pub player_skill_detail_msg: Msg,
    pub player_skill_explain_msg: Msg,
    pub player_skill_name_msg: Msg,
    pub player_skill_detail_msg_mr: Msg,
    pub player_skill_explain_msg_mr: Msg,
    pub player_skill_name_msg_mr: Msg,

    pub hyakuryu_skill: PlHyakuryuSkillBaseUserData,
    pub hyakuryu_skill_recipe: PlHyakuryuSkillRecipeUserData,
    pub hyakuryu_skill_name_msg: Msg,
    pub hyakuryu_skill_explain_msg: Msg,
    pub hyakuryu_skill_name_msg_mr: Msg,
    pub hyakuryu_skill_explain_msg_mr: Msg,

    pub decorations: DecorationsBaseUserData,
    pub decorations_product: DecorationsProductUserData,
    pub decorations_name_msg: Msg,
    pub decorations_name_msg_mr: Msg,

    pub hyakuryu_decos: HyakuryuDecoBaseUserData,
    pub hyakuryu_decos_product: HyakuryuDecoProductUserData,
    pub hyakuryu_decos_name_msg: Msg,

    //pub alchemy_pattern: AlchemyPatturnUserData,
    pub alchemy_pl_skill: AlchemyPlSkillTableUserData,
    /*pub alchemy_grade_worth: GradeWorthTableUserData,
    pub alchemy_rare_type: RareTypeTableUserData,
    pub alchemy_second_skill_lot: SecondSkillLotRateTableUserData,
    pub alchemy_skill_grade_lot: SkillGradeLotRateTableUserData,
    pub alchemy_slot_num: SlotNumTableUserData,
    pub alchemy_slot_worth: SlotWorthTableUserData,*/
    pub items: ItemUserData,
    pub items_name_msg: Msg,
    pub items_explain_msg: Msg,
    pub items_name_msg_mr: Msg,
    pub items_explain_msg_mr: Msg,
    pub material_category_msg: Msg,
    pub material_category_msg_mr: Msg,

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
    pub horn_melody_mr: Msg,
    pub hyakuryu_weapon_buildup: HyakuryuWeaponHyakuryuBuildupUserData,
    pub weapon_chaos_critical: Option<WeaponChaosCriticalUserData>,

    pub maps: BTreeMap<i32, GameMap>,
    pub map_name: Msg,
    pub map_name_mr: Msg,
    pub item_pop_lot: ItemPopLotTableUserData,
    pub airou_armor: OtAirouArmorBaseUserData,
    pub airou_armor_product: OtAirouArmorProductUserData,
    pub dog_armor: OtDogArmorBaseUserData,
    pub dog_armor_product: OtDogArmorProductUserData,
    pub airou_weapon: OtAirouWeaponBaseUserData,
    pub airou_weapon_product: OtAirouWeaponProductUserData,
    pub dog_weapon: OtDogWeaponBaseUserData,
    pub dog_weapon_product: OtDogWeaponProductUserData,
    pub ot_equip_series: OtEquipSeriesUserData,
    pub airou_armor_head_name: Msg,
    pub airou_armor_head_explain: Msg,
    pub airou_armor_chest_name: Msg,
    pub airou_armor_chest_explain: Msg,
    pub dog_armor_head_name: Msg,
    pub dog_armor_head_explain: Msg,
    pub dog_armor_chest_name: Msg,
    pub dog_armor_chest_explain: Msg,
    pub airou_weapon_name: Msg,
    pub airou_weapon_explain: Msg,
    pub dog_weapon_name: Msg,
    pub dog_weapon_explain: Msg,
    pub airou_series_name: Msg,
    pub dog_series_name: Msg,
    pub airou_armor_head_name_mr: Msg,
    pub airou_armor_head_explain_mr: Msg,
    pub airou_armor_chest_name_mr: Msg,
    pub airou_armor_chest_explain_mr: Msg,
    pub dog_armor_head_name_mr: Msg,
    pub dog_armor_head_explain_mr: Msg,
    pub dog_armor_chest_name_mr: Msg,
    pub dog_armor_chest_explain_mr: Msg,
    pub airou_weapon_name_mr: Msg,
    pub airou_weapon_explain_mr: Msg,
    pub dog_weapon_name_mr: Msg,
    pub dog_weapon_explain_mr: Msg,
    pub airou_series_name_mr: Msg,
    pub dog_series_name_mr: Msg,

    pub servant_profile: Msg,

    pub custom_buildup_base: Option<CustomBuildupBaseUserData>,
    pub custom_buildup_armor_open: Option<CustomBuildupArmorOpenUserData>,
    pub custom_buildup_weapon_open: Option<CustomBuildupWeaponOpenUserData>,
    pub custom_buildup_armor_material: Option<CustomBuildupArmorMaterialUserData>,
    pub custom_buildup_weapon_material: Option<CustomBuildupWeaponMaterialUserData>,
    pub custom_buildup_armor_lot: Option<CustomBuildupArmorLotUserData>,
    pub custom_buildup_armor_category_lot: Option<CustomBuildupArmorCategoryLotUserData>,
    pub custom_buildup_equip_skill_detail: Option<CustomBuildupEquipSkillDetailUserData>,
    pub custom_buildup_wep_table: Option<CustomBuildupWepTableUserData>,

    pub random_mystery_difficulty: Option<RandomMysteryDifficultyRateListData>,
    pub random_mystery_enemy: Option<RandomMysteryLotEnemyData>,
    pub random_mystery_rank_release: Option<RandomMysteryMonsterRankReleaseData>,

    pub progress: ProgressCheckerUserData,

    pub enemy_rank: EnemyRankData,
    pub species: SystemEnemyDragonSpeciesData,

    pub switch_action_name: Msg,
    pub switch_action_name_mr: Msg,
    pub weapon_control: Msg,
    pub weapon_control_mr: Msg,

    pub buff_cage: NormalLvBuffCageBaseUserData,
    pub buff_cage_name: Msg,
    pub buff_cage_explain: Msg,

    pub item_shop: ItemShopDisplayUserData,
    pub item_shop_lot: ItemShopLotUserData,
    pub fukudama: ShopFukudamaUserData,
    pub mystery_labo_trade_item: Option<MysteryLaboTradeItemUserData>,
    pub item_mix: ItemMixRecipeUserData,
    pub bbq: BbqConvertUserData,

    pub exchange_item: ExchangeItemUserData,
    pub trade_dust: TradeDustUserData,
    pub trade_feature: TradeFeatureUserData,
    pub trade_rare: TradeRareUserData,
    pub trade: TradeUserData,

    pub spy: OtomoSpyUnitGridUserData,
}

pub struct QuestReward<'a> {
    pub param: &'a QuestDataForRewardUserDataParam,
    pub additional_target_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub common_material_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub additional_quest_reward: Vec<&'a RewardIdLotTableUserDataParam>,
    pub cloth_ticket: Option<&'a RewardIdLotTableUserDataParam>,
}

#[derive(Debug)]
pub enum QuestUnlock<'a> {
    Group(&'a QuestUnlockRelation),
    Talk(&'a QuestUnlockByTalkFlag),
    Clear(&'a QuestUnlockByQuestClear),
    Enemy(&'a QuestUnlockByHuntEnemy),
}

pub struct TimeAttackReward<'a> {
    pub reward: &'a RewardIdLotTableUserDataParam,
    pub rank: &'a RankData,
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
    pub servant: Option<&'a QuestServantData>,
    pub arena: Option<&'a ArenaQuestDataParam>,
    pub unlock: Vec<QuestUnlock<'a>>,
    pub random_group: Option<&'a RandomQuestUnlockByQuestClear>,
    pub time_attack_reward: Vec<TimeAttackReward<'a>>,
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
    pub decos: Vec<Deco<'a>>,
    pub custom_buildup_cost: Option<u32>,
    pub alchemy: BTreeMap<AlchemyPatturnTypes, &'a AlchemyPlSkillTableUserDataParam>,
    pub alchemy_grade: Option<GradeTypes>,
}

pub struct HyakuryuDeco<'a> {
    pub data: &'a HyakuryuDecoBaseUserDataParam,
    pub product: &'a HyakuryuDecoProductUserDataParam,
    pub name: &'a MsgEntry,
}

pub struct HyakuryuSkill<'a> {
    pub data: Option<&'a PlHyakuryuSkillBaseUserDataParam>,
    pub recipe: Option<&'a PlHyakuryuSkillRecipeUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub deco: Option<HyakuryuDeco<'a>>,
}

impl<'a> HyakuryuSkill<'a> {
    pub fn id(&self) -> PlHyakuryuSkillId {
        if let Some(data) = self.data {
            data.id
        } else {
            self.deco.as_ref().unwrap().data.hyakuryu_skill_id
        }
    }

    pub fn color(&self) -> i32 {
        if let Some(data) = self.data {
            data.item_color
        } else {
            self.deco.as_ref().unwrap().data.icon_color
        }
    }
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
    pub name: &'a MsgEntry,
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
    pub overwear: Option<&'a OverwearWeaponProductUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: Option<&'a MsgEntry>,
    pub children: Vec<WeaponId>,
    pub parent: Option<WeaponId>,
    pub hyakuryu_weapon_buildup: BTreeMap<i32, &'a HyakuryuWeaponHyakuryuBuildupUserDataParam>,
    pub update: Option<&'a WeaponUpdateTreeUserDataParam>,
    pub chaos: Option<&'a WeaponChaosCriticalUserDataParam>,
}

pub struct WeaponTree<'a, Param> {
    pub weapons: BTreeMap<WeaponId, Weapon<'a, Param>>,
    pub roots: Vec<WeaponId>,
    pub unpositioned: Vec<WeaponId>,
}

pub struct OtWeapon<'a> {
    pub param: &'a OtWeaponBaseUserDataParam,
    pub product: Option<&'a OtWeaponProductUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
}

pub struct OtArmor<'a> {
    pub param: &'a OtArmorBase,
    pub product: Option<&'a OtArmorProductUserDataParam>,
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
}

pub struct OtEquipSeries<'a> {
    pub series: &'a OtEquipSeriesUserDataParam,
    pub name: &'a MsgEntry,
    pub weapon: Option<OtWeapon<'a>>,
    pub head: Option<OtArmor<'a>>,
    pub chest: Option<OtArmor<'a>>,
}

pub struct MysteryReward<'a> {
    pub lv_lower_limit: u32,
    pub lv_upper_limit: u32,
    pub hagibui_probability: u32,
    pub reward_item: ItemId,
    pub item_num: u32,
    pub quest_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub additional_quest_reward: Vec<&'a RewardIdLotTableUserDataParam>,
    pub special_quest_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub multiple_target_reward: Option<&'a RewardIdLotTableUserDataParam>,
    pub multiple_fix_reward: Option<&'a RewardIdLotTableUserDataParam>,
}

pub struct MonsterEx<'a> {
    pub data: &'a Monster,
    pub name: Option<&'a MsgEntry>,
    pub alias: Option<&'a MsgEntry>,
    pub explain1: Option<&'a MsgEntry>,
    pub explain2: Option<&'a MsgEntry>,
    pub mystery_reward: Vec<MysteryReward<'a>>,
    pub random_quest: Option<&'a LotEnemyData>,
    pub discovery: Option<&'a DiscoverEmSetDataParam>,
    pub rank: Option<u8>,
    pub species: Option<&'a EmSpeciesData>,
    pub family: Option<&'a MsgEntry>,
}

pub struct Servant<'a> {
    pub name: &'a MsgEntry,
}

#[derive(Debug)]
pub struct ArmorCustomBuildupPiece<'a> {
    pub lot: u32,
    pub data: &'a CustomBuildupBaseUserDataParam,
}

#[derive(Debug)]
pub struct ArmorCustomBuildupCategory<'a> {
    pub lot: u32,
    pub pieces: BTreeMap<u16, ArmorCustomBuildupPiece<'a>>,
}

#[derive(Debug)]
pub struct ArmorCustomBuildup<'a> {
    pub categories: BTreeMap<u16, ArmorCustomBuildupCategory<'a>>,
}

#[derive(Debug)]
pub struct WeaponCustomBuildupPiece<'a> {
    pub data: &'a CustomBuildupBaseUserDataParam,
    pub material: &'a CustomBuildupWeaponMaterialUserDataParam,
}

#[derive(Debug)]
pub struct WeaponCustomBuildupCategory<'a> {
    pub pieces: BTreeMap<u16, WeaponCustomBuildupPiece<'a>>,
}

#[derive(Default, Debug)]
pub struct WeaponCustomBuildup<'a> {
    pub categories: BTreeMap<u16, WeaponCustomBuildupCategory<'a>>,
}

pub struct SwitchSkill<'a> {
    pub name: &'a MsgEntry,
}

pub struct BuffCage<'a> {
    pub name: &'a MsgEntry,
    pub explain: &'a MsgEntry,
    pub data: &'a NormalLvBuffCageBaseUserDataParam,
}

pub struct ItemShopLot<'a> {
    pub data: &'a ItemShopLotUserDataParam,
    pub reward_tables: Vec<&'a RewardIdLotTableUserDataParam>,
}

pub struct BbqData<'a> {
    pub param: &'a BbqConvertUserDataParam,
    pub table: Option<&'a RewardIdLotTableUserDataParam>,
}

pub struct PediaEx<'a> {
    pub monsters: BTreeMap<EmTypes, MonsterEx<'a>>,
    pub sizes: HashMap<EmTypes, &'a SizeInfo>,
    pub size_dists: HashMap<i32, &'a [ScaleAndRateData]>,
    pub quests: BTreeMap<i32, Quest<'a>>,
    pub skills: BTreeMap<PlEquipSkillId, Skill<'a>>,
    pub hyakuryu_skills: BTreeMap<PlHyakuryuSkillId, HyakuryuSkill<'a>>,
    pub armors: BTreeMap<PlArmorSeriesTypes, ArmorSeries<'a>>,
    pub armor_buildup: HashMap<i32, Vec<&'a ArmorBuildupTableUserDataParam>>,
    pub meat_names: HashMap<MeatKey, Vec<&'a MsgEntry>>,

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
    pub ot_equip: BTreeMap<OtEquipSeriesId, OtEquipSeries<'a>>,

    pub servant: HashMap<i32, Servant<'a>>,

    pub armor_custom_buildup: HashMap<u32, ArmorCustomBuildup<'a>>,
    pub weapon_custom_buildup: HashMap<u32, WeaponCustomBuildup<'a>>,

    pub supply: HashMap<i32, &'a SupplyDataParam>,
    pub progress: HashMap<i32, &'a ProgressCheckerUserDataParam>,

    pub switch_skills: HashMap<i32, SwitchSkill<'a>>,
    pub buff_cage: BTreeMap<LvBuffCageId, BuffCage<'a>>,
    pub item_shop_lot: Vec<ItemShopLot<'a>>,
    pub bbq: Vec<BbqData<'a>>,
}
