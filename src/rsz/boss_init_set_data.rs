use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.LotInfo",
        0x6b978bab = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct LotInfo {
        pub lot: i32,
        pub block: i32,
        pub id: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.SetInfo",
        0x7fdbc9bb = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SetInfo {
        pub set_name: String,
        pub info: Vec<LotInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.StageInfo",
        0x8AE778DB = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageInfo {
        pub map_type: i32, // snow.QuestMapManager.MapNoType
        pub set_info_list: Vec<SetInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData",
        0x6D38BB8A = 10_00_02,
        0x32A44979 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossInitSetData {
        pub enemy_type: i32, // snow.enemy.EnemyDef.EnemyTypeIndex
        pub stage_info_list: Vec<StageInfo>,
    }
}
