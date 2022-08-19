use super::*;
use crate::{rsz_struct, rsz_with_singleton};
use serde::*;

rsz_struct! {
    #[rsz("snow.data.CustomBuildupBaseUserData.Param",
        0x73E5D74D = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupBaseUserDataParam {
        pub table_no: u32,
        pub id: u16, // snow.data.DataDef.CustomBuildupId
        pub category_id: u16, // snow.data.DataDef.CustomBuildupCategoryId
        pub lv: i32,
        pub icon_color: i32, // snow.data.DataDef.CustomColorTypes
        pub cost: i32,
        pub value_table: Vec<i32>,
        pub lot_table: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupBaseUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupBaseUserData.user",
        0x2783EA03 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupBaseUserData {
        pub param: Vec<CustomBuildupBaseUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupOpenUserData.Param",
        0xC0A94CBE = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupOpenUserDataParam {
        pub rare: RareTypes,
        pub item: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
        pub price: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupOpenUserData",
        0x33783DD3 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupOpenUserData {
        pub param: Vec<CustomBuildupOpenUserDataParam>
    }
}

rsz_with_singleton! {
    #[path("data/Define/Player/Equip/CustomBuildup/CustomBuildupArmorOpenUserData.user")]
    pub struct CustomBuildupArmorOpenUserData(CustomBuildupOpenUserData);

    #[path("data/Define/Player/Equip/CustomBuildup/CustomBuildupWeaponOpenUserData.user")]
    pub struct CustomBuildupWeaponOpenUserData(CustomBuildupOpenUserData);
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorMaterialUserData.Param",
        0x124AD7CC = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorMaterialUserDataParam {
        pub rare: RareTypes,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
        pub price: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorMaterialUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupArmorMaterialUserData.user",
        0xE6ACFE8D = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorMaterialUserData {
        pub param: Vec<CustomBuildupArmorMaterialUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorLotUserData.Param",
        0xE3B85657 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorLotUserDataParam {
        pub table_no: u32,
        pub category_id: u16, // snow.data.DataDef.CustomBuildupCategoryId
        pub id: u16, // snow.data.DataDef.CustomBuildupId
        pub lot_num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorLotUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupArmorLotUserData.user",
        0x088AC537 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorLotUserData {
        pub param: Vec<CustomBuildupArmorLotUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorCategoryLotUserData.Param",
        0x4CFD3C97 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorCategoryLotUserDataParam {
        pub table_no: u32,
        pub lot_num: [u32; 4],
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupArmorCategoryLotUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupArmorCategoryLotUserData.user",
        0x98736C75 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupArmorCategoryLotUserData {
        pub param: Vec<CustomBuildupArmorCategoryLotUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupEquipSkillDetailUserData.Param",
        0x1A65FAFD = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupEquipSkillDetailUserDataParam {
        pub skill_id: PlEquipSkillId,
        pub cost: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupEquipSkillDetailUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupEquipSkillDetailUserData.user",
        0x21DF3AB6 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupEquipSkillDetailUserData {
        pub param: Vec<CustomBuildupEquipSkillDetailUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupWeaponMaterialUserData.Param",
        0x2A2ABE7B = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupWeaponMaterialUserDataParam {
        pub id: u16, // snow.data.DataDef.CustomBuildupId
        pub item: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
        pub price: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupWeaponMaterialUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupWeaponMaterialUserData.user",
        0xC6F1927B = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupWeaponMaterialUserData {
        pub param: Vec<CustomBuildupWeaponMaterialUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupWepTableUserData.Param",
        0x248E1617 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupWepTableUserDataParam {
        pub table_no: u32,
        pub category_id: u16, // snow.data.DataDef.CustomBuildupCategoryId
        pub id: Vec<u16>, // snow.data.DataDef.CustomBuildupId
    }
}

rsz_struct! {
    #[rsz("snow.data.CustomBuildupWepTableUserData",
        path = "data/Define/Player/Equip/CustomBuildup/CustomBuildupWepTableUserData.user",
        0xEDAD3A7C = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct CustomBuildupWepTableUserData {
        pub param: Vec<CustomBuildupWepTableUserDataParam>
    }
}