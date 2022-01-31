use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataBase",
        0xa01ee02d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyDataBase {
        pub caution_to_combat_vision_timer: f32,
        pub caution_to_non_combat_timer: f32,
        pub combat_to_non_combat_timer: f32,
        pub non_combat_kehai_rate: f32,
        pub base_scale: f32,
    }
}
