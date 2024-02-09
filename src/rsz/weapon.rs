use super::common::Versioned;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use crate::rsz_with_singleton;
use serde::*;
use std::ops::Deref;

macro_rules! impl_base {
    ($name:ty, $base:ty) => {
        impl Deref for $name {
            type Target = $base;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
    };
}

pub trait ToBase<Base> {
    fn to_base(&self) -> &Base;
}

macro_rules! impl_tobase_inner {
    ($name:ty $(, $from:ty, $to:ty)*) => {
        $(impl ToBase<$to> for $name {
            fn to_base(&self) -> &$to {
                &ToBase::<$from>::to_base(self).base
            }
        })*
    };
}

macro_rules! impl_tobase {
    ($name:ty, $($base:ty),* ; $last:ty) => {
        impl ToBase<$name> for $name {
            fn to_base(&self) -> &$name {
                self
            }
        }

        impl_tobase_inner!($name, $name $(, $base, $base)*, $last);
    };
}

pub trait MaybeToBase<Base> {
    fn maybe_to_base(&self) -> Option<&Base>;
}

impl<T, Base> MaybeToBase<Base> for T
where
    T: ToBase<Base>,
{
    fn maybe_to_base(&self) -> Option<&Base> {
        Some(self.to_base())
    }
}

macro_rules! impl_maybetobase_none {
    ($name:ty, $base:ty) => {
        impl MaybeToBase<$base> for $name {
            fn maybe_to_base(&self) -> Option<&$base> {
                None
            }
        }
    };
}

macro_rules! params {
    ($outer:ty, $inner:ty) => {
        impl Deref for $outer {
            type Target = [$inner];
            fn deref(&self) -> &Self::Target {
                &self.param
            }
        }
    };
}

// snow.equip.PlWeaponElementTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum PlWeaponElementTypes {
        None = 0,
        Fire = 1,
        Water = 2,
        Thunder = 3,
        Ice = 4,
        Dragon = 5,
        Poison = 6,
        Sleep = 7,
        Paralyze = 8,
        Bomb = 9,
    }
}

// snow.data.ContentsIdSystem.WeaponId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum WeaponId {
        Null = 0, // Not in TDB but seen in data
        None = 0x08000000,
        GreatSword(u32) = 0x08100000..=0x0810FFFF,
        ShortSword(u32) = 0x08200000..=0x0820FFFF,
        Hammer(u32) = 0x08300000..=0x0830FFFF,
        Lance(u32) = 0x08400000..=0x0840FFFF,
        LongSword(u32) = 0x08500000..=0x0850FFFF,
        SlashAxe(u32) = 0x08600000..=0x0860FFFF,
        GunLance(u32) = 0x08700000..=0x0870FFFF,
        DualBlades(u32) = 0x08800000..=0x0880FFFF,
        Horn(u32) = 0x08900000..=0x0890FFFF,
        InsectGlaive(u32) = 0x08A00000..=0x08A0FFFF,
        ChargeAxe(u32) = 0x08B00000..=0x08B0FFFF,
        LightBowgun(u32) = 0x08C00000..=0x08C0FFFF,
        HeavyBowgun(u32) = 0x08D00000..=0x08D0FFFF,
        Bow(u32) = 0x08E00000..=0x08E0FFFF,
        Insect(u32) = 0x08F00000..=0x08F0FFFF,
    }
}

impl WeaponId {
    pub fn to_tag(self) -> String {
        match self {
            WeaponId::Null => "Null".to_string(),
            WeaponId::None => "None".to_string(),
            WeaponId::GreatSword(i) => format!("GreatSword_{i:03}"),
            WeaponId::ShortSword(i) => format!("ShortSword_{i:03}"),
            WeaponId::Hammer(i) => format!("Hammer_{i:03}"),
            WeaponId::Lance(i) => format!("Lance_{i:03}"),
            WeaponId::LongSword(i) => format!("LongSword_{i:03}"),
            WeaponId::SlashAxe(i) => format!("SlashAxe_{i:03}"),
            WeaponId::GunLance(i) => format!("GunLance_{i:03}"),
            WeaponId::DualBlades(i) => format!("DualBlades_{i:03}"),
            WeaponId::Horn(i) => format!("Horn_{i:03}"),
            WeaponId::InsectGlaive(i) => format!("InsectGlaive_{i:03}"),
            WeaponId::ChargeAxe(i) => format!("ChargeAxe_{i:03}"),
            WeaponId::LightBowgun(i) => format!("LightBowgun_{i:03}"),
            WeaponId::HeavyBowgun(i) => format!("HeavyBowgun_{i:03}"),
            WeaponId::Bow(i) => format!("Bow_{i:03}"),
            WeaponId::Insect(i) => format!("Insect_{i:03}"),
        }
    }

    pub fn icon_index(self) -> u32 {
        match self {
            WeaponId::GreatSword(_) => 16,
            WeaponId::ShortSword(_) => 15,
            WeaponId::Hammer(_) => 21,
            WeaponId::Lance(_) => 19,
            WeaponId::LongSword(_) => 17,
            WeaponId::SlashAxe(_) => 23,
            WeaponId::GunLance(_) => 20,
            WeaponId::DualBlades(_) => 18,
            WeaponId::Horn(_) => 22,
            WeaponId::InsectGlaive(_) => 25,
            WeaponId::ChargeAxe(_) => 24,
            WeaponId::LightBowgun(_) => 26,
            WeaponId::HeavyBowgun(_) => 27,
            WeaponId::Bow(_) => 28,
            WeaponId::Insect(_) => 7, // TODO: or 8
            _ => 29,
        }
    }
}

// snow.equip.WeaponBaseData
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct WeaponBaseData {
        pub id: WeaponId,
        pub sort_id: u32,
        pub rare_type: RareTypes,
        pub model_id: u32, // snow.data.ParamEnum.WeaponModelId
        pub base_val: u32,
        pub buy_val: u32,
    }
}

// snow.equip.MainWeaponBaseData
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct MainWeaponBaseData {
        pub base: WeaponBaseData,
        pub atk: i32,
        pub critical_rate: i32,
        pub def_bonus: i32,
        pub hyakuryu_skill_id_list: Vec<PlHyakuryuSkillId>,
        pub slot_num_list: [u32; 4],
        pub hyakuryu_slot_num_list: [u32; 3],
        pub custom_table_no: Versioned<u32, 11_00_01>,
        pub custom_cost: Versioned<u32, 11_00_01>,
    }
}

impl_base!(MainWeaponBaseData, WeaponBaseData);

// snow.equip.ElementWeaponBaseData
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ElementWeaponBaseData {
        pub base: MainWeaponBaseData,
        pub main_element_type: PlWeaponElementTypes,
        pub main_element_val: i32,
    }
}

impl_base!(ElementWeaponBaseData, WeaponBaseData);

// snow.equip.CloseRangeWeaponBaseData
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct CloseRangeWeaponBaseData {
        pub base: ElementWeaponBaseData,
        pub sharpness_val_list: Vec<i32>,
        pub takumi_val_list: Vec<i32>,
    }
}

impl_base!(CloseRangeWeaponBaseData, ElementWeaponBaseData);

macro_rules! melee {
    ($name: ty) => {
        impl_tobase!(
            $name,
            CloseRangeWeaponBaseData,
            ElementWeaponBaseData,
            MainWeaponBaseData;
            WeaponBaseData
        );

        impl_base!($name, CloseRangeWeaponBaseData);
    }
}

rsz_struct! {
    #[rsz("snow.equip.GreatSwordBaseUserData.Param",
        0xE1132DD3 = 15_00_00,
        0x358FB8CB = 14_00_00,
        0xf17703d9 = 10_00_02,
        0x9B59CD38 = 11_00_01,
        0xFBD6F314 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct GreatSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(GreatSwordBaseUserDataParam);
impl_maybetobase_none!(GreatSwordBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GreatSwordBaseUserData",
        0x2d76ecd1 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GreatSwordBaseUserData {
        pub param: Vec<GreatSwordBaseUserDataParam>,
    }
}

params!(GreatSwordBaseUserData, GreatSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ShortSwordBaseUserData.Param",
        0x273ADE5F = 15_00_00,
        0x0B378F62 = 14_00_00,
        0xf71c3203 = 10_00_02,
        0x27D5F4E3 = 11_00_01,
        0x3EAB2B72 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShortSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(ShortSwordBaseUserDataParam);
impl_maybetobase_none!(ShortSwordBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ShortSwordBaseUserData",
        0x5212b08f = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ShortSwordBaseUserData {
        pub param: Vec<ShortSwordBaseUserDataParam>,
    }
}

params!(ShortSwordBaseUserData, ShortSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HammerBaseUserData.Param",
        0x0C2B8EEE = 15_00_00,
        0x133C11C0 = 14_00_00,
        0x7c10448c = 10_00_02,
        0x97061C3A = 11_00_01,
        0x1475A709 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HammerBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(HammerBaseUserDataParam);
impl_maybetobase_none!(HammerBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HammerBaseUserData",
        0x91d46308 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HammerBaseUserData {
        pub param: Vec<HammerBaseUserDataParam>,
    }
}

params!(HammerBaseUserData, HammerBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LanceBaseUserData.Param",
        0x1ED77653 = 15_00_00,
        0x9CB21C57 = 14_00_00,
        0xfe640a21 = 10_00_02,
        0x2265A48B = 11_00_01,
        0x2B5E129D = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct LanceBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(LanceBaseUserDataParam);
impl_maybetobase_none!(LanceBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LanceBaseUserData",
        0x6f0b52cc = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct LanceBaseUserData {
        pub param: Vec<LanceBaseUserDataParam>,
    }
}

params!(LanceBaseUserData, LanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LongSwordBaseUserData.Param",
        0xB1223ADF = 15_00_00,
        0x0DB7A395 = 14_00_00,
        0x37c07dd2 = 10_00_02,
        0x4B3F304F = 11_00_01,
        0xD738140A = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct LongSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(LongSwordBaseUserDataParam);
impl_maybetobase_none!(LongSwordBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LongSwordBaseUserData",
        0xbcb98f04 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct LongSwordBaseUserData {
        pub param: Vec<LongSwordBaseUserDataParam>,
    }
}

params!(LongSwordBaseUserData, LongSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.SlashAxeBaseUserData.Param",
        0xC7A80047 = 15_00_00,
        0x20249D23 = 14_00_00,
        0xcd4494da = 10_00_02,
        0x08FAAB11 = 11_00_01,
        0x1C246CAF = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct SlashAxeBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub slash_axe_bottle_type: SlashAxeBottleTypes,
        pub slash_axe_bottle_element_val: i32,
    }
}

melee!(SlashAxeBaseUserDataParam);
impl_maybetobase_none!(SlashAxeBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.SlashAxeBaseUserData",
        0x185fd335 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct SlashAxeBaseUserData {
        pub param: Vec<SlashAxeBaseUserDataParam>,
    }
}

params!(SlashAxeBaseUserData, SlashAxeBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GunLanceBaseUserData.Param",
        0x75361D36 = 15_00_00,
        0xACDC8033 = 14_00_00,
        0x45ba4e4d = 10_00_02,
        0xC8EC34BA = 11_00_01,
        0x00683D59 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct GunLanceBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub gun_lance_fire_type: GunLanceFireType,
        pub gun_lance_fire_lv: GunLanceFireLv,
    }
}

melee!(GunLanceBaseUserDataParam);
impl_maybetobase_none!(GunLanceBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GunLanceBaseUserData",
        0x9544cb29 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct GunLanceBaseUserData {
        pub param: Vec<GunLanceBaseUserDataParam>,
    }
}

params!(GunLanceBaseUserData, GunLanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.DualBladesBaseUserData.Param",
        0x2DE3DADB = 15_00_00,
        0xA28BFA6D = 14_00_00,
        0xc45eb4a1 = 10_00_02,
        0xF1C26C8C = 11_00_01,
        0x4D07F500 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct DualBladesBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub sub_element_type: PlWeaponElementTypes,
        pub sub_element_val: i32,
    }
}

melee!(DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.DualBladesBaseUserData",
        0x0f4b43c9 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DualBladesBaseUserData {
        pub param: Vec<DualBladesBaseUserDataParam>,
    }
}

params!(DualBladesBaseUserData, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HornBaseUserData.Param",
        0x81A1A012 = 15_00_00,
        0xA535F8A0 = 14_00_00,
        0x09da93b1 = 10_00_02,
        0xC0D8A3C4 = 11_00_01,
        0xE2382F9E = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HornBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub horn_melody_type_list: Vec<i32>, // snow.data.DataDef.HornConcertId
    }
}

melee!(HornBaseUserDataParam);
impl_maybetobase_none!(HornBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HornBaseUserData",
        0xb6dd7468 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HornBaseUserData {
        pub param: Vec<HornBaseUserDataParam>,
    }
}

params!(HornBaseUserData, HornBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.InsectGlaiveBaseUserData.Param",
        0x316CDD53 = 15_00_00,
        0x9A9C1384 = 14_00_00,
        0x68846de6 = 10_00_02,
        0xE746A91F = 11_00_01,
        0x55B00877 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct InsectGlaiveBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub insect_glaive_insect_lv: InsectLevelTypes,
    }
}

melee!(InsectGlaiveBaseUserDataParam);
impl_maybetobase_none!(InsectGlaiveBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.InsectGlaiveBaseUserData",
        0x0e15ac78 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct InsectGlaiveBaseUserData {
        pub param: Vec<InsectGlaiveBaseUserDataParam>,
    }
}

params!(InsectGlaiveBaseUserData, InsectGlaiveBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ChargeAxeBaseUserData.Param",
        0x84CAE3C4 = 15_00_00,
        0xC34EF1B9 = 14_00_00,
        0x64c2b751 = 10_00_02,
        0xFB1BB20B = 11_00_01,
        0x10B9B0F7 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct ChargeAxeBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub charge_axe_bottle_type: ChargeAxeBottleTypes,
    }
}

melee!(ChargeAxeBaseUserDataParam);
impl_maybetobase_none!(ChargeAxeBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ChargeAxeBaseUserData",
        0x242847cf = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ChargeAxeBaseUserData {
        pub param: Vec<ChargeAxeBaseUserDataParam>,
    }
}

params!(ChargeAxeBaseUserData, ChargeAxeBaseUserDataParam);

// snow.data.GameItemEnum.Fluctuation
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum Fluctuation {
        None = 0,
        LeftLittle = 1,
        LeftMuch = 2,
        RightLittle = 3,
        RightMuch = 4,
        RightAndLeftLittle = 5,
        RightAndLeftMuch = 6,
    }
}

impl std::fmt::Display for Fluctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Fluctuation::None => "None",
                Fluctuation::LeftLittle => "L Mild",
                Fluctuation::LeftMuch => "L Severe",
                Fluctuation::RightLittle => "R Mild",
                Fluctuation::RightMuch => "R Severe",
                Fluctuation::RightAndLeftLittle => "LR Mild",
                Fluctuation::RightAndLeftMuch => "LR Severe",
            }
        )
    }
}

// snow.data.GameItemEnum.KakusanType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum KakusanType {
        CloseAttack = 0,
        HorizontalAttack = 1,
    }
}

impl std::fmt::Display for KakusanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                KakusanType::CloseAttack => "Arc Shot",
                KakusanType::HorizontalAttack => "Level Shot",
            }
        )
    }
}

// snow.data.BowWeaponBaseData.CurveTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum CurveTypes {
        Curve00 = 0,
        Curve01 = 1,
        Curve02 = 2,
    }
}

impl std::fmt::Display for CurveTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                CurveTypes::Curve00 => "Recovery",
                CurveTypes::Curve01 => "Affinity",
                CurveTypes::Curve02 => "Brace",
            }
        )
    }
}

// snow.data.GameItemEnum.ShootType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum ShootType {
        None = 0,
        MovingShot = 1,
        MovingShotReload = 2,
        MovingShotSingleAuto = 3,
        MovingReload = 4,
        MovingReloadSingleAuto = 5,
        SingleAuto = 6, //?
        MovingShotReloadSingleAuto = 7,
    }
}

#[derive(Debug)]
pub struct ShootTypeFlags {
    pub moving_shot: bool,
    pub moving_reload: bool,
    pub single_auto: bool,
}

impl ShootType {
    pub fn to_flags(self) -> ShootTypeFlags {
        match self {
            ShootType::None => ShootTypeFlags {
                moving_shot: false,
                moving_reload: false,
                single_auto: false,
            },
            ShootType::MovingShot => ShootTypeFlags {
                moving_shot: true,
                moving_reload: false,
                single_auto: false,
            },
            ShootType::MovingReload => ShootTypeFlags {
                moving_shot: false,
                moving_reload: true,
                single_auto: false,
            },
            ShootType::MovingShotReload => ShootTypeFlags {
                moving_shot: true,
                moving_reload: true,
                single_auto: false,
            },

            ShootType::SingleAuto => ShootTypeFlags {
                moving_shot: false,
                moving_reload: false,
                single_auto: true,
            },
            ShootType::MovingShotSingleAuto => ShootTypeFlags {
                moving_shot: true,
                moving_reload: false,
                single_auto: true,
            },
            ShootType::MovingReloadSingleAuto => ShootTypeFlags {
                moving_shot: false,
                moving_reload: true,
                single_auto: true,
            },
            ShootType::MovingShotReloadSingleAuto => ShootTypeFlags {
                moving_shot: true,
                moving_reload: true,
                single_auto: true,
            },
        }
    }
}

// snow.equip.BulletWeaponBaseUserData.Param
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct BulletWeaponBaseUserDataParam {
        pub base: MainWeaponBaseData,
        pub fluctuation: Fluctuation ,
        pub reload: i32, // snow.data.GameItemEnum.Reload
        pub recoil: i32, // snow.data.GameItemEnum.Recoil
        pub kakusan_type: KakusanType,
        #[serde(serialize_with = "ser_arr")]
        pub bullet_equip_flag_list: [bool; 52],
        #[serde(serialize_with = "ser_arr")]
        pub bullet_num_list: [u32; 52],
        #[serde(serialize_with = "ser_arr")]
        pub bullet_type_list: [ShootType; 52],
    }
}

impl_base!(BulletWeaponBaseUserDataParam, MainWeaponBaseData);

rsz_struct! {
    #[rsz("snow.equip.LightBowgunBaseUserData.Param",
        0xE71A20C2 = 15_00_00,
        0xE3374BA7 = 14_00_00,
        0x1d6c8de0 = 10_00_02,
        0x00A5BA0D = 11_00_01,
        0xBECFF524 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct LightBowgunBaseUserDataParam {
        pub base: BulletWeaponBaseUserDataParam,
        pub rapid_shot_list: Vec<BulletType>,
        pub unique_bullet: BulletType,
    }
}
impl_tobase!(
    LightBowgunBaseUserDataParam,
    BulletWeaponBaseUserDataParam,
    MainWeaponBaseData;
    WeaponBaseData
);
impl_base!(LightBowgunBaseUserDataParam, BulletWeaponBaseUserDataParam);
impl_maybetobase_none!(LightBowgunBaseUserDataParam, ElementWeaponBaseData);
impl_maybetobase_none!(LightBowgunBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LightBowgunBaseUserData",
        0xe12489c3 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct LightBowgunBaseUserData {
        pub param: Vec<LightBowgunBaseUserDataParam>,
    }
}

params!(LightBowgunBaseUserData, LightBowgunBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HeavyBowgunBaseUserData.Param",
        0x4B6831DF = 15_00_00,
        0x7774FBAF = 14_00_00,
        0xcc0cbbf5 = 10_00_02,
        0x1637B3CF = 11_00_01,
        0x9C2C9257 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HeavyBowgunBaseUserDataParam {
        pub base: BulletWeaponBaseUserDataParam,
        pub heavy_bowgun_unique_bullet_type: UniqueBulletType,
    }
}
impl_tobase!(
    HeavyBowgunBaseUserDataParam,
    BulletWeaponBaseUserDataParam,
    MainWeaponBaseData;
    WeaponBaseData
);
impl_base!(HeavyBowgunBaseUserDataParam, BulletWeaponBaseUserDataParam);
impl_maybetobase_none!(HeavyBowgunBaseUserDataParam, ElementWeaponBaseData);
impl_maybetobase_none!(HeavyBowgunBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HeavyBowgunBaseUserData",
        0xe3ffb4d0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HeavyBowgunBaseUserData {
        pub param: Vec<HeavyBowgunBaseUserDataParam>,
    }
}

params!(HeavyBowgunBaseUserData, HeavyBowgunBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.BowBaseUserData.Param",
        0xA63CC9AC = 15_00_00,
        0x3D842715 = 14_00_00,
        0x4d471e21 = 10_00_02,
        0x7D7323A3 = 11_00_01,
        0x843361DB = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct BowBaseUserDataParam {
        pub base: ElementWeaponBaseData,
        pub bow_bottle_power_up_type_list: Vec<BottlePowerUpTypes>,
        pub bow_bottle_equip_flag_list: [bool; 7],
        pub bow_default_charge_lv_limit: BowChageStartLvTypes,
        pub bow_charge_type_list: [BowChargeTypes; 4],
        pub bow_curve_type: CurveTypes,
    }
}
impl_tobase!(
    BowBaseUserDataParam,
    ElementWeaponBaseData,
    MainWeaponBaseData;
    WeaponBaseData
);
impl_base!(BowBaseUserDataParam, ElementWeaponBaseData);
impl_maybetobase_none!(BowBaseUserDataParam, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.BowBaseUserData",
        0x74b733b8 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct BowBaseUserData {
        pub param: Vec<BowBaseUserDataParam>,
    }
}

params!(BowBaseUserData, BowBaseUserDataParam);

rsz_struct! {
    #[rsz()] // Not a TDB type
    #[derive(Debug, Serialize)]
    pub struct WeaponCraftingData {
        pub id: WeaponId,
        pub item_flag: ItemId,
        pub enemy_flag: EmTypes,
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub item: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProcessUserData.Param",
        0xB2CBAE17 = 16_00_00,
        0x61DE6B26 = 15_00_00,
        0x1C22A45A = 14_00_00,
        0xE89FAC6B = 13_00_00,
        0xfc3932c0 = 10_00_02,
        0xE80653C6 = 11_00_01,
        0xC50AC8AC = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponProcessUserDataParam {
        pub base: WeaponCraftingData,
        pub output_item: Vec<ItemId>,
        pub output_item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProcessUserData",
        0xC5EBFCA4 = 14_00_00,
        0x89685262 = 10_00_02,
        0x70E98303 = 11_00_01,
        0x451B7FDF = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponProcessUserData {
        pub param: Vec<WeaponProcessUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProductUserData.Param",
        0x68C085C4 = 16_00_00,
        0x7631F43F = 15_00_00,
        0xA4A781E9 = 14_00_00,
        0x015A23F5 = 13_00_00,
        0xcd1faa31 = 10_00_02,
        0x5FB3B9BC = 11_00_01,
        0x01B50493 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponProductUserDataParam {
        pub base: WeaponCraftingData,
        pub output_item: Vec<ItemId>,
        pub output_item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProductUserData",
        0xb7f1be22 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponProductUserData {
        pub param: Vec<WeaponProductUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChangeUserData.Param",
        0x37D1ECC6 = 16_00_00,
        0xF415D0DB = 15_00_00,
        0x973DE171 = 14_00_00,
        0x398E7E9C = 13_00_00,
        0x342cb994 = 10_00_02,
        0x38ED5D56 = 11_00_01,
        0xE3209936 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponChangeUserDataParam {
        pub base: WeaponCraftingData,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChangeUserData",
        0xb84edaa5 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponChangeUserData {
        pub param: Vec<WeaponChangeUserDataParam>,
    }
}

// snow.data.UpdateTreeData.TreeType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub enum TreeType {
        None = 0,
        Ore = 1,
        TreeType(i32) = 2..=1000, // 2 = Bone and then what a mess
    }
}

// snow.progress.VillageProgress
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum VillageProgress {
        None = 0,
        VillageProgress(i32) = 1..=7
    }
}

impl VillageProgress {
    pub fn display(self) -> Option<String> {
        match self {
            VillageProgress::None => None,
            VillageProgress::VillageProgress(x) => Some(format!("Village {x}★")),
        }
    }
}

// snow.progress.HallProgress
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum HallProgress {
        None = 0,
        HallProgress(i32) = 1..=9
    }
}

impl HallProgress {
    pub fn display(self) -> Option<String> {
        match self {
            HallProgress::None => None,
            HallProgress::HallProgress(8) => Some("Hall 7★Ex".to_owned()),
            HallProgress::HallProgress(x) => Some(format!("Hub {x}★")),
        }
    }
}

// snow.progress.MasterRankProgress
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum MasterRankProgress {
        None = 0,
        MasterRankProgress(i32) = 1..=8
    }
}

impl MasterRankProgress {
    pub fn display(self) -> Option<String> {
        match self {
            MasterRankProgress::None => None,
            MasterRankProgress::MasterRankProgress(x) => Some(format!("MR {x}★")),
        }
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponUpdateTreeUserData.Param",
        0xF7B4AD7E = 14_00_00,
        0xDF1758D2 = 13_00_00,
        0x1b7a9494 = 10_00_02,
        0x9E07F166 = 11_00_01,
        0xE5CDA333 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponUpdateTreeUserDataParam {
        pub tree_type: TreeType,
        pub index: i32,
        pub village_progress: VillageProgress,
        pub hall_progress: HallProgress,
        pub mr_progress: MasterRankProgress,
        pub weapon_id: WeaponId,
        pub next_weapon_type_list: Vec<TreeType>,
        pub next_weapon_index_list: Vec<i32>,
        pub prev_weapon_type: TreeType,
        pub prev_weapon_index: i32,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponUpdateTreeUserData",
        0x5eb36312 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponUpdateTreeUserData {
        pub param: Vec<WeaponUpdateTreeUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuWeaponHyakuryuBuildupUserData.Param",
        0x2634ECA7 = 15_00_00,
        0x4AE9A798 = 14_00_00,
        0x7BA03F20 = 10_00_02,
        0xF1DE3243 = 11_00_01,
        0x37EE6D10 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuWeaponHyakuryuBuildupUserDataParam {
        pub weapon_id: WeaponId,
        pub slot_type: i32,
        pub buildup_id_list: Vec<PlHyakuryuSkillId>,
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuWeaponHyakuryuBuildupUserData",
        path = "data/Define/Player/Weapon/HyakuryuWeaponHyakuryuBuildupData.user",
        0x2ef478cd = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuWeaponHyakuryuBuildupUserData {
        pub param: Vec<HyakuryuWeaponHyakuryuBuildupUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.equip.OverwearWeaponBaseUserData.Param",
        0xF3C6539E = 14_00_00,
        0x80309999 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct OverwearWeaponBaseUserDataParam {
        pub id: u32, // snow.data.ContentsIdSystem.OverwearWeaponId
        pub relative_id: WeaponId,
        pub sort_id: u32,
        pub model_id: u32, // snow.data.ParamEnum.WeaponModelId
        pub rare_type: RareTypes,
    }
}

rsz_struct! {
    #[rsz("snow.equip.OverwearWeaponBaseUserData",
        0xD01E7A0E = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct OverwearWeaponBaseUserData {
        pub param: Vec<OverwearWeaponBaseUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.equip.OverwearWeaponProductUserData.Param",
        0x7F26A930 = 16_00_00,
        0xC9B5598D = 15_00_00,
        0x84442698 = 14_00_00,
        0xDB849A6F = 13_00_00,
        0x9075D1D6 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct OverwearWeaponProductUserDataParam {
        pub id: u32, // snow.data.ContentsIdSystem.OverwearWeaponId
        pub item_flag: ItemId,
        pub enemy_flag: EmTypes,
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub mr_limit_flag: bool,
        pub item: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub material_category: MaterialCategory,
        pub material_category_num: u32,
        pub price: u32,
    }
}

rsz_struct! {
    #[rsz("snow.equip.OverwearWeaponProductUserData",
        0x9777F49C = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct OverwearWeaponProductUserData {
        pub param: Vec<OverwearWeaponProductUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChaosCriticalUserData.Param",
        0x7BD6B0B3 = 14_00_00,
        0x38D0C829 = 13_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponChaosCriticalUserDataParam {
        pub weapon: WeaponId,
        pub chaos_critical_num: i32,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChaosCriticalUserData",
        path = "data/Define/Player/Equip/WeaponChaosCriticalUserData.user",
        0x0F2AE8B7 = 13_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct WeaponChaosCriticalUserData {
        pub param: Vec<WeaponChaosCriticalUserDataParam>
    }
}

// snow.data.OtWeaponData.AtkTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum InsectAtkTypes {
        Smash = 0,
        Blow = 1,
    }
}

// snow.data.InsectData.ButtleTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum InsectButtleTypes {
        Normal = 0,
        JointStruggle = 1,
        Dust = 2,
        Quick = 3,
    }
}

// snow.data.InsectData.DustTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum DustTypes {
        None = 0,
        Paralyze = 1,
        Poison = 2,
        Bomb = 3,
        Heal = 4,
        PoisonParalyze = 5,
        HealPoison = 6,
        BombHeal = 7,
        HealParalyze = 8,
    }
}

impl DustTypes {
    pub fn normalize(self) -> Vec<DustTypes> {
        match self {
            DustTypes::None => vec![],
            DustTypes::Paralyze | DustTypes::Poison | DustTypes::Bomb | DustTypes::Heal => {
                vec![self]
            }
            DustTypes::PoisonParalyze => vec![DustTypes::Poison, DustTypes::Paralyze],
            DustTypes::HealPoison => vec![DustTypes::Heal, DustTypes::Poison],
            DustTypes::BombHeal => vec![DustTypes::Bomb, DustTypes::Heal],
            DustTypes::HealParalyze => vec![DustTypes::Heal, DustTypes::Paralyze],
        }
    }

    pub fn display(self) -> String {
        match self {
            DustTypes::Paralyze => "Paralyze".to_string(),
            DustTypes::Poison => "Poison".to_string(),
            DustTypes::Bomb => "Blast".to_string(),
            DustTypes::Heal => "Heal".to_string(),
            x => match &x.normalize()[..] {
                [] => "".to_string(),
                [a, b] => format!("{} / {}", a.display(), b.display()),
                _ => unreachable!(),
            },
        }
    }
}

// snow.data.InsectData.InsectSkillId
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum InsectSkillId {
        Heal = 0,
        DualExtractiveDef = 1,
        ReduseUseStamina = 2,
        TripleUp = 3,
        DualExtractiveAtk = 4,
        DualExtractiveSpd = 5,
        AutoAttackSpdUp = 6,
        StaminaRecoverUp = 7,
        MultiChargeAttack = 8,
        QuickCharge = 9,
        OnTheSpotCharge = 10,
        ExtractPowderDrop = 11,
        Absorb = 12,
    }
}

rsz_struct! {
    #[rsz("snow.equip.InsectBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct InsectBaseUserDataParam {
        pub base: WeaponBaseData,
        pub insect_atk_type: InsectAtkTypes,
        pub insect_buttle_type: InsectButtleTypes,
        pub insect_atk_table_index_list: Vec<f32>,
        pub insect_speed_table_index_list: Vec<f32>,
        pub insect_heal_table_index_list: Vec<f32>,
        pub insect_skill_id: InsectSkillId,
        pub dust_type: DustTypes,
    }
}

rsz_struct! {
    #[rsz("snow.equip.InsectBaseUserData",
        path = "data/Define/Player/Weapon/Insect/InsectBaseData.user"
    )]
    #[derive(Debug, Serialize)]
    pub struct InsectBaseUserData {
        pub param: Vec<InsectBaseUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.InsectRateTableUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct InsectRateTableUserDataParam {
        pub attack_rate: f32,
        pub heal_rate: f32,
        pub speed_rate: f32
    }
}

rsz_struct! {
    #[rsz("snow.data.InsectRateTableUserData",
        path = "data/Define/Player/Weapon/Insect/InsectRateTableData.user"
    )]
    #[derive(Debug, Serialize)]
    pub struct InsectRateTableUserData {
        pub param: Vec<InsectRateTableUserDataParam>,
    }
}

rsz_with_singleton! {
    #[path("data/Define/Player/Weapon/Insect/InsectProductData.user")]
    pub struct InsectProductData(WeaponProductUserData);
}
