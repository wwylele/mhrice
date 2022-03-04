use super::common::*;
use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.gui.userdata.GuiMapScaleDefineData.MaskSetting",
        0x7545b1a3 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct MaskSetting {
        pub is_mask_on: bool,
        pub type_: i32, // snow.gui.QuestUIManager.QuestMapFLMaskType
        pub pos: ViaVec3,
        pub s_size_w: f32,
        pub s_size_h: f32,
        pub rotation_z: f32,
        pub priority: i32,
    }
}

rsz_struct! {
    #[rsz("snow.gui.userdata.GuiMapScaleDefineData",
        0xe6fb6c9 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiMapScaleDefineData {
        pub map_wide_min_pos: f32,
        pub map_height_min_pos: f32,
        pub map_scale: f32,
        pub is_map_floor: bool,
        pub mask_set_list: Vec<MaskSetting>,
    }
}
