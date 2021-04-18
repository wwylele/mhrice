use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.PlEquipSkillBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct PlEquipSkillBaseUserDataParam {
        pub id: u8, // snow.data.DataDef.PlEquipSkillId, 1 = ID_0
        pub max_level: i32, // 0 = level 1
        pub icon_color: i32,
        pub worth_point_list: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.PlEquipSkillBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct PlEquipSkillBaseUserData {
        pub param: Vec<PlEquipSkillBaseUserDataParam>,
    }
}
