use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.InsideMoveInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataInsideMoveInfo {
        pub block_no: i32,
        pub unique_id: u32,
        pub unique_2nd_id: u32,
        pub unique_3rd_id: u32,
        pub flag: bool,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.BlockBasicInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataBlockBasicInfo {
        pub block_no: i32,
        pub non_combat_stay_sec_time: f32,
        pub combat_stay_sec_time: f32,
        pub move_disable_area: bool,
    }
}

// snow.enemy.EnemyFieldParam.BlockMovePatternType
rsz_enum! {
    #[rsz(i8)]
    #[derive(Debug, Serialize, Eq, PartialEq, Copy, Clone)]
    pub enum BlockMovePatternType {
        InvalidBlockMovePatternNo = 0,
        NaviBlockMove = 1,
        BurrowMove = 2,
        FlyMove = 3,
        DiveMove = 4,
        JumpMove = 5,
        SwimMove = 6,
        MomongaMove = 7,
        SpecialFormMove = 8,
        SuperJumpMove = 9,
        Fly2ndMove = 10,
    }
}

impl BlockMovePatternType {
    pub fn display(self) -> &'static str {
        match self {
            BlockMovePatternType::InvalidBlockMovePatternNo => "",
            BlockMovePatternType::NaviBlockMove => "navigate",
            BlockMovePatternType::BurrowMove => "burrow",
            BlockMovePatternType::FlyMove => "fly",
            BlockMovePatternType::DiveMove => "dive",
            BlockMovePatternType::JumpMove => "jump",
            BlockMovePatternType::SwimMove => "swim",
            BlockMovePatternType::MomongaMove => "momonga",
            BlockMovePatternType::SpecialFormMove => "special",
            BlockMovePatternType::SuperJumpMove => "super jump",
            BlockMovePatternType::Fly2ndMove => "launch",
        }
    }
}

// snow.enemy.EnemyFieldParam.MoveStatusType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Eq, PartialEq)]
    pub enum MoveStatusType {
        None = 0,
        LowStamina = 1,
        Dying = 2,
        LowStmDying = 3,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.LotInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataLotInfo {
        pub next_block_no: i32,
        pub lot_value: u32,
        pub move_pattern: BlockMovePatternType,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.LotPrevBlockInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataLotPrevBlockInfo {
        pub lot_info_list: Vec<EnemyBlockMoveDataLotInfo>,
        pub move_status: MoveStatusType,
        pub prev_block_no: i32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.RouteBlockInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataRouteBlockInfo {
        pub block_no: i32,
        pub lot_prev_block_info: Vec<EnemyBlockMoveDataLotPrevBlockInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.RouteInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataRouteInfo {
        pub route_no: u32,
        pub block_info_list: Vec<EnemyBlockMoveDataRouteBlockInfo>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.StageInfo")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataStageInfo {
        pub map_type: i32,
        pub sleep_point_list: Vec<EnemyBlockMoveDataInsideMoveInfo>,
        pub map_meet_eat_point_list: Vec<EnemyBlockMoveDataInsideMoveInfo>,
        pub map_escape_point_list: Vec<EnemyBlockMoveDataInsideMoveInfo>,
        pub ecological_point_list: [EnemyBlockMoveDataInsideMoveInfo; 0], // seems always empty
        pub active_area_land: bool,
        pub active_area_water: bool,
        pub active_area_special01: bool,
        pub active_area_special02: bool,
        pub block_basic_info_list: Vec<EnemyBlockMoveDataBlockBasicInfo>,
        pub route_info_list: Vec<EnemyBlockMoveDataRouteInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData.CalcRotSpeed")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveDataCalcRotSpeed {
        pub move_speed: f32,
        pub rotate_speed: u32,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyBlockMoveData")]
    #[derive(Serialize, Debug)]
    pub struct EnemyBlockMoveData {
        pub default_move_pattern: BlockMovePatternType,
        pub is_enable_fly_stance_to_fly_move: bool,
        pub stage_info_list: Vec<EnemyBlockMoveDataStageInfo>,
        pub break_gate_early_rate: f32,
        pub target_enemy_block_move_early_rate: f32,
        pub main_out_target_player_early_rate: f32,
        pub damag_from_main_out_player_early_rate: f32,
        pub calc_rot_speed_list: Vec<EnemyBlockMoveDataCalcRotSpeed>,
        pub block_move_rotate_ability_up_flag: bool,
        pub block_move_rotate_ability_upmagnification: f32,
    }
}
