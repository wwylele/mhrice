use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

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

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
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

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum IconRank {
        None = 0,
        Great = 1,
        Lv1 = 2,
        Lv2 = 3,
        Lv3 = 4,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum RankTypes {
        Low = 0,
        Upper = 1,
    }
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum ItemGroupTypes {
        Drink = 0,
        Food = 1,
        Others = 2,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ItemUserDataParam {
        pub id: u32,// snow.data.ContentsIdSystem.ItemId
        pub cariable_filter: CarriableFilter,
        pub type_: ItemTypes,
        pub rare: u8, // snow.data.DataDef.RareTypes, 0 = Ra1
        pub pl_max_count: u32,
        pub ot_max_count: u32,
        pub sort_id: u16,
        pub supply: bool,
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
        pub rank_types: RankTypes,
        pub item_group: ItemGroupTypes,
        pub category_worth: u32,
        pub material_category: Vec<i32>, // snow.data.NormalItemData.MaterialCategory, 2 = Category 0
        pub evaluation_value: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemUserData")]
    #[derive(Debug, Serialize)]
    pub struct ItemUserData {
        pub param: Vec<ItemUserDataParam>,
    }
}
