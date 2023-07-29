use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyEcologicalData.StageInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyEcologicalDataStageInfo {
        pub map_type: i32,
        pub sleep_use_flag: bool,
        pub stamina_use_flag: bool,
        pub ecologic_use_flag: bool,
        pub prey_use_flag: bool,
        pub pop_use_flag: bool,
        pub ecological_point_list: Vec<EnemyBlockMoveDataInsideMoveInfo>,
        pub prey_mure_id: i32,
        pub grass_pop_use_flag: bool,
        pub mushroom_pop_use_flag: bool,
        pub nut_pop_use_flag: bool,
        pub ore_pop_use_flag: bool,
        pub bone_pop_use_flag: bool,
        pub bee_hive_pop_use_flag: bool,
        pub spider_web_pop_use_flag: bool,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyEcologicalData")]
    #[derive(Serialize, Debug)]
    pub struct EnemyEcologicalData {
        pub stage_info_list: Vec<EnemyEcologicalDataStageInfo>,
        pub normal_sleep_timer: f32,
        pub normal_sleep_yuragi_timer: f32,
        pub not_sleep_start_hour: i32,
        pub not_sleep_end_hour: i32,
        pub normal_map_niku_timer: f32,
        pub normal_map_niku_yuragi_timer: f32,
    }
}
