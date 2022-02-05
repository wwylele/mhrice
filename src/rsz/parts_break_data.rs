use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.PartsLockParam",
        0xa98bb235 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsLockParam {
        pub hash_value: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.PartsBreakData",
        0x4a6f1d70 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsBreakData {
        pub parts_condition_id: i32,
        pub effect_container_id: i32,
        pub effect_element_id: i32,
        pub ignore_tag_value: u32, // refers to ignore tag name hash in rcol
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.ConditionPartsBreakData",
        0x1bba24cb = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ConditionPartsBreakData {
        pub condition_id: i32,
        pub parts_break_data_list: Vec<PartsBreakData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.PartsBreakGroupData",
        0x28fa8520 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsBreakGroupData {
        pub parts_group: u16, // snow.enemy.EnemyDef.PartsGroup
        pub parts_lock_group_hash: Vec<PartsLockParam>,
        pub condition_parts_break_data_list: Vec<ConditionPartsBreakData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.PartsLossData",
        0x2e4f23aa = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsLossData {
        pub parts_condition_id: i32,
        pub ignore_tag_value: u32,
        pub parts_loss_effect_container_id: u32,
        pub parts_loss_effect_element_id: u32,
        pub on_ground_effect_container_id: u32,
        pub on_ground_effect_element_id: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.ConditionPartsLossData",
        0x45e48140 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ConditionPartsLossData {
        pub condition_id: i32,
        pub parts_loss_data: PartsLossData,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData.PartsLossGroupData",
        0x845557b3 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartsLossGroupData {
        pub parts_group: u16, // snow.enemy.EnemyDef.PartsGroup
        pub parts_lock_group_hash: Vec<PartsLockParam>,
        pub condition_parts_loss_data_list: Vec<ConditionPartsLossData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyPartsBreakData",
        0xd8364e1a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyPartsBreakData {
        pub parts_break_group_data_list: Vec<PartsBreakGroupData>,
        pub parts_loss_group_data_list: Vec<PartsLossGroupData>,
    }

}
