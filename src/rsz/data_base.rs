use super::*;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataBase")]
    #[derive(Debug, Serialize)]
    pub struct EnemyDataBase {
        pub causion_to_combat_vision_timer: f32,
        pub causion_to_noncombat_timer: f32,
        pub combat_to_noncombat_timer: f32,
        pub noncombat_kehai_rate: f32,
        pub base_scale: f32,
    }
}
