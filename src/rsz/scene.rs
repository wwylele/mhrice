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
        pub c: u8,
        pub d: u8,
        pub e: u8,
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
        pub draw_self: bool,
        pub update_self: bool,
        pub time_scale: f32,
    }
}

rsz_struct! {
    #[rsz("via.Transform",
        0xb0cc69dd = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct Transform {
        pub a: ViaVec4,
        pub b: ViaVec4,
        pub c: ViaVec4,
        pub d: String,
        pub e: u8,
        pub f: u8
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
        pub a: u8,
        pub b: u8,
        pub c: u32,
        pub d: Vec<String>,
        pub rcol_file: Vec<RequestSetGroup>,
        pub fbx_skel_file: Option<String>,
        pub f: u32,
        pub g: u32,
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
        pub a: u8,
        pub path: String,
        pub c: f32,
        pub d: u32,
        pub e: u32,
        pub f: u32,
        pub g: u32,
        pub h: Option<String>,
        pub i: Option<String>,
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
