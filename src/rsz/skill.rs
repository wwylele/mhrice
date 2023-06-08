use super::*;
use crate::rsz_enum;
use crate::rsz_newtype;
use crate::rsz_struct;
use serde::*;

// snow.data.DataDef.PlEquipSkillId
rsz_enum! {
    #[rsz(u8)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub enum PlEquipSkillId {
        None = 0,
        Skill(u8) = 1..=0x6F,
        MrSkill(u8) = 0x70..=0xA7
    }
}

impl PlEquipSkillId {
    pub fn to_tag_id(self) -> Option<u8> {
        match self {
            PlEquipSkillId::None => None,
            PlEquipSkillId::Skill(i) => Some(i),
            PlEquipSkillId::MrSkill(i) => Some(i + 200),
        }
    }

    pub fn to_msg_tag(self) -> String {
        match self.to_tag_id() {
            None => "PlayerSkill_None".to_owned(),
            Some(id) => format!("PlayerSkill_{id:03}"),
        }
    }
}

rsz_struct! {
    #[rsz("snow.data.PlEquipSkillBaseUserData.Param",
        0xBCDBC248 = 15_00_00,
        0x19FEE250 = 13_00_00,
        0xac747724 = 10_00_02,
        0xCCCC00F1 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct PlEquipSkillBaseUserDataParam {
        pub id: PlEquipSkillId,
        pub max_level: i32, // snow.data.PlSkillData.SkillLvTypes, 0 = level 1
        pub icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub worth_point_list: Versioned<u32, 0, 14_99_99>,
        pub category_id_1: Versioned<u16, 15_00_00>, // snow.data.DataDef.PlEquipSkillCategoryId
        pub category_id_2: Versioned<u16, 15_00_00>,
        pub category_id_3: Versioned<u16, 15_00_00>
    }
}

rsz_struct! {
    #[rsz("snow.data.PlEquipSkillBaseUserData",
        path = "data/Define/Player/Skill/PlEquipSkill/PlEquipSkillBaseData.user",
        0x576909aa = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PlEquipSkillBaseUserData {
        pub param: Vec<PlEquipSkillBaseUserDataParam>,
    }
}

// snow.data.plHyakuryuSkill.ApplyRules
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum ApplyRules {
        None = 0,
        ElementNone = 1,
        ElementFire = 2,
        ElementWater = 3,
        ElementThunter = 4,
        ElementIce = 5,
        ElementDragon = 6,
        ElementPoison = 7,
        ElementSleep = 8,
        ElementParalyze = 9,
        ElementBomb = 10,
        ElementNotEqualMain = 11,
        ElementFirstGroup = 12,
        CanEquipTargetBottle = 13,
        Series064 = 14,
        Series065 = 15,
    }
}

// snow.data.GameItemEnum.BulletType
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum BulletType {
        None = 0,
        Normal1 = 1,
        Normal2 = 2,
        Normal3 = 3,
        Kantsu1 = 4,
        Kantsu2 = 5,
        Kantsu3 = 6,
        SanW1 = 7,
        SanW2 = 8,
        SanW3 = 9,
        SanO1 = 10,
        SanO2 = 11,
        SanO3 = 12,
        Tekko1 = 13,
        Tekko2 = 14,
        Tekko3 = 15,
        Kakusan1 = 16,
        Kakusan2 = 17,
        Kakusan3 = 18,
        Poison1 = 19,
        Poison2 = 20,
        Paralyze1 = 21,
        Paralyze2 = 22,
        Sleep1 = 23,
        Sleep2 = 24,
        Genki1 = 25,
        Genki2 = 26,
        Heal1 = 27,
        Heal2 = 28,
        Kijin = 29,
        Kouka = 30,
        Fire = 31,
        FireKantsu = 32,
        Water = 33,
        WaterKantsu = 34,
        Ice = 35,
        IceKantsu = 36,
        Thunder = 37,
        ThunderKantsu = 38,
        Dragon = 39,
        DragonKantsu = 40,
        Zanretsu = 41,
        Ryugeki = 42,
        Capture = 43,
        Setti = 44,
        Gatling = 45,
        Snipe = 46,
        GatlingHeal = 47,
        SnipeHeal = 48,
        WireBullet = 49,
        FullAuto = 50,
        Max = 51, // what is this?
    }
}

// snow.data.BowWeaponBaseData.BottlePowerUpTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Copy, Clone)]
    pub enum BottlePowerUpTypes {
        None = 0,
        ShortRange = 1,
        Poison = 2,
        Paralyze = 3,
        Sleep = 4,
    }
}

// snow.data.ElementData.ElementType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
    pub enum ElementType {
        None = 0,
        Fire = 1,
        Water = 2,
        Thunder = 3,
        Ice = 4,
        Dragon = 5,
        Poison = 6, // Posion
        Sleep = 7,
        Paralyze = 8,
        Bomb = 9,
    }
}

// snow.data.GunLanceFireData.GunLanceFireType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum GunLanceFireType {
        Normal = 0,
        Radial = 1,
        Diffusion = 2,
    }
}

impl std::fmt::Display for GunLanceFireType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GunLanceFireType::Normal => "Normal",
                GunLanceFireType::Radial => "Long",
                GunLanceFireType::Diffusion => "Wide",
            }
        )
    }
}

// snow.data.GunLanceFireData.GunLanceFireLv
rsz_newtype! {
    #[rsz_offset(1)]
    #[derive(Debug, Serialize)]
    #[serde(transparent)]
    pub struct GunLanceFireLv(pub i32);
}

// snow.data.ChargeAxeWeaponBaseData.BottleTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum ChargeAxeBottleTypes {
        Power = 2,
        StrongElement = 1,
    }
}

impl std::fmt::Display for ChargeAxeBottleTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ChargeAxeBottleTypes::Power => "Power",
                ChargeAxeBottleTypes::StrongElement => "Elemental",
            }
        )
    }
}

// snow.data.SlashAxeWeaponBaseData.BottleTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Clone, Copy)]
    pub enum SlashAxeBottleTypes {
        Power = 2,
        StrongElement = 1,
        Poison = 3,
        Paralyze = 4,
        DownStamina = 7,
        Dragon = 8,
    }
}

impl std::fmt::Display for SlashAxeBottleTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SlashAxeBottleTypes::Power => "Power",
                SlashAxeBottleTypes::StrongElement => "Elemental",
                SlashAxeBottleTypes::Poison => "Poison",
                SlashAxeBottleTypes::Paralyze => "Paralysis",
                SlashAxeBottleTypes::DownStamina => "Exhaust",
                SlashAxeBottleTypes::Dragon => "Dragon",
            }
        )
    }
}

// snow.data.HeavyBowgunWeaponData.UniqueBulletType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum UniqueBulletType {
        Snipe = 0,
        Gatling = 1,
    }
}

impl std::fmt::Display for UniqueBulletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                UniqueBulletType::Snipe => "Wyvernsnipe",
                UniqueBulletType::Gatling => "Wyvernheart",
            }
        )
    }
}

// snow.data.BowWeaponBaseData.ChargeTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone)]
    pub enum BowChargeTypes {
        None = 0,
        BurstLv1 = 1,
        BurstLv2 = 2,
        BurstLv3 = 3,
        BurstLv4 = 4,
        BurstLv5 = 5,
        DiffusionLv1 = 6,
        DiffusionLv2 = 7,
        DiffusionLv3 = 8,
        DiffusionLv4 = 9,
        DiffusionLv5 = 10,
        TransfixLv1 = 11,
        TransfixLv2 = 12,
        TransfixLv3 = 13,
        TransfixLv4 = 14,
        TransfixLv5 = 15,
    }
}

impl std::fmt::Display for BowChargeTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                BowChargeTypes::None => "None",
                BowChargeTypes::BurstLv1 => "Rapid Lv1",
                BowChargeTypes::BurstLv2 => "Rapid Lv2",
                BowChargeTypes::BurstLv3 => "Rapid Lv3",
                BowChargeTypes::BurstLv4 => "Rapid Lv4",
                BowChargeTypes::BurstLv5 => "Rapid Lv5",
                BowChargeTypes::DiffusionLv1 => "Spread Lv1",
                BowChargeTypes::DiffusionLv2 => "Spread Lv2",
                BowChargeTypes::DiffusionLv3 => "Spread Lv3",
                BowChargeTypes::DiffusionLv4 => "Spread Lv4",
                BowChargeTypes::DiffusionLv5 => "Spread Lv5",
                BowChargeTypes::TransfixLv1 => "Pierce Lv1",
                BowChargeTypes::TransfixLv2 => "Pierce Lv2",
                BowChargeTypes::TransfixLv3 => "Pierce Lv3",
                BowChargeTypes::TransfixLv4 => "Pierce Lv4",
                BowChargeTypes::TransfixLv5 => "Pierce Lv5",
            }
        )
    }
}

// snow.data.BowWeaponBaseData.BottleTypes
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize)]
    pub enum BowBottleTypes {
        ShortRange = 0,
        Power = 1,
        Poison = 2,
        Paralyze = 3,
        Sleep = 4,
        Blast = 5,
        DownStamina = 6,
        Max = 7,
        None = 8,
    }
}

// snow.data.DataDef.PlHyakuryuSkillId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    pub enum PlHyakuryuSkillId {
        None = 0,
        Skill(u32) = 1..=10000,
    }
}

// snow.data.BowWeaponBaseData.ChageStartLvTypes
rsz_newtype! {
    #[rsz_offset(1)]
    #[derive(Debug, Serialize)]
    #[serde(transparent)]
    pub struct BowChageStartLvTypes(pub i32);
}

// snow.data.InsectGlaiveWeaponBaseData.InsectLevelTypes
rsz_newtype! {
    #[rsz_offset(1)]
    #[derive(Debug, Serialize)]
    #[serde(transparent)]
    pub struct InsectLevelTypes(pub i32);
}

rsz_struct! {
    #[rsz("snow.data.PlHyakuryuSkillBaseUserData.Param",
        0x4557B275 = 15_00_00,
        0xc72df418 = 10_00_02,
        0xD599D6B2 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct PlHyakuryuSkillBaseUserDataParam {
        pub id: PlHyakuryuSkillId,
        pub item_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub apply_rule: ApplyRules,
        pub add_atk: i8,
        pub add_def_list: Vec<i8>,
        pub add_critical_rate_list: Vec<i8>,
        pub add_main_element_val: i8,
        pub add_sub_element_val: i8,
        pub add_bottle_element_val: i8,
        pub add_insect_lv: i8,
        pub add_recoil: i8,
        pub add_reload: i8,
        pub add_fluctuation: i8,
        pub add_bullet_type_list: Vec<BulletType>,
        pub add_lb_bullet_num_list: Vec<i8>,
        pub add_hb_bullet_num_list: Vec<i8>,
        pub add_rapid_shot_list: Vec<BulletType>,
        pub add_build_up_bottle_type: BottlePowerUpTypes,
        pub overwrite_flag_list: [bool; 17],
        pub overwrite_main_element_type: ElementType,
        pub overwrite_main_element_val: u8,
        pub overwrite_sub_element_type: ElementType,
        pub overwrite_sub_element_val: u8,
        pub overwrite_sharpness_val_list: Vec<i32>,
        pub overwrite_takumi_val_list: Vec<i32>,
        pub overwrite_gl_fire_type: GunLanceFireType,
        pub overwrite_gl_fire_lv: GunLanceFireLv,
        pub overwrite_concert_id_list: Vec<i32>, // snow.data.DataDef.HornConcertId
        pub overwrite_caxe_bottle_type: ChargeAxeBottleTypes,
        pub overwrite_saxe_bottle_type: SlashAxeBottleTypes,
        pub overwrite_insect_lv: InsectLevelTypes,
        pub overwrite_hb_unique_bullet: UniqueBulletType,
        pub overwrite_charge_type_list: Vec<BowChargeTypes>,
        pub overwrite_charge_start_lv: BowChageStartLvTypes,
        pub overwrite_curve_types: i32,
        pub overwrite_bottle_equip_flag: BowBottleTypes,
    }
}

rsz_struct! {
    #[rsz("snow.data.PlHyakuryuSkillBaseUserData",
        path = "data/Define/Player/Skill/PlHyakuryuSkill/PlHyakuryuSkillBaseData.user",
        0xaa891f5b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PlHyakuryuSkillBaseUserData {
        pub param: Vec<PlHyakuryuSkillBaseUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.PlHyakuryuSkillRecipeUserData.Param",
        0xB623309B = 15_00_00,
        0x50C9DF1D = 14_00_00,
        0x8FEFA1E4 = 13_00_00,
        0xa10bbbc4 = 10_00_02,
        0xFBAE98EE = 11_00_01,
        0x5DF74986 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct PlHyakuryuSkillRecipeUserDataParam {
        pub recipe_no: u32,
        pub skill_id: PlHyakuryuSkillId,
        pub cost: u32,
        pub recipe_item_id_list:Vec<ItemId>,
        pub recipe_item_num_list:Vec<u32>
    }
}

rsz_struct! {
    #[rsz("snow.data.PlHyakuryuSkillRecipeUserData",
        path = "data/Define/Player/Skill/PlHyakuryuSkill/HyakuryuSkillRecipeData.user",
        0x57557b64 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PlHyakuryuSkillRecipeUserData {
        pub param: Vec<PlHyakuryuSkillRecipeUserDataParam>,
    }
}

// snow.equip.DecorationsId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum DecorationsId {
        None = 0,
        Deco(u32) = 1..=108,
        MrDeco(u32) = 109..=1000,
    }
}

impl DecorationsId {
    pub fn to_msg_tag(self) -> String {
        match self {
            DecorationsId::None => "Decorations_None".to_owned(),
            DecorationsId::Deco(i) => format!("Decorations_{i:03}"),
            DecorationsId::MrDeco(i) => format!("Decorations_{:04}", i + 200),
        }
    }
}

rsz_struct! {
    #[rsz("snow.data.DecorationsBaseUserData.Param",
        0x5298A390 = 15_00_00,
        0x18C397F2 = 14_00_00,
        0xE302DF49 = 13_00_00,
        0x041e623d = 10_00_02,
        0x5A02A160 = 11_00_01
    )]
    #[derive(Debug, Serialize)]
    pub struct DecorationsBaseUserDataParam {
        pub id: DecorationsId,
        pub sort_id: u32,
        pub rare: RareTypes,
        pub icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub base_price: u32,
        pub decoration_lv: i32, // snow.data.DataDef.DecorationsSlotLvTypes
        pub skill_id_list: [PlEquipSkillId; 2],
        pub skill_lv_list: [i32; 2],
    }
}

rsz_struct! {
    #[rsz("snow.data.DecorationsBaseUserData",
        path = "data/Define/Player/Equip/Decorations/DecorationsBaseData.user",
        0x7a6f3daa = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DecorationsBaseUserData {
        pub param: Vec<DecorationsBaseUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.DecorationsProductUserData.Param",
        0x56AF86EF = 16_00_00,
        0x0884DACE = 15_00_00,
        0x8A090F5C = 14_00_00,
        0x8B4D41E5 = 13_00_00,
        0x1B205AA6 = 10_00_02,
        0x63B17027 = 11_00_01,
        0xA6F02F12 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct DecorationsProductUserDataParam {
        pub id: DecorationsId,
        pub item_flag: ItemId,
        pub enemy_flag: EmTypes,
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub item_id_list: Vec<ItemId>,
        pub item_num_list: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.DecorationsProductUserData",
        path = "data/Define/Player/Equip/Decorations/DecorationsProductData.user",
        0x320514c0 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct DecorationsProductUserData {
        pub param: Vec<DecorationsProductUserDataParam>,
    }
}

// snow.equip.DecorationsId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum HyakuryuDecoId {
        None = 0x00100000,
        Deco(u32) = 0x00100001..=0x0010FFFF
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuDecoBaseUserData.Param",
        0x6C079912 = 15_00_00,
        0x6790376E = 14_00_00,
        0xd431117c = 10_00_02,
        0x68E28EFF = 11_00_01,
        0x07A343F4 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuDecoBaseUserDataParam {
        pub id: HyakuryuDecoId,
        pub sort_id: u32,
        pub rare: RareTypes,
        pub icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub base_price: u32,

        pub decoration_lv: i32, // snow.data.DataDef.HyakuryuDecoSlotTypes
        pub hyakuryu_skill_id: PlHyakuryuSkillId,
        pub weapon_equip_flag: Vec<bool>,
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuDecoBaseUserData",
        path = "data/Define/Player/Equip/HyakuryuDeco/HyakuryuDecoBaseData.user",
        0xf5419ea6 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuDecoBaseUserData {
        pub param: Vec<HyakuryuDecoBaseUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuDecoProductUserData.Param",
        0x7AAD8317 = 16_00_00,
        0x62B6332B = 15_00_00,
        0xECBBA797 = 14_00_00,
        0x7FED1C2D = 13_00_00,
        0x4788eebf = 10_00_02,
        0xB62082FD = 11_00_01,
        0xA18827A0 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuDecoProductUserDataParam {
        pub id: HyakuryuDecoId,
        pub item_flag: ItemId,
        pub enemy_flag: EmTypes,
        pub progress_flag: i32, // snow.data.DataDef.UnlockProgressTypes
        pub item_id_list: Vec<ItemId>,
        pub item_num_list: Vec<u32>,
        pub material_category: MaterialCategory,
        pub point: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.HyakuryuDecoProductUserData",
        path = "data/Define/Player/Equip/HyakuryuDeco/HyakuryuDecoProductData.user",
        0x1bbfd96d = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct HyakuryuDecoProductUserData {
        pub param: Vec<HyakuryuDecoProductUserDataParam>,
    }
}
