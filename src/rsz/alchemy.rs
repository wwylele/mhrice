use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

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

rsz_struct! {
    #[rsz("snow.data.AlchemyPatturnUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPatturnUserDataParam {
        pub patturn: i32, // snow.data.AlchemyPatturnData.PatturnTypes, 0 = Alchemy1
        pub color: ColorTypes,
        pub cost_village_point: u32,
        pub usable_item_list: Vec<u32>, // snow.data.ContentsIdSystem.ItemId
        pub cost_material_point: u32,
        pub require_talisman_num: u32,
        pub output_min_num: u32,
        pub output_max_num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPatturnUserData")]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPatturnUserData {
        pub param_list: Vec<AlchemyPatturnUserDataParam>,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum GradeTypes {
        C = 0,
        B = 1,
        A = 2,
        S = 3,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPlSkillTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPlSkillTableUserDataParam {
        pub sort_id: i32,
        pub skill_id: u8, // snow.data.DataDef.PlEquipSkillId, 1 = skill 0
        pub grade: GradeTypes,
        pub patturn: i32, // snow.data.AlchemyPatturnData.PatturnTypes, 0 = Alchemy1
        pub pick_rate: u32,
        pub skill1_rate_list: Vec<u32>,
        pub miss_rate_list: Vec<u32>,
        pub skill2_rate_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.AlchemyPlSkillTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct AlchemyPlSkillTableUserData {
        pub param: Vec<AlchemyPlSkillTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.GradeWorthTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct GradeWorthTableUserDataParam {
        pub grade_point: u32,
        pub add_point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.GradeWorthTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct GradeWorthTableUserData {
        pub param: Vec<GradeWorthTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.RareTypeTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct RareTypeTableUserDataParam {
        pub worth_point: u32,
        pub rare_type_list: Vec<u8>, // snow.data.DataDef.RareTypes, 0 = Ra1
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.RareTypeTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct RareTypeTableUserData {
        pub param: Vec<RareTypeTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SecondSkillLotRateTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct SecondSkillLotRateTableUserDataParam {
        pub skill1_grade: GradeTypes,
        pub probability_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SecondSkillLotRateTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct SecondSkillLotRateTableUserData {
        pub param: Vec<SecondSkillLotRateTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SkillGradeLotRateTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct SkillGradeLotRateTableUserDataParam {
        pub patturn_type: i32, // snow.data.AlchemyPatturnData.PatturnTypes, 0 = Alchemy1
        pub probability1_list: Vec<u32>,
        pub probability_list: Vec<u32>,
        pub probability2_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SkillGradeLotRateTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct SkillGradeLotRateTableUserData {
        pub param: Vec<SkillGradeLotRateTableUserDataParam>
    }
}

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
    #[rsz("snow.data.alchemy.SlotNumTableUserData.SkillParam")]
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
    #[rsz("snow.data.alchemy.SlotNumTableUserData.SlotParam")]
    #[derive(Debug, Serialize)]
    pub struct SlotNumTableUserDataSlotParam {
        pub slot_param: Vec<SlotNumTableUserDataSkillParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotNumTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct SlotNumTableUserData {
        pub param: Vec<SlotNumTableUserDataSlotParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotWorthTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct SlotWorthTableUserDataParam {
        pub worth_point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.alchemy.SlotWorthTableUserData")]
    #[derive(Debug, Serialize)]
    pub struct SlotWorthTableUserData {
        pub param: Vec<SlotWorthTableUserDataParam>
    }
}
