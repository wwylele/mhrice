use super::common::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz("via.Folder",
        0xc35a0392 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Folder {
        pub name: Option<String>,
        pub tag: Option<String>,
        pub update_self: bool,
        pub draw_self: bool,
        pub paolumu: bool,
        pub path: Option<String>,
    }
}

rsz_struct! {
    #[rsz("via.GameObject",
        0xcbcfba78 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GameObject {
        pub name: Option<String>,
        pub tag: Option<String>,
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
        pub position: Vec4,
        pub rotation: Vec4,
        pub scale: Vec4,
        pub zinogre: Option<String>,
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
        0xcd10d77e = 0
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
    #[rsz("via.gui.Control",
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
        0x8fb90e73 = 0
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
        0x5f9047f4 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct TreeLayer {
        pub v0: u32,
        pub v1: u32,
        pub v2: u8,
        pub v3: Option<String>,
        pub v4: u32,
        pub v5: u32,
        pub v6: u32,
        pub v7: u32,
        pub v8: u32,
        pub v9: u8,
        pub v10: u8,
        pub v11: u8,
        pub v12: u32,
        pub v13: u32,
        pub v14: u8,
        pub v15: u8,
        pub v16: u8,
        pub v17: u8,
        pub v18: u32,
        pub v19: u32,
        pub v20: u32,
        pub v21: u32,
        pub v22: u8,
    }
}

// untested
rsz_struct! {
    #[rsz("via.motion.MotionBank",
        0xebf452c8 = 0
    )]
    #[derive(Debug, Serialize)]
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
    #[derive(Debug, Serialize)]
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
        0xb8e5e915 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Motion {
        pub v0: u8,
        pub v1: u32,
        pub v2: u32,
        pub v3: u8,
        pub v4: u8,

        pub v5: u32,
        pub v6: u8,
        pub v7: u8,
        pub v8: Option<String>,
        pub v9: Option<String>,
        pub v10: Option<String>,
        pub v11: Option<String>,
        pub v12: u32,
        pub v13: u8,
        pub v14: Vec<TreeLayer>, // 0x198 Layer
        pub v15: Vec<MotionBank>, // 0x348
        pub v16: Vec<DynamicMotionBank>, // 800 DynamicMotionBank
        pub v17: u8,
        pub v18: u32,
        pub v19: u32,
        pub v20: u8,
        pub v21: u32,
        pub v22: u32,
        pub v23: u32,

    }
}
