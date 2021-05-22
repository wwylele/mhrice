use super::*;
use crate::{rsz_enum, rsz_struct};
use serde::*;

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum PlArmorId {
        None = 0,
        Head(u32) = 0x0C100000..=0x0C10FFFF,
        Chest(u32) = 0x0C200000..=0x0C20FFFF,
        Arm(u32) = 0x0C300000..=0x0C30FFFF,
        Waist(u32) = 0x0C400000..=0x0C40FFFF,
        Leg(u32) = 0x0C500000..=0x0C50FFFF,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone)]
    pub enum SexualEquipableFlag {
        MaleOnly = 0,
        FemaleOnly = 1,
        Both = 2,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ArmorBaseUserDataParam {
        pub pl_armor_id: PlArmorId,
        pub is_valid: bool,
        pub series: i32,
        pub sort_id: u32,
        pub model_id: u32,
        pub rare: RareTypes,
        pub value: u32,
        pub buy_value: u32,
        pub sexual_equipable: SexualEquipableFlag,
        pub symbol_color1: bool,
        pub symbol_color2: bool,
        pub def_val: i32,
        pub fire_reg_val: i32,
        pub water_reg_val: i32,
        pub ice_reg_val: i32,
        pub thunder_reg_val: i32,
        pub dragon_reg_val: i32,
        pub buildup_table: i32, // snow.data.ArmorBuildupData.TableTypes
        pub buff_formula: i32, // snow.data.GameItemEnum.SeriesBufType
        pub decorations_num_list: [u32; 3],
        pub skill_list: Vec<PlEquipSkillId>,
        pub skill_lv_list: Vec<i32>,
        pub id_after_ex_change: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct ArmorBaseUserData {
        pub param: Vec<ArmorBaseUserDataParam>
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone)]
    pub enum EquipDifficultyGroup {
        Lower = 0,
        Higher = 1,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorSeriesUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ArmorSeriesUserDataParam {
        pub armor_series: i32,
        pub difficulty_group: EquipDifficultyGroup,
        pub is_collabo: bool,
        pub index: u32,
        pub overwear_sort_index: u32,
        pub sexual_enable: SexualEquipableFlag,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorSeriesUserData")]
    #[derive(Debug, Serialize)]
    pub struct ArmorSeriesUserData {
        pub param: Vec<ArmorSeriesUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorProductUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ArmorProductUserDataParam {
        pub id: PlArmorId,
        pub item_flag: ItemId,
        pub enemy_flag: EmTypes,
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub item: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
        pub output_item: Vec<ItemId>,
        pub output_item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorProductUserData")]
    #[derive(Debug, Serialize)]
    pub struct ArmorProductUserData {
        pub param: Vec<ArmorProductUserDataParam>
    }
}
