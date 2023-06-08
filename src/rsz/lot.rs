use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_with_singleton;
use serde::*;

// snow.QuestManager.QuestRank
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum QuestRank {
        Low = 0,
        High = 1,
        Master = 2,
    }
}

// snow.data.LotDataManager.EnemyRewardPopTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum EnemyRewardPopTypes {
        None = 0,
        MainBody = 1,
        PartsLoss1 = 2,
        PartsLoss2 = 3,
        DropItem = 4,
        DropItem2 = 5,
        DropItem3 = 6,
        DropItem4 = 7,
        DropItem5 = 8,
        DropItem6 = 9,
        Unique1 = 10,
    }
}

// snow.data.PartsBreakInfo.BrokenPartsTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum BrokenPartsTypes {
        None = 0,
        RandomId(i32) = 1..=100,
    }
}

// snow.data.PartsBreakInfo.BreakLvTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum BreakLvTypes {
        None = 0,
        Lv1 = 1,
        // There is also Lv2 and Lv3, but seems unused?
    }
}

rsz_struct! {
    #[rsz("snow.data.MonsterLotTableUserData.Param",
        0xE287C2BA = 16_00_00,
        0x5CD699D5 = 15_00_00,
        0x7B3D579C = 14_00_00,
        0xF4AB0924 = 13_00_00,
        0xbdcef7e9 = 10_00_02,
        0x8293106B = 11_00_01,
        0xD673A082 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct MonsterLotTableUserDataParam {
        pub em_types: EmTypes,
        pub quest_rank: QuestRank,
        pub target_reward_item_id_list: Vec<ItemId>,
        pub target_reward_num_list: Vec<u32>,
        pub target_reward_probability_list: Vec<u32>,
        pub enemy_reward_type_list: Vec<EnemyRewardPopTypes>,
        pub hagitory_reward_item_id_list: Vec<ItemId>,
        pub hagitory_reward_num_list: Vec<u32>,
        pub hagitory_reward_probability_list: Vec<u32>,
        pub capture_reward_item_id_list: Vec<ItemId>,
        pub capture_reward_num_list: Vec<u32>,
        pub capture_reward_probability_list: Vec<u32>,
        pub parts_break_list: Vec<BrokenPartsTypes>,
        pub parts_break_lv_list: Vec<BreakLvTypes>,
        pub parts_break_reward_item_id_list: Vec<ItemId>,
        pub parts_break_reward_num_list: Vec<u32>,
        pub parts_break_reward_probability_list: Vec<u32>,
        pub parts_break_reward_type_list: [EnemyRewardPopTypes; 0], // seems unused
        pub drop_reward_type_list: Vec<EnemyRewardPopTypes>,
        pub drop_reward_item_id_list: Vec<ItemId>,
        pub drop_reward_num_list: Vec<u32>,
        pub drop_reward_probability_list: Vec<u32>,
        pub otomo_reward_item_id_list: Vec<ItemId>,
        pub otomo_reward_num_list: Vec<u32>,
        pub otomo_reward_probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.MonsterLotTableUserData",
        path = "data/System/RewardSystem/LotTable/MonsterLotTableData.user",
        0xdda16506 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MonsterLotTableUserData {
        pub param: Vec<MonsterLotTableUserDataParam>,
    }
}

rsz_with_singleton! {
    #[path("data/System/RewardSystem/LotTable/MonsterLotTableData.user")]
    pub struct MonsterLotTableUserDataLrHr(MonsterLotTableUserData);

    #[path("data/System/RewardSystem/LotTable/MonsterLotTableData_MR.user")]
    pub struct MonsterLotTableUserDataMr(MonsterLotTableUserData);
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDropItemInfoData.EnemyDropItemTableData.EnemyDropItemInfo",
        0xfe443c24 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyDropItemInfo {
        pub percentage: u32,
        pub enemy_reward_pop_type: EnemyRewardPopTypes,
        pub drop_item_model_type: i32, // snow.enemy.SystemEnemyDropItemMoveData.ModelTypes
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDropItemInfoData.EnemyDropItemTableData",
        0x10beea0e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyDropItemTableData {
        pub percentage: u32,
        pub enemy_drop_item_info_list: Vec<EnemyDropItemInfo>,
        pub max_num: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDropItemInfoData",
        0x08aa30ec = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyDropItemInfoData {
        pub enemy_drop_item_table_data_tbl: Vec<EnemyDropItemTableData>,
        pub marionette_rewad_pop_type: EnemyRewardPopTypes,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakRewardData.EnemyPartsBreakRewardInfo.PartsBreakGroupConditionInfo",
        0x92ab2acb = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsBreakGroupConditionInfo {
        pub parts_group: u16,
        pub parts_break_level: u16,
    }
}

// snow.enemy.EnemyPartsBreakRewardData.ConditionType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum EnemyPartsBreakRewardDataConditionType {
        All = 0,
        Other = 1,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakRewardData.EnemyPartsBreakRewardInfo",
        0xE287185B = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyPartsBreakRewardInfo {
        pub parts_break_condition_list: Vec<PartsBreakGroupConditionInfo>,
        pub condition_type: EnemyPartsBreakRewardDataConditionType,
        pub broken_parts_type: BrokenPartsTypes,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakRewardData",
        0xd0ef6d2d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyPartsBreakRewardData {
        pub enemy_parts_break_reward_infos: Vec<EnemyPartsBreakRewardInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.data.PartsTypeTextUserData.TextInfo",
        0x5703332E = 16_00_00,
        0x064C963B = 15_00_00,
        0xA8057F49 = 14_00_00,
        0x745031D8 = 13_00_00,
        0x62EF585F = 10_00_02,
        0x94AEE5A5 = 11_00_01,
        0xD233F698 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsTypeTextUserDataTextInfo {
       pub enemy_type_list: Vec<EmTypes>,
       pub text: Guid,
       pub text_for_monster_list: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.data.PartsTypeTextUserData.PartsTypeInfo",
        0xED99FA5B = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsTypeInfo {
        pub broken_parts_types: BrokenPartsTypes,
        pub text_infos: Vec<PartsTypeTextUserDataTextInfo>
    }
}

rsz_struct! {
    #[rsz("snow.data.PartsTypeTextUserData",
        path = "data/Define/Quest/System/QuestRewardSystem/PartsTypeTextData.user",
        0x03a010a7 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsTypeTextUserData {
        pub params: Vec<PartsTypeInfo>
    }
}

// snow.enemy.EnemyDef.PopType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum PopType {
        MarionetteStart = 0,
        EnemyReward = 1,
    }
}

// snow.access.EnemyRewardPopBehavior.MotionTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MotionTypes {
        HagiSlow = 0,
        HagiFast = 1,
        Gathering = 2,
        CarryEggGathering = 3,
        Mining = 4,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.HagiPopParameter",
        0x851831A2 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct HagiPopParameter {
        // snow.enemy.PopParameterBase
        pub pop_type: PopType,
        pub pop_id: u32,

        pub motion_type: MotionTypes,
        pub base_max_hagi_count: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPopParameterData",
        0xE8DDA9D5 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyPopParameterData {
        // The conanical type of the elements are snow.enemy.PopParameterBase
        // But data all have the subtype
        pub system_pop_parameters: Vec<HagiPopParameter>,
        pub unique_pop_parameters: Vec<HagiPopParameter>,
    }
}
