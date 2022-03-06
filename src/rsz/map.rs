use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.stage.props.PopMaterialController",
        0x2748d05a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PopMaterialController {
        pub enabled: bool,
        pub ctrl_setting_data: ExternUser<()>, // snow.stage.props.PopMaterialControlSettingData
        pub blink_cycle_span: f32,
    }
}

rsz_struct! {
    #[rsz("snow.access.PlayerInfluencePopMarker",
        0x2b3d2c6c = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PlayerInfluencePopMarker {
        pub base: ObjectPopMarker,
        pub test_bell_trigger: u32,
        pub creeping_point_adjust_transform: ViaQuaternion,
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType

    }
}

rsz_struct! {
    #[rsz("snow.access.ItemPopBehavior",
        0xdae0b08f = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct ItemPopBehavior {
        pub enabled: bool,
        pub pop_id: i32, // snow.stage.StageDef.SaisyuPopId
        pub pop_icon: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconPatternNo
        pub pop_icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub pop_category: i32, // snow.access.ItemPopMarker.ItemPopCategory
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType
        pub action_target_point_offset: ViaVec3,
        pub one_time_only_flag: bool,
    }
}

rsz_struct! {
    #[rsz("snow.access.ItemPopVisualController",
        0x89989dcf = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemPopVisualController {
        pub enabled: bool,
        pub parts_no: i32,
        pub dissolve_cluster_name: String,
        pub dissolve_cluster_name_sub: String,
        pub dissolve_time: f32,
        pub dissolve_timer: f32,

    }
}

rsz_struct! {
    #[rsz("snow.stage.StageRestrictObserver",
        0xe8f69abc = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct StageRestrictObserver {
        pub enabled: bool,
        pub restrict_type: i32, // snow.RistrictTargetType
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.RelicNoteUnlock",
        0xf2852b01 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RelicNoteUnlock {
        pub enabled: bool,
        pub note_map_no: i32, // snow.QuestMapManager.MapNoType
        pub relic_id: i32, // snow.stage.StageDef.RelicId
    }
}

rsz_struct! {
    #[rsz("snow.gui.GuiCommonNpcHeadMessage",
        0xae15ae5b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiCommonNpcHeadMessage {
        pub enabled: bool,
        pub pos_data: ExternUser<()>, // snow.gui.userdata.GuiNpcHeadMessagePosData
    }
}

// snow.access.ObjectPopMarker.AccessableDigree.DirectionType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum DirectionType {
        Vertical = 0,
        Horizontal = 1,
        Undefined = -1,
    }
}

rsz_struct! {
    #[rsz("snow.access.ObjectPopMarker.AccessableDigree",
        0xa042b98d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct AccessableDigree {
        pub enable: bool,
        pub direction: DirectionType,
        pub start_degree: f32,
        pub end_degree: f32,
    }
}

// snow.access.ObjectPopSensor.DetectedInfo.RegisterRequirementType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum RegisterRequirementType {
        MarkerCoreSensorOutline = 0,
        SensorCoreMarkerOutline = 1,
    }
}

// snow.access.ObjectPopMarker
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ObjectPopMarker {
        pub enabled: bool,
        pub category: i32, // snow.access.ObjectPopMarker.MarkerCategory
        pub id: u32,
        pub control_id: u32,
        pub is_detectable: bool,
        pub is_accessible: bool,
        pub accessible_degree_list: Vec<AccessableDigree>,
        pub register_requirement: RegisterRequirementType,
        pub permit_exceptional_access: bool,
        pub permit_exceptional_access2: bool,
        pub action_pos: ViaVec3,
        pub action_dir: ViaVec3,

    }
}

// snow.access.NpcFacilityPopMarker.FlowType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum FlowType {
        Undefined = 0,
        NpcOnly = 1,
        NpcAndFacility = 2,
        FacilityOnly = 3,
        NpcAndFacilitySkipStart = 4,
        NpcAndFacilitySkipBoth = 5,
    }
}

rsz_struct! {
    #[rsz("snow.access.NpcFacilityPopMarker",
        0x3ad748a3 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NpcFacilityPopMarker {
        pub base: ObjectPopMarker,
        pub access_flow: FlowType,
        pub focus_camera_flag: bool,
        pub camera_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.hit.RSCAPIWrapper",
        0x0c8d525e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RSCAPIWrapper {
        pub enabled: bool,
    }
}

// snow.stage.StageDef.CampType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum CampType {
        BaseCamp = 0,
        SubCamp1 = 1,
        SubCamp2 = 2,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.TentBehavior",
        0xee52e52d = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct TentBehavior {
        pub enabled: bool,
        pub camp_type: CampType,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.CampFindCheck",
        0x44a3363f = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct CampFindCheck {
        pub enabled: bool,
        pub camp_type: CampType,
        pub check_hight: f32,
        pub check_radius: f32,
    }
}

rsz_struct! {
    #[rsz("snow.access.SupplyBoxBehavior",
        0x83471751 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SupplyBoxBehavior {
        pub enabled: bool,
    }
}

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

rsz_struct! {
    #[rsz("snow.data.ItemPopLotTableUserData.Param",
        0x92603c1b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemPopLotTableUserDataParam {
        pub pop_id: i32, // snow.stage.StageDef.SaisyuPopId
        pub field_type: i32, // snow.QuestMapManager.MapNoType // this can be -1 (None)
        pub lot_count: u32,
        pub lower_id: Vec<ItemId>,
        pub lower_num: Vec<u32>,
        pub lower_probability: Vec<u32>,
        pub upper_id: Vec<ItemId>,
        pub upper_num: Vec<u32>,
        pub upper_probability: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemPopLotTableUserData",
        0x4aa4200a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemPopLotTableUserData {
        pub param: Vec<ItemPopLotTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.WireLongJumpUnlock",
        0x618631a5 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WireLongJumpUnlock {
        pub enabled: bool,
        pub wire_long_jump_id: i32,// snow.stage.StageDef.WireLongJumpId
        pub unlock_cost: i32,
        pub unlock_time_tag: f32,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EnvironmentEffectManager",
        0x96116d19 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentEffectManager {
        pub enabled: bool,
        pub is_enable_auto_culling: bool,
        pub is_auto_play: bool,
        pub is_culling_not_affect_update: bool,
        pub external_propertys: Vec<()>, // via.effect.script.EnvironmentEffectManager.ExternalProperty
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EPVDataElement.GroupInfo",
        0xb09b23c6 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVDataElementGroupInfo {
        pub group_type_lv0: i32, // via.effect.GroupType
        pub group_type_lv1: i32, // via.effect.GroupType
        pub user_variable_lv0: String,
        pub user_variable_lv1: String,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EffectCustomExternParameter",
        0xa5fc82a9 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectCustomExternParameter {
        pub name: String,
        pub v1: u32,
        pub v2: u32,
        pub v3: u32,
        pub v4: u32,
        pub resource_index: u32,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.GroupNameParameter",
        0x13c0298b = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GroupNameParameter {
        pub enable: bool,
        pub group_name_lv0: String,
        pub group_name_lv1: String,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EffectManager.LODInfo",
        0x47bd3c89 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectManagerLODInfo {
        pub level: i32, // via.effect.script.EffectManager.LODLevel
        pub effect_light_off: bool,
        pub effect_decal_off: bool,
        pub effect_radial_blur_off: bool,
        pub effect_color_correct_off: bool,
        pub effect_camera_shake_off: bool,
        pub effect_lensflare_off: bool,
        pub effect_cloth_chain_mesh_off: bool,
        pub spawn_volume_ratio: f32,
        pub action_cut_off: bool,
        pub collision_cut_off: bool,
        pub down_level_lighting: bool,
        pub soft_particle_cut_off: bool,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EPVStandardData.Element",
        0x46abcc6 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVStandardDataElement {
        // via.effect.ProviderData
        pub resources: Vec<String>,
        // via.effect.script.EPVDataElement
        pub trigger_id: u32,
        pub stop_trigger_id: u32,
        #[serde(skip)]
        aligner: Aligner<8>,
        pub guid: Guid,
        pub effect_cache: bool,
        pub group_info_list: Vec<EPVDataElementGroupInfo>,
        // via.effect.script.EPVStandardData.Element
        pub id: u32,
        pub relation_type: i32, // via.effect.script.EPVDataBase.EffectRelationType
        pub initialization_parent_scale: bool,
        pub parent_degree: i32,
        pub relation_child_id: i32,
        pub joint_name: String,
        pub unparent_frame: f32,
        pub offset: ViaVec3,
        pub rotation_order: i32, // via.math.RotationOrder
        pub base_rotation: i32, // via.effect.script.EPVDataBase.RotateBase
        pub relation_rotation_axis: u32,
        pub rotation: ViaVec3,
        pub scale: ViaVec3,
        pub life_frame: f32,
        pub end_type: i32, // via.effect.script.EffectManager.EffectEndType
        pub action_on_provider_destroy: i32, // via.effect.script.EffectCommonDefine.EffectActionOnProviderDestroy
        pub action_on_provider_disappear: i32, // via.effect.script.EffectCommonDefine.EffectActionOnParentDisappea
        pub is_camera_direction: bool,
        pub is_landing: bool,
        pub is_use_terrain_normal: bool,
        pub search_terrain_distance: f32,
        pub cam_node_billbard_sec: f32,
        pub separate_parent_scale: bool,
        pub delay_frame: f32,
        pub loop_frame: u32,
        pub loop_dispersion_frame: u32,
        pub play_speed: f32,
        pub relation_motion_speed: bool,
        pub is_relation_mirror: bool,
        pub color_enable: bool,
        pub color: u32, // via.Color
        pub mask_enable: bool,
        pub mask_bit: u32,
        pub action_on_culling: i32, // via.effect.script.EffectCommonDefine.EffectActionOnCulling
        pub extern_parameters: Vec<EffectCustomExternParameter>,
        pub group_name_parameters: Vec<GroupNameParameter>,
        pub custom_lod: bool,
        pub lod_levels: Vec<EffectManagerLODInfo>,
        pub user_bit: u32,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EPVStandardData",
        0x15efc35c = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVStandardData {
        pub enabled: bool,
        pub data: Vec<EPVStandardDataElement>,
        pub version: u32,
    }
}

rsz_struct! {
    #[rsz("snow.EffectPlayerFadeByDepth.EffectPlayerFadeByDepthParam",
        0xb8e54062 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectPlayerFadeByDepthParam {
        pub enable: bool,
        pub near_start: f32,
        pub near_end: f32,
        pub far_start: f32,
        pub far_end: f32,
    }
}

rsz_struct! {
    #[rsz("snow.EffectPlayerFadeByDepthData",
        0xefafe567 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectPlayerFadeByDepthData {
        pub enabled: bool,
        pub param: EffectPlayerFadeByDepthParam
    }
}
rsz_struct! {
    #[rsz("snow.EnvironmentEffectManagerHelper",
        0xba1fd0c9 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentEffectManagerHelper {
        pub enabled: bool
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EPVStandard",
        0xc66ae2d3 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVStandard {
        pub enabled: bool,
        pub follow_game_object: Zero // The type is GameObject, but always null?
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.UniqueBehavior_pop010",
        0x2AC55689 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct UniqueBehaviorPop010 {
        pub enabled: bool,
        pub appearable_quest_no_list: Vec<i32>,
        pub sync_id: i32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.TentVisualController",
        0xa2a80c76 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct TentVisualController {
        pub enabled: bool,
        pub camp_type: CampType,
        pub model_type: i32, // snow.stage.props.TentVisualController.ModelType
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType
    }
}

rsz_struct! {
    #[rsz("snow.access.GimmickPopMarker",
        0x23a2972 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GimmickPopMarker {
        pub base: ObjectPopMarker,
    }
}
