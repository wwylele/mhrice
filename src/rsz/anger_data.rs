use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData.SeparateData",
        0x9bb00587 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyAngerSeparateData {
        pub val: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData",
        0x96BC1534 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyAngerData {
        pub data_info: [EnemyAngerSeparateData; 4],
        pub timer: i32,
        pub hyakuryu_cool_timer: i32,
        pub mot_rate: f32,
        pub atk_rate: f32,
        pub def_rate: f32,
        pub compensation_rate: [f32; 10],
        pub hyakuryu_compensation_rate: [f32; 10],
        pub anger_stay_add_sec: f32,
        pub life_area_timer_rate: f32,
    }
}
