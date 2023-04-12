use super::common::*;
use super::*;
use crate::rsz_struct;
use nalgebra_glm::*;
use once_cell::sync::Lazy;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyMysteryCorePartsData.MysteryCoreEffectSettingInfo")]
    #[derive(Debug, Serialize)]
    pub struct MysteryCoreEffectSettingInfo {
        pub effect_const_joint_name: String,
        pub effect_const_offset: Vec3,
        pub effect_const_scale: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMysteryCorePartsData",
        0x795FE090 = 12_00_00,
        0x106CA3B1 = 10_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMysteryCorePartsData {
        pub is_core_candidate: bool,
        pub maximum_activity_vital: i32,
        pub activate_percentage: u32,
        pub link_parts_list: Vec<u16>, // snow.enemy.EnemyMysteryCorePartsData.VitalSharePartsGroup
        pub effect_info_list: Vec<MysteryCoreEffectSettingInfo>,
        pub prohibit_same_apply_group: Versioned<i32, 12_00_00>, // snow.enemy.EnemyMysteryCorePartsData.CoreGroup
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMysteryMaximumActivityReleaseInfo")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMysteryMaximumActivityReleaseInfo {
        pub release_damage_rate: f32,
        pub carry_over_limit_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.AttackRate")]
    #[derive(Debug, Serialize)]
    pub struct SystemMysteryUserDataAttackRate {
        pub maximum_activity: f32,
        pub great_activity: f32,
        pub activity: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.MotSpeedRate")]
    #[derive(Debug, Serialize)]
    pub struct SystemMysteryUserDataMotSpeedRate {
        pub maximum_activity: f32,
        pub great_activity: f32,
        pub activity: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyUniqueMysteryData.ConditionDamageData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyUniqueMysteryDataConditionDamageData {
        pub flash_damage_use_preset_type: i32,
        pub flash_damage_data: FlashDamageData,
        pub fall_trap_use_preset_type: i32,
        pub fall_trap_data: FallTrapDamageData,
        pub fall_quick_sand_use_preset_type: i32,
        pub fall_quick_sand_data: FallQuickSandDamageData,
        pub fall_otomo_trap_use_preset_type: i32,
        pub fall_otomo_trap_data: FallOtomoTrapDamageData,
        pub shock_trap_use_preset_type: i32,
        pub shock_trap_data: ShockTrapDamageData,
        pub shock_otomo_trap_use_preset_type: i32,
        pub shock_otomo_trap_data: ShockTrapDamageData,
    }
}

rsz_struct! {
    #[rsz("snow.camera.CameraSuggestion_PlayerCamera.EnemyCameraZoomParam")]
    #[derive(Debug, Serialize)]
    pub struct EnemyCameraZoomParam {
        pub zoom_keep_time: f32,
        pub zoom_camera_offset_add: f32,
        pub zoom_time: f32,
        pub zoom_curve: i32, // via.curve.EaseType
        pub return_to_default_time: f32,
        pub return_to_default_curve: i32, // via.curve.EaseType
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyUniqueMysteryData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyUniqueMysteryData {
        pub maximum_activity_core_num: i32,
        pub mystery_core_parts_data: Vec<EnemyMysteryCorePartsData>,
        pub maximum_activity_release_last_attack_parts: u16, // snow.enemy.EnemyDef.PartsGroup
        pub shell_scale_list_max: i32,
        pub return_to_great_activity_time_use_data_type: i32,
        pub return_to_great_activity_time_sec: f32,
        pub maximum_activity_release_info_use_data_type: i32,
        pub maximum_activity_release_info: Vec<EnemyMysteryMaximumActivityReleaseInfo>,
        pub maximum_to_activity_need_release_num_use_data_type: i32,
        pub maximum_to_activity_need_release_num: Vec<u32>,
        pub maxiimum_activity_min_continue_time_use_data_type: i32,
        pub maxiimum_activity_min_continue_time_sec: f32,
        pub maxiimum_activity_failed_end_time_use_data_type: i32,
        pub maxiimum_activity_failed_end_time_sec: f32,
        pub add_anger_rate_use_data_type: i32,
        pub add_anger_rage: f32,
        pub mystery_core_break_damage_rate_use_data_type: i32,
        pub mystery_core_break_damage_rate: f32,
        pub mystery_core_break_parts_damage_rate_use_data_type: i32,
        pub mystery_core_break_parts_damage_rate: f32,
        pub attack_rate_use_data_type: i32,
        pub attack_rate: Vec<SystemMysteryUserDataAttackRate>,
        pub mot_speed_rate_use_data_type: i32,
        pub mot_speed_rate: Vec<SystemMysteryUserDataMotSpeedRate>,
        pub mystery_debuff_time_rate_use_data_type: i32,
        pub mystery_debuff_time_rate: Vec<f32>,
        pub condition_damage_data: Vec<EnemyUniqueMysteryDataConditionDamageData>,
        pub camera_zoom_request_param_use_data_type: i32,
        pub camera_zoom_request_param: EnemyCameraZoomParam,
        pub camera_zoom_enable_far_use_data_type: i32,
        pub camera_zoom_enable_far: f32,
        pub curia_scale_use_data_type: i32,
        pub curia_scale: f32,
        pub non_combat_fly_pos_offset_use_data_type: i32,
        pub non_combat_fly_pos_offset: Vec3,
        pub const_joint_use_data_type: i32,
        pub const_joint_name_hash: u32,
    }
}

pub mod unique_mystery {
    use super::*;
    use anyhow::{Context, Result};
    macro_rules! def_unique_mystery {
        ($($name:ident [$($vhash:literal=$version:literal),*] {
            $(
                $(#[$inner_meta:meta])*
                $inner_vis:vis $field_name:ident : $field_type:ty
            ),*$(,)?
        })*) => {
            pub mod extra {
                use super::*;
                $(rsz_struct! {
                    #[rsz()]
                    #[derive(Debug, Serialize)]
                    pub struct $name {
                        $(
                            $(#[$inner_meta])* #[allow(dead_code)]
                            $inner_vis $field_name : $field_type,
                        )*
                    }
                })*
            }

            $(rsz_struct! {
                #[rsz({concat!("snow.enemy.", stringify!($name), "UniqueMysteryData")}
                    $(,$vhash = $version)*
                )]
                #[derive(Debug, Serialize)]
                pub struct $name {
                    #[serde(flatten)]
                    pub base: Flatten<EnemyUniqueMysteryData>,
                    #[serde(flatten)]
                    pub extra: extra::$name
                }
            })*

            #[derive(Debug, Serialize)]
            pub enum Extra {
                $($name(extra::$name),)*
            }

            #[derive(Debug, Serialize)]
            pub struct EnemyUniqueMysteryDataWrapper {
                #[serde(flatten)]
                pub base: EnemyUniqueMysteryData,
                pub extra: Extra,
            }

            pub fn unique_mystery_type_map() -> HashMap<u32, RszTypeInfo> {
                let mut m = HashMap::new();
                $(register::<$name>(&mut m);)*
                m
            }

            pub mod loader {
                use std::rc::*;
                use anyhow::{anyhow, Context, Result};
                use super::FromRsz;
                $(
                    #[allow(non_snake_case)]
                    pub fn $name(rsz: super::AnyRsz) -> Result<super::EnemyUniqueMysteryDataWrapper> {
                        let downcasted = rsz.downcast::<super::$name>()
                            .with_context(||format!("Unexpected type for {}", <super::$name>::SYMBOL))?;
                        let value = Rc::try_unwrap(downcasted)
                            .map_err(|_|anyhow!("Shared node for {}", <super::$name>::SYMBOL))?;
                        Ok(super::EnemyUniqueMysteryDataWrapper {
                            base: value.base.0,
                            extra: super::Extra::$name(value.extra)
                        })
                    }
                )*
            }

            pub static UNIQUE_MYSTERY_TYPE_MAP: Lazy<HashMap<u32, fn(AnyRsz) -> Result<EnemyUniqueMysteryDataWrapper>>> = Lazy::new(|| {
                HashMap::from_iter([
                    $((<$name>::type_hash(), loader::$name as fn(AnyRsz) -> Result<EnemyUniqueMysteryDataWrapper>),)*
                ])
            });
        };
    }
    def_unique_mystery! {
        Em001_00 [] {}
        Em001_02 [0x09F99CEE = 11_00_01, 0xB48B2175 = 12_00_00, 0x7E40EC28 = 13_00_00] {
            pub extend_time_sec: Versioned<f32, 13_00_00>,
            pub recover_rate_by_max_hp: Versioned<f32, 13_00_00>,
        }
        Em001_07 [] {}
        Em002_00 [] {}
        Em002_02 [0x14B4271E = 11_00_01, 0x2C1FE57C = 12_00_00, 0x223A2314 = 13_00_00] {
            pub extend_time_sec: Versioned<f32, 13_00_00>,
            pub recover_rate_by_max_hp: Versioned<f32, 13_00_00>,
        }
        Em002_07 [] {}
        Em003_00 [] {}
        Em004_00 [] {}
        Em007_00 [] {}
        Em007_07 [] {}
        Em019_00 [] {}
        Em020_00 [] {}
        Em023_00 [0x9271138B = 10_00_02, 0xC5916624 = 11_00_01, 0x94A0B8D1 = 12_00_00] {
            pub extend_time_sec: f32,
            pub recover_rate_by_max_hp: f32,
            pub pump_up_mystery_core_vital_rate: Versioned<f32, 12_00_00>,
        }
        Em023_05 [0xEF633848 = 10_00_02, 0x28DDDFC3 = 11_00_01, 0x47FD2BD7 = 12_00_00, 0x02000E1A = 13_00_00] {
            pub extend_time_sec: f32,
            pub recover_rate_by_max_hp: f32,
            pub pump_up_mystery_core_vital_rate: Versioned<f32, 13_00_00>,
        }
        Em024_00 [] {}
        Em024_08 [] {}
        Em025_00 [] {}
        Em025_08 [] {}
        Em027_00 [] {}
        Em032_00 [] {}
        Em037_00 [] {}
        Em037_02 [] {}
        Em042_00 [] {}
        Em044_00 [] {}
        Em047_00 [] {}
        Em054_00 [] {}
        Em057_00 [] {}
        Em057_07 [] {}
        Em059_00 [] {}
        Em060_00 [] {}
        Em060_07 [] {}
        Em061_00 [] {}
        Em062_00 [] {}
        Em071_00 [] {
            pub extend_maxiimum_activity_time: f32,
            pub recovery_maxiimum_activity_hp: f32,
        }
        Em071_05 [0x7929AF5B = 13_00_00, 0x235101DE = 14_00_00] {
            pub maximum_activity_release_last_attack_unique: Versioned<Vec<u16>, 14_00_00>, // snow.enemy.EnemyDef.PartsGroup
            pub extend_maxiimum_activity_time: Versioned<f32, 14_00_00>,
            pub recovery_maxiimum_activity_hp: Versioned<f32, 14_00_00>,
            pub after_burst_virus_rate_recovery: Versioned<f32, 14_00_00>,
        }
        Em072_00 [] {}
        Em077_00 [] {}
        Em081_00 [] {}
        Em082_00 [] {}
        Em082_02 [] {}
        Em082_07 [] {}
        Em086_05 [] {}
        Em086_08 [] {}
        Em089_00 [] {}
        Em089_05 [] {
            pub demon_one_ready_limit: f32,
            pub demon_two_ready_limit: f32,
            pub demon_one_damage_rate: f32,
            pub demon_two_damage_rate: f32,
            pub add_mystery_maximum_time_limit: f32,
            pub add_mystery_maximum_hp_rate: f32,
        }
        Em090_00 [] {}
        Em090_01 [] {}
        Em091_00 [] {}
        Em092_00 [] {}
        Em093_00 [] {}
        Em093_01 [] {}
        Em094_00 [] {}
        Em094_01 [] {}
        Em095_00 [] {}
        Em095_01 [] {}
        Em096_00 [] {}
        Em097_00 [] {}
        Em098_00 [] {}
        Em099_00 [] {}
        Em099_05 [] {}
        Em100_00 [] {}
        Em102_00 [] {}
        Em107_00 [] {}
        Em108_00 [] {}
        Em109_00 [] {}
        Em118_00 [] {}
        Em118_05 [] {}
        Em124_00 [] {}
        Em131_00 [] {}
        Em132_00 [] {}
        Em133_00 [] {}
        Em134_00 [] {}
        Em135_00 [] {}
        Em136_00 [] {}
        Em136_01 [] {}

    }
}

pub use unique_mystery::{EnemyUniqueMysteryDataWrapper, UNIQUE_MYSTERY_TYPE_MAP};
