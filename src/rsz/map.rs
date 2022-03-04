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

rsz_struct! {
    #[rsz("snow.gui.GuiQuestStart",
        0xe295d8a4 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiQuestStart {
        pub enabled: bool,
        pub start_wait_frame_max: f32,
    }
}

rsz_struct! {
    #[rsz("snow.gui.GuiQuestEnd",
        0x5246719b = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiQuestEnd {
        pub enabled: bool,
        pub pnl_quest_end: GuiControl,
        pub pnl_quest_clear: GuiPanel,
    }
}

rsz_struct! {
    #[rsz("snow.gui.QuestUIManager",
        0x66c59972 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestUIManage {
        pub enabled: bool,
        pub entry_quest_map_ui_prefab_list: Vec<Prefab>,
        pub entry_quest_detail_map_ui_prefab_list: Vec<Prefab>,
        pub entry_quest_detail_map_for_start_menu_ui_prefab_list: Vec<Prefab>,
        pub entry_quest_map_scale_define_data_list: Vec<Option<ExternUser<GuiMapScaleDefineData>>>,
        pub entry_quest_detail_map_icon_list_ec_item_data: ExternUser<()>, // snow.gui.userdata.GuiMapDetailIconListEcItemData
        pub entry_quest_detail_map_icon_list_ec_other_data: ExternUser<()>, // snow.gui.userdata.GuiMapDetailIconListEcOtherData
        pub entry_quest_detail_map_icon_list_g_pop_data: ExternUser<()>, // snow.gui.userdata.GuiMapDetailIconListG_PopData
        pub entry_quest_detail_map_icon_list_field_gimmick_data: ExternUser<()>, // snow.gui.userdata.GuiMapDetailIconListFieldGimmickData
        pub map_sign_timer_max: f32,
        pub map_first_mask_timer_max: f32,
        pub is_enable_reward_skill_flag: bool,
    }
}

rsz_struct! {
    #[rsz("snow.gui.GuiHoldBoxChange",
        0x1973d0e3 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiHoldBoxChange {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.TrialNaviSignToTargetMonster",
        0x52d82992 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct TrialNaviSignToTargetMonster {
        pub enabled: bool,
        pub trial_navi_sign_data: ExternUser<()>, // snow.TrialNaviSignData
    }
}

rsz_struct! {
    #[rsz("via.navigation.ObstacleFilterInfo",
        0x727d8279 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ObstacleFilterInfo {
        pub v0: u8,
        pub v1: String,
        pub v2: u32,
    }
}

rsz_struct! {
    #[rsz("via.navigation.ObstacleFilterSet",
        0x3fc440e4 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ObstacleFilterSet {
        pub v0: String,
        pub filters: Vec<ObstacleFilterInfo>
    }
}

rsz_struct! {
    #[rsz("via.navigation.NavigationSurface",
        0x2edbaa75 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NavigationSurface {
        // Navigation
        pub v0: String,
        pub v1: ViaVec4,
        pub v2a: u64,
        pub v2b: u64,
        pub v3: u8,
        pub v4: u8,
        pub v5: u8,
        pub v6: u32,
        pub v7: u32,
        pub v8: u8,
        pub v9: u8,
        pub v10: u8,
        pub v11: u8,
        pub v12: Vec<()>, //? 0xd0 getFilters via.navigation.FilterInfo
        pub v13: Vec<()>,//? 0xe0 getFilterGroups via.navigation.FilterSet
        pub v14: u32,
        pub v15: u32,
        pub v16: u32,
        pub v17: u8,
        pub v18: u8,
        pub v19: u8,
        pub v20: u32,
        pub v21: u32,
        pub v22: u32,
        pub v23: u32,
        pub v24: u32,
        pub v25: u8,
        pub v26: u8,
        pub v27: u8,
        pub v28: u32,
        pub v29: u8,
        pub v30: u32,
        pub v31: u8,
        pub v32: u8,
        pub v33: u32,
        pub v34: u32,
        pub v35: u32,
        pub v36: Vec<ObstacleFilterSet>, // 0xf8
        pub v37: u32,

        // NavigationSurface
        pub v38: u32,
        pub v39: u32,
        pub v40: u32,
        pub v41: u32,
        pub v42: u8,
        pub v43: u32,
        pub v44: u32,
        pub v45: u32,
        pub v46: u32,
        pub v47: u32,
        pub v48: u32,
        pub v49: u32,
        pub v50: u32,
        pub v51: u32,
        pub v52: u8,
        pub v53: u32,
        pub v54: u8,
        pub v55: u8,
        pub v56: u8,
        pub v57: u8,
        pub v58: u32,
        pub v59: u8,
        pub v60: u8,
        pub v61: u32,
        pub v62: u8,
        pub v63: u8,
        pub v64: u32,
        pub v65: u8,
        pub v66: u32,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.ObjectEffectManager",
        0xcb881936 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ObjectEffectManager {
        pub enabled: bool,
        pub data_container: Prefab,
        pub external_data_containers: Vec<()>, // via.effect.script.ObjectEffectManager.ExternalDataContainer
        #[serde(skip)]
        aligner: Aligner<8>,
        pub target_game_object: Guid,
        pub external_propertys: Vec<()>, // via.effect.script.ObjectEffectManager.ExternalProperty
        pub disable_vfx_trigger_track: bool,
        pub disable_vfx_range_track: bool,
        pub disable_motion_jack_check: bool,
        pub vfx_trigger_track_use_highest_weight: bool,
        pub vfx_range_track_use_highest_weight: bool,
        pub static_data_container: bool,
        pub disable_by_empty_data_container: bool,
    }
}
