use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData.SeparateData",
        0x9bb00587 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyAngerSeparateData {
        val: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData",
        0xfd3c0d3a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyAngerData {
        data_info: Vec<Rc<EnemyAngerSeparateData>>,
        timer: i32,
        hyakuryu_cool_timer: i32,
        mot_rate: f32,
        atk_rate: f32,
        def_rate: f32,
        compensation_rate: Vec<f32>,
        hyakuryu_compensation_rate: Vec<f32>,
        anger_stay_add_sec: f32,
        life_area_timer_rate: f32,
    }
}
