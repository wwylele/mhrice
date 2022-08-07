use super::*;
use crate::rsz_bitflags;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

// snow.enemy.EnemyDef.ExtractiveType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
    pub enum ExtractiveType {
        Red = 0,
        White = 1,
        Orange = 2,
        None = 3,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsData",
        0x0F4645E0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyPartsData {
        pub vital: i32,
        pub master_vital: i32,
        pub extractive_type: ExtractiveType,
    }
}

// snow.enemy.EnemyDataTune.PartsBreakData.IgnoreCondition
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum PartsBreakDataIgnoreCondition {
        None = 0,
        InTimes = 1,
        Equal = 2,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.PartsBreakData",
        0x76BFB370 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DataTunePartsBreakData {
        pub break_level: i32,
        pub vital: i32,
        pub master_vital: i32,
        pub ignore_condition: PartsBreakDataIgnoreCondition,
        pub ignore_check_count: i32,
        pub reward_data: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.EnemyPartsBreakData",
        0x00474562 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DataTuneEnemyPartsBreakData {
        pub parts_group: u16, // snow.enemy.EnemyDef.PartsGroup
        pub parts_break_data_list: Vec<DataTunePartsBreakData>,
    }
}

// snow.enemy.EnemyDataTune.PartsLossData.PermitDamageAttrEnum
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum PermitDamageAttrEnum {
        Slash = 0,
        Strike = 1,
        All = 2,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.PartsLossData",
        0x860B5A4B = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DataTunePartsLossData {
        pub vital: i32,
        pub master_vital: i32,
        pub permit_damage_attr: PermitDamageAttrEnum,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.EnemyPartsLossData",
        0x4f9e8a18 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DataTuneEnemyPartsLossData {
        pub parts_group: u16, // snow.enemy.EnemyDef.PartsGroup
        pub parts_loss_data: DataTunePartsLossData,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnablePartsGroup",
        0x9c77985e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnablePartsGroup {
        pub enable_parts: Vec<bool>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.MultiPartsVital",
        0xD4323CD7 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MultiPartsVital {
        pub vital: i32,
        pub master_vital: i32,
    }
}

// snow.enemy.EnemyDef.DamageCategoryFlag
rsz_bitflags! {
    pub struct DamageCategoryFlag: u32 {
        const MARIONETTE_FRIENDLY_FIRE = 0x00000001;
        const MARIONETTE_START         = 0x00000002;
        const PARTS_LOSS               = 0x00000004;
        const FALL_TRAP                = 0x00000008;
        const SHOCK_TRAP               = 0x00000010;
        const PARALYZE                 = 0x00000020;
        const SLEEP                    = 0x00000040;
        const STUN                     = 0x00000080;
        const MARIONETTE_L             = 0x00000100;
        const FLASH                    = 0x00000200;
        const SOUND                    = 0x00000400;
        const GIMMICK_L                = 0x00000800;
        const EM2_EM_L                 = 0x00001000;
        const GIMMICK_KNOCK_BACK       = 0x00002000;
        const HIGH_PARTS               = 0x00004000;
        const MARIONETTE_M             = 0x00008000;
        const GIMMICK_M                = 0x00010000;
        const EM2_EM_M                 = 0x00020000;
        const MULTI_PARTS              = 0x00040000;
        const ELEMENT_WEAK             = 0x00080000;
        const PARTS_BREAK              = 0x00100000;
        const SLEEP_END                = 0x00200000;
        const STAMINA                  = 0x00400000;
        const PARTS                    = 0x00800000;
        const MARIONETTE_S             = 0x01000000;
        const GIMMICK_S                = 0x02000000;
        const EM2_EM_S                 = 0x04000000;
        const STEEL_FANG               = 0x08000000;
        const MYSTERY_MAXIMUM_ACTIVITY_RELEASE = 0x10000000;
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMultiPartsSystemVitalData",
        0x12DECB46 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMultiPartsSystemVitalData {
        #[serde(flatten)]
        pub base: Flatten<EnemyMultiPartsVitalData>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMultiPartsVitalData",
        0xCCB7000D = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMultiPartsVitalData {
        pub use_type: UseDataType,
        pub priority: u32,
        pub enable_parts_data: [EnablePartsGroup; 1],
        pub enable_last_attack_parts: Vec<String>,
        pub is_enable_hyakuryu: bool,
        pub is_enable_overwrite_down: bool,
        pub is_prio_damage_customize: bool,
        pub prio_damage_catagory_flag: DamageCategoryFlag,
        pub is_not_use_difficulty_rate: bool,
        pub is_multi_rate_ex: bool,
        pub multi_parts_vital_data: Vec<MultiPartsVital>,
        pub enable_parts_names: Vec<String>,
        pub enable_parts_values: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyGimmickVitalData",
        0xb6c04bfd = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyGimmickVitalData {
        pub vital_s: i32,
        pub vital_m: i32,
        pub vital_l: i32,
        pub vital_knock_back: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMarionetteVitalData",
        0x4e4e9113 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMarionetteVitalData {
        pub vital_s: i32,
        pub vital_m: i32,
        pub vital_l: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune.CharacterContollerTune", // yep, official typo
        0xdb63fcc7 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct CharacterContollerTune {
        pub radius: f32,
        pub offset_y: f32,
    }
}

// snow.hit.HitWeight
rsz_enum! {
    #[rsz(u8)]
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Serialize)]
    pub enum HitWeight {
        PushedOnly = 0,
        SSSS = 1,
        SSS = 2,
        SS = 3,
        S = 4,
        Normal = 5,
        L = 6,
        LL = 7,
        LLL = 8,
        LLLL = 9,
        SpUse = 10,
        NoMove = 11,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyDataTune",
        0xC170C4DC = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyDataTune {
        pub base_hp_vital: i32,
        pub master_hp_vital: i32,
        pub enemy_parts_data: Vec<EnemyPartsData>,
        pub enemy_parts_break_data_list: Vec<DataTuneEnemyPartsBreakData>,
        pub enemy_parts_loss_data_list: Vec<DataTuneEnemyPartsLossData>,
        pub enemy_multi_parts_vital_system_data: [EnemyMultiPartsSystemVitalData; 3],
        pub enemy_multi_parts_vital_data_list: Vec<EnemyMultiPartsVitalData>,
        pub gimmick_vital_data: EnemyGimmickVitalData,
        pub marionette_vital_data: EnemyMarionetteVitalData,
        pub terrain_action_check_dist: f32,
        pub adjust_wall_point_offset: f32,
        pub character_controller_tune_data: Vec<CharacterContollerTune>,
        pub weight: HitWeight,
        pub dying_village_hp_vital_rate: f32,
        pub dying_low_level_hp_vital_rate: f32,
        pub dying_high_level_hp_vital_rate: f32,
        pub dying_master_class_hp_vital_rate: f32,
        pub capture_village_hp_vital_rate: f32,
        pub capture_low_level_hp_vital_rate: f32,
        pub capture_high_level_hp_vital_rate: f32,
        pub capture_master_level_hp_vital_rate: f32,
        pub self_sleep_recover_hp_vital_rate: f32,
        pub self_sleep_time: f32,
        pub in_combat_self_sleep_flag: bool,
        pub dummy_shadow_scale: f32,
        // "group shell"?
        pub max_num_for_normal_quest: i32,
        pub max_num_for_hyakuryu_quest: i32,
        pub max_sound_damage_count: i32,
    }
}
