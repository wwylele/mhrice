use super::*;
use crate::{rsz_bitflags, rsz_enum, rsz_struct};
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz("via.physics.RequestSetColliderUserData",
        0x6de94d21 = 0
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct RequestSetColliderUserData {
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,
    }
}

rsz_struct! {
    #[rsz("via.physics.UserData",
        0x16593069 = 0
    )]
    #[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
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
        pub parent_user_data: Option<PhysicsUserData>,
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
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,

        // snow.hit.userdata.CommonAttachShapeData
        pub custom_shape_type: CustomShapeType,
        pub ring_radius: f32,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitAttackShapeData",
        0x4A74B073 = 15_00_00,
        0xb8f622d3 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitAttackShapeData {
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,

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

// snow.hit.AttackElement
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum AttackElement {
        None = 0x00000000,
        Fire = 0x00000001,
        Thunder = 0x00000002,
        Water = 0x00000003,
        Ice = 0x00000004,
        Dragon = 0x00000005,
        Heal = 0x00000006,
    }
}

// snow.hit.DebuffType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum DebuffType {
        None = 0x00000000,
        Fire = 0x00000001,
        Thunder = 0x00000002,
        Water = 0x00000003,
        Ice = 0x00000004,
        Dragon = 0x00000005,
        Sleep = 0x00000006,
        Paralyze = 0x00000007,
        Poison = 0x00000008,
        NoxiousPoison = 0x00000009,
        Bomb = 0x0000000A,
        BubbleS = 0x0000000B,
        BubbleRedS = 0x0000000C,
        RedS = 0x0000000D,
        BubbleL = 0x0000000E,
        DefenceDown = 0x0000000F,
        ResistanceDown = 0x00000010,
        Stink = 0x00000011,
        Capture = 0x00000012,
        OniBomb = 0x00000013,
        Kijin = 0x00000014,
        Kouka = 0x00000015,
        Bleeding = 0x00000016,
        ParalyzeShort = 0x00000017,
        Virus = 0x00000018,
    }
}

// snow.hit.GuardableType
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum GuardableType {
        Guardable = 0,
        SkillGuardable = 1,
        NoGuardable = 2,
    }
}

impl GuardableType {
    pub fn display(self) -> &'static str {
        match self {
            GuardableType::Guardable => "Yes",
            GuardableType::SkillGuardable => "Needs Guard Up",
            GuardableType::NoGuardable => "No",
        }
    }
}

// snow.hit.DamageType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum DamageType {
        Invalid = 0x00000000,
        None = 0x00000001,
        KnockBack = 0x00000002,
        FallDown = 0x00000003,
        Buttobi = 0x00000004,
        ButtobiNoDown = 0x00000005,
        ButtobiSp = 0x00000006,
        ButtobiNoEscape = 0x00000007,
        Upper = 0x00000008,
        ButtobiSlamDown = 0x00000009,
        WindS = 0x0000000A,
        WindM = 0x0000000B,
        WindL = 0x0000000C,
        QuakeS = 0x0000000D,
        QuakeL = 0x0000000E,
        EarS = 0x0000000F,
        EarL = 0x00000010,
        EarLL = 0x00000011,
        CatchAttack = 0x00000012,
        CatchingAttack = 0x00000013,
        MarionetteAttack = 0x00000014,
        GrappleAttack = 0x00000015,
        OtomoConstrain = 0x00000016,
        BuffAttack = 0x00000017,
        BuffDefence = 0x00000018,
        BuffStamina = 0x00000019,
        HitCheck = 0x0000001A,
        Deodorant = 0x0000001B,
        BuffInk = 0x0000001C,
        Flash = 0x0000001D,
        Sound = 0x0000001E,
        Tornado = 0x0000001F,
        TornadoAttack = 0x00000020,
        RecoverableUpper = 0x00000021,
        NoMediationCatchAttack = 0x00000022,
        PositionRise = 0x00000023,
        Beto = 0x00000024,
        QuakeIndirect = 0x00000025,
        ButtobiHm = 0x00000026,
        NoMediationCatchAttackWithSmash = 0x00000027,
        PlFreeze = 0x00000028,
        ExFly = 0x00000029,
    }
}

impl DamageType {
    pub fn display(self, value: i16) -> String {
        // "[...]" are types not found in data and unverified
        let mut result = match self {
            DamageType::Invalid => "Invalid",
            DamageType::None => "None",
            DamageType::KnockBack => "Knockbacks",
            DamageType::FallDown => "Tripping",
            DamageType::Buttobi => "Flying away",
            DamageType::ButtobiNoDown => "Quick recovery flying",
            DamageType::ButtobiSp => "Special flying",
            DamageType::ButtobiNoEscape => "Wirefall disabled flying",
            DamageType::Upper => "Flying up",
            DamageType::ButtobiSlamDown => "Slam down",
            DamageType::WindS => "Minor wind pressure",
            DamageType::WindM => "Major wind pressure",
            DamageType::WindL => "Dragon wind pressure",
            DamageType::QuakeS => "Minor tremor",
            DamageType::QuakeL => "Major tremor",
            DamageType::EarS => "Weak roar",
            DamageType::EarL => "Strong roar",
            DamageType::EarLL => "Powerful roar",
            DamageType::CatchAttack => "Catch",
            DamageType::CatchingAttack => "Catching attack",
            DamageType::MarionetteAttack => "[Wyvern ride]",
            DamageType::GrappleAttack => "Turf war",
            DamageType::OtomoConstrain => "[Buddy]",
            DamageType::BuffAttack => "[Buff attack]",
            DamageType::BuffDefence => "[Buff defence]",
            DamageType::BuffStamina => "[Buff stamina]",
            DamageType::HitCheck => "[HitCheck]",
            DamageType::Deodorant => "[Deodorant]",
            DamageType::BuffInk => "[BuffInk]",
            DamageType::Flash => "Flash",
            DamageType::Sound => "[Sound]",
            DamageType::Tornado => "Tornado",
            DamageType::TornadoAttack => "Tornado attack",
            DamageType::RecoverableUpper => "Flying up recoverable",
            DamageType::NoMediationCatchAttack => "Force catch",
            DamageType::PositionRise => "[PositionRise]",
            DamageType::Beto => "Webbing",
            DamageType::QuakeIndirect => "[QuakeIndirect]",
            DamageType::ButtobiHm => "[ButtobiHm]",
            DamageType::NoMediationCatchAttackWithSmash => "[NoMediationCatchAttackWithSmash]",
            DamageType::PlFreeze => "Freeze",
            DamageType::ExFly => "High fly up",
        }
        .to_owned();
        if value != 0 {
            result = format!("{result} {value}")
        }
        result
    }
}

// snow.hit.ObjectBreakType
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum ObjectBreakType {
        None = 0x00,
        PowerS = 0x01,
        PowerM = 0x02,
        PowerL = 0x03,
        AbsolutePower = 0x04,
        Deathblow = 0x05,
        ForceBreak = 0x06,
        IntroBindVoice = 0x07,
    }
}

// snow.hit.Em2EmDamageType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum Em2EmDamageType {
        None = 0,
        S = 1,
        M = 2,
        L = 3,
    }
}

// snow.hit.MarionetteEnemyDamageType
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum MarionetteEnemyDamageType {
        None = 0,
        DmanageS = 1,
        DmanageM = 2,
        DmanageL = 3,
    }
}

// snow.hit.HitAttr
rsz_bitflags! {
    pub struct HitAttr: u16 {
        const CALC_HIT_DIRECTION = 0x0001;
        const CALC_HIT_DIRECTION_BASED_ROOT_POS = 0x0002;
        const ALL_DIR_GUARDABLE = 0x0004;
        const USE_HIT_STOP = 0x0008;
        const USE_HIT_SLOW = 0x0010;
        const ABS_HIT_STOP = 0x0020;
        const USE_CYCLE_HIT = 0x0040;
        const CHECK_RAY_CAST = 0x0080;
        const IGNORE_END_DELAY = 0x0100;
        const USE_DIRECTION_OBJECT = 0x0200;
        const OVERRIDE_COLLISION_RESULT_BY_RAY = 0x0400;
        const HIT_POS_CORRECTION = 0x0800;
    }
}

// snow.hit.AttackAttr
rsz_bitflags! {
    pub struct AttackAttr: u32 {
        const ELEMENT_S = 0x00000001;
        const BOMB = 0x00000002;
        const ENSURE_DEBUFF = 0x00000004;
        const TRIGGER_MARIONETTE_START = 0x00000008;
        const FORCE_KILL = 0x00000010;
        const KOYASHI = 0x00000020;
        const ALLOW_DISABLED = 0x00000040;
        const PREVENT_CHEEP_TECH = 0x00000080;
        const HYAKURYU_BOSS_ANGER_END_SP_STOP = 0x00000100;
        const FORCE_PARTS_LOSS_PERMIT_DAMAGE_ATTR_SLASH = 0x00000200;
        const KEEP_RED_DAMAGE = 0x00000400;
        const CREATURE_NET_SEND_DAMAGE = 0x00000800;
        const EM_HP_STOP1 = 0x00001000;
        const RESTRAINT_PL_ONLY = 0x00002000;
        const RESTRAINT_ALL = 0x00004000;
        const SUCK_BLOOD = 0x00008000;
        const FORCE_PARTS_LOSS_PERMIT_DAMAGE_ATTR_STRIKE = 0x00010000;
        const FROZEN_ZAKO_EM = 0x00020000;
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmBaseHitAttackRSData")]
    #[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
    pub struct EmBaseHitAttackRSData {
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,

        pub priority: u8, // snow.hit.HitPriority
        pub hit_start_delay: i16,
        pub hit_end_delay: i16,
        pub hit_id_update_loop_num: i8,
        pub hit_id_update_delaies: Vec<i16>,
        pub damage_type: DamageType,
        pub hit_attr: HitAttr,
        pub damage_degree: MeqF32,
        pub power: u8,
        pub base_piyo_value: i8,
        pub base_attack_attr: AttackAttr,
        pub object_break_type: ObjectBreakType,
        pub base_damage: i32,
        pub base_attack_element: AttackElement,
        pub base_attack_element_value: u8,
        pub base_debuff_type: DebuffType,
        pub base_debuff_value: u8,
        pub base_debuff_sec: MeqF32,
        pub base_debuff_type2: DebuffType,
        pub base_debuff_value2: u8,
        pub base_debuff_sec2: MeqF32,
        pub base_debuff_type3: DebuffType,
        pub base_debuff_value3: u8,
        pub base_debuff_sec3: MeqF32,
        pub hit_se: u32,


        pub damage_type_value: i16,
        pub guardable_type: GuardableType,
        pub base_em2em_damage_type: Em2EmDamageType,
        pub hit_mark_type: i32, // snow.hit.EnemyHitMarkType
        pub base_hyakuryu_object_break_damage: i16,
        pub marionette_enemy_base_damage: i16,
        pub marionette_enemy_damage_type: MarionetteEnemyDamageType,
        pub marionette_enemy_base_damage_s: i16,
        pub marionette_enemy_base_damage_m: i16,
        pub marionette_enemy_base_damage_l: i16,
        pub marionette_unique_damage_list: Vec<i16>,
        pub is_mystery_debuff: bool,
        pub mystery_debuff_sec: MeqF32,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmHitAttackRSData",
        0x14A06E60 = 15_00_00,
        0xFB4BC1AC = 14_00_00,
        0x54158991 = 10_00_02,
        0xC510FF49 = 12_00_00, // what changed in this?
    )]
    #[derive(Debug, Serialize)]
    pub struct EmHitAttackRsData {
        #[serde(flatten)]
        pub base: Flatten<EmBaseHitAttackRSData>,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.EmShellHitAttackRSData",
        0x6A120158 = 15_00_00,
        0x206C4FE7 = 14_00_00,
        0x7cfb2121 = 10_00_02,
        0x6FB5D79D = 12_00_00, // what changed in this?
    )]
    #[derive(Debug, Serialize)]
    pub struct EmShellHitAttackRsData {
        #[serde(flatten)]
        pub base: Flatten<EmBaseHitAttackRSData>,
    }
}

rsz_struct! {
    #[rsz("snow.hit.userdata.HitAttackAppendShapeData",
        0x378de5ea = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct HitAttackAppendShapeData {
        pub name: String,
        pub parent_user_data: Option<PhysicsUserData>,

        // snow.hit.userdata.CommonAttachShapeData
        pub custom_shape_type: CustomShapeType,
        pub ring_radius: f32,

        pub affect_gesture: bool,
        pub dir_type: i32, // snow.motion.WindDirectionType
        pub direction: Vec3,
        pub wind_preset_type: i32, // snow.motion.WindPresetType
        pub max_rate_area: f32,
        pub is_stop_on_separate: bool,
    }
}
