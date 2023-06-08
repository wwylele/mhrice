use super::common::Versioned;
use super::*;
use crate::rsz_bitflags;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_with_singleton;
use serde::*;
use std::cmp::*;

// snow.quest.QuestType
rsz_bitflags! {
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
        const KYOUSEI  = 0x00000400;
    }
}

impl QuestType {
    pub fn icon_index(&self) -> u32 {
        if self.contains(QuestType::HUNTING) {
            return 0;
        }
        if self.contains(QuestType::KILL) {
            return 1;
        }
        if self.contains(QuestType::CAPTURE) {
            return 2;
        }
        if self.contains(QuestType::BOSSRUSH) {
            return 6;
        }
        if self.contains(QuestType::COLLECTS) {
            return 3;
        }
        if self.contains(QuestType::TOUR) {
            return 7;
        }
        if self.contains(QuestType::ARENA) {
            return 4;
        }
        if self.contains(QuestType::SPECIAL) {
            return 5;
        }
        if self.contains(QuestType::HYAKURYU) {
            return 8;
        }
        if self.contains(QuestType::TRAINING) {
            return 0;
        }

        0
    }
}

// snow.quest.QuestLevel
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
        QL7Ex = 7, // also added as "QL8Ex" in v14... what is this?
    }
}

// snow.quest.EnemyLv
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum EnemyLevel {
        Village = 0,
        Low = 1,
        High = 2,
        Master = 3,
    }
}

// snow.quest.QuestOrderType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, Eq, PartialEq)]
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
        M1 = 16,
        M2 = 17,
        M3 = 18,
        M4 = 19,
        M5 = 20,
        M6 = 21,
        M10 = 22,
        M20 = 23,
        M30 = 24,
        M40 = 25,
        M50 = 26,
        M60 = 27,
        M100 = 28,
        Only1 = 29,
    }
}

impl std::fmt::Display for QuestOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                QuestOrderType::None => "None",
                QuestOrderType::Under2 => "At most 2 people",
                QuestOrderType::H2 => "HR 2",
                QuestOrderType::H3 => "HR 3",
                QuestOrderType::H4 => "HR 4",
                QuestOrderType::H5 => "HR 5",
                QuestOrderType::H6 => "HR 6",
                QuestOrderType::H7 => "HR 7",
                QuestOrderType::H8 => "HR 8",
                QuestOrderType::H20 => "HR 20",
                QuestOrderType::H30 => "HR 30",
                QuestOrderType::H40 => "HR 40",
                QuestOrderType::H45 => "HR 45",
                QuestOrderType::H50 => "HR 50",
                QuestOrderType::H90 => "HR 90",
                QuestOrderType::H100 => "HR 100",
                QuestOrderType::M1 => "MR 1",
                QuestOrderType::M2 => "MR 2",
                QuestOrderType::M3 => "MR 3",
                QuestOrderType::M4 => "MR 4",
                QuestOrderType::M5 => "MR 5",
                QuestOrderType::M6 => "MR 6",
                QuestOrderType::M10 => "MR 10",
                QuestOrderType::M20 => "MR 20",
                QuestOrderType::M30 => "MR 30",
                QuestOrderType::M40 => "MR 40",
                QuestOrderType::M50 => "MR 50",
                QuestOrderType::M60 => "MR 60",
                QuestOrderType::M100 => "MR 100",
                QuestOrderType::Only1 => "Solo",
            }
        )
    }
}

// snow.quest.QuestTargetType
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
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

// snow.QuestManager.BossSetCondition
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
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
        HpEmx2 = 13,
        InitRandom = 14,
        SwapRandom = 15,
        FsmControl = 16,
        EntryTime = 17,
    }
}

// snow.QuestManager.SwapSetCondition
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum SwapSetCondition {
        None = 0,
        QuestTimer = 1,
    }
}

// snow.QuestManager.SwapStopType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapStopType {
        None = 0,
        LowerHp = 1,
    }
}

// snow.QuestManager.SwapExecType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SwapExecType {
        None = 0,
        FreeExtra = 1,
    }
}

// snow.quest.BattleBGMType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum BattleBgmType {
        Default = 0,
        C01 = 1,
        C02 = 2,
        C03 = 3,
        C04 = 4,
        C05 = 5,
        C06 = 6,
        C07 = 7,
        Sp01 = 8,
        Sp02 = 9,
    }
}

// snow.quest.ClearBGMType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum ClearBgmType {
        Default = 0,
        SpClear01 = 1,
    }
}

// snow.enemy.EnemyDef.EmTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EmTypes {
        Em(u32) = 0x0000..=0x0FFF,
        Ems(u32) = 0x1000..=0x1FFF,
    }
}

impl EmTypes {
    fn order_index(&self) -> u32 {
        match *self {
            EmTypes::Em(i) => (i & 0xFF) << 16 | (i & 0xF00),
            EmTypes::Ems(i) => (i & 0xFF) << 16 | (i & 0xF00) | 0x80000000,
        }
    }
}

impl PartialOrd<EmTypes> for EmTypes {
    fn partial_cmp(&self, other: &EmTypes) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EmTypes {
    fn cmp(&self, other: &EmTypes) -> Ordering {
        self.order_index().cmp(&other.order_index())
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData.Param",
        0x92F4BE2D = 16_00_00,
        0x3CE9A177 = 15_00_00,
        0xA5344203 = 14_00_00,
        0x04AAB3CD = 13_00_00,
        0x708b71d8 = 10_00_02,
        0x6213ED03 = 11_00_01,
        0x7ADC2497 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataParam {
        pub quest_no: i32,
        pub dbg_name: String,
        pub quest_type: QuestType,
        pub quest_level: QuestLevel,
        pub enemy_level: EnemyLevel,
        pub map_no: i32, // snow.QuestMapManager.MapNoType
        pub base_time: u32,
        pub time_variation: u32,
        pub time_limit: u32,
        pub quest_life: u32,
        pub order_type: Vec<QuestOrderType>,
        pub target_type: [QuestTargetType; 2],
        pub tgt_em_type: [EmTypes; 2],
        pub tgt_item_id: [ItemId; 2],
        pub tgt_num: [u32; 2],
        pub boss_em_type: Vec<EmTypes>,
        pub init_extra_em_num: u8,
        pub swap_em_rate: [u8; 2],
        pub boss_set_condition: Vec<BossSetCondition>,
        pub boss_set_param: Vec<u32>,
        pub swap_set_condition: [SwapSetCondition; 2],
        pub swap_set_param: [u8; 2],
        pub swap_exit_time: [u8; 2],
        pub is_swap_exit_marionette: bool,
        pub swap_stop_type: SwapStopType,
        pub swap_stop_param: u8,
        pub swap_exec_type: SwapExecType,
        pub rem_money: u32,
        pub rem_village_point: u32,
        pub rem_rank_point: u32,
        pub supply_tbl: u32,
        pub icon: Vec<i32>, // TODO: snow.gui.SnowGuiCommonUtility.Icon.EnemyIconFrameForQuestOrder
        pub is_from_npc: bool,
        pub is_tutorial: bool,
        pub fence_active_sec: u16,
        pub fence_default_active: bool,
        pub fence_default_wait_sec: u16,
        pub fence_reload_sec: u16,
        pub is_use_pillar: Vec<bool>,
        pub battle_bgm_type: BattleBgmType,
        pub clear_bgm_type: ClearBgmType,
        pub auto_match_hr: u16,
        pub dbg_client:String,
        pub dbg_content: String,
    }
}

impl NormalQuestDataParam {
    // Some of the logic is from snow.quest.QuestData.getTargetEmTypeLis
    pub fn has_target(&self, em_type: EmTypes) -> bool {
        if self.target_type.first() == Some(&QuestTargetType::AllMainEnemy) {
            let count = if let Some(&count) = self.tgt_num.first() {
                count as usize
            } else {
                eprintln!("tgt_num empty for quest {}", self.quest_no);
                return false;
            };

            return self
                .boss_em_type
                .iter()
                .zip(&self.boss_set_condition)
                .filter(|&(_, &condition)| {
                    // Nice logic
                    condition != BossSetCondition::InitRandom
                        && condition != BossSetCondition::SwapRandom
                })
                .take(count)
                .any(|(&em, _)| em == em_type);
        }

        self.tgt_em_type.contains(&em_type)
    }

    // snow.quest.QuestUtility.isKingdomQuest
    pub fn is_kingdom(&self) -> bool {
        (self.quest_no / 100000) % 10 == 4 && (self.quest_no / 10000) % 10 == 5
    }

    // snow.quest.QuestUtility.isServantRequestQuest
    pub fn is_servant_request(&self) -> bool {
        (self.quest_no / 100000) % 10 == 4 && (self.quest_no / 10000) % 10 == 6
    }

    pub fn anomaly_level(&self) -> Option<i32> {
        // snow.quest.QuestUtility.isMysteryQuest
        if (self.quest_no / 10000) % 10 != 8 || (self.quest_no / 100000) % 10 == 9 {
            return None;
        }
        Some((self.quest_no / 100) % 10)
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestData",
        0x299bae19 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestData {
        pub param: Vec<NormalQuestDataParam>,
    }
}

rsz_with_singleton! {
    #[path("Quest/QuestData/NormalQuestData.user")]
    pub struct BaseNormalQuestDataLrHr(NormalQuestData);

    #[path("Quest/QuestData/NormalQuestData_MR.user")]
    pub struct BaseNormalQuestDataMr(NormalQuestData);

    #[path("Quest/QuestData/DlQuestData.user")]
    pub struct DlNormalQuestDataLrHr(NormalQuestData);

    #[path("Quest/QuestData/DlQuestData_MR.user")]
    pub struct DlNormalQuestDataMr(NormalQuestData);
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

// snow.enemy.EnemyDef.NandoYuragi
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum NandoYuragi {
        False = 0,
        True1 = 1, // small fluctuation
        True2 = 2, // large fluctuation
    }
}

// snow.enemy.EnemyDef.EnemyIndividualType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum EnemyIndividualType {
        Normal = 0,
        Mystery = 1,
        OverMysteryStrengthDefault = 2,
        OverMysteryStrengthLv1 = 3,
        OverMysteryStrengthLv2 = 4,
        OverMysteryStrengthLv3 = 5,
    }
}

pub trait EnemyParam {
    fn vital_tbl(&self, i: usize) -> Option<u16>;
    fn attack_tbl(&self, i: usize) -> Option<u16>;
    fn parts_tbl(&self, i: usize) -> Option<u16>;
    fn other_tbl(&self, i: usize) -> Option<u16>;
    fn stamina_tbl(&self, i: usize) -> Option<u8>;
    fn scale(&self, i: usize) -> Option<u8>;
    fn scale_tbl(&self, i: usize) -> Option<i32>;
    fn difficulty(&self, i: usize) -> Option<NandoYuragi>;
    fn boss_multi(&self, i: usize) -> Option<u16>;
}

macro_rules! impl_enemy_param {
    ($t:ty) => {
        impl EnemyParam for $t {
            fn vital_tbl(&self, i: usize) -> Option<u16> {
                self.vital_tbl.get(i).copied().map(Into::into)
            }
            fn attack_tbl(&self, i: usize) -> Option<u16> {
                self.attack_tbl.get(i).copied().map(Into::into)
            }
            fn parts_tbl(&self, i: usize) -> Option<u16> {
                self.parts_tbl.get(i).copied().map(Into::into)
            }
            fn other_tbl(&self, i: usize) -> Option<u16> {
                self.other_tbl.get(i).copied().map(Into::into)
            }
            fn stamina_tbl(&self, i: usize) -> Option<u8> {
                self.stamina_tbl.get(i).copied()
            }
            fn scale(&self, i: usize) -> Option<u8> {
                self.scale.get(i).copied()
            }
            fn scale_tbl(&self, i: usize) -> Option<i32> {
                self.scale_tbl.get(i).copied()
            }
            fn difficulty(&self, i: usize) -> Option<NandoYuragi> {
                self.difficulty.get(i).copied()
            }
            fn boss_multi(&self, i: usize) -> Option<u16> {
                self.boss_multi.get(i).copied().map(Into::into)
            }
        }
    };
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy.Param",
        0x46FFE13A = 14_00_00,
        0x7E1E92C8 = 10_00_02,
        0x3663ECD6 = 11_00_01,
        0xA718C7EA = 12_00_00,
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct NormalQuestDataForEnemyParam {
        pub quest_no: i32,
        pub ems_set_no: i32, //EmsSetNo,
        pub zako_vital: u8,
        pub zako_attack: u8,
        pub zako_parts: u8,
        pub zako_other: u8,
        pub zako_multi: u8,
        pub route_no: Vec<u8>,
        pub init_set_name: Vec<String>,
        pub sub_type_v11: Versioned<Vec<u8>, 11_00_01>,
        pub individual_type: Vec<EnemyIndividualType>,
        pub sub_type: Versioned<Vec<u8>, 10_00_00, 10_99_99>,
        pub vital_tbl: Vec<u16>,
        pub attack_tbl: Vec<u16>,
        pub parts_tbl: Vec<u16>,
        pub other_tbl: Vec<u16>,
        pub stamina_tbl: Vec<u8>,
        pub scale: Vec<u8>,
        pub scale_tbl: Vec<i32>, // snow.enemy.EnemyDef.BossScaleTblType
        pub difficulty: Vec<NandoYuragi>,
        pub boss_multi: Vec<u8>,
    }
}

impl_enemy_param!(NormalQuestDataForEnemyParam);

impl NormalQuestDataForEnemyParam {
    pub fn sub_type(&self, i: usize) -> Option<u8> {
        if let Some(s) = &self.sub_type.0 {
            s.get(i).cloned()
        } else if let Some(s) = &self.sub_type_v11.0 {
            s.get(i).cloned()
        } else {
            None
        }
    }
}

rsz_struct! {
    #[rsz("snow.quest.NormalQuestDataForEnemy",
        path = "Quest/QuestData/NormalQuestDataForEnemy.user",
        0xd1f4bc61 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalQuestDataForEnemy {
        pub param: Vec<NormalQuestDataForEnemyParam>,
    }
}

rsz_with_singleton! {
    #[path("Quest/QuestData/NormalQuestDataForEnemy.user")]
    pub struct BaseNormalQuestDataForEnemyLrHr(NormalQuestDataForEnemy);

    #[path("Quest/QuestData/NormalQuestDataForEnemy_MR.user")]
    pub struct BaseNormalQuestDataForEnemyMr(NormalQuestDataForEnemy);

    #[path("Quest/QuestData/DlQuestDataForEnemy.user")]
    pub struct DlNormalQuestDataForEnemyLrHr(NormalQuestDataForEnemy);

    #[path("Quest/QuestData/DlQuestDataForEnemy_MR.user")]
    pub struct DlNormalQuestDataForEnemyMr(NormalQuestDataForEnemy);
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.VitalRateTableData",
        0x11ea886d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct VitalRateTableData {
        pub vital_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.AttackRateTableData",
        0xa9a1e8ba = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AttackRateTableData {
        pub attack_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.PartsRateTableData",
        0x0c501c3d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsRateTableData {
        pub parts_vital_rate: f32,
        pub mystery_core_vital_rate: f32
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.OtherRateTableData",
        0x334ea69a = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct OtherRateTableData {
        pub defense_rate: f32,
        pub damage_element_rate_a: f32,
        pub damage_element_rate_b: f32,
        pub stun_rate: f32,
        pub tired_rate: f32,
        pub paralyze_rate: f32,
        pub sleep_rate: f32,
        pub marionette_rate: f32,
        pub damage_element_first_rate_a: f32,
        pub damage_element_first_rate_b: f32,
        pub stun_first_rate: f32,
        pub tired_first_rate: f32,
        pub paralyze_first_rate: f32,
        pub sleep_first_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData.MultiData",
        0x4efdc07a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MultiData {
        pub two: f32,
        pub three: f32,
        pub four: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData.MultiRateTableData",
        0xe9130b0b = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct MultiRateTableData {
        pub multi_data_list: [MultiData; 13],
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDifficultyRateData",
        path = "enemy/user_data/system_difficulty_rate_data.user",
        0xC776EEC0 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct SystemDifficultyRateData {
        pub vital_rate_table_list: Vec<VitalRateTableData>,
        pub attack_rate_table_list: Vec<AttackRateTableData>,
        pub parts_rate_table_list: Vec<PartsRateTableData>,
        pub other_rate_table_list: Vec<OtherRateTableData>,
        pub multi_rate_table_list: Vec<MultiRateTableData>,
    }
}

rsz_with_singleton! {
    #[path("enemy/user_data/system_difficulty_rate_data.user")]
    pub struct SystemDifficultyRateDataNormal(SystemDifficultyRateData);

    #[path("Quest/RandomMystery/RandomMysterySystemDifficutyRateBaseData.user")]
    pub struct SystemDifficultyRateDataAnomaly(SystemDifficultyRateData);
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.ScaleAndRateData",
        0x72b0bab3 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct ScaleAndRateData {
        pub scale: f32,
        pub rate: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData.RandomScaleTableData",
        0xB0D72295 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomScaleTableData {
        pub type_: i32, // snow.enemy.EnemyDef.BossScaleTblType
        pub scale_and_rate_data: Vec<ScaleAndRateData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossRandomScaleData",
        path = "enemy/user_data/system_boss_random_scale_data.user",
        0xc45db706 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossRandomScaleData {
        pub random_scale_table_data_list: Vec<RandomScaleTableData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemySizeListData.SizeInfo",
        0xBA8ABC2C = 16_00_00,
        0x360A1DE6 = 15_00_00,
        0xEE952AF3 = 14_00_00,
        0xE6D33804 = 13_00_00,
        0xB66C1F4D = 10_00_02,
        0x198674C0 = 11_00_01,
        0xF9EB17D2 = 12_00_00,
    )]
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
    #[rsz("snow.enemy.SystemEnemySizeListData",
        path = "enemy/user_data/system_enemy_sizelist_data.user",
        0xab121e9c = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemySizeListData {
        pub size_info_list: Vec<SizeInfo>,
    }
}

// snow.quest.DiscoverMap
pub const DISCOVER_MAP_LIST: [i32; 7] = [1, 4, 2, 3, 5, 12, 13];

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData.Param",
        0x185CA52D = 16_00_00,
        0x62A65B0C = 15_00_00,
        0xF1533F35 = 14_00_00,
        0xFEDEF6E7 = 13_00_00,
        0xa9f8ec2d = 10_00_02,
        0x761D06CD = 11_00_01,
        0x15B3EE53 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetDataParam {
        pub em_type: EmTypes,
        pub cond_village: VillageProgress,
        pub cond_low: HallProgress,
        pub cond_high: HallProgress,
        pub cond_master: MasterRankProgress,
        pub map_flag: [bool; 7],
        pub route_no: [u8; 7],
        pub init_set_name: [String; 7],
        pub sub_type: [u8; 4],
        pub vital_tbl: [u8; 4],
        pub attack_tbl: [u8; 4],
        pub parts_tbl: [u8; 4],
        pub other_tbl: [u8; 4],
        pub stamina_tbl: [u8; 4],
        pub scale: [u8; 4],
        pub scale_tbl:[i32; 4], // snow.enemy.EnemyDef.BossScaleTblType
        pub difficulty: [NandoYuragi; 4],
        pub boss_multi: [u8; 4],
    }
}

impl_enemy_param!(DiscoverEmSetDataParam);

rsz_struct! {
    #[rsz("snow.quest.DiscoverEmSetData",
        path = "Quest/QuestData/DiscoverEmSetData.user",
        0x250dcb35 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DiscoverEmSetData {
        pub param: Vec<DiscoverEmSetDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.MainTargetRewardLotNumDefineUserData.Param",
        0x266cce0b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MainTargetRewardLotNumDefineUserDataParam {
        pub target_num: u32,
        pub base_lot_num: u32,
        pub max_lot_num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.MainTargetRewardLotNumDefineUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/MainTargetLotNumDefineData.user",
        0x360c1a50 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MainTargetRewardLotNumDefineUserData {
        pub param: Vec<MainTargetRewardLotNumDefineUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.QuestDataForRewardUserData.Param",
        0x5a13ba06 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestDataForRewardUserDataParam {
        pub quest_numer: i32,
        pub target_reward_add_num: u32,
        pub additional_target_reward_table_index: u32,
        pub common_material_add_num: u32,
        pub common_material_reward_table_index: u32,
        pub additional_quest_reward_table_index: Vec<u32>,
        pub cloth_ticket_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.QuestDataForRewardUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/QuestDataForRewardData.user",
        0x424e2f4b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestDataForRewardUserData {
        pub param: Vec<QuestDataForRewardUserDataParam>
    }
}

rsz_with_singleton! {
    #[path("data/Define/Quest/System/QuestRewardSystem/QuestDataForRewardData.user")]
    pub struct QuestDataForRewardUserDataLrHr(QuestDataForRewardUserData);

    #[path("data/Define/Quest/System/QuestRewardSystem/QuestDataForRewardData_MR.user")]
    pub struct QuestDataForRewardUserDataMr(QuestDataForRewardUserData);
}

// snow.data.ItemLotTable.LotRule
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum LotRule {
        Random = 0,
        RandomOut1 = 1,
        RandomOut2 = 2,
        RandomOut3 = 3,
        FirstFix = 4,
    }
}

rsz_struct! {
    #[rsz("snow.data.RewardIdLotTableUserData.Param",
        0xF572E24F = 15_00_00,
        0x7CF9C05C = 14_00_00,
        0x766F6930 = 13_00_00,
        0x11de5dc7 = 10_00_02,
        0x0F2FF775 = 11_00_01,
        0xF895584E = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct RewardIdLotTableUserDataParam {
        pub id: u32,
        pub lot_rule: LotRule,
        pub item_id_list: Vec<ItemId>,
        pub num_list: Vec<u32>,
        pub probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.RewardIdLotTableUserData",
        0xdb631ed5 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RewardIdLotTableUserData {
        pub param: Vec<RewardIdLotTableUserDataParam>
    }
}

rsz_with_singleton! {
    #[path("data/Define/Quest/System/QuestRewardSystem/RewardIdLotTableData.user")]
    pub struct RewardIdLotTableUserDataLrHr(RewardIdLotTableUserData);

    #[path("data/Define/Quest/System/QuestRewardSystem/RewardIdLotTableData_MR.user")]
    pub struct RewardIdLotTableUserDataMr(RewardIdLotTableUserData);
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestData.WaveData",
        0x3BA5E192 = 16_00_00,
        0xC2723DE8 = 15_00_00,
        0x743D1792 = 14_00_00,
        0xC19E2A63 = 13_00_00,
        0x43C9A46C = 10_00_02,
        0x65A33E4E = 11_00_01,
        0xED7B2916 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestDataWaveData {
        pub boss_em: EmTypes,
        pub boss_sub_type: u32,
        pub order_table_no: i32,
        pub em_table: [EmTypes; 4],
        pub boss_em_nando_tbl_no: i32,
        pub wave_em_nando_tbl_no: i32,
    }
}

// snow.quest.nHyakuryuQuest.Attr
rsz_bitflags! {
    pub struct HyakuryuQuestAttr: u8 {
        const FIX_WAVE_ORDER = 1;
        const LOT_HIGH_EX = 2;
        const LOT_TRUE_ED = 4;
        const FINAL_BOSS_KILL = 8;
    }
}

// snow.quest.nHyakuryuQuest.Category
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum HyakuryuQuestCategory {
        Normal = 0,
        Nushi = 1,
    }
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestData",
        0xA1F4430F = 15_00_00,
        0xAB666DA4 = 14_00_00,
        0x238dcfc8 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestData {
        pub quest_no: i32,
        pub random_seed: i32,
        pub attr: HyakuryuQuestAttr,
        pub wave_data: [HyakuryuQuestDataWaveData; 3],
        pub quest_lv: QuestLevel,
        pub map_no: i32, // snow.QuestMapManager.MapNoType
        pub category: HyakuryuQuestCategory,
        pub is_village: bool,
        pub base_time: u8,
        pub start_block_no: u8,
        pub end_block_no: u8,
        pub extra_em_wave_no: u8,
        pub extra_em_nando_tbl_no: i8,
        pub nushi_order_tbl_no: u8,
        pub hm_unlock_tbl_no: u8,
        pub sub_target: [QuestTargetType; 6],
        pub sub_target5_wave_no: u8,
    }
}

rsz_struct! {
    #[rsz("snow.quest.HyakuryuQuestDataTbl",
        path = "Quest/Hyakuryu/QuestData/FixHyakuryuQuestData.user",
        0xB0022BC2 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuQuestDataTbl {
        pub data_list: Vec<HyakuryuQuestData>,
        pub data_list_310: Vec<HyakuryuQuestData>,
        pub data_list_320: Vec<HyakuryuQuestData>,
        pub data_list_350: Vec<HyakuryuQuestData>,
        pub data_list_370: Vec<HyakuryuQuestData>,
        pub data_list_380: Vec<HyakuryuQuestData>,
        pub data_list_390: Vec<HyakuryuQuestData>,
    }
}

rsz_struct! {
    #[rsz("snow.data.MysteryRewardItemUserData.Param",
        0x0E0867EE = 16_00_00,
        0x3F3B3205 = 15_00_00,
        0x1F3C6123 = 14_00_00,
        0xE158B503 = 13_00_00,
        0xc3438c68 = 10_00_02,
        0x15C40983 = 11_00_01,
        0x0C5BB4D4 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryRewardItemUserDataParam {
        pub em_type: EmTypes,
        pub lv_lower_limit: u32,
        pub lv_upper_limit: u32,
        pub is_special_mystery: Versioned<bool, 15_00_00>,
        pub quest_no: i32,
        pub hagibui_probability: u32,
        pub reward_item: ItemId,
        pub item_num: u32,
        pub quest_reward_table_index: u32,
        pub additional_quest_reward_table_index: Vec<u32>,
        pub special_quest_reward_table_index: Versioned<u32, 11_00_01>,
        pub multiple_target_reward_table_index: Versioned<u32, 11_00_01>,
        pub multiple_fix_reward_table_index: Versioned<u32, 11_00_01>,
        pub mystery_reward_table: Versioned<i32, 15_00_00>, // snow.quest.nRandomMysteryQuest.MysteryRewardType
    }
}

rsz_struct! {
    #[rsz("snow.data.MysteryRewardItemUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/MysteryRewardItemUserData.user",
        0x1479db1b = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryRewardItemUserData {
        pub param: Vec<MysteryRewardItemUserDataParam>
    }
}

// snow.player.PlayerWeaponType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum PlayerWeaponType {
        GreatSword = 0,
        SlashAxe = 1,
        LongSword = 2,
        LightBowgun = 3,
        HeavyBowgun = 4,
        Hammer = 5,
        GunLance = 6,
        Lance = 7,
        ShortSword = 8,
        DualBlades = 9,
        Horn = 10,
        ChargeAxe = 11,
        InsectGlaive = 12,
        Bow = 13,
    }
}

impl PlayerWeaponType {
    pub fn name(self) -> &'static str {
        match self {
            PlayerWeaponType::GreatSword => "Great sword",
            PlayerWeaponType::SlashAxe => "Switch axe",
            PlayerWeaponType::LongSword => "Long sword",
            PlayerWeaponType::LightBowgun => "Light bowgun",
            PlayerWeaponType::HeavyBowgun => "Heavy bowgun",
            PlayerWeaponType::Hammer => "Hammer",
            PlayerWeaponType::GunLance => "Gunlance",
            PlayerWeaponType::Lance => "Lance",
            PlayerWeaponType::ShortSword => "Sword & sheild",
            PlayerWeaponType::DualBlades => "Dual blades",
            PlayerWeaponType::Horn => "Hunting horn",
            PlayerWeaponType::ChargeAxe => "Charge blade",
            PlayerWeaponType::InsectGlaive => "Insect glaive",
            PlayerWeaponType::Bow => "Bow",
        }
    }
}

rsz_struct! {
    #[rsz("snow.ai.QuestServantDataList.SelectQuestServantInfo",
        0xaf9d45b8 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct SelectQuestServantInfo {
        pub servant_id: i32, // snow.ai.ServantDefine.ServantId
        pub weapon_type: PlayerWeaponType, // snow.player.PlayerWeaponType
    }
}

rsz_struct! {
    #[rsz("snow.ai.QuestServantDataList.QuestServantData",
        0x2C633CFF = 15_00_00,
        0x2A6F910D = 14_00_00,
        0xBED4B9C5 = 13_00_00,
        0x29c450b1 = 10_00_02,
        0x44843229 = 11_00_01,
        0xF1AA25A7 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestServantData {
        pub quest_no: i32,
        pub servant_info_list: Vec<SelectQuestServantInfo>
    }
}

rsz_struct! {
    #[rsz("snow.ai.QuestServantDataList",
        path = "servant/prefab/ServantManager/QuestServantDataList.user",
        0x35f0b1a7 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestServantDataList {
        pub quest_servant_data_list: Vec<QuestServantData>
    }
}

rsz_struct! {
    #[rsz("snow.quest.SupplyData.Param",
        0x7076F574 = 15_00_00,
        0x8EA61FDD = 14_00_00,
        0x6E57F8E4 = 13_00_00,
        0xDF132E6C = 10_00_02,
        0xCB4B87E1 = 11_00_01,
        0x536C3809 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct SupplyDataParam {
        pub id: i32,
        pub item_id: Vec<ItemId>,
        pub num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.SupplyData",
        0xa9c0b003 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct SupplyData {
        pub param: Vec<SupplyDataParam>
    }
}

rsz_with_singleton! {
    #[path("Quest/SupplyData/SupplyData.user")]
    pub struct SupplyDataLrHr(SupplyData);

    #[path("Quest/SupplyData/SupplyData_MR.user")]
    pub struct SupplyDataMr(SupplyData);
}

// snow.progress.ProgressHRCategory
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum ProgressHRCategory {
        Low = 0,
        High = 1,
        HRUnlock = 2,
    }
}

rsz_struct! {
    #[rsz("snow.data.checker.ProgressCheckerUserData.Param",
        0x1DA2EC17 = 15_00_00,
        0xC389194D = 14_00_00,
        0x0D6992CA = 13_00_00,
        0x418a9339 = 10_00_02,
        0xbd01b69b = 11_00_01,
        0x6BE6DF3B = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct ProgressCheckerUserDataParam {
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub village: VillageProgress,
        pub hall: HallProgress,
        pub mr: MasterRankProgress,
        pub quest_no: i32, // snow.quest.QuestNo
        pub talk_flag: i32, // snow.npc.TalkFlag
        pub talk_flag_hall: HallProgress,
        pub enable_progress_hr_check: bool,
        pub progress_hr: ProgressHRCategory,
    }
}

rsz_struct! {
    #[rsz("snow.data.checker.ProgressCheckerUserData",
        path = "data/Manager/FlagDataManager/ProgressChecker.user",
        0x27f929e2 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ProgressCheckerUserData {
        pub param_list: Vec<ProgressCheckerUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.quest.ArenaQuestData.ArenaTalismanSkillData",
        0xB109452C = 15_00_00,
        0x873C6753 = 13_00_00,
        0x2F06A351 = 11_00_01,
        0x6E4837CF = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ArenaTalismanSkillData {
        pub id: PlEquipSkillId,
        pub lv: i32,
    }
}

rsz_struct! {
    #[rsz("snow.ItemWork",
        0x9DF5D9BC = 15_00_00,
        0xD5BE703B = 14_00_00,
        0x56D8E30A = 13_00_00,
        0x33517B1D = 12_00_00,
        0x6E3FD481 = 11_00_01,
        0xFDAB4A8B = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemWork {
        pub item: ItemId,
        pub num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.quest.ArenaQuestData.ArenaPlayer",
        0x11E250DA = 15_00_00,
        0x0BB2475D = 14_00_00,
        0x11445F02 = 13_00_00,
        0x8FDD5FC1 = 12_00_00,
        0x01A94B6D = 11_00_01,
        0x954AFD0F = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ArenaPlayer {
        pub wep_id: WeaponId,
        pub wep_action: Vec<i32>, // snow.data.DataDef.PlWeaponActionId
        pub wep_action2: Vec<i32>,
        pub deco_wep: Vec<DecorationsId>,
        pub hyakuryu_skill: Vec<PlHyakuryuSkillId>,
        pub bowgun_custom: i32, // snow.data.BowgunCustomize.BowgunCustomizeTypes
        pub insect_id: WeaponId,

        pub armor_helm: PlArmorId,
        pub armor_lv_helm: u32,
        pub deco_helm: Vec<DecorationsId>,

        pub armor_body: PlArmorId,
        pub armor_lv_body: u32,
        pub deco_body: Vec<DecorationsId>,

        pub armor_arm: PlArmorId,
        pub armor_lv_arm: u32,
        pub deco_arm: Vec<DecorationsId>,

        pub armor_waist: PlArmorId,
        pub armor_lv_waist: u32,
        pub deco_waist: Vec<DecorationsId>,

        pub armor_leg: PlArmorId,
        pub armor_lv_leg: u32,
        pub deco_leg: Vec<DecorationsId>,

        pub lv_buff_cage_id: LvBuffCageId,
        pub talisman_id: u32, // snow.data.DataDef.PlTalismanId
        pub talisman_skill: Vec<ArenaTalismanSkillData>,
        pub deco_talisman: Vec<DecorationsId>,
        pub pouch: Vec<ItemWork>,
        pub ganner_pouch: Vec<ItemWork>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.ArenaQuestData.Param",
        0xB1AC8454 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ArenaQuestDataParam {
        pub quest_no: i32,
        pub em2em_adjust_data: f32,
        pub dodge_blocking_damage_rate_m: f32,
        pub dodge_blocking_damage_rate_s: f32,
        pub shoot_wall_hit_damage_rate_list: Vec<f32>,
        pub final_attack_point_add_by_target_enemy_damage_max_hp_rate: f32,
        pub start_wait_loop_sub_time_max_hp_rate: f32,
        pub base_gimmik_damage: i16,
        pub rank_time_s: i32,
        pub rank_time_a: i32,
        pub rank_time_b: i32,
        pub rank_point_rate_a: f32,
        pub rank_point_rate_s: f32,
        pub arena_pl: Vec<ArenaPlayer>
    }
}

rsz_struct! {
    #[rsz("snow.quest.ArenaQuestData",
        path = "Quest/Arena/ArenaQuestData.user",
        0x371AAF74 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ArenaQuestData {
        pub param: Vec<ArenaQuestDataParam>,
        pub param1: Vec<ArenaQuestDataParam>,
        pub param2: Vec<ArenaQuestDataParam>,
        pub param3: Vec<ArenaQuestDataParam>,
        pub param_mr: Vec<ArenaQuestDataParam>,
        pub param_mr1: Vec<ArenaQuestDataParam>,

    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockListGroup",
        0x409AA50F = 15_00_00,
        0xC924586B = 14_00_00,
        0xA732DC3F = 13_00_00,
        0x1197D013 = 12_00_00,
        0xA06B3109 = 11_00_01,
        0x8D10F382 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockListGroup {
        pub quest_no_array: Vec<i32>, // snow.quest.QuestNo
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockRelation",
        0x5C091ADE = 15_00_00,
        0xC67F73E3 = 14_00_00,
        0xB522D865 = 13_00_00,
        0x2945BA18 = 12_00_00,
        0xB125A5B1 = 11_00_01,
        0xF675949F = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockRelation {
        pub order: i32,
        pub request_count: i32,
        pub request_group_idx: Vec<i32>,
        pub request_talk_flag: Vec<i32>, // snow.npc.TalkFlag
        pub release_group_idx: Vec<u32>,
        pub set_talk_flag: i32, // snow.npc.TalkFlag

    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockByTalkFlag",
        0x1B20E603 = 15_00_00,
        0x99B7B190 = 14_00_00,
        0xA2C096E7 = 13_00_00,
        0xA48596FD = 12_00_00,
        0x2C741CB6 = 11_00_01,
        0x0AC85EF5 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockByTalkFlag {
        pub quest_no: i32,
        pub talk_flag: i32, // snow.npc.TalkFlag
        pub is_clear: bool,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockByQuestClear.UnlockQuestInfoList",
        0x3F25BF14 = 15_00_00,
        0x3021A0EC = 14_00_00,
        0xDAC7F830 = 13_00_00,
        0x964C501B = 12_00_00,
        0xC70EDBDF = 11_00_01,
        0xBFF62D69 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct UnlockQuestInfoList {
        pub unlock_quest: i32,
        pub is_clear: bool,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockByQuestClear",
        0x2C78AAD6 = 15_00_00,
        0xC3FA2284 = 14_00_00,
        0x54117276 = 13_00_00,
        0xA5FCF42F = 12_00_00,
        0x64C8ACEE = 11_00_01,
        0x80AACD3E = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockByQuestClear {
        pub unlock_quest_no_list: Vec<UnlockQuestInfoList>,
        pub clear_quest_no_list: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.RandomQuestUnlockByQuestClear.RandomQuestList",
        0x8664155C = 15_00_00,
        0xC5EE12A5 = 14_00_00,
        0xEB5E6CB1 = 13_00_00,
        0xE0AC5830 = 12_00_00,
        0xB1FBBB0C = 11_00_01,
        0x67F434DF = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomQuestList {
        pub random_quest: i32,
        pub is_triger: bool,
        pub rate: i32,
        pub is_clear: bool,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.RandomQuestUnlockByQuestClear",
        0xA90AC50E = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomQuestUnlockByQuestClear {
        pub random_group: Vec<RandomQuestList>
    }
}

// snow.progress.quest.EnemyRank
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub enum EnemyRank {
        None = 0,
        Village = 1,
        Low = 2,
        High = 3,
        Master = 4,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockByHuntEnemy",
        0x5893228E = 15_00_00,
        0xCDEA2434 = 14_00_00,
        0x0023438C = 13_00_00,
        0xECB2250E = 12_00_00,
        0x8457868E = 11_00_01,
        0x9505EE84 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockByHuntEnemy {
        pub unlock_quest_no: i32,
        pub hunt_em_type: i32, // snow.enemy.EnemyDef.EnemyTypeIndex
        pub enemy_rank: EnemyRank,
        pub is_clear: bool,
    }
}

rsz_struct! {
    #[rsz("snow.progress.quest.QuestUnlockRequestListUserData",
        path = "Progress/quest/userdata/QuestUnlockRequestList.user",
        0xB5FA4A1A = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUnlockRequestListUserData {
        pub quest_group: Vec<QuestUnlockListGroup>,
        pub relation: Vec<QuestUnlockRelation>,
        pub quest_unlock_by_talk_flag: Vec<QuestUnlockByTalkFlag>,
        pub quest_unlock_by_quest_clear: Vec<QuestUnlockByQuestClear>,
        pub random_quest_unlock_by_quest_clear: Vec<RandomQuestUnlockByQuestClear>,
        pub quest_unlock_by_hunt_enemy: Vec<QuestUnlockByHuntEnemy>,
    }
}

// snow.quest.RewardRank
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum RewardRank {
        RankSS = 0,
        RankS = 1,
        RankA = 2,
        RankB = 3,
    }
}

rsz_struct! {
    #[rsz("snow.data.TimeAttackRewardUserData.RankData",
        0x5DF7AD18 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RankData {
        pub rank: RewardRank,
        pub clear_time: f32,
        pub reward_table_id: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.TimeAttackRewardUserData.TimeAttackRewardData",
        0x696B9A7C = 15_00_00,
        0x82AA9CA6 = 14_00_00,
        0x1142EF9A = 13_00_00,
        0xC01F0CE3 = 12_00_00,
        0x6EA33C1C = 11_00_01,
        0x8D1A9F51 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TimeAttackRewardData {
        pub quest_no: i32,
        pub rank_data_list: Vec<RankData>,
    }
}

rsz_struct! {
    #[rsz("snow.data.TimeAttackRewardUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/TimeAttackRewardUserData.user",
        0xCED3F823 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct TimeAttackRewardUserData {
        pub data_list: Vec<TimeAttackRewardData>,
    }
}

rsz_struct! {
    #[rsz("snow.npc.QuestGroup",
        0x8EC7568B = 15_00_00,
        0x24364BAE = 14_00_00,
        0x98FC5F75 = 13_00_00,
        0x5F8315C6 = 12_00_00,
        0xE9A5D28D = 11_00_01,
        0x654A07EC = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestGroup {
        pub quest_no_array: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.npc.TalkConditionQuestListUserData",
        path = "npc/userdata/common/TalkConditionQuestList.user",
        0xB78D58EF = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TalkConditionQuestListUserData {
        pub quest_group: [QuestGroup; 2],
    }
}

// snow.quest.MissionDef.Rank
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum MissionRank {
        Any = 0,
        Low = 1,
        High = 2,
        Master = 3,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NPCMissionData",
        0xF25FAF77 = 16_00_00,
        0x8FF4F56E = 15_00_00,
        0xB1000D1F = 10_00_02,
        0xEA502473 = 11_00_01,
        0x63D6F917 = 12_00_00,
        0x67D10722 = 13_00_00,
        0xAD34C61D = 14_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct NPCMissionData {
        pub index: i32,
        pub id: i32,
        pub start_flag: i32, // snow.npc.TalkFlag
        pub end_flag: i32, // snow.npc.TalkFlag
        pub client_npc: i32, // snow.NpcDefine.NpcID
        pub reward_type: i32, // snow.quest.MissionDef.NPCRewardIcon
        pub reward_id: Vec<i32>,
        pub reward_type_sub: i32, // snow.quest.MissionDef.NPCRewardIcon
        pub reward_id_sub: Vec<i32>,
        pub reward_num_sub: Vec<i32>,
        pub mission_type: i32, // snow.quest.MissionDef.MissionType
        pub rank: MissionRank,
        pub em_type: EmTypes,
        pub map_no: i32, // snow.QuestMapManager.MapNoType
        pub item_id: Vec<ItemId>,
        pub tgt_num: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.NPCMissionDataList",
        0xDAB9C3B9 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct NPCMissionDataList {
        pub param: Vec<NPCMissionData>,
    }
}

rsz_with_singleton! {
    #[path("Quest/MissionData/NPCMissionData.user")]
    pub struct NPCMissionDataListLrHr(NPCMissionDataList);

    #[path("Quest/MissionData/NPCMissionDataList_MR.user")]
    pub struct NPCMissionDataListMr(NPCMissionDataList);
}
