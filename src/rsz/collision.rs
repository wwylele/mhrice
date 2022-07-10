use super::*;
use crate::{rsz_bitflags, rsz_enum, rsz_struct, rsz_versioned_choice};
use serde::*;

rsz_struct! {
    #[rsz("via.physics.RequestSetColliderUserData",
        0x6de94d21 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RequestSetColliderUserData {
        pub name: String,
        pub zero: Zero,
    }
}

rsz_struct! {
    #[rsz("via.physics.UserData",
        0x16593069 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PhysicsUserData {
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitDamageRSData",
        0x2fe4e6f5 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitDamageRsData {
        pub name: String,
        pub base: Option<PhysicsUserData>,
        pub parts_group: u16, // snow.enemy.EnemyDef.PartsGroup
    }
}

// snow.hit.CustomShapeType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum CustomShapeType {
        None = 0,
        Cylinder = 1,
        HoledCylinder = 2,
        TrianglePole = 3,
        Donuts = 4,
        DonutsCylinder = 5,
    }
}

// snow.hit.LimitedHitAttr
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum LimitedHitAttr {
        None = 0,
        LimitedStan = 1,
    }
}

// snow.enemy.EnemyDef.HitSoundAttr
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum HitSoundAttr {
        Default = 0,
        Silence = 1,
        Yarn = 2,
        Em082BubbleBreakOnce = 3,
        Em082OnibiBubbleBreakOnce = 4,
        Em082BubbleBreakMultiple = 5,
        Em082BubbleBreakMultipleLast = 6,
        EnemyIndex036IceArm = 7,
        EnemyIndex035FloatingRock = 8,
        EnemyIndex038FloatingRock = 9,
        EnemyIndex042CarryRock = 10,
        EnemyIndex042CaryyPot = 11,
        EnemyIndex094Ice = 12,
        EnemyIndex095MossArm = 13,
        EnemyIndex095MossHead = 14,
        EnemyIndex095RockArm = 15,
        EnemyIndex095RockHead = 16,
        EnemyIndex079Shell = 17,
    }
}

// snow.hit.DamageAttr
rsz_bitflags! {
    pub struct DamageAttr: u16 {
        const ALLOW_DISABLE = 1;
        const NO_BREAK_CONST_OBJECT = 2;
        const NO_BREAK_CONST_OBJECT_UNIQUE = 4;
    }
}

// snow.enemy.EnemyDef.BaseHitMarkType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum BaseHitMarkType {
        Normal = 0,
        Moderate = 1,
        Max = 2,
        Invalid = 3,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitDamageShapeData",
        0x2b4a32fe = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitDamageShapeData {
        pub name: String,
        pub base: PhysicsUserData,
        pub custom_shape_type: CustomShapeType,
        pub ring_radius: f32,
        pub limited_hit_attr: LimitedHitAttr,
        pub hit_sound_attr: HitSoundAttr,
        pub hit_pos_correction: f32,
        pub meat: i32, // snow.enemy.EnemyDef.Meat
        pub damage_attr: DamageAttr,
        pub base_hit_mark_type: BaseHitMarkType,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.Em135_00HitDamageShapeUniqueData",
        0x1b6de095 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct Em135_00HitDamageShapeUniqueData {
        #[serde(flatten)]
        pub base: Flatten<EmHitDamageShapeData>,
        pub back_leech_id: u16,
    }
}
