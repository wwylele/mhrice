use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.LotInfo")]
    #[derive(Debug, Serialize)]
    pub struct LotInfo {
        pub lot: i32,
        pub block: i32,
        pub id: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.SetInfo")]
    #[derive(Debug, Serialize)]
    pub struct SetInfo {
        pub set_name: Utf16String,
        pub info: Vec<LotInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData.StageInfo")]
    #[derive(Debug, Serialize)]
    pub struct StageInfo {
        pub map_type: i32,
        pub set_info_list: Vec<SetInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBossInitSetData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossInitSetData {
        pub enemy_type: i32,
        pub stage_info_list: Vec<StageInfo>,
    }
}
