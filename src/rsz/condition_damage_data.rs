use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StockData")]
    #[derive(Debug, Serialize)]
    pub struct StockData {
        default_limit: f32,
        add_limit: f32,
        max_limit: f32,
        sub_value: f32,
        sub_interval: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ParalyzeDamageData")]
    #[derive(Debug, Serialize)]
    pub struct ParalyzeDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.SleepDamageData")]
    #[derive(Debug, Serialize)]
    pub struct SleepDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StunDamageData")]
    #[derive(Debug, Serialize)]
    pub struct StunDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StaminaDamageData")]
    #[derive(Debug, Serialize)]
    pub struct StaminaDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub sub_stamina: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyFlashDamageParam.DamageLvData")]
    #[derive(Debug, Serialize)]
    pub struct FlashDamageLvData {
        activate_count: i32,
        active_time: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FlashDamageData")]
    #[derive(Debug, Serialize)]
    pub struct FlashDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub damage_lvs: Vec<FlashDamageLvData>,
        pub ignore_refresh_stance: u32, // Stand, Fly, Diving, Wall, Ceiling
        pub max_distance: f32,
        pub min_distance: f32,
        pub angle: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.PoisonDamageData")]
    #[derive(Debug, Serialize)]
    pub struct PoisonDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.BlastDamageData")]
    #[derive(Debug, Serialize)]
    pub struct BlastDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub blast_damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.MarionetteStartDamageData")]
    #[derive(Debug, Serialize)]
    pub struct MarionetteStartDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub use_data: u32, // Common, Unique
        pub nora_first_limit: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.WaterDamageData.AdjustMeatDownData")]
    #[derive(Debug, Serialize)]
    pub struct AdjustMeatDownData {
        hard_meat_adjust_value: f32,
        soft_meat_adjust_value: f32,
        judge_meat_value: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.WaterDamageData")]
    #[derive(Debug, Serialize)]
    pub struct WaterDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub melee_adjust: AdjustMeatDownData,
        pub shot_adjust: AdjustMeatDownData,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FireDamageData")]
    #[derive(Debug, Serialize)]
    pub struct FireDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub hit_damage_rate: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.IceDamageData")]
    #[derive(Debug, Serialize)]
    pub struct IceDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub motion_speed_rate: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyThunderDamageParam.AdjustParamData")]
    #[derive(Debug, Serialize)]
    pub struct ThunderAdjustParamData {
        pub hit_damage_to_stun_rate: f32,
        pub hit_damage_to_stun_max: f32,
        pub hit_damage_to_stun_min: f32,
        pub default_stun_damage_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ThunderDamageData")]
    #[derive(Debug, Serialize)]
    pub struct ThunderDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub stun_meat_adjust: ThunderAdjustParamData,
        pub normal_meat_adjust: ThunderAdjustParamData,
        pub stun_active_limit: i32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallTrapDamageData")]
    #[derive(Debug, Serialize)]
    pub struct FallTrapDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub render_offset_y: f32,
        pub render_offset_speed: f32,
        pub render_offset_reset_time: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallQuickSandDamageData")]
    #[derive(Debug, Serialize)]
    pub struct FallQuickSandDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub render_offset_y: f32,
        pub render_offset_speed: f32,
        pub render_offset_reset_time: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallOtomoTrapDamageData")]
    #[derive(Debug, Serialize)]
    pub struct FallOtomoTrapDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub already_poison_stock_value: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ShockTrapDamageData")]
    #[derive(Debug, Serialize)]
    pub struct ShockTrapDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.CaptureDamageData")]
    #[derive(Debug, Serialize)]
    pub struct CaptureDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.KoyashiDamageData")]
    #[derive(Debug, Serialize)]
    pub struct KoyashiDamageData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.SteelFangData")]
    #[derive(Debug, Serialize)]
    pub struct SteelFangData {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
        pub active_limit_count: i32,
        pub preset_type: u32,
        pub is_unique_target_param: u32,
        pub max_distance: f32,
        pub min_distance: f32,
        pub angle: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyConditionDamageData {
        pub paralyze: ParalyzeDamageData,
        pub sleep: SleepDamageData,
        pub stun: StunDamageData,
        pub stamina: StaminaDamageData,
        pub flash: FlashDamageData,
        pub poison: PoisonDamageData,
        pub blast: BlastDamageData,
        pub ride: MarionetteStartDamageData,
        pub water: WaterDamageData,
        pub fire: FireDamageData,
        pub ice: IceDamageData,
        pub thunder: ThunderDamageData,
        pub fall_trap: FallTrapDamageData,
        pub fall_quick_sand: FallQuickSandDamageData,
        pub fall_otomo_trap: FallOtomoTrapDamageData,
        pub shock_trap: ShockTrapDamageData,
        pub shock_otomo_trap: ShockTrapDamageData,
        pub capture: CaptureDamageData,
        pub dung: KoyashiDamageData,
        pub steel_fang: SteelFangData,
        // 0 = use, 1 = not use
        pub use_paralyze: u8,
        pub use_sleep: u8,
        pub use_stun: u8,
        pub use_stamina: u8,
        pub use_flash: u8,
        pub use_poison: u8,
        pub use_blast: u8,
        pub use_ride: u8,
        pub use_water: u8,
        pub use_fire: u8,
        pub use_ice: u8,
        pub use_thunder: u8,
        pub use_fall_trap: u8,
        pub use_fall_quick_sand: u8,
        pub use_fall_otomo_trap: u8,
        pub use_shock_trap: u8,
        pub use_shock_otomo_trap: u8,
        pub use_capture: u8,
        pub use_dung: u8,
        pub use_steel_fang: u8,
    }
}
