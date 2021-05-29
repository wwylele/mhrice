use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData.SeparateData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyAngerSeparateData {
        val: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyAngerData")]
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
