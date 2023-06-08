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

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData.PointData",
        0xAFD0E9BA = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyStaminaPointData {
        pub percent: u32,
        pub point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData.SeparateData",
        0x75D654B7 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyStaminaSeparateData {
        pub recov_start_point: f32,
        pub recov_end_point: f32,
        pub st_point_array: Vec<EnemyStaminaPointData>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData.NikuEatInfo",
        0xCF07EAEE = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct NikuEatInfo {
        pub stamina: f32,
        pub probability: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData.SetMeatInfo",
        0x9A774574 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct SetMeatInfo {
        pub stamina_rate: f32,
        pub check_time: f32,
        pub distance: f32,
        pub rate: f32,
        pub is_any_same_block_check: bool,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData.PredatorData",
        0x451278F1 = 16_00_00,
        0xAFC977D5 = 15_00_00,
        0xC3A498B9 = 14_00_00,
        0x7BB6F24A = 13_00_00,
        0xF3B97A73 = 12_00_00,
        0x4445F2C8 = 11_00_01,
        0x717E3985 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct PredatorData {
        pub is_preditor: i32, // snow.enemy.EnemyStaminaData.PredatorData.PredatorExec
        pub action_stance: i32, // snow.enemy.EnemyDef.PredatorActionStance
        pub enable_radius: f32,
        pub em_types: Vec<EmTypes>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyStaminaData",
        0xED067F88 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyStaminaData {
        pub mot_rate: f32,
        pub tired_sec: f32,
        pub hyakuryu_tired_sec: f32,
        pub map_niku_eat_sec: f32,
        pub mouth_to_null_distance: f32,
        pub in_combat_map_meat_eat_flag: bool,
        pub data_info: Vec<EnemyStaminaSeparateData>,
        pub map_niku: Vec<NikuEatInfo>,
        pub is_enable_set_meat_eat: bool,
        pub set_meat_eat_sec: f32,
        pub set_meat_condition_sec: f32,
        pub set_meat_recov_start_point: f32,
        pub set_meat_recov_end_point: f32,
        pub set_meat_normal_list: Vec<SetMeatInfo>,
        pub set_meat_combat_list: Vec<SetMeatInfo>,
        pub predator_data_info: PredatorData,
    }
}
