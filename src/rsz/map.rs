use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_sumtype;
use nalgebra_glm::*;
use serde::*;

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
    #[derive(Debug, Serialize)]
    pub struct PlayerInfluencePopMarker {
        pub base: ObjectPopMarker,
        pub test_bell_trigger: u32,
        pub creeping_point_adjust_transform: Quat,
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType

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
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType
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
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType
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
    #[derive(Debug, Serialize)]
    pub struct StageFacilityPopMarker {
        pub base: ObjectPopMarker,
        pub map_floor_type: i32, // snow.stage.StageDef.MapFloorType
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
        pub stop_on_ready: Versioned<bool, 13_00_00, 0xFFFFFFFF>,
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
    #[derive(Debug, Serialize)]
    pub struct KeyHash {
        pub key: String,
        pub hash: u32,
    }
}

rsz_struct! {
    #[rsz("snow.stage.StageDemoCameraSceneRequeter.RequestData",
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
        pub reset_delay_time: Versioned<f32, 13_00_00, 0xFFFFFFFF>,
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
        pub is_apply_all_targets: Versioned<bool, 13_00_00, 0xFFFFFFFF>,
        pub targets: Vec<StageObjectStateControllerTargetObject>,
        pub target_object_names: Versioned<Vec<String>, 13_00_00, 0xFFFFFFFF>,
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
        pub target_type: Versioned<i32, 13_00_00, 0xFFFFFFFF>, // snow.stage.StageObjectEffectController.TargetType
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
        pub data: ExternUser<()>, // snow.stage.StageObjectEffectUserData,
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
