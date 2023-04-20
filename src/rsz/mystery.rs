use super::common::*;
use super::*;
use crate::{rsz_enum, rsz_struct};
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
        pub special_mystery_quest_attack_rate: Versioned<f32, 15_00_00>,
        pub special_mystery_quest_mot_speed_rate: Versioned<f32, 15_00_00>,
        pub special_mystery_quest_hp_tbl_no: Versioned<i32, 15_00_00>,
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

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.SuperNovaSpiralShellPresetData.ShellInfo")]
    #[derive(Debug, Serialize)]
    pub struct ShellInfo {
        pub shell_id: i32, // snow.shell.EmCommonShellManager.EmShell320_ID
        pub turn_dir: u8, // snow.enemy.EnemyDef.LeftRightDirection
        pub init_ang_ofs: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.SuperNovaSpiralShellPresetData")]
    #[derive(Debug, Serialize)]
    pub struct SuperNovaSpiralShellPresetData {
        pub shell_list: Vec<ShellInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.TimeSecPresetData")]
    #[derive(Debug, Serialize)]
    pub struct TimeSecPresetData {
        pub time_sec: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.RatePresetData")]
    #[derive(Debug, Serialize)]
    pub struct RatePresetData {
        pub rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.PercentagePresetData")]
    #[derive(Debug, Serialize)]
    pub struct PercentagePresetData {
        pub percentage: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.MaximumActivityReleaseInfoPresetData")]
    #[derive(Debug, Serialize)]
    pub struct MaximumActivityReleaseInfoPresetData {
        pub maximum_activity_release_info_list: Vec<EnemyMysteryMaximumActivityReleaseInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.MaximumToActivityNeedReleaseNumPresetData")]
    #[derive(Debug, Serialize)]
    pub struct MaximumToActivityNeedReleaseNumPresetData {
        pub num_list: Vec<u32>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.AttackRatePresetData")]
    #[derive(Debug, Serialize)]
    pub struct AttackRatePresetData {
        pub attack_rate_list: Vec<SystemMysteryUserDataAttackRate>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.MotSpeedRatePresetData")]
    #[derive(Debug, Serialize)]
    pub struct MotSpeedRatePresetData {
        pub mot_speed_rate_list: Vec<SystemMysteryUserDataMotSpeedRate>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData.MysteryDebuffTimeRatePresetData")]
    #[derive(Debug, Serialize)]
    pub struct MysteryDebuffTimeRatePresetData {
        pub rate_list: Vec<f32>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemDamageUserData.MRConditionDamageResistData")]
    #[derive(Debug, Serialize)]
    pub struct MRConditionDamageResistData {
        pub default_add_rate: f32,
        pub attack_by_marionette_add_rate: f32,
        pub sub_active_time_start_count: i32,
        pub sub_active_time: f32,
        pub min_active_time: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMysteryUserData",
        path = "enemy/user_data/system_mystery_data.user",
        0x618459CD = 10_00_02,
        0x80D85F8E = 13_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct SystemMysteryUserData {
        pub maximum_activity_end_sign_effect_start_time_sec: f32,
        pub anger_poind_add_rate_min: f32,
        pub maximum_activity_block_move_disable_time_sec: f32,
        pub mystery_core_re_pop_interval_sec: f32,
        pub mystery_core_break_damage_ui_height_ofs: f32,
        pub mystery_core_break_damage_ui_color: i32, // snow.gui.GuiDamageDisp.ColorType
        pub mystery_core_dying_vital_rate: f32,
        pub super_nova_shell_fade_out_frame: f32,
        pub super_nova_spiral_shell_preset_list: Vec<SuperNovaSpiralShellPresetData>,
        pub super_nova_spiral_shell_dissolve_frame: f32,
        pub super_nova_spiral_shell_curia_model_scale: f32,
        pub super_nova_spiral_shell_ground_slope_min: f32,
        pub super_nova_spiral_shell_ground_slope_max: f32,
        pub super_nova_spiral_shell_ground_slope_percent: f32,
        pub maximum_activity_enable_damage_reaction: u64, // now.enemy.EnemyDef.IgnoreNoDamageReactionFlag
        pub great_activity_enable_damage_reaction: u64, // now.enemy.EnemyDef.IgnoreNoDamageReactionFlag
        pub activity_enable_damage_reaction: u64, // now.enemy.EnemyDef.IgnoreNoDamageReactionFlag
        pub maximum_activity_enable_condition_damage: i32, // snow.enemy.EnemyDef.ConditionDamageTypeFlag
        pub great_activity_enable_condition_damage: i32, // snow.enemy.EnemyDef.ConditionDamageTypeFlag
        pub activity_enable_condition_damage: i32, // snow.enemy.EnemyDef.ConditionDamageTypeFlag
        pub return_to_great_activity_time_sec_preset_data_list: Vec<TimeSecPresetData>,
        pub maximum_activity_release_info_preset_data_list: Vec<MaximumActivityReleaseInfoPresetData>,
        pub maximum_to_activity_need_release_num_preset_data_list: Vec<MaximumToActivityNeedReleaseNumPresetData>,
        pub maxiimum_activity_min_continue_time_sec_preset_data_list: Vec<TimeSecPresetData>,
        pub maxiimum_activity_failed_end_time_sec_preset_data_list: Vec<TimeSecPresetData>,
        pub add_anger_rate_preset_data_list: Vec<RatePresetData>,
        pub mystery_core_break_damage_rate_preset_data_list: Vec<PercentagePresetData>,
        pub mystery_core_break_parts_damage_rate_preset_data_list: Vec<PercentagePresetData>,
        pub attack_rate_preset_data_list: Vec<AttackRatePresetData>,
        pub mot_speed_rate_preset_data_list: Vec<MotSpeedRatePresetData>,
        pub mystery_debuff_time_rate_preset_data_list: Vec<MysteryDebuffTimeRatePresetData>,
        pub flash_data: Vec<PresetFlashData>,
        pub fall_trap_data: Vec<PresetFallTrapData>,
        pub fall_quick_sand_data: Vec<PresetFallQuickSandData>,
        pub fall_otomo_trap_data: Vec<PresetFallOtomoTrapData>,
        pub shock_trap_data: Vec<PresetShockTrapData>,
        pub shock_otomo_trap_data: Vec<PresetShockOtomoTrapData>,
        pub camera_zoom_request_param: EnemyCameraZoomParam,
        pub camera_zoom_enable_far: f32,
        pub material_interpolation_frame: f32,
        pub mystery_stun_damage_resist_data: MRConditionDamageResistData,
        pub mystery_core_dmg_hit_vfx_wait_game_loop_timer: Versioned<u32, 13_00_00>,
    }
}

// snow.enemy.EnemyDef.OverMysteryStrengthLevel
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum OverMysteryStrengthLevel {
        Default = 0,
        Lv1 = 1,
        Lv2 = 2,
        Lv3 = 3,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyUniqueOverMysteryData.StrenghtLevelData")]
    #[derive(Debug, Serialize)]
    pub struct StrengthLevelData {
        pub strength_level: OverMysteryStrengthLevel,
        pub need_research_level: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyUniqueOverMysteryData.OverMysteryBurstData")]
    #[derive(Debug, Serialize)]
    pub struct OverMysteryBurstData {
        pub need_research_level: u32,
        pub enable_vital_rate: f32,
        pub release_vital_rate: f32,
        pub mot_speed_rate: f32,
        pub attack_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyUniqueOverMysteryData",
        0x8E698E94 = 15_00_00,
        0xAE6C07A4 = 14_00_00,
        0x4BCDD77F = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyUniqueOverMysteryData {
        pub strength_level_list: Vec<StrengthLevelData>,
        pub over_mystery_burst_list: Vec<OverMysteryBurstData>,
        pub overmystery_burst_min_continue_time_use_data_type: Versioned<i32, 14_00_00>,
        pub overmystery_burst_min_continue_time: Versioned<f32, 14_00_00>,
        pub special_mystery_quest_attack_rate: Versioned<f32, 15_00_00>,
        pub special_mystery_quest_mot_speed_rate: Versioned<f32, 15_00_00>,
        pub special_mystery_quest_hp_tbl_no: Versioned<i32, 15_00_00>,
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
        Em001_02 [] {
            pub extend_time_sec: Versioned<f32, 13_00_00>,
            pub recover_rate_by_max_hp: Versioned<f32, 13_00_00>,
        }
        Em001_07 [] {}
        Em002_00 [] {}
        Em002_02 [] {
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
        Em023_00 [] {
            pub extend_time_sec: f32,
            pub recover_rate_by_max_hp: f32,
            pub pump_up_mystery_core_vital_rate: Versioned<f32, 12_00_00>,
        }
        Em023_05 [] {
            pub extend_time_sec: f32,
            pub recover_rate_by_max_hp: f32,
            pub pump_up_mystery_core_vital_rate: Versioned<f32, 13_00_00>,
        }
        Em024_00 [] {}
        //Em024_08 [] {}
        Em025_00 [] {}
        //Em025_08 [] {}
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
        Em071_05 [] {
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
        //Em086_08 [] {}
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
