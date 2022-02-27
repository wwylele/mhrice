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
        #[serde(skip)]
        pub endn_align: Aligner<16>,
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
        pub a: u8,
        pub media_type: WwiseMediaType
    }
}
