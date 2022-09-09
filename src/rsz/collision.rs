use super::*;
use crate::{rsz_bitflags, rsz_enum, rsz_struct};
use serde::*;

rsz_struct! {
    #[rsz("via.physics.RequestSetColliderUserData",
        0x6de94d21 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct RequestSetColliderUserData {
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,
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

rsz_struct! {
    #[rsz("snow.hit.userdata.DummyHitAttackShapeData",
        0xf5735618 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct DummyHitAttackShapeData {
        pub base: Flatten<RequestSetColliderUserData>,

        // snow.hit.userdata.CommonAttachShapeData
        pub custom_shape_type: CustomShapeType,
        pub ring_radius: f32,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitAttackShapeData",
        0xb8f622d3 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitAttackShapeData {
        pub base: Flatten<RequestSetColliderUserData>,

        // snow.hit.userdata.CommonAttachShapeData
        pub custom_shape_type: CustomShapeType,
        pub ring_radius: f32,

        pub shape_attr: i32, // snow.hit.EmShapeAttr
        pub check_terrain_shape_scale: f32,
        pub hit_terrain_interrupt_type: i32, // snow.hit.EnemyHitTerrainInterruptType
        pub enemy_hit_check_vec: i32, // snow.hit.EnemyHitCheckVec
        pub shake_wall_info_id: i32,
        pub hit_character_interrupt_type: i32, // snow.hit.EnemyHitCharacterInterruptType
        pub condition_match_hit_attr: u16, // snow.hit.AttackConditionMatchHitAttr
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitAttackRSData",
        0x54158991 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitAttackRSData {
        #[serde(flatten)]
        pub base: Flatten<RequestSetColliderUserData>,

        // snow.hit.userdata.BaseHitAttackRSData
        pub priority: u8, // snow.hit.HitPriority
        pub hit_start_delay: i16,
        pub hit_end_delay: i16,
        pub hit_id_update_loop_num: i8,
        pub hit_id_update_delaies: Vec<i16>,
        pub damage_type: i32, // snow.hit.DamageType
        pub hit_attr: u16, // snow.hit.HitAttr
        pub damage_degree: f32,
        pub power: u8,
        pub base_piyo_value: i8,
        pub base_attack_attr: u32, // snow.hit.AttackAttr,
        pub object_break_type: u8, // snow.hit.ObjectBreakType
        pub base_damage: i32,
        pub base_attack_element: i32, // snow.hit.AttackElement
        pub base_attack_element_value: u8,
        pub base_debuff_type: i32, // snow.hit.DebuffType
        pub base_debuff_value: u8,
        pub base_debuff_sec: f32,
        pub base_debuff_type2: i32, // snow.hit.DebuffType
        pub base_debuff_value2: u8,
        pub base_debuff_sec2: f32,
        pub base_debuff_type3: i32, // snow.hit.DebuffType
        pub base_debuff_value3: u8,
        pub base_debuff_sec3: f32,
        pub hit_se: u32,


        pub damage_type_value: i16,
        pub guardable_type: u8, // snow.hit.GuardableType
        pub base_em2em_damage_type: i32, // snow.hit.Em2EmDamageType
        pub hit_mark_type: i32, // snow.hit.EnemyHitMarkType
        pub base_hyakuryu_object_break_damage: i16,
        pub marionette_enemy_base_damage: i16,
        pub marionette_enemy_damage_type: u8, // snow.hit.MarionetteEnemyDamageType
        pub marionette_enemy_base_damage_s: i16,
        pub marionette_enemy_base_damage_m: i16,
        pub marionette_enemy_base_damage_l: i16,
        pub marionette_unique_damage_list: Vec<i16>,
        pub is_mystery_debuff: bool,
        pub mystery_debuff_sec: f32,

    }
}
