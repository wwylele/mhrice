use super::*;
use crate::rsz_enum;
use crate::rsz_newtype;
use crate::rsz_struct;
use serde::*;

// snow.data.GameItemEnum.CarriableFilter
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum CarriableFilter {
        All = 0,
        Quest = 1,
        Hyakuryu = 2,
        Lobby = 3,
    }
}

// snow.data.DataDef.ItemTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub enum ItemTypes {
        Consume = 0,
        Tool = 1,
        Material = 2,
        OffcutsMaterial = 3,
        Bullet = 4,
        Bottle = 5,
        Present = 6,
        PayOff = 7,
        CarryPayOff = 8,
        Carry = 9,
        Judge = 10,
        Antique = 11,
    }
}

// snow.data.GameItemEnum.IconRank
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum IconRank {
        None = 0,
        Great = 1,
        Lv1 = 2,
        Lv2 = 3,
        Lv3 = 4,
        Mystery = 5,
    }
}

// snow.data.DataDef.RankTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum RankTypes {
        Low = 0,
        Upper = 1,
        Master = 2,
    }
}

// snow.data.NormalItemData.ItemGroupTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum ItemGroupTypes {
        Drink = 0,
        Food = 1,
        Others = 2,
    }
}

// snow.data.ContentsIdSystem.ItemId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum ItemId {
        Null = 0, // not defined in TDB, but appears in some overwear data
        None = 0x04000000,
        Normal(u32) = 0x04100000..=0x0410FFFF,
        Ec(u32) = 0x04200000..=0x0420FFFF, // TODO: I_EC_0057 and up is offseted
    }
}

// snow.data.DataDef.RareTypes
rsz_newtype! {
    #[rsz_offset(1)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    #[serde(transparent)]
    pub struct RareTypes(pub u8);
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
    pub enum MaterialCategory {
        None = 0,
        LrHr(i32) = 1..=85,
        ArmorSphere = 86,
        Mr(i32) = 153..=1000,
    }
}

// Eh, please
impl MaterialCategory {
    pub fn from_msg_id(id: i32) -> MaterialCategory {
        if id < 200 {
            MaterialCategory::LrHr(id - 1)
        } else {
            MaterialCategory::Mr(id - 200)
        }
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemUserData.Param",
        0xD0C19D16 = 13_00_00,
        0xc4940266 = 10_00_02,
        0xB8376E37 = 11_00_01,
        0xBF248F26 = 12_00_00
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemUserDataParam {
        pub id: ItemId,
        pub cariable_filter: CarriableFilter,
        pub type_: ItemTypes,
        pub rare: RareTypes,
        pub pl_max_count: u32,
        pub ot_max_count: u32,
        pub sort_id: u32,
        pub supply: bool,
        pub can_put_in_dog_pouch: bool,
        pub show_item_window: bool,
        pub show_action_window: bool,
        pub infinite: bool,
        pub default: bool,
        pub icon_can_eat: bool,
        pub icon_item_rank: IconRank,
        pub effect_rare: bool,
        pub icon_chara: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconPatternNo
        pub icon_color: i32, // snow.gui.SnowGuiCommonUtility.Icon.ItemIconColor
        pub se_type: i32, // snow.data.GameItemEnum.SeType
        pub sell_price: u32,
        pub buy_price: u32,
        pub item_action_type: i32, // snow.data.GameItemEnum.ItemActionType
        pub rank_type: RankTypes,
        pub item_group: ItemGroupTypes,
        pub category_worth: u32,
        pub material_category: Vec<MaterialCategory>,
        pub evaluation_value: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemUserData",
        path = "data/System/ContentsIdSystem/Item/Normal/ItemData.user",
        0x66200423 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemUserData {
        pub param: Vec<ItemUserDataParam>,
    }
}

// snow.data.ContentsIdSystem.LvBuffCageId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
    pub enum LvBuffCageId {
        CommonNone = 0x18000000,
        CommonError = 0x18000001,
        CommonMax = 0x18000002,
        Normal(u32) = 0x18100000..= 0x1810FFFF
    }
}

rsz_struct! {
    #[rsz("snow.data.NormalLvBuffCageBaseUserData.Param",
        0x1026C5DC = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalLvBuffCageBaseUserDataParam {
        pub id: LvBuffCageId,
        pub sort_index: u32,
        pub rarity: RareTypes,
        pub model_lv: i32, // snow.equip.LvBuffCageModelLv
        pub model_color_index: ColorTypes,
        pub status_buff_limit: Vec<u32>,
        pub status_buff_add_value: Vec<u32>,
        pub status_buff_all_add_value: Vec<u32>,
        pub status_start_revise_val: Vec<u32>,
        pub element_revise_val: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.NormalLvBuffCageBaseUserData",
        path = "data/System/ContentsIdSystem/LvBuffCage/Normal/NormalLvBuffCageBaseData.user",
        0x849E4F82 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct NormalLvBuffCageBaseUserData {
        pub param: Vec<NormalLvBuffCageBaseUserDataParam>
    }
}
