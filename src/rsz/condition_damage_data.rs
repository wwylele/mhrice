use super::*;
use crate::rsz_bitflags;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StockData",
        0xc777885c = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct StockData {
        pub default_limit: f32,
        pub add_limit: f32,
        pub max_limit: f32,
        pub sub_value: f32,
        pub sub_interval: f32,
    }
}

// snow.enemy.EnemyConditionDamageData.ConditionDamageDataBase
// flattend base
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ConditionDamageDataBase {
        pub default_stock: StockData,
        pub ride_stock: StockData,
        pub max_stock: f32,
        pub active_time: f32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
        pub add_tired_time: f32,
        pub damage_interval: f32,
        pub damage: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ParalyzeDamageData",
        0x137585f6 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ParalyzeDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.SleepDamageData",
        0xad7bd1de = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SleepDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StunDamageData",
        0x118cd23e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct StunDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.StaminaDamageData",
        0x9d03fcc2 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct StaminaDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub sub_stamina: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyFlashDamageParam.DamageLvData",
        0x203e344e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FlashDamageLvData {
        pub activate_count: i32,
        pub active_time: f32,
    }
}

// snow.enemy.EnemyFlashDamageParam.StanceStatusFlags
rsz_bitflags! {
    pub struct StanceStatusFlags: u32 {
        const STAND = 1;
        const FLY = 2;
        const DIVING = 4;
        const WALL = 8;
        const CEILING = 16;
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FlashDamageData",
        0x3441c585 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FlashDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub damage_lvs: Vec<FlashDamageLvData>,
        pub ignore_refresh_stance: StanceStatusFlags,
        pub max_distance: f32,
        pub min_distance: f32,
        pub angle: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.PoisonDamageData",
        0x91bdc48a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PoisonDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.BlastDamageData",
        0x006e8e35 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct BlastDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub blast_damage: f32,
        pub preset_type: u32,
    }
}

// snow.enemy.EnemyDef.UseDataType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq)]
    pub enum UseDataType {
        Common = 0,
        Unique = 1,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.MarionetteStartDamageData",
        0x5f80df06 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MarionetteStartDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub use_data: UseDataType,
        pub nora_first_limit: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.WaterDamageData.AdjustMeatDownData",
        0xc351f144 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AdjustMeatDownData {
        pub hard_meat_adjust_value: f32,
        pub soft_meat_adjust_value: f32,
        pub judge_meat_value: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.WaterDamageData",
        0x59904df2 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WaterDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub slash_strike_adjust: AdjustMeatDownData,
        pub shell_adjust: AdjustMeatDownData,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FireDamageData",
        0x86f141aa = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FireDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub hit_damage_rate: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.IceDamageData",
        0xdc55cb29 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct IceDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub motion_speed_rate: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyThunderDamageParam.AdjustParamData",
        0x97f24030 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ThunderAdjustParamData {
        pub hit_damage_to_stun_rate: f32,
        pub hit_damage_to_stun_max: f32,
        pub hit_damage_to_stun_min: f32,
        pub default_stun_damage_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ThunderDamageData",
        0x33653378 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ThunderDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub stun_meat_adjust: ThunderAdjustParamData,
        pub normal_meat_adjust: ThunderAdjustParamData,
        pub stun_active_limit: i32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallTrapDamageData",
        0x75132b74 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FallTrapDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub render_offset_y: f32,
        pub render_offset_speed: f32,
        pub render_offset_reset_time: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallQuickSandDamageData",
        0x667f7652 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FallQuickSandDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub render_offset_y: f32,
        pub render_offset_speed: f32,
        pub render_offset_reset_time: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.FallOtomoTrapDamageData",
        0x16e8d19d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct FallOtomoTrapDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub already_poison_stock_value: f32,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.ShockTrapDamageData",
        0x77145ad3 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ShockTrapDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.CaptureDamageData",
        0x0d61e436 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct CaptureDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.KoyashiDamageData",
        0x975b8e3f = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct KoyashiDamageData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub preset_type: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData.SteelFangData",
        0xf4c0bd7b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SteelFangData {
        #[serde(flatten)]
        pub base: ConditionDamageDataBase,
        pub active_limit_count: i32,
        pub preset_type: u32,
        pub is_unique_target_param: bool,
        pub max_distance: f32,
        pub min_distance: f32,
        pub angle: f32,
    }
}

rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum ConditionDamageDataUsed {
        Use = 0,
        NotUse = 1,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionDamageData",
        0x2ca2f7c2 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyConditionDamageData {
        pub paralyze_data: ParalyzeDamageData,
        pub sleep_data: SleepDamageData,
        pub stun_data: StunDamageData,
        pub stamina_data: StaminaDamageData,
        pub flash_data: FlashDamageData,
        pub poison_data: PoisonDamageData,
        pub blast_data: BlastDamageData,
        pub marionette_data: MarionetteStartDamageData,
        pub water_data: WaterDamageData,
        pub fire_data: FireDamageData,
        pub ice_data: IceDamageData,
        pub thunder_data: ThunderDamageData,
        pub fall_trap_data: FallTrapDamageData,
        pub fall_quick_sand_data: FallQuickSandDamageData,
        pub fall_otomo_trap_data: FallOtomoTrapDamageData,
        pub shock_trap_data: ShockTrapDamageData,
        pub shock_otomo_trap_data: ShockTrapDamageData,
        pub capture_data: CaptureDamageData,
        pub koyashi_data: KoyashiDamageData,
        pub steel_fang_data: SteelFangData,
        pub use_paralyze: ConditionDamageDataUsed,
        pub use_sleep: ConditionDamageDataUsed,
        pub use_stun: ConditionDamageDataUsed,
        pub use_stamina: ConditionDamageDataUsed,
        pub use_flash: ConditionDamageDataUsed,
        pub use_poison: ConditionDamageDataUsed,
        pub use_blast: ConditionDamageDataUsed,
        pub use_ride: ConditionDamageDataUsed,
        pub use_water: ConditionDamageDataUsed,
        pub use_fire: ConditionDamageDataUsed,
        pub use_ice: ConditionDamageDataUsed,
        pub use_thunder: ConditionDamageDataUsed,
        pub use_fall_trap: ConditionDamageDataUsed,
        pub use_fall_quick_sand: ConditionDamageDataUsed,
        pub use_fall_otomo_trap: ConditionDamageDataUsed,
        pub use_shock_trap: ConditionDamageDataUsed,
        pub use_shock_otomo_trap: ConditionDamageDataUsed,
        pub use_capture: ConditionDamageDataUsed,
        pub use_dung: ConditionDamageDataUsed,
        pub use_steel_fang: ConditionDamageDataUsed,
    }
}
