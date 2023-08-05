use super::*;
use crate::{rsz_enum, rsz_struct};
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct PosInfoBase {
        pub block_no: i32,
        pub unique_id: u32,
        pub pos: Vec3,
    }
}

rsz_enum! {
    #[rsz(i8)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum BlockMovePosSetDataPosType {
        None = 0x00,
        BlockMovePos = 0x01,
        BurrowPortal = 0x02,
        BurrowWarp = 0x03,
        BurrowMovePoint = 0x04,
        TakeOffLandingPoint = 0x05,
        FlyRiseViaPoint = 0x06,
        FlyChangePoint = 0x07,
        GlidingViaPoint = 0x08,
        DiveStartEndPoint = 0x09,
        DiveChangePoint = 0x0A,
        DiveViaPoint = 0x0B,
        JumpChangePoint = 0x0C,
        JumpViaPoint = 0x0D,
        NoNaviMovePos = 0x0E,
        SwimStartEndPoint = 0x0F,
        SwimChangePoint = 0x10,
        SwimViaPoint = 0x11,
        MomongaStartPoint = 0x12,
        MomongaLoopPoint = 0x13,
        MomongaEndPoint = 0x14,
        SpecialFromNaviMove = 0x15,
        SuperJumpChangePoint = 0x16,
        SuperJumpViaPoint = 0x17,
        SuperNoNaviMovePos = 0x18,
    }
}

rsz_struct! {
    #[rsz("snow.BlockMovePosSetData.MovePosInfo")]
    #[derive(Debug, Serialize)]
    pub struct BlockMovePosSetDataMovePosInfo {
        #[serde(flatten)]
        pub base: PosInfoBase,
        pub pos_type: BlockMovePosSetDataPosType,
        pub active_area_land: bool,
        pub active_area_water: bool,
        pub active_area_special01: bool,
        pub active_area_special02: bool,
    }
}

rsz_struct! {
    #[rsz("snow.BlockMovePosSetData")]
    #[derive(Debug, Serialize)]
    pub struct BlockMovePosSetData {
        pub map_no: i32,
        pub pos_info_list: Vec<BlockMovePosSetDataMovePosInfo>
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum EnemyInsideMoveInfoPosType {
        NeckSwingPos = 0x00000000,
        SleepPos = 0x00000001,
        WaterDrinkPos = 0x00000002,
        MapMeetEatPos = 0x00000003,
        InsectHolePos = 0x00000004,
        WaterFallPos = 0x00000005,
        EggEatPos = 0x00000006,
        VegetablesEggEatPos = 0x00000007,
        HoneyEatPos = 0x00000008,
        CactusPos = 0x00000009,
        FishEatPos = 0x0000000A,
        MushroomPos = 0x0000000B,
        RockPos = 0x0000000C,
        WoodNutPos = 0x0000000D,
        LeavingATracePos = 0x0000000E,
        LeavingATrace2Pos = 0x0000000F,
        LeavingATrace3Pos = 0x00000010,
        LeavingATrace4Pos = 0x00000011,
        PopGrass = 0x00000012,
        PopMushroom = 0x00000013,
        PopNut = 0x00000014,
        PopOre = 0x00000015,
        PopBone = 0x00000016,
        PopBeeHive = 0x00000017,
        PopSpiderWeb = 0x00000018,
        PredationPos = 0x00000019,
        RunawayPos = 0x0000001A,
        None = 0x0000001B,
        RandomPos = 0x0000001C,
    }
}

rsz_struct! {
    #[rsz("snow.EnemyInsideMoveInfo")]
    #[derive(Debug, Serialize)]
    pub struct EnemyInsideMoveInfo {
        #[serde(flatten)]
        pub base: PosInfoBase,
        pub radius: f32,
        pub pos_type: EnemyInsideMoveInfoPosType,
        pub active_area_land: bool,
        pub active_area_water: bool,
        pub active_area_special01: bool,
        pub active_area_special02: bool,
        pub angle: f32,
        pub render_offset: bool,
    }
}

rsz_struct! {
    #[rsz("snow.InsideMovePosSetData")]
    #[derive(Debug, Serialize)]
    pub struct InsideMovePosSetData {
        pub map_no: i32,
        pub pos_info_list: Vec<EnemyInsideMoveInfo>
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum EnemyBossInitSetInfoPosType {
        None = 0x00,
    }
}

rsz_struct! {
    #[rsz("snow.EnemyBossInitSetInfo")]
    #[derive(Debug, Serialize)]
    pub struct EnemyBossInitSetInfo {
        #[serde(flatten)]
        pub base: PosInfoBase,
        pub unique_name: String,
        pub radius: f32,
        pub pos_type: EnemyBossInitSetInfoPosType,
        pub angle: f32,
    }
}

rsz_struct! {
    #[rsz("snow.BossInitSetPosSetData")]
    #[derive(Debug, Serialize)]
    pub struct BossInitSetPosSetData {
        pub map_no: i32,
        pub pos_info_list: Vec<EnemyBossInitSetInfo>
    }
}
