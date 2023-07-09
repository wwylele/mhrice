use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz("via.Folder",
        0xadd98040 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct Folder {
        pub name: Option<String>,
        pub tag: Option<String>,
        pub update_self: bool,
        pub draw_self: bool,
        pub paolumu: bool,
        pub path: Option<String>,
        pub x: u64,
        pub y: u64,
        pub z: u64
    }
}

rsz_struct! {
    #[rsz("via.GameObject",
        0x0ce8a1f8 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct GameObject {
        pub name: Option<String>,
        pub tag: Option<String>,
        pub update_self: bool,
        pub draw_self: bool,
        pub time_scale: f32,
    }
}

rsz_struct! {
    #[rsz("via.Transform",
        0x239d8ecd = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct Transform {
        pub position: Vec4,
        pub rotation: Vec4,
        pub scale: Vec4,
        pub zinogre: Option<String>,
        pub same_joints_constraint: bool, // 102
        pub absolute_scaling: bool, // 103
        pub joint_segment_scale: bool, // 104
        pub joint_fast_lock_scene: bool // 105
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
        LastBossMR = 7,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseMediaLoader",
        0x432472D1 = 15_00_00,
        0x1b22997a = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct WwiseMediaLoader {
        pub enabled: bool,
        pub media_type: WwiseMediaType
    }
}

rsz_struct! {
    #[rsz("via.physics.RequestSetCollider.RequestSetGroup",
        0x240da282 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RequestSetGroup {
        pub path: Option<String>
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

rsz_struct! {
    #[rsz("via.gui.GUI",
        0x6e83e485 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct ViaGui {
        pub enabled: bool,
        pub path: Option<String>,
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
    #[rsz("via.render.MaterialParam",
        0xf4ce7894 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MaterialParam {
        v0: Option<String>,
        v1: Option<String>,
        v2a: f32,
        v2b: f32,
        v2c: f32,
        v2d: f32,
        v3: Option<String>,
        v4: u32,
        v5: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.render.Mesh",
        0x9b2876b9 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct ViaMesh {
        render_output_id: u32,
        enabled: bool,
        mesh_path: Option<String>,
        mdf2_path: Option<String>,
        static_mesh: bool, // x3d0
        small_object_culling_factor: f32, // x35c
        vbvh_rebuild_factor: f32,// x360
        frustum_culling: bool, // -
        occlusion_culling: bool, // x3d0 >> 10
        what: bool, // x3d0 >> 0x12
        what2: bool, // x3d0 >> 0x10
        occluder: bool, // x3d0 >> 0x11
        occluder_by_lod: u8, // x128
        draw_late_occluder: bool, // x3d0 >> 0x15
        draw_voxelize: bool, // x3d0 >> 6
        draw_envmap: bool, // x3d0 >> 7
        draw_depth_blocker: bool, // x3d0 >> 8
        draw_depth_occlusion: bool, // x3d0 >> x21
        draw_raytracing: bool, // x3d0 >> x14
        ignore_depth: bool, // x3d0 >> xb
        ignore_depth_transparent_correction: bool, // x3d0 >> xc
        view_space_scaling: bool, // x3d0 >> xe
        mask_ignore_depth_mesh: bool, // x3d0 >>xd
        beauty_mask_flag: bool, // x3d0 >>x1d
        receive_sssss_flag: bool, // x3d0 >>x1e
        streaming_priority: u32, // x2e0
        draw_priority_bias: i32, // x354
        v27: u32,//material: Vec<MaterialParam>, // x358
        stencil_value: u8, // x364
        material: Vec<MaterialParam>,
        v30: u8,  // x3d0 >>0x1f
        v31: bool, // x3d0 >> 0x20
        decal_recive_mode: i32,  // x36c
        draw_pos_decal: bool, // x3d0 >> 0xf
        mesh_decal_priority: u32,// x2e8
        draw_shadow_cast: bool,  // x3d0 >> 9
        shadow_cast_mode: i32, //v36: u32, // x3d0 >> x16 (+8)
        real_mesh_shadow: bool, // no
        draw_far_cascade_shadow_cast: bool, // x3d0 >> 5
        lod_mode: i32, // 2ec
        lod_level: u32, // 2f0
        lod_follow_target: Guid,
        enable_lod_effective_range: bool, // x308
        //v43
        lod_effective_range_s: u32, // x30c~x310
        lod_effective_range_r: u32,
        //v44
        shadow_lod_mode: i32, // x314
        shadow_lod_level: u32, // x318
        enable_shadow_lod_effective_range: bool, // x31c
        //v47
        shadow_lod_effective_range_s: u32, // x320~x324
        shadow_lod_effective_range_r: u32,
        shader_lod: i32, // x368
        parts_enable: Vec<bool>,
        parts_caching: bool, // x169
        normal_recalculation_enable: bool, // x371
        nr_edge_excluded: bool, // x373
        use_blend_shape_real_time_lod: bool, // x118
        use_blend_shape_normal: bool, // x119
        blend_shape_channel_weights: Vec<f32>,
        draw_aabb: bool, // x12d
        draw_occluder: bool, // x12e
        v58: u8,
    }
}

rsz_struct! {
    #[rsz("via.gui.Control",
        0xD4FB5933 = 13_00_00,
        0x2cf3efdb = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiControl {
        // PlayObject
        v0: Option<String>,
        v1: Option<String>,
        v2: u8,
        v3: u8,
        v4: u16,
        // TransformObject
        v5: Vec4,
        v6: Vec4,
        v7: Vec4,
        v8: u8,
        v9: u32,
        v10: u32,
        v11: u32,
        v12: u32,
        v13: u8,
        // Control
        v14: u32,
        v15: u32,
        v16: u8,
        v17a: u32,
        v17b: u32,
        v17c: u32,
        v17d: u32,
        v18: u8,
        v19a: u32,
        v19b: u32,
        v19c: u32,
        v20: u32,
    }
}

rsz_struct! {
    #[rsz("via.gui.Panel",
        0xAF5F0789 = 13_00_00,
        0xfcc2b758 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GuiPanel {
        #[serde(flatten)]
        pub base: Flatten<GuiControl>,
        pub v21: u32,
        pub v22: f32,
        pub v23: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.Prefab",
        0xa0e05e19 = 0
    )]#[derive(Debug, Serialize)]
    pub struct Prefab {
        pub v0: u8,
        pub v1: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.navigation.ObstacleFilterInfo",
        0x727d8279 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ObstacleFilterInfo {
        pub v0: u8,
        pub v1: Option<String>,
        pub v2: u32,
    }
}

rsz_struct! {
    #[rsz("via.navigation.ObstacleFilterSet",
        0x3fc440e4 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ObstacleFilterSet {
        pub v0: Option<String>,
        pub filters: Vec<ObstacleFilterInfo>
    }
}

rsz_struct! {
    #[rsz("via.navigation.NavigationSurface",
        0x7EEFD7FF = 13_00_00,
        0x2edbaa75 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct NavigationSurface {
        // Navigation
        pub v0: Option<String>,
        pub v1: Vec4,
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
    #[rsz("via.physics.MeshShape",
        0xa5aad20d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct MeshShape {
        pub v0: Zero, // this should be a sub object?
        pub v1: Option<String>,
        pub v2: Mat4x4
    }
}

rsz_struct! {
    #[rsz("via.physics.FilterInfo",
        0xfdca9c46 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PhysicsFilterInfo {
        pub v0: u32,
        pub v1: u32,
        pub v2: u32,
        pub v3: u32,
        pub v4: u32
    }
}

rsz_struct! {
    #[rsz("via.physics.Collider",
        0x1eba41d0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Collider {
        pub v0: u8,
        pub v1: u8,
        pub v2: MeshShape,
        pub v3: PhysicsFilterInfo,
        pub v4: PhysicsUserData,
        pub v5: Option<String>,
        pub v6: Option<String>,
        pub v7: Option<String>,
        pub v8: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.physics.Colliders",
        0x41d1c09a = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct Colliders {
        pub v0: u8,
        pub v1: u8,
        pub v2: u32,
        pub v3: Vec<String>,
        pub v4: u8,
        pub v5: Vec<Collider>,
    }
}

rsz_struct! {
    #[rsz("via.motion.TreeLayer",
        0x30136f3e = 10_00_02,
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct TreeLayer {
        pub v0: u8,
        pub v1: u32,
        pub v2: u32,
        pub v3: u32,
        pub v4: u8,
        pub v5: Option<String>,
        pub v6: u32,
        pub v7: u32,
        pub v8: u32,
        pub v9: u8,
        pub v10: u32,
        pub v11: u32,
        pub v12: u8,
        pub v13: u8,
        pub v14: u8,
        pub v15: u32,
        pub v16: u32,
        pub v17: u8,
        pub v18: u8,
        pub v19: u8,
        pub v20: u8,
        pub v21: u32,
        pub v22: u32,
        pub v23: u32,
        pub v24: u32,
        pub v25: u8,
    }
}

// untested
rsz_struct! {
    #[rsz("via.motion.MotionBank",
        0xebf452c8 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct MotionBank {
        v0: Option<String>,
        v1: u32,
        v2: u32,
        v3: u32,
    }
}

// untested
rsz_struct! {
    #[rsz("via.motion.DynamicMotionBank",
        0xf0c2477a = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct DynamicMotionBank {
        v0: u32,
        v1: u8,
        v2: u32,
        v3: u8,
        v4: u32,
        v5: u8,
        v6: u32,
        v7: Option<String>,
        v8: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.motion.Motion",
        0x18a2f773 = 10_00_02
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct Motion {
        pub v0: u8,
        pub v1: u32,
        pub v2: u32,
        pub v3: u8,
        pub v4: u8,
        pub v5: u32,

        pub v6: u32,
        pub v7: u8,
        pub v8: u8,
        pub v9: Option<String>,
        pub v10: Option<String>,
        pub v11: Option<String>,
        pub v12: Option<String>,
        pub v13: u32,
        pub v14: u8,
        pub v15: Vec<TreeLayer>, // 0x198 Layer
        pub v16: Vec<MotionBank>, // 0x348
        pub v17: Vec<DynamicMotionBank>, // 800 DynamicMotionBank
        pub v18: u8,
        pub v19: u32,
        pub v20: u8,
        pub v21: u32,
        pub v22: u8,
        pub v23: u8,
        pub v24: u32,
        pub v25: u32,
        pub v26: u32,

    }
}

rsz_struct! {
    #[rsz("via.behaviortree.BehaviorTree.CoreHandle",
        0xcc8a7873 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct CoreHandle {
        pub enabled: bool,
        pub resource: Option<String>,
        pub v2: u8,
        pub v3: u32,
        pub v4: Guid,
        pub v5: u8,
        pub v6: u32
    }
}

rsz_struct! {
    #[rsz("via.behaviortree.BehaviorTree",
        0xc45759a0 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct BehaviorTree {
        pub enabled: bool,
        pub v1: Vec<CoreHandle>,
        pub v2: Option<String>,
        pub v3: u32,
        pub v4: u32,
        pub v5: u8,
        pub v6: u8,
        pub v7: u8,
        pub v8: Zero, // A child to something
    }
}

rsz_struct! {
    #[rsz("via.motion.MotionFsm2Layer")]
    #[derive(Debug, Serialize)]
    pub struct MotionFsm2Layer {
        #[serde(flatten)]
        pub base: Flatten<CoreHandle>,

        pub v7: Option<String>,
        pub v8: u32,
        pub v9: u32,
        pub v10: u8,
        pub v11: u32,
        pub v12: u8,
        pub v13: u32,
        pub v14: u8,
        pub v15: u32,
        pub v16: u8,
    }
}

rsz_struct! {
    #[rsz("via.motion.MotionFsm2")]
    #[derive(Debug, Serialize)]
    pub struct MotionFsm2 {
        #[serde(flatten)]
        pub base: Flatten<BehaviorTree>,
        pub v9: u8,
        pub v10: u8,
        pub v11: Vec<MotionFsm2Layer>,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseCullingManager.CullingInfo")]
    #[derive(Debug, Serialize)]
    pub struct CullingInfo {
        pub weight: u32,
        pub anger_weight: u32,
        pub distance: f32,
        pub check_target: bool,
        pub is_anger: bool,
        pub is_sleep: bool,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseCullingTarget")]
    #[derive(Debug, Serialize)]
    pub struct WwiseCullingTarget {
        pub enabled: bool,
        pub culling_info: CullingInfo,
        pub wwise_culling_target_settings_data: ExternUser<()>, // snow.WwiseCullingTargetSettingsData
    }
}

rsz_struct! {
    #[rsz("snow.wwise.SoundMotionSequence")]
    #[derive(Debug, Serialize)]
    pub struct SoundMotionSequencet {
        pub enabled: bool,
    }
}

rsz_struct! {
    #[rsz("via.render.VolumeOccludee")]
    #[derive(Debug, Serialize)]
    pub struct VolumeOccludee {
        pub v0: u8,
        pub v1: u8,
        pub v2: u8,
        // via.OBB
        pub obb0_coord: Mat4x4,
        pub obb0_extend: Vec3,
        pub obb1_coord: Mat4x4,
        pub obb1_extend: Vec3,
        pub v5: u8,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEcTrigger.TriggerData")]
    #[derive(Debug, Serialize)]
    pub struct TriggerData {
        pub type_: i32, // snow.wwise.WwiseEcTrigger.TriggerType
        pub trigger: u32,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseEcTrigger")]
    #[derive(Debug, Serialize)]
    pub struct WwiseEcTrigger {
        pub enabled: bool,
        pub trigger_data: Vec<TriggerData>
    }
}

rsz_struct! {
    #[rsz("via.physics.CharacterController")]
    #[derive(Debug, Serialize)]
    pub struct CharacterController {
        // via.physics.CollidableBase
        pub enabled: bool,
        pub v1: u8,
        pub v2: u32,
        pub v3: Option<String>,

        pub v4: Vec4,
        pub v5: u32,
        pub v6: u8,
        pub v7: u8,
        pub v8: u8,
        pub v9: u8,
        pub v10: u32,
        pub v11: u32,
        pub v12: u8,
        pub v13: Vec4,
        pub v14: u32,
        pub v15: u32,
        pub v16: u32,
        pub v17: u32,
        pub v18: u32,
        pub v19: u32,
        pub v20: u8,
        pub v21: u8,
        pub v22: u8,
        pub v23: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.motion.ChainWind")]
    #[derive(Debug, Serialize)]
    pub struct ChainWind {
        pub v0: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.motion.Chain")]
    #[derive(Debug, Serialize)]
    pub struct Chain {
        // via.motion.SecondaryAnimation
        pub v0: u8,
        pub v1: u8,
        pub v2: u32,
        pub v3: u32,

        pub v4: Option<String>,
        pub v5: u32,
        pub v6: u8,
        pub v7: u32,
        pub v8: u32,
        pub v9: u32,
        pub v10: ChainWind,
        pub v11: u32,
        pub v12: u32,
        pub v13: u32,
        pub v14: u8,
        pub v15: u8,
        pub v16: u32,
        pub v17: u32,
        pub v18: u8,
        pub v19: u8,
        pub v20: u8,
        pub v21: u8,
        pub v22: u8,
        pub v23: u32,
    }
}

rsz_struct! {
    #[rsz("via.motion.IkLookAt2.JointParam")]
    #[derive(Debug, Serialize)]
    pub struct JointParam {
        pub v0: u32,
        pub v1: u32,
        pub v2: u32,
    }
}

// Untested
rsz_struct! {
    #[rsz("via.motion.IkLookAt.EyeJointParam")]
    #[derive(Debug, Serialize)]
    pub struct EyeJointParam {
        pub v0: Option<String>,
        pub v1: u32,
        pub v2: u32,
        pub v3: u32,
        pub v4: u32,
    }
}

rsz_struct! {
    #[rsz("via.motion.IkLookAt2")]
    #[derive(Debug, Serialize)]
    pub struct IkLookAt2 {
        // via.motion.SecondaryAnimation
        pub v0: u8,
        pub v1: u8,
        pub v2: u32,
        pub v3: u32,

        pub v4: Option<String>,
        pub v5: u8,
        pub v6: u8,
        pub v7: Option<String>,
        pub v8: Option<String>,
        pub v9: Option<String>,
        pub v10: Vec<JointParam>,
        pub v11: u32,
        pub v12: u32,
        pub v13: u32,
        pub v14: u32,
        pub v15: u32,
        pub v16: u32,
        pub v17: u32,
        pub v18: u32,
        pub v19: u32,
        pub v20: u32,
        pub v21: u32,
        pub v22: u32,
        pub v23: u32,
        pub v24: u32,
        pub v25: u32,
        pub v26: u32,
        pub v27: u32,
        pub v28: u32,
        pub v29: u32,
        pub v30: u32,
        pub v31: u32,
        pub v32: u32,
        pub v33: u32,
        pub v34: u8,
        pub v35: u8,
        pub v36: u8,
        pub v37: u8,
        pub v38: u8,
        pub v39: u8,
        pub v40: u8,
        pub v41: u8,
        pub v42: u32,
        pub v43: u32,
        pub v44: u32,
        pub v45: u8,
        pub v46: u32,
        pub v47: Vec<EyeJointParam>,
        pub v48: u8,
        pub v49: u8,
        pub v50: u8,
        pub v51: u8,
        pub v52: u8,
        pub v53: u8,
        pub v54: u8,
        pub v55: u8,
        pub v56: u8,
    }
}

rsz_struct! {
    #[rsz("snow.wwise.WwiseScreenTarget")]
    #[derive(Debug, Serialize)]
    pub struct WwiseScreenTarget {
        pub enabled: bool,
    }
}

// untested
rsz_struct! {
    #[rsz("via.AnimationCurve")]
    #[derive(Debug, Serialize)]
    pub struct AnimationCurve {
        pub v0: Vec<Vec4>,
        pub v1: u32,
        pub v2: u32,
        pub v3: u32,
        pub v4: u32,
        pub v5: u32,
        pub v6: u32,
    }
}

// untested
rsz_struct! {
    #[rsz("via.AnimationCurve3D")]
    #[derive(Debug, Serialize)]
    pub struct AnimationCurve3d {
        pub v0: Vec<Vec4>,
        pub v1: Vec<Vec4>,
        pub v2: Vec<Vec4>,
        pub v3: u32,
        pub v4: u32,
        pub v5: u32,
        pub v6: u32,
        pub v7: u32,
        pub v8: u32,
    }
}

rsz_struct! {
    #[rsz("via.render.Primitive")]
    #[derive(Debug, Serialize)]
    pub struct Primitive {
        // via.render.RenderEntity
        pub v0: u32,
        pub v1: u8,
        pub v2: u8,
        pub v3: u8,
        pub v4: u8,
        pub v5: u8,
        pub v6: u8,
        pub v7: u8,
        pub v8: u8,
        pub v9: u8,
        pub v10: u8,

        pub v11: u32,
        pub v12: u8,
        pub v13: u32,
        pub v14: u32,
        pub v15: u32,
        pub v16: u32,
        pub v17: u32,
        pub v18: u32,
        pub v19: u32,
    }
}

rsz_struct! {
    #[rsz("via.navigation.map.InteractionShapeOBB")]
    #[derive(Debug, Serialize)]
    pub struct InteractionShapeOBB {
        pub obb_coord: Mat4x4,
        pub obb_extend: Vec3,
    }
}

rsz_struct! {
    #[rsz("via.navigation.AIMapEffector")]
    #[derive(Debug, Serialize)]
    pub struct AIMapEffector {
        pub enabled: bool,
        pub v1: Option<String>,
        pub v2: Vec<String>,
        pub v3: Vec<()>, // ???
        pub v4: Option<String>,
        pub v5: u32,
        pub v6: u32,
        pub v7: u32,
        pub v8: u32,
        pub v9: InteractionShapeOBB,
        pub v10: u8,
        pub v11: u8,
        pub v12: u32,
        pub v13: u8,
        pub v14: u8,
    }
}

rsz_struct! {
    #[rsz("via.wwise.WwiseSphere")]
    #[derive(Debug, Serialize)]
    pub struct WwiseSphere {
        pub v0: Vec4
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
// types below are not necessary for generating the website, but I added them anyway to dump more scenes

rsz_struct! {
    #[rsz("via.render.WaterMesh")]
    #[derive(Debug, Serialize)]
    pub struct WaterMesh {
        pub v0: u32,
        pub v1: u8,
        pub v2: u8,
        pub v3: u8,
        pub v4: u8,
        pub v5: u8,
        pub v6: u8,
        pub v7: u8,
        pub v8: u8,
        pub v9: u8,
        pub v10: u8,
        pub v11: u32,
        pub v12: u32,
        pub v13: u32,
        pub v14: u8,
        pub v15: u8,
        pub v16: u32,
        pub v17: Option<String>,
        pub v18: Option<String>,
        pub v19: u8,
        pub v20: u32,
        pub v21: u8,
        pub v22: u8,
    }
}
