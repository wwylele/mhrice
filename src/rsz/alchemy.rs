use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

// snow.data.DataDef.ColorTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum ColorTypes {
        Red = 0,
        Orange = 1,
        Yellow = 2,
        Green = 3,
        Blue = 4,
        White = 5,
        Purple = 6,
    }
}

// snow.data.AlchemyPatturnData.PatturnTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum AlchemyPatturnTypes {
        Alchemy1 = 0,
        Alchemy2 = 1,
        Alchemy3 = 2,
        Alchemy4 = 3,
        Alchemy5 = 4,
        AlchemyShinki = 7,
        AlchemyTensei = 8,
        AlchemyKyokkou = 9,
        AlchemyHaki = 10,
        AlchemyEnkan = 11,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPatturnUserData.Param",
        0x2A7C3B8F = 15_00_00,
        0x8F4D1882 = 14_00_00,
        0xF023B4E2 = 13_00_00,
        0x41f213e8 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPatturnUserDataParam {
        pub patturn: AlchemyPatturnTypes,
        pub color: ColorTypes,
        pub cost_village_point: u32,
        pub usable_item_list: Vec<ItemId>,
        pub cost_material_point: u32,
        pub require_talisman_num: u32,
        pub output_min_num: u32,
        pub output_max_num: u32,
        pub cheat_check_priority: Versioned<u32, 13_00_00>, // this is turned into i32 in 15_00_00 but I am not bother changing it
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPatturnUserData",
        path = "data/Define/Lobby/Facility/Alchemy/AlchemyPatturnData.user",
        0xcc163e12 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPatturnUserData {
        pub param_list: Vec<AlchemyPatturnUserDataParam>,
    }
}

// snow.data.AlchemyPlSkillTableData.GradeTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum GradeTypes {
        C = 0,
        B = 1,
        A = 2,
        S = 3,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPlSkillTableUserData.Param",
        0x6BA8C9A7 = 15_00_00,
        0x7CB91C71 = 13_00_00,
        0x6C87D64B = 10_00_02,
        0xEE208F08 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPlSkillTableUserDataParam {
        pub sort_id: i32,
        pub skill_id: PlEquipSkillId,
        pub grade: GradeTypes,
        pub patturn: AlchemyPatturnTypes,
        pub pick_rate: u32,
        pub grade_pick_rate: Versioned<[u32; 2], 15_00_00>,
        pub skill1_rate_list: Vec<u32>,
        pub miss_rate_list: Vec<u32>,
        pub skill2_rate_list: Vec<u32>,
        pub skill1_mystery_worth: Versioned<Vec<u32>, 15_00_00>,
        pub skill2_mystery_worth: Versioned<Vec<u32>, 15_00_00>,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPlSkillTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/AlchemyPlSkillTable.user",
        0x251E564F = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPlSkillTableUserData {
        pub param: Vec<AlchemyPlSkillTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.GradeWorthTableUserData.Param",
        0x8aaef4d8 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GradeWorthTableUserDataParam {
        pub grade_point: u32,
        pub add_point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.GradeWorthTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/GradeWorthTable.user",
        0x8f3edc1d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GradeWorthTableUserData {
        pub param: Vec<GradeWorthTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.RareTypeTableUserData.Param",
        0xfaa5f87d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RareTypeTableUserDataParam {
        pub worth_point: u32,
        pub rare_type_list: Vec<RareTypes>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.RareTypeTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/RareTypeTable.user",
        0x3a39f3cd = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RareTypeTableUserData {
        pub param: Vec<RareTypeTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SecondSkillLotRateTableUserData.Param",
        0x9515f151 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SecondSkillLotRateTableUserDataParam {
        pub skill1_grade: GradeTypes,
        pub probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SecondSkillLotRateTableUserData",
    path = "data/Define/Lobby/Facility/Alchemy/SecondSkillLotRateTable.user",
        0x2b059c58 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SecondSkillLotRateTableUserData {
        pub param: Vec<SecondSkillLotRateTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SkillGradeLotRateTableUserData.Param",
        0x60AA7E6F = 15_00_00,
        0x8FEBC6B1 = 14_00_00,
        0xed421f20 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SkillGradeLotRateTableUserDataParam {
        pub patturn_type: AlchemyPatturnTypes,
        pub probability1_list: Vec<u32>,
        pub probability_list: Vec<u32>,
        pub probability2_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SkillGradeLotRateTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/SkillGradeLotRateTable.user",
        0x05992de0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SkillGradeLotRateTableUserData {
        pub param: Vec<SkillGradeLotRateTableUserDataParam>
    }
}

// snow.data.alchemy.SlotNumTableUserData.GradeTypesForUserData
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum GradeTypesForSlotNumTable {
        C = 0,
        B = 1,
        A = 2,
        S = 3,
        None = 4,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotNumTableUserData.SkillParam",
        0xF8EFB8DE = 14_00_00,
        0xa284155e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlotNumTableUserDataSkillParam {
        pub skill1_grade: GradeTypesForSlotNumTable,
        pub skill2_grade: GradeTypesForSlotNumTable,
        pub table0_probability_list: Vec<u32>,
        pub table1_probability_list: Vec<u32>,
        pub table2_probability_list: Vec<u32>,
        pub table3_probability_list: Vec<u32>,
        pub table4_probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotNumTableUserData.SlotParam",
        0x931389cf = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlotNumTableUserDataSlotParam {
        pub slot_param: Vec<SlotNumTableUserDataSkillParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotNumTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/SlotNumTable.user",
        0x4a3cbaa7 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlotNumTableUserData {
        pub param: Vec<SlotNumTableUserDataSlotParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotWorthTableUserData.Param",
        0x1a50e70a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlotWorthTableUserDataParam {
        pub worth_point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotWorthTableUserData",
        path = "data/Define/Lobby/Facility/Alchemy/SlotWorthTable.user",
        0xe2f54f75 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlotWorthTableUserData {
        pub param: Vec<SlotWorthTableUserDataParam>
    }
}
