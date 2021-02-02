use super::*;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyPartsData {
        vital: i32,
        extractive_type: u32, // red, white, orange, none
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.PartsBreakData")]
    #[derive(Debug, Serialize)]
    pub struct DataTunePartsBreakData {
        break_level: i32,
        vital: i32,
        ignore_condition: i32, // none, in_times, equal
        ignore_check_count: i32,
        reward_data: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.EnemyPartsBreakData")]
    #[derive(Debug, Serialize)]
    pub struct DataTuneEnemyPartsBreakData {
        parts_group: u32,
        parts_break_data_list: Vec<DataTunePartsBreakData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.PartsLossData")]
    #[derive(Debug, Serialize)]
    pub struct DataTunePartsLossData {
        vital: i32,
        permit_damage_attr: u32, // slash, strike, all
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.EnemyPartsLossData")]
    #[derive(Debug, Serialize)]
    pub struct DataTuneEnemyPartsLossData {
        parts_group: u32,
        parts_loss_data: DataTunePartsLossData,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnablePartsGroup")]
    #[derive(Debug, Serialize)]
    pub struct EnablePartsGroup {
        pub enable_parts: Vec<u8>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.MultiPartsVital")]
    #[derive(Debug, Serialize)]
    pub struct MultiPartsVital {
        vital: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMultiPartsSystemVitalData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMultiPartsSystemVitalData {
        //?
        use_type: u32, // common, unique
        enable_parts_data_num: i32,
        priority: u32,
        enable_parts_data: EnablePartsGroup,
        enable_last_attack_parts: Vec<u32>, //?
        is_enable_hyakuryu: u8,
        is_enable_overwrite_down: u8,
        is_prio_damage_customize: u8,
        prio_damage_catagory_flag: u32, // see DamageCategoryFlag
        is_multi_rate_ex: u8,
        multi_parts_vital_data: Vec<MultiPartsVital>,
        enable_parts_names: Vec<Utf16String>,
        enable_parts_values: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMultiPartsVitalData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMultiPartsVitalData {
        //?
        use_type: u32, // common, unique
        enable_parts_data_num: i32,
        priority: u32,
        enable_parts_data: EnablePartsGroup,
        enable_last_attack_parts: Vec<u32>, //?
        is_enable_hyakuryu: u8,
        is_enable_overwrite_down: u8,
        is_prio_damage_customize: u8,
        prio_damage_catagory_flag: u32, // see DamageCategoryFlag
        is_multi_rate_ex: u8,
        multi_parts_vital_data: Vec<MultiPartsVital>,
        enable_parts_names: Vec<Utf16String>,
        enable_parts_values: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyGimmickVitalData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyGimmickVitalData {
        pub vital_s: i32,
        pub vital_m: i32,
        pub vital_l: i32,
        pub vital_knock_back: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteVitalData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMarionetteVitalData {
        pub vital_s: i32,
        pub vital_m: i32,
        pub vital_l: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.CharacterContollerTune")] // yep, official typo
    #[derive(Debug, Serialize)]
    pub struct CharacterContollerTune {
        pub radius: f32,
        pub offset_y: f32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune")]
    #[derive(Debug, Serialize)]
    pub struct EnemyDataTune {
        pub base_hp_vital: i32,
        pub enemy_parts_data: Vec<EnemyPartsData>,
        pub enemy_parts_break_data_list: Vec<DataTuneEnemyPartsBreakData>,
        pub enemy_parts_loss_data_list: Vec<DataTuneEnemyPartsLossData>,
        pub enemy_multi_parts_vital_system_data: Vec<EnemyMultiPartsSystemVitalData>,
        pub enemy_multi_parts_vital_data_list: Vec<EnemyMultiPartsVitalData>,
        pub gimmick_vital_data: EnemyGimmickVitalData,
        pub marionette_vital_data: EnemyMarionetteVitalData,
        pub terrain_action_check_dist: f32,
        pub adjust_wall_point_offset: f32,
        pub character_controller_tune_data: Vec<CharacterContollerTune>,
        pub weight: u32,
        pub dying_village_hp_vital_rate: f32,
        pub dying_low_level_hp_vital_rate: f32,
        pub dying_high_level_hp_vital_rate: f32,
        pub capture_village_hp_vital_rate: f32,
        pub capture_low_level_hp_vital_rate: f32,
        pub capture_high_level_hp_vital_rate: f32,
        pub self_sleep_recover_hp_vital_rate: f32,
        pub self_sleep_time: f32,
        pub in_combat_self_sleep_flag: u32,
        pub dummy_shadow_scale: f32,
        // "group shell"?
        pub max_num_for_normal_quest: i32,
        pub max_num_for_hyakuryu_quest: i32,
    }
}
