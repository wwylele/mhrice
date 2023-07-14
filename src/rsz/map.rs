use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_sumtype;
use crate::rsz_sumuser;
use nalgebra_glm::*;
use serde::*;

// snow.stage.StageDef.MapFloorType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum MapFloorType {
        MapOutdoor = 0,
        MapIndoor = 1,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.PopMaterialController",
        0x5a71bf68 = 10_00_02
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
        0xf4a0af8f = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct PlayerInfluencePopMarker {
        pub base: ObjectPopMarker,
        pub test_bell_trigger: u32,
        pub creeping_point_adjust_transform: Quat,
        pub map_floor_type: MapFloorType,
    }
}

rsz_struct! {
    #[rsz("snow.access.ItemPopBehavior",
        0x0c330360 = 10_00_02,
        0xB05CAFAA = 11_00_01,
        0x25726033 = 12_00_00,
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct ItemPopBehavior {
        pub enabled: bool,
        pub pop_id: i32, // snow.stage.StageDef.SaisyuPopId
        pub pop_icon: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconPatternNo
        pub pop_icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub pop_category: i32, // snow.access.ItemPopMarker.ItemPopCategory
        pub map_floor_type: MapFloorType,
        pub action_target_point_offset: Vec3,
        pub one_time_only_flag: bool,
    }
}

rsz_struct! {
    #[rsz("snow.access.ItemPopVisualController",
        0xf4a1f2fd = 10_00_02
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
        0x95cff58e = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageRestrictObserver {
        pub enabled: bool,
        pub restrict_type: i32, // snow.RistrictTargetType
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.RelicNoteUnlock",
        0x2d783185 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct RelicNoteUnlock {
        pub enabled: bool,
        pub note_map_no: i32, // snow.QuestMapManager.MapNoType
        pub relic_id: i32, // snow.stage.StageDef.RelicId
    }
}

rsz_struct! {
    #[rsz("snow.gui.GuiCommonNpcHeadMessage",
        0xb596a217 = 10_00_02
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
    #[derive(Debug, Serialize, Clone)]
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
    #[derive(Debug, Serialize, Clone)]
    pub struct ObjectPopMarker {
        pub enabled: bool,
        pub category: i32, // snow.access.ObjectPopMarker.MarkerCategory
        pub id: u32,
        pub control_id: u32,
        pub is_detectable: bool,
        pub is_accessible: bool,
        pub is_accessible_ot_dog_pouch: bool,
        pub accessible_degree_list: Vec<AccessableDigree>,
        pub register_requirement: RegisterRequirementType,
        pub permit_exceptional_access: bool,
        pub permit_exceptional_access2: bool,
        pub action_pos: Vec3,
        pub action_dir: Vec3,

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
        0x4129B38E = 10_00_02
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
        0x71b43d6c = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct RSCAPIWrapper {
        pub enabled: bool,
    }
}

// snow.stage.StageDef.CampType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum CampType {
        BaseCamp = 0,
        SubCamp1 = 1,
        SubCamp2 = 2,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.TentBehavior",
        0x936b8a1f = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct TentBehavior {
        pub enabled: bool,
        pub camp_type: CampType,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.CampFindCheck",
        0xF9AAD01D = 15_00_00,
        0x8F5C6E20 = 14_00_00,
        0x5588A2F2 = 10_00_02,
        0xA0F8501B = 11_00_01,
        0xE158D6D7 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct CampFindCheck {
        pub enabled: bool,
        pub camp_type: CampType,
        pub check_hight: f32,
        pub check_radius: f32,
        pub sub_camp_find_chat_type: i32, // snow.gui.NpcGuideChatManager.NpcGuideChatType
    }
}

rsz_struct! {
    #[rsz("snow.access.SupplyBoxBehavior",
        0xfe7e7863 = 10_00_02
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
        pub pos: Vec3,
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
    #[rsz("snow.gui.userdata.GuiMap07DefineData.MapHyakuryuLayoutSetting",
        0x8e07822f = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct MapHyakuryuLayoutSetting {
        pub add_hyakuryu_texture_pos: Vec3,
        pub add_hyakuryu_mask_pos: Vec3,
        pub hyakuryu_mask_size_w: f32,
        pub hyakuryu_mask_size_h: f32,
    }
}

rsz_struct! {
    #[rsz("snow.gui.userdata.GuiMap07DefineData",
        0x162be1f2 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiMap07DefineData {
        #[serde(flatten)]
        pub base: Flatten<GuiMapScaleDefineData>,
        pub map_layout_setting: Vec<MapHyakuryuLayoutSetting>,
        pub start_menu_map_layout_setting: Vec<MapHyakuryuLayoutSetting>,
        pub detail_map_layout_setting: Vec<MapHyakuryuLayoutSetting>,

    }
}

rsz_struct! {
    #[rsz("snow.gui.GuiQuestStart",
        0x0EFDD92B = 14_00_00,
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
        0xF9C7D3BA = 14_00_00,
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
        0x2F9C4016 = 14_00_00,
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
        0x3D6F990A = 14_00_00,
        0x1973d0e3 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiHoldBoxChange {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.TrialNaviSignToTargetMonster",
        0xAD983E8A = 14_00_00,
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
        0x52ACBF9A = 13_00_00,
        0xCC942F81 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct ObjectEffectManager {
        pub enabled: bool,
        pub disable_request_effect: bool,
        pub data_container: Prefab,
        pub external_data_containers: Vec<()>, // via.effect.script.ObjectEffectManager.ExternalDataContainer
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
        0xEF9B637A = 15_00_00,
        0xFD1D44F1 = 14_00_00,
        0x47242315 = 13_00_00,
        0xa1800433 = 10_00_02,
        0xF99CD6BA = 11_00_01,
        0xF070EF26 = 12_00_00,
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
        pub master_id: Vec<ItemId>,
        pub master_num: Vec<u32>,
        pub master_probability: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemPopLotTableUserData",
        path = "data/Define/Stage/ItemPop/ItemPopLotTableData.user",
        0x4aa4200a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemPopLotTableUserData {
        pub param: Vec<ItemPopLotTableUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.WireLongJumpUnlock",
        0x20e1f770 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct WireLongJumpUnlock {
        pub enabled: bool,
        pub wire_long_jump_id: i32,// snow.stage.StageDef.WireLongJumpId
        pub unlock_cost: i32,
        pub unlock_time_tag: f32,
        pub fish_check_dist: f32,
        pub fish_check_under_dist: f32,
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EnvironmentEffectManager",
        0xb75b4280 = 10_00_02
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
        0x92AB9523 = 13_00_00,
        0x6C9FC765 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVStandardDataElement {
        // via.effect.ProviderData
        pub resources: Vec<String>,
        // via.effect.script.EPVDataElement
        pub trigger_id: u32,
        pub stop_trigger_id: u32,
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
        pub offset: Vec3,
        pub rotation_order: i32, // via.math.RotationOrder
        pub base_rotation: i32, // via.effect.script.EPVDataBase.RotateBase
        pub relation_rotation_axis: u32,
        pub rotation: Vec3,
        pub scale: Vec3,
        pub life_frame: f32,
        pub end_type: i32, // via.effect.script.EffectManager.EffectEndType
        pub action_on_provider_destroy: i32, // via.effect.script.EffectCommonDefine.EffectActionOnProviderDestroy
        pub action_on_provider_disappear: i32, // via.effect.script.EffectCommonDefine.EffectActionOnParentDisappea
        pub is_camera_direction: bool,
        pub is_landing: bool,
        pub is_use_terrain_normal: bool,
        pub search_terrain_distance: f32,
        pub ignore_landing_position: bool,
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
        0x28ca23b3 = 10_00_02
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
        0x92968a55 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectPlayerFadeByDepthData {
        pub enabled: bool,
        pub param: EffectPlayerFadeByDepthParam
    }
}
rsz_struct! {
    #[rsz("snow.EnvironmentEffectManagerHelper",
        0xc726bffb = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentEffectManagerHelper {
        pub enabled: bool
    }
}

rsz_struct! {
    #[rsz("via.effect.script.EPVStandard",
        0xbb538de1 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EPVStandard {
        pub enabled: bool,
        pub follow_game_object: Zero // The type is GameObject, but always null?
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.UniqueBehavior_pop010",
        0x57fc39bb = 10_00_02
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
        0x5D60733F = 13_00_00,
        0xa33f23fe = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct TentVisualController {
        pub enabled: bool,
        pub camp_type: CampType,
        pub model_type: i32, // snow.stage.props.TentVisualController.ModelType
        pub map_floor_type: MapFloorType,
        pub include_objects: Vec<Guid>, // Is this correct??
    }
}

rsz_struct! {
    #[rsz("snow.access.GimmickPopMarker",
        0x3F8D6838 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct GimmickPopMarker {
        pub base: ObjectPopMarker,
    }
}

rsz_struct! {
    #[rsz("snow.access.StageFacilityPopMarker",
        0xd5857982 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct StageFacilityPopMarker {
        pub base: ObjectPopMarker,
        pub map_floor_type: MapFloorType,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.FishingPoint",
        0x813ab1b3 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct FishingPoint {
        pub enabled: bool,
        pub fishing_point_data: ExternUser<()>, // snow.stage.pop.FishingPointData
        pub camera_type: i32, // snow.camera.PlayerCamera.CameraDataType_Fishing
        pub fish_spawn_data: ExternUser<FishSpawnData>, // snow.stage.pop.userdata.FishSpawnData
        pub fish_territory_point: Vec3,
        pub fish_territory_radius: f32,
        pub fish_num_max: i32,
        pub fishing_point_id: i32, // snow.stage.StageDef.FishingPointId
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.FishingPointBuoy",
        0x437e0021 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct FishingPointBuoy {
        pub enabled: bool,
        pub bite_offset: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.userdata.FishSpawnData.FishSpawnRate",
        0x167f3f35 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct FishSpawnRate {
        pub fish_id: i32, // snow.stage.StageDef.FishId
        pub spawn_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.userdata.FishSpawnData.FishSpawnGroupInfo",
        0xc64892d1 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct FishSpawnGroupInfo {
        pub fish_spawn_rate_list: Vec<FishSpawnRate>
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.userdata.FishSpawnData",
        0x5fae0ab5 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct FishSpawnData {
        pub spawn_group_list_info_low: Vec<FishSpawnGroupInfo>,
        pub spawn_group_list_info_high: Vec<FishSpawnGroupInfo>,
        pub spawn_group_list_info_master: Vec<FishSpawnGroupInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageSceneLoader",
        0x776045C9 = 13_00_00,
        0x64ef47ac = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageSceneLoader {
        pub enabled: bool,
        pub stop_on_ready: Versioned<bool, 13_00_00>,
        pub target_scene_names: Vec<String>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageGridRegister",
        0x54957a07 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageGridRegister {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.m31IsletArrivalChecker",
        0x9ee9de11 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct M31IsletArrivalChecker {
        pub enabled: bool,
        pub check_pos: Vec3,
        pub enable_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageAppTagSetter",
        0xce1cbdb1 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageAppTagSetter {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.access.ItemPopIgnoreOtomoGathering",
        0xb88a28ac = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemPopIgnoreOtomoGathering {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageSceneStateController.TargetScene",
        0x261aa8f3 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct TargetScene {
        pub key: String,
        pub key_hash: u32,
        pub scene_name: String,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageSceneStateController",
        0x2e2825d8 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageSceneStateController {
        pub enabled: bool,
        pub data: ExternUser<()>, // snow.stage.StageSceneStateUserData
        pub targets: Vec<TargetScene>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectControllerBase.KeyHash",
        0xb7d4fea1 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct KeyHash {
        pub key: String,
        pub hash: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageDemoCameraSceneRequeter.RequestData",
        0xD381EC25 = 15_00_00,
        0xf64754ac = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageDemoCameraSceneRequeterRequestData {
        pub target_types: Vec<i32>, // snow.camera.DemoCamera.RequestType
        pub scene_key: KeyHash,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageDemoCameraSceneRequeter",
        0x8461F441 = 15_00_00,
        0x109e1f98 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageDemoCameraSceneRequeter {
        pub enabled: bool,
        pub request_data: Vec<StageDemoCameraSceneRequeterRequestData>,
        pub start_request: i32, // snow.stage.StageDemoCameraSceneRequeter.RequestType
        pub start_delay_frame: i32,
        pub end_request: i32, // snow.stage.StageDemoCameraSceneRequeter.RequestType
        pub end_delay_frame: i32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageAreaMoveSceneRequester.RequestData",
        0x69196f07 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageAreaMoveSceneRequesterRequestData {
        pub target_types: Vec<i32>, // snow.stage.StageManager.AreaMoveQuest
        pub scene_key: KeyHash,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageAreaMoveSceneRequester",
        0x7374e8b0 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageAreaMoveSceneRequester {
        pub enabled: bool,
        pub request_data: Vec<StageAreaMoveSceneRequesterRequestData>,
        pub delay_frame: i32,
    }
}

rsz_struct! {
    #[rsz("snow.camera.KillCameraConditionRegisterBlockNo",
        0xeb767f8a = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct KillCameraConditionRegisterBlockNo {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageManager.QuestAreaMoveRequest",
        0xf3eed444 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestAreaMoveRequest {
        pub area_move_type: i32, // snow.stage.StageManager.AreaMoveQuest
        pub player_area_move_type: i32, // snow.player.PlayerBase.AreaMoveType
        pub warp_type: i32, // snow.stage.StagePointManager.WarpType
        pub area_no_type: i32, // snow.stage.StageDef.AreaNoType
    }
}

rsz_struct! {
    #[rsz("snow.access.QuestAreaMovePopMarker.QuestPhaseCondition",
        0x1BE4C28F = 13_00_00,
        0xa5848ab1 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestPhaseCondition {
        pub quest_phase: Vec<i32>, // snow.quest.QuestPhase
    }
}

rsz_struct! {
    #[rsz("snow.access.QuestAreaMovePopMarker.CountCondition",
        0x1BE4C28F = 13_00_00,
        0x743729a7 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct CountCondition {
        pub count: i32,
    }
}

rsz_sumtype! {
    #[derive(Debug, Serialize)]
    pub enum Condition {
        QuestPhaseCondition(QuestPhaseCondition),
        CountCondition(CountCondition),
    }
}

rsz_struct! {
    #[rsz("snow.access.QuestAreaMovePopMarker.AreaMoveInfo",
        0x1BE4C28F = 13_00_00,
        0xc5f315ce = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct AreaMoveInfo {
        pub quest_area_move_request: QuestAreaMoveRequest,
        pub conditions: Vec<Condition>,
    }
}

rsz_struct! {
    #[rsz("snow.access.QuestAreaMovePopMarker",
        0x1BE4C28F = 13_00_00,
        0x554c80d7 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct QuestAreaMovePopMarker {
        pub base: ObjectPopMarker,
        pub area_move_infos: Vec<AreaMoveInfo>,
        pub reset_delay_time: Versioned<f32, 13_00_00>,
        pub mr_area_move_set_pos: Vec3,
        pub mr_area_move_set_angle: Quat,
        pub mr_area_move_offset_pos: Vec<Vec3>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectStateController.TargetObject",
        0x593EB8B0 = 14_00_00,
        0xD18507FA = 13_00_00,
        0x502b2c47 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageObjectStateControllerTargetObject {
        pub key: String,
        pub key_has: u32,
        pub game_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectStateController",
        0x919F4F5E = 13_00_00,
        0x7736aadc = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageObjectStateController {
        pub enabled: bool,
        pub data: ExternUser<()>, // snow.stage.StageObjectStateUserData
        pub is_apply_all_targets: Versioned<bool, 13_00_00>,
        pub targets: Vec<StageObjectStateControllerTargetObject>,
        pub target_object_names: Versioned<Vec<String>, 13_00_00>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.pop.OtomoReconSpot",
        0x7d930914 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct OtomoReconSpot {
        pub enabled: bool,
        pub spot_index: i32,
        pub unlock_time_lag: f32,
        pub find_distance: f32,
        pub spot_effet_key: Option<String>,
        pub unlock_key: Option<String>,
    }
}

// Untested
rsz_struct! {
    #[rsz("snow.stage.StageObjectMotionController.MotionTarget",
        0xD5218D29 = 13_00_00,
        0x21ab60fc = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageObjectStateControllerMotionTarget {
        pub key: String,
        pub key_hash: u32,
        pub game_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectMotionController",
        0x10d6929c = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageObjectMotionController {
        pub enabled: bool,
        pub data: ExternUser<()>, // snow.stage.StageObjectMotionUserData
        pub auto_play_keys: Vec<KeyHash>,
        pub targets: Vec<StageObjectStateControllerMotionTarget>,
    }
}

// Untested
rsz_struct! {
    #[rsz("snow.stage.StageObjectEffectController.EffectFollowTarget",
        0xAEB948D2 = 13_00_00,
        0x4b47ef79 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectFollowTarget {
        pub key: String,
        pub key_hash: u32,
        pub target_type: Versioned<i32, 13_00_00>, // snow.stage.StageObjectEffectController.TargetType
        pub game_object: Guid,
    }
}

// Untested
rsz_struct! {
    #[rsz("snow.stage.StageObjectEffectController.EffectKeyHash",
        0xD03A05BE = 13_00_00,
        0x4610af1e = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EffectKeyHash {
        pub key: String,
        pub hash: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectEffectController",
        0xD03A05BE = 13_00_00,
        0x0501d21d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct StageObjectEffectController {
        pub enabled: bool,
        pub effect_container_owner: i32, // snow.stage.StageObjectEffectController.EffectContainerOwner
        pub object_effect_manager_owner: Guid,
        pub data: Option<ExternUser<()>>, // snow.stage.StageObjectEffectUserData,
        pub follow_targets: Vec<EffectFollowTarget>,
        pub auto_request_keys: Vec<EffectKeyHash>,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseContainerApp",
        0x111dddbb = 10_00_02,
        0xCBA62068 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WwiseContainerApp {
        pub enabled: bool,

        pub v1: Vec<Option<String>>,
        pub v2: u16,
        pub v3: u8,

        pub is_master: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.MysteryItemPopIgnore",
        0x87A76C7B = 13_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryItemPopIgnore {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.camera.PhotoSubject")]
    #[derive(Debug, Serialize)]
    pub struct PhotoSubject {
        pub enabled: bool,
        pub set_type: i32, // snow.camera.PhotoSubject.PhotoSubjectType
        pub set_em_type: EmTypes,
        pub set_env_creature_type: i32, // snow.envCreature.EnvironmentCreatureType
        pub set_npc_id: i32, // snow.NpcDefine.NpcID
        pub set_joint_name: Option<String>,
        pub check_pos_ofs_y: f32,
        pub check_position_offset: Vec3,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureItem")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureItem {
        pub enabled: bool,
        pub env_creature_item_id: ItemId,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcPopBehavior")]
    #[derive(Debug, Serialize)]
    pub struct EcPopBehavior {
        pub enabled: bool,
        pub action_target_point_offset: Vec3,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureDrop")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureDrop {
        pub enabled: bool,
        pub drop_ec_item: Prefab,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.sequence.EcMotionSequenceCtrl")]
    #[derive(Debug, Serialize)]
    pub struct EcMotionSequenceCtrl {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureFindFlagSetter")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvironmentCreatureFindFlagSetter {
        pub trigger_type: i32, // snow.envCreature.EnvironmentCreatureFindFlagSetter.FindTriggerType
        pub find_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureBase.HyakuryuArea")]
    #[derive(Debug, Serialize, Clone)]
    pub struct HyakuryuArea {
        pub start_area_no: i32,
        pub end_area_no: i32,
    }
}

rsz_struct! {
    #[rsz("snow.RSCController")]
    #[derive(Debug, Serialize)]
    pub struct RSCController {
        pub enabled: bool,
        pub damage_resource_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.DamageReceiver")]
    #[derive(Debug, Serialize)]
    pub struct DamageReceiver {
        pub enabled: bool,
        pub enable_cycle_hit: bool,
        pub history_size: i32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureBase")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvironmentCreatureBase {
        // snow.CharacterBase
        pub enabled: bool,
        pub unique_id: u32,
        pub type_: i32, // snow.envCreature.EnvironmentCreatureType
        pub ec_rim_color_data: Option<ExternUser<()>>, // snow.envCreature.EcRimColorData
        pub sub_type: u32,
        pub move_area_size: Vec3,
        pub move_area_angle: Vec3,
        pub move_area_offset: Vec3,
        pub culling_distance: f32,
        pub repop_wait_time: f32,
        pub move_radius: f32,
        pub map_floor_type: MapFloorType,
        pub hyakuryu_area_no_list: Vec<HyakuryuArea>,
        pub find_flag_setter: EnvironmentCreatureFindFlagSetter,
        pub is_ignore_valid_check: bool,
        pub auto_register_map_icon: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec024.MaterialData")]
    #[derive(Debug, Serialize, Clone)]
    pub struct Ec024MaterialData {
        pub min_value: f32,
        pub max_value: f32,
        pub dist: f32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcMaterialCurveController")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EcMaterialCurveController {
        pub enabled: bool,
        pub material_curve_data: ExternUser<()>, // snow.envCreature.EcMaterialCurveList
        pub inter_time: f32,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEc025")]
    #[derive(Debug, Serialize, Clone)]
    pub struct WwiseEc025 {
        pub enabled: bool,
        pub enemy_hit_trigger: u32,
        pub terrain_hit_trigger: u32,
        pub shot_sign_set_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec009PartsCtrl")]
    #[derive(Debug, Serialize)]
    pub struct Ec009PartsCtrl {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureLvBuff")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureLvBuff {
        pub enabled: bool,
        pub lv_buff_data: ExternUser<()>, // snow.envCreature.LvBuffUserData
        pub auto_dist: f32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcMaterialControllerBase.EcMaterialData")]
    #[derive(Debug, Serialize)]
    pub struct EcMaterialData {
        pub min_value: f32,
        pub max_value: f32,
        pub property_name: String,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec009MaterialContoller.Ec009MaterialData")]
    #[derive(Debug, Serialize)]
    pub struct Ec009MaterialData {
        pub intensity_dist: f32,
        pub rate: f32,
        pub rate_length: f32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec009MaterialContoller")]
    #[derive(Debug, Serialize)]
    pub struct Ec009MaterialContoller {
        pub enabled: bool,
        pub material_data_list: Vec<EcMaterialData>,
        pub min_dist: f32,
        pub max_dist: f32,
        pub min_max_dist: f32,
        pub ec009_material_data_list: Vec<Ec009MaterialData>,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcLinearMaterialController.EcLinearMaterialData")]
    #[derive(Debug, Serialize)]
    pub struct EcLinearMaterialData {
        pub target_intensity: f32,
        pub timer: f32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcLinearMaterialController")]
    #[derive(Debug, Serialize)]
    pub struct EcLinearMaterialController {
        pub enabled: bool,
        pub material_data_list: Vec<EcMaterialData>,
        pub ec_linear_material_data_list: Vec<EcLinearMaterialData>
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureBuff")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureBuff {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec006PartsCtrl")]
    #[derive(Debug, Serialize)]
    pub struct Ec006PartsCtrl {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec021PartsCtrl")]
    #[derive(Debug, Serialize)]
    pub struct Ec021PartsCtrl {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec001PartsCtrl")]
    #[derive(Debug, Serialize)]
    pub struct Ec001PartsCtrl {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureWireBuff")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureWireBuff {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.KakeriWingPartsFlap")]
    #[derive(Debug, Serialize)]
    pub struct KakeriWingPartsFlap {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureTrap")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureTrap {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec037PartsCtrl")]
    #[derive(Debug, Serialize)]
    pub struct Ec037PartsCtrl {
        pub enabled: bool,
    }
}

// Not a real type. Extracted from snow.envCreature.EcPhotoBase
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize, Clone)]
    pub struct EcPhotoBaseExtra {
        pub player_search_dist: f32,
        pub player_search_angle: f32,
        pub look_at_offset_y: f32,
        pub reaction_frame_data: ExternUser<()>, // snow.envCreature.EcPhotoBase.EcPhotoReactionFrameSettingData
    }
}

// snow.stage.StageTimeChange.StageTime
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct StageTime {
        pub hour: u32,
        pub minute: u32,
        pub seconds: u32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreaturePhoto")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreaturePhoto {
        pub enabled: bool,
        pub start_time: StageTime,
        pub end_time: StageTime,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcMovePathList")]
    #[derive(Debug, Serialize)]
    pub struct EcMovePathList {
        pub enabled: bool,
        pub move_data: ExternUser<()>, // snow.envCreature.EcMovePathData
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureLongWire")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureLongWire {
        pub enabled: bool,
        pub view_wire_long_jump_path: bool,
        pub route_viewer_length: u32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec019Ref")]
    #[derive(Debug, Serialize)]
    pub struct Ec019Ref {
        pub enabled: bool,
        pub ref_root: Guid,
        pub ref_top: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.access.JumpTypeBehavior")]
    #[derive(Debug, Serialize)]
    pub struct JumpTypeBehavior {
        pub enabled: bool,
        pub jump_type: i32, // snow.access.JumpTypeBehavior.WireLongJumpType
        pub disable_wall_check: bool,
        pub wall_through_frame: f32,
        pub disable_pl_input: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec019Trajectory")]
    #[derive(Debug, Serialize)]
    pub struct Ec019Trajectory {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.envCreature.Ec019TrajectorySetting
        pub player_common_data: ExternUser<()>, // snow.player.PlayerUserDataCommon
        pub check_terrain_interval: i32,
        pub check_terrain_time_max: i32,
        pub ec019: Guid,
        pub top: Guid,
        pub collision_filter: String, // via.physics.CollisionFilterResourceHolder
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectZoneController.FollowTarget")]
    #[derive(Debug, Serialize)]
    pub struct FollowTarget {
        pub key: String,
        pub key_hash: u32,
        pub game_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectZoneController")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectZoneController {
        pub enabled: bool,
        pub check_zako_enemy_count_in_frame: i32,
        pub data: ExternUser<()>, // snow.stage.StageObjectZoneUserData
        pub follow_targets: Vec<FollowTarget>,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec055Manager")]
    #[derive(Debug, Serialize)]
    pub struct Ec055Manager {
        pub enabled: bool,
        pub table: ExternUser<()>, // snow.envCreature.Ec055ElementTable
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec055Group")]
    #[derive(Debug, Serialize)]
    pub struct Ec055Group {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.envCreature.Ec055GroupSetting
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureActionController`1<snow.envCreature.Ec054.Action>")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvironmentCreatureActionControllerEc054Action {
        pub action_data: ExternUser<()>, // snow.envCreature.EnvironmentCreatureActionUserData`1<snow.envCreature.Ec054.Action>
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureActionController`1<snow.envCreature.Ec055.Action>")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvironmentCreatureActionControllerEc055Action {
        pub action_data: ExternUser<()>, // snow.envCreature.EnvironmentCreatureActionUserData`1<snow.envCreature.Ec055.Action>
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec055.JointOffset")]
    #[derive(Debug, Serialize, Clone)]
    pub struct Ec055JointOffset {
        pub position: Vec3,
        pub rotation: Quat,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec055CollisionInfoRegister")]
    #[derive(Debug, Serialize)]
    pub struct Ec055CollisionInfoRegister {
        pub enabled: bool,
        pub boundary_coord: Mat4x4,
        pub boundary_extend: Vec3,
        pub collision_filter: String, // via.physics.CollisionFilterResourceHolder
        pub required_collision_material_attribute_names: Vec<String>,
        pub exclude_collosion_material_attribute_names: Vec<String>,
        pub check_ray_length: f32,
        pub auto_register: bool,
    }
}

// Untested
rsz_struct! {
    #[rsz("snow.envCreature.EcMaterialCurve")]
    #[derive(Debug, Serialize)]
    pub struct EcMaterialCurve {
        pub curve: AnimationCurve,
        pub curve_3d: AnimationCurve3d,
        pub curve_w: AnimationCurve,
        pub material_type: i32, // snow.envCreature.EcMaterialCurve.MaterialParamType
        pub float_flag: bool,
        pub float4_flag: bool,
        pub material_name: Option<String>,
        pub property_name: Option<String>,
        pub start_up: bool,
        pub loop_on: bool,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcMaterialCurveList")]
    #[derive(Debug, Serialize)]
    pub struct EcMaterialCurveList {
        pub curve_list: Vec<EcMaterialCurve>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectMaterialController")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectMaterialController {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.stage.StageObjectMaterialUserData
        pub targets: Vec<Guid>,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureRemoteSync")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureRemoteSync {
        pub enabled: bool,
        pub own_sync_targets: u32, // snow.stage.StageObjectRemoteSync.OwnSyncTarget
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageGuideMessageRequester.GuideMessageInfoBase")]
    #[derive(Debug, Serialize)]
    pub struct GuideMessageInfoBase {
        pub has_condition: bool,
        pub condition: Option<ExternUser<()>>, // snow.stage.StageGuideMessgaeConditionData
        pub condition_type: i32, // snow.stage.StageGuideMessageRequester.GuideMessageInfoBase.ConditionType
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageGuideMessageRequester.NpcGuideChatInfo")]
    #[derive(Debug, Serialize)]
    pub struct NpcGuideChatInfo {
        #[serde(flatten)]
        pub base: Flatten<GuideMessageInfoBase>,
        pub request_npc_guide_type: i32, // snow.gui.NpcGuideChatManager.NpcGuideChatType
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageGuideMessageRequester.HunterNoteInfo")]
    #[derive(Debug, Serialize)]
    pub struct HunterNoteInfo {
        #[serde(flatten)]
        pub base: Flatten<GuideMessageInfoBase>,
        pub note_id: i32, // snow.data.HunterNoteSystem.NoteID
    }
}

rsz_sumtype! {
    #[derive(Debug, Serialize)]
    pub enum GuideMessageInfo {
        NpcGuideChatInfo(NpcGuideChatInfo),
        HunterNoteInfo(HunterNoteInfo),
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageGuideMessageRequester")]
    #[derive(Debug, Serialize)]
    pub struct StageGuideMessageRequester {
        pub enabled: bool,
        pub guide_message_infos: Vec<GuideMessageInfo>,
        pub map_layer: i32, // snow.stage.StageGuideMessageRequester.MapLayerType
        pub map_zone: i32, // snow.stage.StageGuideMessageRequester.MapZoneType
        pub enable_only_draw: bool,
        pub request_condition: Option<ExternUser<()>>, // snow.stage.StageGuideMessgaeConditionData
        pub look_at_target: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.camera.TargetCamera_Target")]
    #[derive(Debug, Serialize)]
    pub struct TargetCameraTarget {
        pub enabled: bool,
        pub lock_on_param_config: bool,
        pub lock_on_far_limit: f32,
        pub lock_on_far_limit_y: f32,
        pub yield_target: bool,
        pub yield_target_first: bool,
        pub limited_zone: bool,
        pub zone_ray_length: f32,
        pub use_fixed_zone_block_no: bool,
        pub fixed_zone_block_no: i32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureSwitchMarionetteModeType")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureSwitchMarionetteModeType {
        pub enabled: bool,
        pub marionette_mode_type: u8, // snow.player.PlayerDefine.MarionetteModeType
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEc056")]
    #[derive(Debug, Serialize)]
    pub struct WwiseEc056 {
        pub enabled: bool,
        pub data: ExternUser<()>, // snow.wwise.WwiseEc056.Ec056EffectSoundListData
    }
}

rsz_struct! {
    #[rsz("snow.FieldZone")]
    #[derive(Debug, Serialize)]
    pub struct FieldZone {
        pub enabled: bool
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EcQuestSwitch")]
    #[derive(Debug, Serialize)]
    pub struct EcQuestSwitch {
        pub enabled: bool,
        pub appearable_quest_no_list: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec052SearchZone")]
    #[derive(Debug, Serialize)]
    pub struct Ec052SearchZone {
        pub enabled: bool,
        pub setting_data: ExternUser<()>, // snow.envCreature.Ec052SearchZoneSettingData
        pub collision_enable_area_no: i32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectTimerHolder")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectTimerHolder {
        pub enabled: bool,
        pub time_count: i32,
        pub data: ExternUser<()>, // snow.stage.StageObjectTimerUserData
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec053Manager")]
    #[derive(Debug, Serialize)]
    pub struct Ec053Manager {
        pub enabled: bool,
        pub manager_setting_data: ExternUser<()>, // snow.envCreature.Ec053ManagerSettingData
        pub collision_enable_area_no: i32,
        pub jump_splash_effect_key: String,
        pub landing_splash_effect_key: String,
        pub enemy_hith_effect_key: String,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectPathMoveController")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectPathMoveController {
        pub enabled: bool,
        pub data: ExternUser<()>, // snow.stage.StageObjectPathMoveUserData
        pub path_holder_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEc053")]
    #[derive(Debug, Serialize)]
    pub struct WwiseEc053 {
        pub enabled: bool,
        pub play_distance: f32,
        pub trigger: u32,
        pub player_hit_trigger: u32,
        pub enemy_hit_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectPathHolder.PathData")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectPathHolderPathData {
        pub key: String,
        pub key_hash: u32,
        pub point_div: u32,
        pub bazier_angle: f32,
        pub bazier_controll_angle: f32,
        pub point_objects: Vec<Guid>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectPathHolder")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectPathHolder {
        pub enabled: bool,
        pub path_list: Vec<StageObjectPathHolderPathData>
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEc050")]
    #[derive(Debug, Serialize)]
    pub struct WwiseEc050 {
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec054Manager")]
    #[derive(Debug, Serialize)]
    pub struct Ec054Manager {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.envCreature.Ec054ManagerSetting
        pub surface_collision_attribute_name: String
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureWalkController")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureWalkController {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.envCreature.EnvironmentCreatureWalkSetting
        pub default_up_vector_type: i32, // snow.envCreature.EnvironmentCreatureWalkController.UpVectorType
        pub collision_filter: String, // via.physics.CollisionFilterResourceHolder
        pub target_position: Vec3,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec054DetectZone")]
    #[derive(Debug, Serialize)]
    pub struct Ec054DetectZone {
        pub enabled: bool,
        pub setting: ExternUser<()>, //snow.envCreature.Ec054DetectZoneSetting
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEc054")]
    #[derive(Debug, Serialize)]
    pub struct WwiseEc054 {
        pub enabled: bool,
        pub un_stick_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec051CirclePathContoroller")]
    #[derive(Debug, Serialize)]
    pub struct Ec051CirclePathContoroller {
        pub enabled: bool,
        pub enter_point: Guid,
        pub exit_point: Guid,
        pub is_use_x_dir_positive_result: bool,
        pub radius: f32,
        pub move_speed: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.FieldGimmickBase")]
    #[derive(Debug, Serialize, Clone)]
    pub struct FieldGimmickBase {
        pub enabled: bool,
        pub unique_id: u32,
        pub type_: i32, // snow.stage.FieldGimmickManager.FieldGimmickType
        pub map_floor_type: MapFloorType,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum Ec055GroupLotteryRatio {
        High = 0,
        Normal = 1,
        Low = 2,
        Zero = 3,
    }
}

pub mod ec {
    use super::*;
    use anyhow::{Context, Result};
    macro_rules! def_ec {
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
                    #[derive(Debug, Serialize, Clone)]
                    pub struct $name {
                        $(
                            $(#[$inner_meta])* #[allow(dead_code)]
                            $inner_vis $field_name : $field_type,
                        )*
                    }
                })*
            }

            $(rsz_struct! {
                #[rsz({concat!("snow.envCreature.", stringify!($name))}
                    $(,$vhash = $version)*
                )]
                #[derive(Debug, Serialize, Clone)]
                pub struct $name {
                    #[serde(flatten)]
                    pub base: Flatten<EnvironmentCreatureBase>,
                    #[serde(flatten)]
                    pub extra: extra::$name
                }
            })*

            #[derive(Debug, Serialize)]
            pub enum Extra {
                $($name(extra::$name),)*
            }

            #[derive(Debug, Serialize)]
            pub struct EnvironmentCreatureWrapper {
                #[serde(flatten)]
                pub base: EnvironmentCreatureBase,
                pub extra: Extra,
            }

            pub fn ec_type_map() -> HashMap<u32, RszTypeInfo> {
                let mut m = HashMap::new();
                $(register::<$name>(&mut m);)*
                m
            }

            pub mod loader {
                use anyhow::{Context, Result};
                use super::FromRsz;
                $(
                    #[allow(non_snake_case)]
                    pub fn $name(rsz: &super::AnyRsz) -> Result<super::EnvironmentCreatureWrapper> {
                        let downcasted = rsz.downcast_ref::<super::$name>()
                            .with_context(||format!("Unexpected type for {}", <super::$name>::SYMBOL))?;
                        Ok(super::EnvironmentCreatureWrapper {
                            base: downcasted.base.0.clone(),
                            extra: super::Extra::$name(downcasted.extra.clone())
                        })
                    }
                )*
            }

            pub static EC_TYPE_MAP: Lazy<HashMap<&'static str, fn(&AnyRsz) -> Result<EnvironmentCreatureWrapper>>> = Lazy::new(|| {
                HashMap::from_iter([
                    $((<$name>::SYMBOL, loader::$name as fn(&AnyRsz) -> Result<EnvironmentCreatureWrapper>),)*
                ])
            });
        };
    }

    def_ec! {
        Ec001 [] {}
        Ec002 [] {}
        Ec004 [] {
            pub threat_dist: f32,
        }
        Ec005 [] {}
        Ec006 [] {}
        Ec007 [] {}
        Ec008 [] {
            pub move_interval: f32,
            pub move_interval_low: f32,
        }
        Ec009 [] {}
        Ec010 [] {
            pub height_limit: f32,
            pub front_ground_length: f32,

        }
        Ec011 [] {}
        Ec012 [] {
            pub height_limit: f32,
            pub front_wall_length: f32,
            pub front_ground_length: f32,
            pub muteki_time: i32,
            pub max_life: i32,
            pub dash_interval_time_inflate: f32,
            pub dash_interval_time_shake: f32,
            pub wait_time_inflate: f32,
            pub wait_time_shake: f32,
            pub random_move_radius: f32,
        }
        Ec014 [] {}
        Ec015 [] {}
        Ec017 [] {}
        Ec018 [] {}
        Ec019 [] { // Great Wirebug
            pub wire_long_jump_id: i32, // snow.stage.StageDef.WireLongJumpId
            pub is_relay: bool,
            pub move_interval: f32,
            pub move_interval_low: f32,
            pub speed: f32,
            pub attenuate_speed_rate: f32,
            pub move_distance: f32,
            pub scale: f32,
            pub offset_y: f32,
            pub offset_y_rate: f32,
            pub air_wait_time: f32,
            pub air_wait_timer: f32,
            pub now_offset_y: f32,
            pub starting_flag: bool,
        }
        Ec021 [] {}
        Ec022 [] {
            pub move_interval: f32,
            pub adjust_rage: f32,
            pub move_speed: f32,
            pub random_scale: f32,
        }
        Ec023 [] { // Giganha
            pub gravity: f32,
            pub reach_time: f32,
            pub add_radian_cog: f32,
            pub add_radian_tail: f32,
            pub random_rot_rate: u32,
            pub rot_length: f32,
            pub rot_length_y: f32,
            pub rot_rate: f32,
            pub rot_rate_use: bool,
            pub force_vertical_jump_dist: f32,
        }
        Ec024 [] {
            pub is_leader: bool,
            pub speed: f32,
            pub handring: f32,
            pub air_handring: f32,
            pub jump_interval: f32,
            pub last_jump_interval: f32,
            pub jump_interval_shake: i32,
            pub jump_power: f32,
            pub gravity: f32,
            pub jump_vec_limit: f32,
            pub follow_offset: Vec3,
            pub follow_handring: f32,
            pub follow_target_rate: f32,
            pub ink_cool_time: f32,
            pub init_round_index: i32,
            pub ink_range: f32,
            pub target_min_dist: f32,
            pub target_max_dist: f32,
            pub speed_max_rate: f32,
            pub speed_min_rate: f32,
            pub add_speed_rate: f32,
            pub material_data: Ec024MaterialData,
        }
        Ec025 [] { // Pincercrab
            pub active_time: f32,
            pub shot_span_time: f32,
            pub first_shot_span_time: f32,
            pub target_adjust_rate: f32,
            pub chain_delay_time: f32,
            pub cool_time: f32,
            pub aim_id: i32,
            pub chain_list: Vec<Guid>,
            pub search_dist: f32,
            pub wait_max_angle: f32,
            pub not_found_enemy_target_pos: Vec3,
            pub ref_material_curve: Option<EcMaterialCurveController>,
            pub ref_wwise: Option<WwiseEc025>,
        }
        Ec026 [] { // Echobat
            pub adjust_rate: f32,
            pub move_speed: f32,
            pub aim_id: i32,
            pub search_dist: f32,
            pub vigilance_down_offset: Vec3,
            pub attack_offset_y: f32,
            pub trans_scale_curve_x: ExternUser<()>, // snow.envCreature.Ec026TransScaleCurve
            pub trans_scale_curve_y: ExternUser<()>, // snow.envCreature.Ec026TransScaleCurve
            pub trans_scale_curve_z: ExternUser<()>, // snow.envCreature.Ec026TransScaleCurve
        }
        Ec027 [] {
            pub photo: EcPhotoBaseExtra
        }
        Ec028 [] {
            pub photo: EcPhotoBaseExtra
        }
        Ec029 [] {
            pub photo: EcPhotoBaseExtra,
            pub reaction_point_index_list: Vec<i32>,
        }
        Ec030 [] {
            pub photo: EcPhotoBaseExtra
        }
        Ec031 [] {
            pub photo: EcPhotoBaseExtra,
            pub location_type: i32, // snow.envCreature.Ec031.LocationType
            pub lava_timer: f32,
        }
        Ec032 [] {}
        Ec033 [] {}
        Ec034 [] {}
        Ec035 [] {}
        Ec036 [] { // Tricktoad
            pub active_time: f32,
            pub attract_time: f32,
            pub blink_curve: ExternUser<()>, // snow.envCreature.Ec036BlinkData
            pub ref_material_curve: Option<EcMaterialCurveController>
        }
        Ec037 [] {
            pub ref_material_curve: Option<EcMaterialCurveController>
        }
        Ec038 [] {}
        Ec050 [] {
            pub photo: EcPhotoBaseExtra,
            pub shortcut_setting_data: ExternUser<()>, // snow.envCreature.Ec050.Ec050OrbitShortcutSettingData
            pub path_holder_object: Guid,
            pub dissolve_check_camera_dist: f32,
            pub near_camera_dissolve_time: f32,

        }
        Ec051 [] {
            pub photo: EcPhotoBaseExtra,
            pub action_start_distance: f32,
            pub zone_data: ExternUser<()>, // snow.envCreature.Ec051.Ec051ReactionZoneData
        }
        Ec052 [] { // Slicercrab
            pub setting_data: ExternUser<()>, // snow.envCreature.Ec052SettingData
            pub wait_material_set_key: String,
            pub active_material_set_key: String,
            pub shot_sign_key: String,
            pub in_shot_material_key: String,
            pub reset_shot_material_key: String,
            pub cool_time_material_set_key: String,
            pub shot_effect_key: String,
            pub gun_joint_name: String,
            pub gun_tip_joint_name: String,
            pub shot_sign_state_name: String,
            pub cool_down_state_name: String,
        }
        Ec053 [] {
            pub setting_data: ExternUser<()>, // snow.envCreature.Ec053SettingData
            pub path_key: String,
            pub ref_path_holder_object: Guid,
        }
        Ec054 [] {
            pub setting_data: ExternUser<()>, // snow.envCreature.Ec054SettingData
            pub action_controller: EnvironmentCreatureActionControllerEc054Action,
            pub const_props: Guid,
        }
        Ec055 [] { // Starburst Bug
            pub setting: ExternUser<()>, // snow.envCreature.Ec055Setting,
            pub default_element_lottery_ratio: Ec055GroupLotteryRatio,
            pub action_controller: EnvironmentCreatureActionControllerEc055Action,
            pub joint_offsets: Vec<Ec055JointOffset>,
            pub camera_target_object: Guid,
        }
        Ec056 [] {
            pub setting: ExternUser<()>, // snow.envCreature.Ec056Setting
            pub guide_message_object: Guid,
        }
        Ec057 [] {}
        Ec058 [] {
            pub body_effect_key: String,
            pub get_effect_key: String,
        }
    }
}

pub use ec::{EnvironmentCreatureWrapper, EC_TYPE_MAP};

pub mod fg {
    use super::*;
    use anyhow::{Context, Result};
    macro_rules! def_fg {
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
                    #[derive(Debug, Serialize, Clone)]
                    pub struct $name {
                        $(
                            $(#[$inner_meta])* #[allow(dead_code)]
                            $inner_vis $field_name : $field_type,
                        )*
                    }
                })*
            }

            $(rsz_struct! {
                #[rsz({concat!("snow.stage.", stringify!($name))}
                    $(,$vhash = $version)*
                )]
                #[derive(Debug, Serialize, Clone)]
                pub struct $name {
                    #[serde(flatten)]
                    pub base: Flatten<FieldGimmickBase>,
                    #[serde(flatten)]
                    pub extra: extra::$name
                }
            })*

            #[allow(clippy::large_enum_variant)]
            #[derive(Debug, Serialize)]
            pub enum Extra {
                $($name(extra::$name),)*
            }

            #[derive(Debug, Serialize)]
            pub struct FieldGimmickWrapper {
                #[serde(flatten)]
                pub base: FieldGimmickBase,
                pub extra: Extra,
            }

            pub fn fg_type_map() -> HashMap<u32, RszTypeInfo> {
                let mut m = HashMap::new();
                $(register::<$name>(&mut m);)*
                m
            }

            pub mod loader {
                use anyhow::{Context, Result};
                use super::FromRsz;
                $(
                    #[allow(non_snake_case)]
                    pub fn $name(rsz: &super::AnyRsz) -> Result<super::FieldGimmickWrapper> {
                        let downcasted = rsz.downcast_ref::<super::$name>()
                            .with_context(||format!("Unexpected type for {}", <super::$name>::SYMBOL))?;
                        Ok(super::FieldGimmickWrapper {
                            base: downcasted.base.0.clone(),
                            extra: super::Extra::$name(downcasted.extra.clone())
                        })
                    }
                )*
            }

            pub static FG_TYPE_MAP: Lazy<HashMap<&'static str, fn(&AnyRsz) -> Result<FieldGimmickWrapper>>> = Lazy::new(|| {
                HashMap::from_iter([
                    $((<$name>::SYMBOL, loader::$name as fn(&AnyRsz) -> Result<FieldGimmickWrapper>),)*
                ])
            });
        };
    }

    def_fg! {
        Fg001 [] {
            pub create_interval_time: f32,
            pub move_area_size: Vec3,
            pub move_area_angle: Vec3,
            pub move_area_offset: Vec3,
            pub wait_time: f32,
            pub starting_time: f32,
            pub life_time: f32,
            pub waiting_calc_interval_time: f32,
            pub starting_effect_dist: f32,
            pub starting_effect_interval_time: f32,

        }

        Fg002 [] {
            pub select_probability: i32,
            pub search_dist: f32,
            pub cool_time: f32,
            pub fg002_type_list: Vec<i32>, // snow.envCreature.EnvironmentCreatureType
            pub probability_list: Vec<i32>,
        }

        Fg003 [] {
            pub table_id: u32,
        }

        Fg004 [] {}

        Fg005 [] {
            pub group_id: i32, // snow.stage.Fg005.GroupId
            pub fg005_id: i32, // snow.stage.Fg005.Fg005Id
            pub pre_active_time: f32,
            pub active_time: f32,
            pub finish_time: f32,
            pub enable_color_change: bool,
            pub color_idle: u32, // via.Color
            pub color_standby: u32, // via.Color
            pub color_change_time: f32,
            pub dither_time: f32,
            pub ref_object_body: Guid,
            pub ref_object_idle: Guid,
            pub ref_object_active: Guid,
            pub ref_parts_motion: Option<Motion>,
        }

        Fg006 [] {
            pub pre_active_time: f32,
            pub active_time: f32,
            pub post_process_time: f32,
            pub ref_interlock_object: Guid,
        }

        Fg007 [] {
            pub getted_dist: f32,
            pub repop_time: f32,
            pub enable_effect_dist: f32,
            pub sync_id: i32,
            pub repop_timer: f32,
        }

        Fg023 [] {
            pub is_require_map_icon: bool,
        }

        Fg025 [] {
            pub is_require_map_icon: bool,
        }

        Fg027 [] {
            pub is_require_map_icon: bool,
        }

        Fg028 [] {
            pub is_require_map_icon: bool,
        }

        Fg029 [] {
            pub is_require_map_icon: bool,
        }

        Fg030 [] {
            pub is_require_map_icon: bool,
            pub current_state: i32, // snow.stage.Fg030.State
            pub placement_type: i32,
            pub effect_parent_object: Guid,
        }

        Fg031 [] {
            pub is_require_map_icon: bool,
            pub effect_keys: Vec<KeyHash>
        }

        Fg032 [] {
            pub is_require_map_icon: bool,
        }
    }
}

pub use fg::{FieldGimmickWrapper, FG_TYPE_MAP};

rsz_struct! {
    #[rsz("snow.stage.BeltScrollController.ScrollSetting")]
    #[derive(Debug, Serialize)]
    pub struct ScrollSetting {
        pub property_name: String,
        pub scroll_direction: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.BeltScrollController")]
    #[derive(Debug, Serialize)]
    pub struct BeltScrollController {
        pub enabled: bool,
        pub scroll_type: i32, // snow.stage.BeltScrollController.ScrollType
        pub property_list: Vec<ScrollSetting>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.FieldGimmickDelegate")]
    #[derive(Debug, Serialize)]
    pub struct FieldGimmickDelegate {
        pub enabled: bool,
        pub type_: i32, // snow.stage.FieldGimmickManager.FieldGimmickType
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFieldGimmickTrigger.TriggerData")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFieldGimmickTriggerTriggerData {
        pub trigger: u32,
        pub stop_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFieldGimmickTrigger")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFieldGimmickTrigger {
        pub enabled: bool,
        pub on_stanby_trigger: Vec<WwiseFieldGimmickTriggerTriggerData>,
        pub on_active_trigger: Vec<WwiseFieldGimmickTriggerTriggerData>,
        pub on_finish_trigger: Vec<u32>,
        pub screen_space: i32, // snow.wwise.WwiseChangeSpaceWatcher.ScreenSpace
        pub trigger_update_time: f32,
        pub stop_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.Fg023EnemyHitCounter")]
    #[derive(Debug, Serialize)]
    pub struct Fg023EnemyHitCounter {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectCollisionController")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectCollisionController {
        pub enabled: bool,
        pub hit_data_type: i32, // snow.stage.StageObjectCollisionController.HitDataType
        pub data: ExternUser<()>, // snow.stage.StageObjectCollisionUserData
    }
}

rsz_struct! {
    #[rsz("snow.stage.FieldGimmickRemoteSync")]
    #[derive(Debug, Serialize)]
    pub struct FieldGimmickRemoteSync {
        pub enabled: bool,
        pub own_sync_targets: u32, // snow.stage.StageObjectRemoteSync.OwnSyncTarget
    }
}

rsz_struct! {
    #[rsz("snow.stage.Fg023StateMachine")]
    #[derive(Debug, Serialize)]
    pub struct Fg023StateMachine {
        pub enabled: bool,
        pub remote_sync_object: Guid,
        pub user_data: ExternUser<()>, // snow.stage.Fg023UserData
        pub collision_enable_area_no: i32,
        pub attack_data_key: String,
        pub type_change_attack_data_key: String,
        pub break_rock_key: String,
        pub break_ivy_effect_key: String,
        pub marker_effect_key: String,
        pub hammock_object: Guid,
        pub rock_object: Guid,
        pub ivy_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFg023")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFg023 {
        pub enabled: bool,
        pub state: i32, // snow.stage.Fg023State
        pub fall_trigger: u32,
        pub rock_break_trigger: u32,
        pub enemy_hit_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageObjectDelegate")]
    #[derive(Debug, Serialize)]
    pub struct StageObjectDelegate {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseGimmickHitOverWriter")]
    #[derive(Debug, Serialize)]
    pub struct WwiseGimmickHitOverWriter {
        pub over_write_hit_trigger_strike: u32,
        pub over_write_hit_trigger_slash: u32,
        pub over_write_hit_trigger_shell: u32,
        pub over_write_hit_trigger_ignore_meat: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageEffectCullingController")]
    #[derive(Debug, Serialize)]
    pub struct StageEffectCullingController {
        pub enabled: bool,
        pub condition_data: ExternUser<()>, // snow.stage.StageEffectCullingConditionData
    }
}

rsz_struct! {
    #[rsz("snow.stage.StuckFloorZone")]
    #[derive(Debug, Serialize)]
    pub struct StuckFloorZone {
        pub enabled: bool,
        pub soak_target_max: i32,
        pub zone_key_hash: KeyHash,
        pub zone_index: i32,
        pub soak_collision_attribute_name: String,
        pub surface_collision_attribute_name: String,
        pub bottom_collision_attribute_name: String,
        pub bottom_collision_filter: String,
        pub player_setting: ExternUser<()>, // snow.stage.PlayerStuckFloorSetting
        pub enemy_setting: ExternUser<()>, // snow.stage.EnemyStuckFloorSetting
    }
}

rsz_struct! {
    #[rsz("snow.stage.Fg028StateMachine")]
    #[derive(Debug, Serialize)]
    pub struct Fg028StateMachine {
        pub enabled: bool,
        pub remote_sync_object: Guid,
        pub setting: ExternUser<()>, // snow.stage.Fg028UserData
        pub zone_controller_object: Guid,
        pub guide_message_requester_object: Guid,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Ec054BiteArea")]
    #[derive(Debug, Serialize)]
    pub struct Ec054BiteArea {
        pub enabled: bool,
        pub setting: ExternUser<()>, // snow.envCreature.Ec054BiteAreaSetting
        pub surface_collision_attribute_name: String
    }
}

rsz_struct! {
    #[rsz("snow.stage.DamageStockHolder")]
    #[derive(Debug, Serialize)]
    pub struct DamageStockHolder {
        pub enabled: bool,
        pub stock_count: i32,
        pub attack_owners: Vec<i32>, // snow.hit.DamageFlowOwnerType
    }
}

rsz_struct! {
    #[rsz("snow.stage.Fg027StateMachine.KeyHash")]
    #[derive(Debug, Serialize)]
    pub struct Fg027StateMachineKeyHash {
        pub key: String,
        pub hash: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.Fg027StateMachine")]
    #[derive(Debug, Serialize)]
    pub struct Fg027StateMachine {
        pub enabled: bool,
        pub remote_sync_object: Guid,
        pub attack_hit_keys: Vec<Fg027StateMachineKeyHash>,
        pub attack_effect_keys: Vec<Fg027StateMachineKeyHash>,
        pub damage_motion_keys: Vec<Fg027StateMachineKeyHash>,
        pub poison_fade_out_material_keys: Vec<Fg027StateMachineKeyHash>,
        pub poison_disable_state_keys: Vec<Fg027StateMachineKeyHash>,
        pub setting: ExternUser<()>, // snow.stage.Fg027SettingUserData
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFieldGimmickStateTrigger.StateTrigger")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFieldGimmickStateTriggerStateTrigger {
        pub trigger: u32,
        pub stop_trigger: u32,
        pub state_list: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFieldGimmickStateTrigger")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFieldGimmickStateTrigger {
        pub enabled: bool,
        pub trigger_update_time: f32,
        pub stop_distance: f32,
        pub state_trigger_list: Vec<WwiseFieldGimmickStateTriggerStateTrigger>
    }
}

rsz_struct! {
    #[rsz("snow.wwise.SnowWwiseGenerator")]
    #[derive(Debug, Serialize)]
    pub struct SnowWwiseGenerator {
        pub enabled: bool,
        pub start_up: bool,
        pub closest: bool,
        pub trigger_id: Vec<u32>,
        pub stop_triggre_id: u32,
        pub fade_out_time: f32,
        pub self_update: bool,
        pub distance_check: bool,
        pub auto_stop_distance: f32,
        pub is_playing: bool,
        pub target: Zero, // via.GameObject
        pub mr_phase_state_start: i32, // snow.quest.QuestPhase
        pub mr_phase_state_last: i32, // snow.quest.QuestPhase
        pub is_mg032_culling: bool,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseFg007")]
    #[derive(Debug, Serialize)]
    pub struct WwiseFg007 {
        pub enabled: bool,
        pub get_trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Fg003ECDataList.Fg003ECData")]
    #[derive(Debug, Serialize)]
    pub struct Fg003ECData {
        pub type_: i32, // snow.envCreature.EnvironmentCreatureType
        pub rate: i32,
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.Fg003ECDataList")]
    #[derive(Debug, Serialize)]
    pub struct Fg003ECDataList {
        pub data_list: Vec<Fg003ECData>
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureData.Fg003TableData")]
    #[derive(Debug, Serialize)]
    pub struct Fg003TableData {
        pub min_num: u32,
        pub max_num: u32,
        pub ec_data: ExternUser<Fg003ECDataList>
    }
}

rsz_struct! {
    #[rsz("snow.envCreature.EnvironmentCreatureData")]
    #[derive(Debug, Serialize)]
    pub struct EnvironmentCreatureData {
        pub drop_total_num: u32,
        pub drop_a_num: u32,
        pub drop_b_num: u32,
        pub drop_c_num: u32,
        pub random_total_num: u32,
        pub fg003_table_data: Vec<Fg003TableData>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.PropsBase")]
    #[derive(Debug, Serialize)]
    pub struct PropsBase {
        pub enabled: bool,
        pub props_unique_id: u32,
        pub block_no: i32,
        pub separate_block_section: i32, // snow.ZoneDef.BlockSectionAttr
        pub group_id: i32,
        pub block_section: i32, // snow.ZoneDef.BlockSectionAttr
        pub map_floor_type: MapFloorType,
        pub register_map_icon_on_quest_start: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.BreakableStatus")]
    #[derive(Debug, Serialize)]
    pub struct BreakableStatus {
        pub enabled: bool,
        pub vital: f32,
        pub reset_vital_rate: f32,
        pub one_hit_break: bool,
        pub no_break: bool,
        pub breakable_deathblow_only: bool,
        pub force_break_deathblow: bool,
        pub enable_fake_obj_break_type_ml2deathblow: bool,
        pub enable_fake_obj_break_type_absolutepower2l: bool,
        pub force_break_intro_bind_voice: bool,
        pub guts: bool,
        pub guts_time: f32,
        pub interlocked_breakable_obj: Guid,
        pub sync_break: bool,
        pub is_hit_final_wave_boss: bool,
        pub draw_vital: bool,
        pub vital_draw_offset: Vec3,
        pub draw_damage_type: i32, // snow.stage.props.BreakableStatus.DamageDrawType
        pub parts_object_list: Vec<Guid>,
        pub ignore_hit_owner_type: Vec<i8>, // snow.hit.HitOwnerType
        pub calc_damage_master_only: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.ObjectStateSettings.ObjectStateInfo")]
    #[derive(Debug, Serialize)]
    pub struct ObjectStateInfo {
        pub target_object: Guid,
        pub is_update: bool,
        pub is_draw: bool,
        pub is_collidable: bool,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.ObjectStateSettings.Param")]
    #[derive(Debug, Serialize)]
    pub struct ObjectStateSettingsParam {
        pub key_name: String,
        pub enable: bool,
        pub info_list: Vec<ObjectStateInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.ObjectStateSettings")]
    #[derive(Debug, Serialize)]
    pub struct ObjectStateSettings {
        pub enabled: bool,
        pub setting_list: Vec<ObjectStateSettingsParam>,
        pub default_state_param_index: i32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.PropsDelegate")]
    #[derive(Debug, Serialize)]
    pub struct PropsDelegate {
        pub enabled: bool,
        pub props_type: i32, // snow.stage.StageDef.PropsType
    }
}

// not a real type
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvCreatureLotteryRateInfo {
        pub object_type: i32, // snow.envCreature.EnvironmentCreatureType
        pub drop_rate: f32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.EnvCreatureLotteryRateInfo_Anthill")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvCreatureLotteryRateInfoAnthill {
        #[serde(flatten)]
        pub base: EnvCreatureLotteryRateInfo,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.EnvCreatureLotteryData_Anthill")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvCreatureLotteryDataAnthill {
        pub lottery_data_list: Vec<EnvCreatureLotteryRateInfoAnthill>,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.EnvCreatureLotteryRateInfo_Bush")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvCreatureLotteryRateInfoBush {
        #[serde(flatten)]
        pub base: EnvCreatureLotteryRateInfo,
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.EnvCreatureLotteryData_Bush")]
    #[derive(Debug, Serialize, Clone)]
    pub struct EnvCreatureLotteryDataBush {
        pub lottery_data_list: Vec<EnvCreatureLotteryRateInfoBush>,
    }
}

rsz_sumuser! {
    #[derive(Debug, Serialize, Clone)]
    pub enum EnvCreatureLotteryData {
        Anthill(EnvCreatureLotteryDataAnthill),
        Bush(EnvCreatureLotteryDataBush)
    }
}

rsz_struct! {
    #[rsz("snow.stage.props.DropObjectBehavior")]
    #[derive(Debug, Serialize, Clone)]
    pub struct DropObjectBehavior {
        pub enabled: bool,
        pub env_creature_lottery_data: ExternUser<EnvCreatureLotteryData>, // snow.stage.props.EnvCreatureLotteryData
        pub drop_offset: Vec3,
        pub is_need_hunter_note_check: bool,
        pub hunter_note_unlock_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseBreakableObj")]
    #[derive(Debug, Serialize)]
    pub struct WwiseBreakableObj {
        pub enabled: bool,
        pub on_damaged: Vec<u32>,
        pub on_suicide: Vec<u32>,
        pub on_break: u32,
        pub on_break_lost_vital: Vec<u32>,
        pub screen_space: i32, // snow.wwise.WwiseChangeSpaceWatcher.ScreenSpace
        pub stop_distance: f32,
    }
}

rsz_struct! {
    #[rsz("snow.gui.userdata.GuiMapDetailIconListG_PopData.MapDetailIconListG_PopData")]
    #[derive(Debug, Serialize)]
    pub struct MapDetailIconListGPopData {
        pub stage_flag_list: Vec<bool>,
        pub type_: i32, // snow.gui.QuestUIManager.MapDetailIconListCategoryType
        pub item_id: ItemId,
        pub pop_id: i32, // snow.stage.StageDef.SaisyuPopId PopId
        pub name: Guid,
        pub explain: Guid,
        pub icon_no: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconPatternNo
        pub icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
    }
}

rsz_struct! {
    #[rsz("snow.gui.userdata.GuiMapDetailIconListG_PopData",
        path = "gui/01_Common/Map/MapDetailIconListUserData/GuiMapDetailIconListG_PopData.user"
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiMapDetailIconListGPopData {
        pub map_icon_data_list: Vec<MapDetailIconListGPopData>,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
// types below are not necessary for generating the website, but I added them anyway to dump more scenes

rsz_struct! {
    #[rsz("snow.stage.props.HyakuryuBarricadeBehavior")]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuBarricadeBehavior {
        pub enabled: bool,
        pub type_: i32, // snow.stage.props.HyakuryuBarricadeBehavior.Type
        pub block_index: i32,
    }
}
