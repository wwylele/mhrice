use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
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

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
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

rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
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
            WeaponId::GreatSword(i) => format!("GreatSword_{:03}", i),
            WeaponId::ShortSword(i) => format!("ShortSword_{:03}", i),
            WeaponId::Hammer(i) => format!("Hammer_{:03}", i),
            WeaponId::Lance(i) => format!("Lance_{:03}", i),
            WeaponId::LongSword(i) => format!("LongSword_{:03}", i),
            WeaponId::SlashAxe(i) => format!("SlashAxe_{:03}", i),
            WeaponId::GunLance(i) => format!("GunLance_{:03}", i),
            WeaponId::DualBlades(i) => format!("DualBlades_{:03}", i),
            WeaponId::Horn(i) => format!("Horn_{:03}", i),
            WeaponId::InsectGlaive(i) => format!("InsectGlaive_{:03}", i),
            WeaponId::ChargeAxe(i) => format!("ChargeAxe_{:03}", i),
            WeaponId::LightBowgun(i) => format!("LightBowgun_{:03}", i),
            WeaponId::HeavyBowgun(i) => format!("HeavyBowgun_{:03}", i),
            WeaponId::Bow(i) => format!("Bow_{:03}", i),
            WeaponId::Insect(i) => format!("Insect_{:03}", i),
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

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct MainWeaponBaseData {
        pub base: WeaponBaseData,
        pub atk: i32,
        pub critical_rate: i32,
        pub def_bonus: i32,
        pub hyakuryu_skill_id_list: Vec<PlHyakuryuSkillId>,
        pub slot_num_list: [u32; 3],
    }
}

impl_base!(MainWeaponBaseData, WeaponBaseData);

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
    #[rsz("snow.equip.GreatSwordBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct GreatSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(GreatSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GreatSwordBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct GreatSwordBaseUserData {
        pub param: Vec<GreatSwordBaseUserDataParam>,
    }
}

params!(GreatSwordBaseUserData, GreatSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ShortSwordBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ShortSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(ShortSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ShortSwordBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct ShortSwordBaseUserData {
        pub param: Vec<ShortSwordBaseUserDataParam>,
    }
}

params!(ShortSwordBaseUserData, ShortSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HammerBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct HammerBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(HammerBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HammerBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct HammerBaseUserData {
        pub param: Vec<HammerBaseUserDataParam>,
    }
}

params!(HammerBaseUserData, HammerBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LanceBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct LanceBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(LanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LanceBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct LanceBaseUserData {
        pub param: Vec<LanceBaseUserDataParam>,
    }
}

params!(LanceBaseUserData, LanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LongSwordBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct LongSwordBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
    }
}

melee!(LongSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.LongSwordBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct LongSwordBaseUserData {
        pub param: Vec<LongSwordBaseUserDataParam>,
    }
}

params!(LongSwordBaseUserData, LongSwordBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.SlashAxeBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct SlashAxeBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub slash_axe_bottle_type: SlashAxeBottleTypes,
        pub slash_axe_bottle_element_val: i32,
    }
}

melee!(SlashAxeBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.SlashAxeBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct SlashAxeBaseUserData {
        pub param: Vec<SlashAxeBaseUserDataParam>,
    }
}

params!(SlashAxeBaseUserData, SlashAxeBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GunLanceBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct GunLanceBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub gun_lance_fire_type: GunLanceFireType,
        pub gun_lance_fire_lv: GunLanceFireLv,
    }
}

melee!(GunLanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.GunLanceBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct GunLanceBaseUserData {
        pub param: Vec<GunLanceBaseUserDataParam>,
    }
}

params!(GunLanceBaseUserData, GunLanceBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.DualBladesBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct DualBladesBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub sub_element_type: PlWeaponElementTypes,
        pub sub_element_val: i32,
    }
}

melee!(DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.DualBladesBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct DualBladesBaseUserData {
        pub param: Vec<DualBladesBaseUserDataParam>,
    }
}

params!(DualBladesBaseUserData, DualBladesBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HornBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct HornBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub horn_melody_type_list: Vec<i32>, // snow.data.DataDef.HornConcertId
    }
}

melee!(HornBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HornBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct HornBaseUserData {
        pub param: Vec<HornBaseUserDataParam>,
    }
}

params!(HornBaseUserData, HornBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.InsectGlaiveBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct InsectGlaiveBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub insect_glaive_insect_lv: InsectLevelTypes,
    }
}

melee!(InsectGlaiveBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.InsectGlaiveBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct InsectGlaiveBaseUserData {
        pub param: Vec<InsectGlaiveBaseUserDataParam>,
    }
}

params!(InsectGlaiveBaseUserData, InsectGlaiveBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ChargeAxeBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ChargeAxeBaseUserDataParam {
        pub base: CloseRangeWeaponBaseData,
        pub charge_axe_bottle_type: ChargeAxeBottleTypes,
    }
}

melee!(ChargeAxeBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.ChargeAxeBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct ChargeAxeBaseUserData {
        pub param: Vec<ChargeAxeBaseUserDataParam>,
    }
}

params!(ChargeAxeBaseUserData, ChargeAxeBaseUserDataParam);

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

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum KakusanType {
        CloseAttack = 0,
        HorizontalAttack = 1,
    }
}

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
        pub bullet_equip_flag_list: [bool; 50],
        #[serde(serialize_with = "ser_arr")]
        pub bullet_num_list: [u32; 50],
        #[serde(serialize_with = "ser_arr")]
        pub bullet_type_list: [ShootType; 50],
    }
}

impl_base!(BulletWeaponBaseUserDataParam, MainWeaponBaseData);

rsz_struct! {
    #[rsz("snow.equip.LightBowgunBaseUserData.Param")]
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

rsz_struct! {
    #[rsz("snow.equip.LightBowgunBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct LightBowgunBaseUserData {
        pub param: Vec<LightBowgunBaseUserDataParam>,
    }
}

params!(LightBowgunBaseUserData, LightBowgunBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.HeavyBowgunBaseUserData.Param")]
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

rsz_struct! {
    #[rsz("snow.equip.HeavyBowgunBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct HeavyBowgunBaseUserData {
        pub param: Vec<HeavyBowgunBaseUserDataParam>,
    }
}

params!(HeavyBowgunBaseUserData, HeavyBowgunBaseUserDataParam);

rsz_struct! {
    #[rsz("snow.equip.BowBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct BowBaseUserDataParam {
        pub base: ElementWeaponBaseData,
        pub bow_bottle_power_up_type_list: Vec<BottlePowerUpTypes>,
        pub bow_bottle_equip_flag_list: Vec<bool>,
        pub bow_default_charge_lv_limit: BowChageStartLvTypes,
        pub bow_charge_type_list: Vec<BowChargeTypes>,
        pub bow_curve_type: i32, // snow.data.BowWeaponBaseData.CurveTypes
    }
}
impl_tobase!(
    BowBaseUserDataParam,
    ElementWeaponBaseData,
    MainWeaponBaseData;
    WeaponBaseData
);
impl_base!(BowBaseUserDataParam, ElementWeaponBaseData);

rsz_struct! {
    #[rsz("snow.equip.BowBaseUserData")]
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
    #[rsz("snow.data.WeaponProcessUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct WeaponProcessUserDataParam {
        pub base: WeaponCraftingData,
        pub output_item: Vec<ItemId>,
        pub output_item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProcessUserData")]
    #[derive(Debug, Serialize)]
    pub struct WeaponProcessUserData {
        pub param: Vec<WeaponProcessUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProductUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct WeaponProductUserDataParam {
        pub base: WeaponCraftingData,
        pub output_item: Vec<ItemId>,
        pub output_item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponProductUserData")]
    #[derive(Debug, Serialize)]
    pub struct WeaponProductUserData {
        pub param: Vec<WeaponProductUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChangeUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct WeaponChangeUserDataParam {
        pub base: WeaponCraftingData,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponChangeUserData")]
    #[derive(Debug, Serialize)]
    pub struct WeaponChangeUserData {
        pub param: Vec<WeaponChangeUserDataParam>,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub enum TreeType {
        None = 0,
        Ore = 1,
        TreeType(i32) = 2..=1000, // 2 = Bone
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum VillageProgress {
        None = 0,
        VillageProgress(i32) = 1..=7
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum HallProgress {
        None = 0,
        HallProgress(i32) = 1..=9
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponUpdateTreeUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct WeaponUpdateTreeUserDataParam {
        pub tree_type: TreeType,
        pub index: i32,
        pub village_progress: VillageProgress,
        pub hall_progress: HallProgress,
        pub weapon_id: WeaponId,
        pub next_weapon_type_list: Vec<TreeType>,
        pub next_weapon_index_list: Vec<i32>,
        pub prev_weapon_type: TreeType,
        pub prev_weapon_index: i32,
    }
}

rsz_struct! {
    #[rsz("snow.data.WeaponUpdateTreeUserData")]
    #[derive(Debug, Serialize)]
    pub struct WeaponUpdateTreeUserData {
        pub param: Vec<WeaponUpdateTreeUserDataParam>,
    }
}
