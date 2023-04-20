use super::condition_damage_data::*;
use super::*;
use crate::rsz_struct;
use nalgebra_glm::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetParalyzeData",
        0x17a55907 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetParalyzeData {
        #[serde(flatten)]
        pub base: Flatten<ParalyzeDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetSleepData",
        0xdc7c6f18 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetSleepData {
        #[serde(flatten)]
        pub base: Flatten<SleepDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetStunData",
        0x1f170ab4 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetStunData {
        #[serde(flatten)]
        pub base: Flatten<StunDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetStaminaData",
        0x3325de97 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetStaminaData {
        #[serde(flatten)]
        pub base: Flatten<StaminaDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFlashData",
        0x4afcc366 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetFlashData {
        #[serde(flatten)]
        pub base: Flatten<FlashDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetPoison",
        0xc96e7e0e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetPoison {
        #[serde(flatten)]
        pub base: Flatten<PoisonDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetBlastData",
        0xb69fdb89 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetBlastData {
        #[serde(flatten)]
        pub base: Flatten<BlastDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetWater",
        0xf8267eb8 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetWater {
        #[serde(flatten)]
        pub base: Flatten<WaterDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFireData",
        0x3db2de86 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetFireData {
        #[serde(flatten)]
        pub base: Flatten<FireDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetIceData",
        0x270cf819 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetIceData {
        #[serde(flatten)]
        pub base: Flatten<IceDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetThunderData",
        0x725a8ef4 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetThunderData {
        #[serde(flatten)]
        pub base: Flatten<ThunderDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallTrapData",
        0xb4233c06 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetFallTrapData {
        #[serde(flatten)]
        pub base: Flatten<FallTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallQuickSandData",
        0xb9746c2a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetFallQuickSandData {
        #[serde(flatten)]
        pub base: Flatten<FallQuickSandDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallOtomoTrapData",
        0x64c58663 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetFallOtomoTrapData {
        #[serde(flatten)]
        pub base: Flatten<FallOtomoTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetShockTrapData",
        0xbaaae127 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetShockTrapData {
        #[serde(flatten)]
        pub base: Flatten<ShockTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetShockOtomoTrapData",
        0x58465db7 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetShockOtomoTrapData {
        #[serde(flatten)]
        pub base: Flatten<ShockTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetCaptureData",
        0x4c70d9e8 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetCaptureData {
        #[serde(flatten)]
        pub base: Flatten<CaptureDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetKoyashiData",
        0x18bcaf06 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetKoyashiData {
        #[serde(flatten)]
        pub base: Flatten<KoyashiDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetSteelFangData",
        0x747ea8e0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PresetSteelFangData {
        #[serde(flatten)]
        pub base: Flatten<SteelFangData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData",
        path = "enemy/user_data/system_condition_damage_preset_data.user",
        0xe6f93283 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyConditionPresetData {
        pub paralyze_data: Vec<PresetParalyzeData>,
        pub sleep_data: Vec<PresetSleepData>,
        pub stun_data: Vec<PresetStunData>,
        pub flash_data: Vec<PresetFlashData>,
        pub blast_data: Vec<PresetBlastData>,
        pub stamina_data: Vec<PresetStaminaData>,
        pub poison_data: Vec<PresetPoison>,
        pub fire_data: Vec<PresetFireData>,
        pub water_data: Vec<PresetWater>,
        pub ice_data: Vec<PresetIceData>,
        pub thunder_data: Vec<PresetThunderData>,
        pub fall_trap_data: Vec<PresetFallTrapData>,
        pub fall_quick_sand_data: Vec<PresetFallQuickSandData>,
        pub fall_otomo_trap_data: Vec<PresetFallOtomoTrapData>,
        pub shock_trap_data: Vec<PresetShockTrapData>,
        pub shock_otomo_trap_data: Vec<PresetShockOtomoTrapData>,
        pub capture_data: Vec<PresetCaptureData>,
        pub koyashi_data: Vec<PresetKoyashiData>,
        pub steel_fang_data: Vec<PresetSteelFangData>,
    }
}

pub trait ConditionDamage<ConditionPresetType>: Sized {
    const UNIQUE_INDEX: usize;

    fn preset_get_inner(preset: &ConditionPresetType) -> &Self;
    fn get_preset_index(&self) -> u32;
    fn preset_from_package(package: &EnemyConditionPresetData) -> &[ConditionPresetType];

    fn or_preset<'a>(&'a self, package: &'a EnemyConditionPresetData) -> &'a Self
    where
        ConditionPresetType: 'static,
    {
        let index = if let Ok(index) = usize::try_from(self.get_preset_index()) {
            index
        } else {
            eprintln!(
                "Very large preset index in {}",
                std::any::type_name::<ConditionPresetType>()
            );
            return self;
        };
        if index > Self::UNIQUE_INDEX {
            eprintln!(
                "Unknown preset index {} in {}",
                index,
                std::any::type_name::<ConditionPresetType>()
            );
            return self;
        }
        Self::preset_from_package(package)
            .get(index)
            .map(|p| Self::preset_get_inner(p))
            .unwrap_or(self)
    }

    fn verify(package: &EnemyConditionPresetData) -> Result<()> {
        if Self::preset_from_package(package).len() != Self::UNIQUE_INDEX {
            bail!("UNIQUE_INDEX mismatch");
        }
        Ok(())
    }
}

macro_rules! cond {
    ($(($damage:ty, $preset:ty, $preset_member:ident, $unique:expr),)*) => {
        $(impl ConditionDamage<$preset> for $damage {
            const UNIQUE_INDEX: usize = $unique;

            fn preset_get_inner(preset: &$preset) -> &Self {
                &preset.base
            }
            fn get_preset_index(&self) -> u32 {
                self.preset_type
            }
            fn preset_from_package(package: &EnemyConditionPresetData) -> &[$preset] {
                &package.$preset_member
            }
        })*

        impl EnemyConditionPresetData {
            pub fn verify(&self) -> Result<()> {
                $(<$damage as ConditionDamage<$preset>>::verify(self).context(stringify!($damage))?;)*
                Ok(())
            }
        }
    };
}

cond! {
    (ParalyzeDamageData, PresetParalyzeData, paralyze_data, 6),
    (SleepDamageData, PresetSleepData, sleep_data, 4),
    (StunDamageData, PresetStunData, stun_data, 6),
    (FlashDamageData, PresetFlashData, flash_data, 4),
    (BlastDamageData, PresetBlastData, blast_data, 6),
    (StaminaDamageData, PresetStaminaData, stamina_data, 3),
    (PoisonDamageData, PresetPoison, poison_data, 7),
    (FireDamageData, PresetFireData, fire_data, 2),
    (WaterDamageData, PresetWater, water_data, 2),
    (IceDamageData, PresetIceData, ice_data, 2),
    (ThunderDamageData, PresetThunderData, thunder_data, 2),
    (FallTrapDamageData, PresetFallTrapData, fall_trap_data, 1),
    (FallQuickSandDamageData, PresetFallQuickSandData, fall_quick_sand_data, 1),
    (FallOtomoTrapDamageData, PresetFallOtomoTrapData, fall_otomo_trap_data, 1),
    (ShockTrapDamageData, PresetShockTrapData, shock_trap_data, 1),
    (ShockTrapDamageData, PresetShockOtomoTrapData, shock_otomo_trap_data, 1),
    (CaptureDamageData, PresetCaptureData, capture_data, 1),
    (KoyashiDamageData, PresetKoyashiData, koyashi_data, 1),
    (SteelFangData, PresetSteelFangData, steel_fang_data, 2),
}

rsz_struct! {
    #[rsz("snow.enemy.BindWireTotalNumParam")]
    #[derive(Debug, Serialize)]
    pub struct BindWireTotalNumParam {
        pub wire_index: i32,
        pub wire_data: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.BindWireStrength")]
    #[derive(Debug, Serialize)]
    pub struct BindWireStrength {
        pub wire_index: i32,
        pub base_strength: f32,
        pub base_length_min: f32,
        pub base_length_max: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.BindStartPullAdjustParam")]
    #[derive(Debug, Serialize)]
    pub struct BindStartPullAdjustParam {
        pub enable_frame: f32,
        pub disable_dist: f32,
        pub pull_dist: f32,

    }
}

rsz_struct! {
    #[rsz("snow.enemy.MarionetteWireGaugeParam")]
    #[derive(Debug, Serialize)]
    pub struct MarionetteWireGaugeParam {
        pub marionette_gauge_point_max: f32,
        pub base_spend_marionette_gauge_point_per_sec: f32,
        pub weak_attack_hit_regain_marionette_gauge_point: f32,
        pub strong_attack_hit_regain_marionette_gauge_point: f32,
        pub dodge_hit_regain_marionette_gauge_point: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMarionetteUserData.SystemMarionetteStartDamageData")]
    #[derive(Debug, Serialize)]
    pub struct SystemMarionetteStartDamageData {
        #[serde(flatten)]
        pub base: Flatten<MarionetteStartDamageData>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.HitStopInfo")]
    #[derive(Debug, Serialize)]
    pub struct HitStopInfo {
        pub hit_stop_frame: i16,
        pub border_mario_damage_l_bottom: i16,
        pub border_mario_damage_l_top: i16,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteAttackAdjustInfo.AdjustValueByDirection")]
    #[derive(Debug, Serialize)]
    pub struct AdjustValueByDirection {
        pub front: f32,
        pub back: f32,
        pub left: f32,
        pub right: f32,
        pub neutral: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteAttackAdjustInfo")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMarionetteAttackAdjustInfo {
        pub week_attack_info: AdjustValueByDirection,
        pub strong_attack_info: AdjustValueByDirection,
        pub final_attack_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteAttackRate")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMarionetteAttackRate {
        pub week: f32,
        pub strong: f32,
        pub final_: f32,
        pub shoot: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteAttackModeRate")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMarionetteAttackModeRate {
        pub attack_rate: EnemyMarionetteAttackRate,
        pub parts_attack_rate: EnemyMarionetteAttackRate
    }
}

rsz_struct! {
    #[rsz("snow.enemy.MarionetteModePower")]
    #[derive(Debug, Serialize)]
    pub struct MarionetteModePower {
        pub attack_mode_rate: EnemyMarionetteAttackModeRate
    }
}

rsz_struct! {
    #[rsz("snow.enemy.MarionetteModeReward")]
    #[derive(Debug, Serialize)]
    pub struct MarionetteModeReward {
        pub attack_mode_rate: EnemyMarionetteAttackModeRate,
        pub drop_item_max_num: i32,
        pub max_wall_hit_drop_item_num: i32,
        pub wall_hit_drop_item_max: i32,
        pub wall_hit_drop_item_cont: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemMarionetteUserData",
        path = "enemy/user_data/system_mario_data.user",
    )]
    #[derive(Debug, Serialize)]
    pub struct SystemMarionetteUserData {
        pub bind_wire_joint_name_hash: u32,
        pub wire_total_num_param: Vec<BindWireTotalNumParam>,
        pub otomo_wire_total_num_param: BindWireTotalNumParam,
        pub props_wire_total_num_param: Vec<BindWireTotalNumParam>,
        pub wire_strength_param: Vec<BindWireStrength>,
        pub otomo_wire_strength_param: BindWireStrength,
        pub otomo_wire_start_pull_adjust_param: BindStartPullAdjustParam,
        pub props_wire_strength_param: Vec<BindWireStrength>,
        pub is_props_wire_damage_reaction: bool,
        pub props_bind_wire_max_num: u32,
        pub wire_gauge_recover_time_sec: f32,
        pub marionette_wire_gauge_param: MarionetteWireGaugeParam,
        pub zako_hit_marionette_gauge_point_add_rate: f32,
        pub final_attack_limit_time: f32,
        pub zako_hit_final_attack_point_add_rate: f32,
        pub final_attack_point_add_by_target_enemy_damage_max_hp_rate: f32,
        pub final_attack_point_add_rate_by_arround_enemy_damage_in_hyakuryu: f32,
        pub final_attack_move_end_time: f32,
        pub is_cancel_pre_input_on: bool,
        pub cancel_pre_input_last_sec: f32,
        pub free_run_target_pos_update_time_frame: f32,
        pub free_run_dash_transition_frame: f32,
        pub mario_damage_type_stock_value: f32,
        pub mario_cool_time: f32,
        pub marionette_start_damage_data: SystemMarionetteStartDamageData,
        pub ride_on_pop_radius: f32,
        pub start_wait_loop_sub_time_max_hp_rate: f32,
        pub near_attack_sa_time: f32,
        pub far_attack_sa_time: f32,
        pub dodge_sa_time: f32,
        pub quick_dodge_sa_time: f32,
        pub near_attack_concel_forbidden_time: f32,
        pub far_attack_cancel_forbidden_time: f32,
        pub damage_cancel_forbidden_time: f32,
        pub marionette_damage_reduce_wire_time: f32,
        pub marionette_dodge_blocking_timer_m: f32,
        pub marionette_dodge_blocking_timer_s: f32,
        pub marionette_dodge_blocking_add_final_attack_gauge_point_m: i16,
        pub marionette_dodge_blocking_add_final_attack_gauge_point_s: i16,
        pub dodge_blocking_hit_stop_frame_m: f32,
        pub dodge_blocking_hit_stop_frame_s: f32,
        pub dodge_blocking_damage_rate_m: f32,
        pub dodge_blocking_damage_rate_s: f32,
        pub dodge_blocking_damage_mot_speed_rate_s: f32,
        pub dodge_blocking_damage_mot_speed_rate_m: f32,
        pub getup_invicible_time_m: f32,
        pub getup_invicible_time_s: f32,
        pub friendly_fire_damage_border: f32,
        pub friendly_fire_gauge_damage: i32,
        pub friendly_damage_rate: f32,
        pub shoot_wall_hit_time_s: f32,
        pub dash_shoot_wall_hit_time_s: f32,
        pub shoot_wall_hit_damage_rate_list: Vec<f32>,
        pub shoot_wall_hit_damage_add_rate_dash: f32,
        pub shoot_wall_hit_damage_add_rate_good_timing: f32,
        pub shoot_enemy_hit_damage_rate: f32,
        pub challenge_start_use_wire_gauge_num: i32,
        pub max_challenge_num: u32,
        pub finish_shoot_challenge_free_run_dash_transition_frame: f32,
        pub last_challenge_input_enable_time_frame: f32,
        pub challgne_create_wire_num_list: Vec<u32>,
        pub shoot_down_time_sec_list: Vec<f32>,
        pub challenge_start_input_wait_mot_speed_list: Vec<f32>,
        pub finish_shoot_challenge_free_camera_transition_frame: f32,
        pub drop_item_percentage: u32,
        pub drop_item_max_num: i32,
        pub drop_item_model_type: i32, // snow.enemy.SystemEnemyDropItemMoveData.ModelTypes
        pub marionette_enemy_target_continue_time_sec: f32,
        pub marionette_enemy_target_cool_time_sec: f32,
        pub attack_hit_stop_info: Vec<HitStopInfo>,
        pub marionette_yarare_damage_border: i32,
        pub marionette_gauge_damage: i32,
        pub attack_parts_vital_adjust_rate: f32,
        pub marionette_l_cool_time_frame: i32,
        pub ignore_marionette_ride_on_option_damage_rate: f32,
        pub solo_target_ofs: Vec3,
        pub attack_adjust_info: EnemyMarionetteAttackAdjustInfo,
        pub combo_after_mot_interpolate_add_frame: u32,
        pub mode_power: MarionetteModePower,
        pub mode_reward: MarionetteModeReward,
        pub otomo_bind_wire_max_num: u32,
        pub otomo_bind_wire_second_num: u32,
        pub otomo_bind_wire_total_num_param_list: Vec<BindWireTotalNumParam>,
        pub otomo_bind_wire_strength_param_list: Vec<BindWireStrength>,
        pub twin_marionette_start_wait_time: Versioned<f32, 15_00_00>,
        pub twin_marionette_finak_attack_point_add_rate: Versioned<f32, 15_00_00>,
        pub twin_marionette_gauge_point_sub_rate: Versioned<f32, 15_00_00>,
        pub delay_sarvant_twin_marionette_sec: Versioned<f32, 15_00_00>,
        pub twin_marionette_final_attack_guide_delay_sec: Versioned<f32, 15_00_00>,
    }
}
