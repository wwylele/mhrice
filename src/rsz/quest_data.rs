use super::*;
use crate::rsz_bitflags;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_bitflags! {
    #[derive(Serialize)]
    pub struct QuestType: u32 {
        const HUNTING  = 0x00000001;
        const KILL     = 0x00000002;
        const CAPTURE  = 0x00000004;
        const BOSSRUSH = 0x00000008;
        const COLLECTS = 0x00000010;
        const TOUR     = 0x00000020;
        const ARENA    = 0x00000040;
        const SPECIAL  = 0x00000080;
        const HYAKURYU = 0x00000100;
        const TRAINING = 0x00000200;
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum QuestLevel {
        QL1 = 0,
        QL2 = 1,
        QL3 = 2,
        QL4 = 3,
        QL5 = 4,
        QL6 = 5,
        QL7 = 6,
        QL7Ex = 7,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum EnemyLevel {
        Village = 0,
        Low = 1,
        High = 2,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum QuestOrderType {
        None = 0,
        Under2 = 1,
        H2 = 2,
        H3 = 3,
        H4 = 4,
        H5 = 5,
        H6 = 6,
        H7 = 7,
        H8 = 8,
        H20 = 9,
        H30 = 10,
        H40 = 11,
        H45 = 12,
        H50 = 13,
        H90 = 14,
        H100 = 15,
    }
}

rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum QuestTargetType {
        None = 0,
        ItemGet = 1,
        Hunting = 2,
        Kill = 3,
        Capture = 4,
        AllMainEnemy = 5,
        EmTotal = 6,
        FinalBarrierDefense = 7,
        FortLevelUp = 8,
        PlayerDown = 9,
        FinalBoss = 10,
        HuntingMachine = 11,
        DropItem = 12,
        EmStun = 13,
        EmElement = 14,
        EmCondition = 15,
        EmCntWeapon = 16,
        EmCntHmBallista = 17,
        EmCntHmCannon = 18,
        EmCntHmGatling = 19,
        EmCntHmTrap = 20,
        EmCntHmFlameThrower = 21,
        EmCntHmNpc = 22,
        EmCntHmDragnator = 23,
        ExtraEmRunaway = 24,
    }
}

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum BossSetCondition {
        None = 0,
        Default = 1,
        Free1 = 2,
        Free2 = 3,
        Free3 = 4,
        Timer1 = 5,
        Timer2 = 6,
        Em1Hp = 7,
        Em2Hp = 8,
        Em3Hp = 9,
        Em4Hp = 10,
        Em5Hp = 11,
        HpEmx1 = 12,
        InitRandom = 13,
    }
}

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapSetCondition {
        None = 0,
        QuestTimer = 1,
    }
}

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapStopType {
        None = 0,
        LowerHp = 1,
    }
}

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapExecType {
        None = 0,
        FreeExtra = 1,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum BattleBgmType {
        Default = 0,
        SpBattle01 = 1,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum ClearBgmType {
        Default = 0,
        SpClear01 = 1,
    }
}

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EmTypes {
        Em(u32) = 0x0000..=0x0FFF,
        Ems(u32) = 0x1000..=0x1FFF,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData.Param",
        0x54BDF6C5 = 0,
        0x814CED42 = 5,
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataParam {
        pub quest_no: i32,
        pub quest_type: QuestType,
        pub quest_level: QuestLevel,
        pub enemy_level: EnemyLevel,
        pub map_no: i32,
        pub base_time: u32,
        pub time_variation: u32,
        pub time_limit: u32,
        pub quest_life: u32,
        pub order_type: Vec<QuestOrderType>,
        pub target_type: Vec<QuestTargetType>,
        pub tgt_em_type: Vec<EmTypes>,
        pub tgt_item_id: Vec<u32>,
        pub tgt_num: Vec<u32>,
        pub boss_em_type: Vec<EmTypes>,
        pub init_extra_em_num: u8,
        pub swap_em_rate: Vec<u8>,
        pub boss_set_condition: Vec<BossSetCondition>,
        pub boss_set_param: Vec<u32>,
        pub swap_set_condition: Vec<SwapSetCondition>,
        pub swap_set_param: Vec<u8>,
        pub swap_exit_time: Versioned<Vec<u8>, 5, {u32::MAX}>,
        pub is_swap_exit_marionette: Versioned<bool, 5, {u32::MAX}>,
        pub swap_stop_type: SwapStopType,
        pub swap_stop_param: u8,
        pub swap_exec_type: SwapExecType,
        pub rem_money: u32,
        pub rem_village_point: u32,
        pub rem_rank_point: u32,
        pub supply_tbl: u32,
        pub icon: Vec<i32>, // TODO: snow.gui.SnowGuiCommonUtility.Icon.EnemyIconFrameForQuestOrder
        pub is_from_npc: Versioned<bool, 0, 4>,
        pub is_tutorial: bool,
        pub fence_default_active: bool,
        pub fence_active_sec: u16,
        pub fence_default_wait_sec: u16,
        pub fence_reload_sec: u16,
        pub is_use_pillar: Vec<bool>,
        pub auto_match_hr: u16,
        pub battle_bgm_type: BattleBgmType,
        pub clear_bgm_type: ClearBgmType,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData")]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestData {
        pub param: Vec<NormalQuestDataParam>,
    }
}

/*rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum EmsSetNo {
        None = 0,
        M01Base = 1,
        M01Tour = 2,
        M01Tutorial = 3,
        M02Base = 4,
        M02Tour = 5,
        M02Qn000313 = 6,
        M02Qn000412 = 7,
        M02Qn010212 = 8,
        M02Qn010516 = 9,
        M03Base  = 10,
        M03Tour  = 11,
        M03Qn000315  = 12,
        M03Qn010213  = 13,
        M03Qn010517  = 14,
        M04Base  = 15,
        M04Tour  = 16,
        M04Qn000205  = 17,
        M04Qn000209  = 18,
        M04Qn010112  = 19,
        M04Qn010412  = 20,
        M05Base  = 21,
        M05Tour  = 22,
        M01Qn000104  = 23,
        M01Qn000105  = 24,
        M01Qn000208  = 25,
        M01Qn00310  = 26,
        M05Qn000410  = 27,
        M05Qn000414  = 28,
        M01Qn010111  = 29,
        M01Qn010418  = 30,
        M05Qn010617  = 31,
    }
}*/

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum NandoYuragi {
        False = 0,
        True1 = 1,
        True2 = 2,
    }
}

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize, Clone)]
    pub struct SharedEnemyParam { // non-TDB type
        pub route_no: Vec<u8>,
        pub init_set_name: Vec<String>,
        pub sub_type: Vec<u8>,
        pub vital_tbl: Vec<u8>,
        pub attack_tbl: Vec<u8>,
        pub parts_tbl: Vec<u8>,
        pub other_tbl: Vec<u8>,
        pub stamina_tbl: Vec<u8>,
        pub scale: Vec<u8>,
        pub scale_tbl: Vec<i32>, // snow.enemy.EnemyDef.BossScaleTblType
        pub difficulty: Vec<NandoYuragi>,
        pub boss_multi: Vec<u8>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy.Param")]
    #[derive(Debug, Serialize, Clone)]
    pub struct NormalQuestDataForEnemyParam {
        pub quest_no: i32,
        pub ems_set_no: i32, //EmsSetNo,
        pub zako_vital: u8,
        pub zako_attack: u8,
        pub zako_parts: u8,
        pub zako_other: u8,
        pub zako_multi: u8,
        pub param: SharedEnemyParam,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy")]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataForEnemy {
        pub param: Vec<NormalQuestDataForEnemyParam>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.VitalRateTableData")]
    #[derive(Debug, Serialize)]
    pub struct VitalRateTableData {
        pub vital_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.AttackRateTableData")]
    #[derive(Debug, Serialize)]
    pub struct AttackRateTableData {
        pub attack_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.PartsRateTableData")]
    #[derive(Debug, Serialize)]
    pub struct PartsRateTableData {
        pub parts_vital_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.OtherRateTableData")]
    #[derive(Debug, Serialize)]
    pub struct OtherRateTableData {
        pub defense_rate: f32,
        pub damage_element_rate_a: f32,
        pub damage_element_rate_b: f32,
        pub stun_rate: f32,
        pub tired_rate: f32,
        pub marionette_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData.MultiData")]
    #[derive(Debug, Serialize)]
    pub struct MultiData {
        pub two: f32,
        pub three: f32,
        pub four: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData")]
    #[derive(Debug, Serialize)]
    pub struct MultiRateTableData {
        pub multi_data_list: [MultiData; 12],
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData")]
    #[derive(Debug, Serialize)]
    pub struct SystemDifficultyRateData {
        pub vital_rate_table_list: Vec<VitalRateTableData>,
        pub attack_rate_table_list: Vec<AttackRateTableData>,
        pub parts_rate_table_list: Vec<PartsRateTableData>,
        pub other_rate_table_list: Vec<OtherRateTableData>,
        pub multi_rate_table_list: Vec<MultiRateTableData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.ScaleAndRateData")]
    #[derive(Debug, Serialize, Clone)]
    pub struct ScaleAndRateData {
        pub scale: f32,
        pub rate: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.RandomScaleTableData")]
    #[derive(Debug, Serialize)]
    pub struct RandomScaleTableData {
        pub type_: i32, // snow.enemy.EnemyDef.BossScaleTblType
        pub scale_and_rate_data: Vec<ScaleAndRateData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossRandomScaleData {
        pub random_scale_table_data_list: Vec<RandomScaleTableData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemySizeListData.SizeInfo")]
    #[derive(Debug, Serialize)]
    pub struct SizeInfo {
        pub em_type: EmTypes,
        pub base_size: f32,
        pub small_boarder: f32,
        pub big_boarder: f32,
        pub king_boarder: f32,
        pub no_size_scale: bool,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemySizeListData")]
    #[derive(Debug, Serialize)]
    pub struct EnemySizeListData {
        pub size_info_list: Vec<SizeInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData.Param")]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetDataParam {
        pub em_type: EmTypes,
        pub cond_village: i32, // snow.progress.VillageProgress, 1 = Village progress 0
        pub cond_low: i32,
        pub cond_high: i32,
        pub map_flag: [bool; 5],
        pub param: SharedEnemyParam,
    }
}

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData")]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetData {
        pub param: Vec<DiscoverEmSetDataParam>,
    }
}
