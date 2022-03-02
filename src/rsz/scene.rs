use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ViaVec4 {
        #[serde(skip)]
        pub begin_align: Aligner<16>,
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
}

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ViaQuaternion {
        #[serde(skip)]
        pub begin_align: Aligner<16>,
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
}

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ViaVec3 {
        #[serde(skip)]
        pub begin_align: Aligner<16>,
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub _w: f32,
    }
}

rsz_struct! {
    #[rsz("via.Folder",
        0xc35a0392 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Folder {
        pub name: String,
        pub tag: String,
        pub update_self: bool,
        pub draw_self: bool,
        pub paolumu: bool,
        pub path: String,
    }
}

rsz_struct! {
    #[rsz("via.GameObject",
        0xcbcfba78 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GameObject {
        pub name: String,
        pub tag: String,
        pub update_self: bool, // 14
        pub draw_self: bool, // 15
        pub time_scale: f32,
    }
}

rsz_struct! {
    #[rsz("via.Transform",
        0xb0cc69dd = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Transform {
        pub position: ViaVec4,
        pub rotation: ViaVec4,
        pub scale: ViaVec4,
        pub zinogre: String,
        pub same_joints_constraint: bool,
        pub absolute_scaling: bool
    }
}

// snow.wwise.WwiseMediaLoadManager.MediaType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum WwiseMediaType {
        Title = 0,
        Village = 1,
        Quest = 2,
        OneAreaQuest = 3,
        Hyakuryu = 4,
        LastBoss = 5,
        Event = 6,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseMediaLoader",
        0xaa0cd978 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WwiseMediaLoader {
        pub enabled: bool,
        pub media_type: WwiseMediaType
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

rsz_struct! {
    #[rsz("via.physics.RequestSetCollider.RequestSetGroup",
        0x240da282 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RequestSetGroup {
        pub path: String
    }
}

rsz_struct! {
    #[rsz("via.physics.RequestSetCollider",
        0x63d05354 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RequestSetCollider {
        pub enabled: bool,
        pub develop_draw: bool, // ? removed in code
        pub develop_draw_fill_mode: i32, // via.physics.FillMode ? removed in code
        pub barioth: Vec<String>, // +0x40
        pub rcol_file: Vec<RequestSetGroup>,
        pub fbx_skel_file: Option<String>,
        pub num_execute_workers: i32,
        pub skip_ids_mask: u32,
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
    #[rsz("via.gui.GUI",
        0xcd10d77e = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ViaGui {
        pub enabled: bool,
        pub path: String,
        pub play_speed: f32,
        pub segment: u32,
        pub soft_particle_dist_type: i32, // via.gui.SoftParticleDistType
        pub soft_particle_dist: f32,
        pub render_output_id: u32,
        pub render_target: Option<String>,
        pub gui_sound: Option<String>,
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

rsz_struct! {
    #[rsz("via.render.MaterialParam",
        0xf4ce7894 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MaterialParam {
        v0: String,
        v1: String,
        v2a: f32,
        v2b: f32,
        v2c: f32,
        v2d: f32,
        v3: String,
        v4: u32,
        v5: String,
    }
}

rsz_struct! {
    #[rsz("via.render.Mesh",
        0x8b919d87 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ViaMesh {
        render_output_id: u32, // 398
        enabled: bool, // 12a
        mesh_path: Option<String>,
        mdf2_path: Option<String>,
        static_mesh: bool, // 398 << 0x12
        small_object_culling_factor: f32, // 364
        draw_default: bool, // 398 << 9
        frustum_culling: bool, // 398 << 0x11
        occlusion_culling: bool, // << xf
        occluder: bool, // << x10
        occluder_by_lod: u8, // 128
        draw_late_occluder: bool, // 398 << x14 RenderInFront: bool?
        draw_voxelize: bool, // << 5
        draw_envmap: bool, // << 6
        draw_depth_blocker: bool, // << 7
        draw_depth_occlusion: bool, // << 0x20
        draw_raytracing: bool, // << x13
        ignore_depth: bool, // <<xA
        ignore_depth_transparent_correction: bool, // <<xB
        view_spae_scaling: bool, // <<xD
        mask_ignore_depth_mesh: bool, // <<xC
        beauty_mask_flag: bool, // <<x1C
        receive_sssss_flag: bool, // <<x1D
        streaming_priority: u32, // 0x2e8
        draw_priority_bias: i32, // 0x35c
        transparent_bias: f32, // 0x360
        stencil_value: u8, // 0x368
        material: Vec<MaterialParam>, // 0x131, 0x3a0, 0x3a8 // via.render.MaterialParam?
        use_stencil_value_priority: bool, // x398 << 0x1e
        ignore_depth_stencil_value_write: bool, // < 0x1f
        decal_recive_mode: i32, // 0x36c // via.landscape.DecalReciveMode
        draw_pos_decal: bool, // 0x398 << 0xe
        mesh_decal_priority: u32, // 0x2F0
        draw_shadow_cast: bool, // 0x398 << 8
        shadow_cast_mode: i32, // 0x398 << 0x15 // via.render.ShadowCastMode
        real_mesh_shadow: bool, // 0x163
        draw_far_cascade_shadow_cast: bool, // 0x398 << 4
        lod_mode: i32, // 0x2f4 // via.render.LodMode
        lod_level: u32, // 0x2f8
        #[serde(skip)]
        aligner: Aligner<8>,
        lod_follow_target: Guid, // 0x300...
        enable_lod_effective_range: bool, // 0x310
        lod_effective_range_s: u32, // 0x314..
        lod_effective_range_r: u32,
        shadow_lod_mode: i32, // 0x31c // via.render.LodMode
        shadow_lod_level: u32, // deci800
        enable_shadow_lod_effective_range: bool, // 0x324
        shadow_lod_effective_range_s: u32, // 0x328..
        shadow_lod_effective_range_r: u32,
        parts_enable: Vec<bool>, // ? 0x168?
        parts_caching: bool, // 0x166
        normal_recalculation_enable: bool, // 0x371
        nr_edge_excluded: bool, // 0x372
        use_blend_shape_normal: bool, // 0x118
        blend_shape_channel_weights: Vec<f32>, // 0x1b8
        draw_aabb: bool, // 0x12d
        draw_occluder: bool, // 0x12e
        v54: u8, // debug
    }
}

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
    #[derive(Debug, Serialize)]
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
