use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.ItemShopDisplayUserData.Param",
        0x7A76147D = 13_00_00,
        0x314B7EEA = 12_00_00,
        0x7F0146D8 = 11_00_01,
        0xCFD40504 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemShopDisplayUserDataParam {
        pub id: ItemId,
        pub sort_id: u32,
        pub village_progress: VillageProgress,
        pub hall_progress: HallProgress,
        pub mr_progress: MasterRankProgress,
        pub is_unlock_after_alchemy: bool,
        pub is_bargin_object: bool,
        pub flag_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemShopDisplayUserData",
        path = "data/Facility/ItemShop/VillageShop/ShopDisplayData.user",
        0x81DA6AE6 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemShopDisplayUserData {
        pub param: Vec<ItemShopDisplayUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.ShopFukudamaLotTableUserData.Param",
        0xADA27A0A = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShopFukudamaLotTableUserDataParam {
        pub percentage: Vec<i32>,
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.ShopFukudamaLotTableUserData.Table",
        0x72386953 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShopFukudamaLotTableUserDataTable {
        pub percentage_table: Vec<ShopFukudamaLotTableUserDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.ShopFukudamaLotTableUserData",
        path = "data/Facility/ItemShop/VillageShop/ItemShopFukudamaLotTableUserData.user",
        0x8E88CEBB = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShopFukudamaLotTableUserData {
        pub normal_lot_table: Vec<ShopFukudamaLotTableUserDataTable>,
        pub amiibo_lot_table: Vec<ShopFukudamaLotTableUserDataTable>,
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.ShopFukudamaUserData.Param",
        0x3FC2B801 = 10_00_02,
        0x2F3791A4 = 11_00_01,
        0x0779A339 = 12_00_00,
        0xA9771CAB = 13_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShopFukudamaUserDataParam {
        pub item_id: ItemId,
        pub item_num: i32,
        pub fukudama_num: i32,
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.ShopFukudamaUserData",
        path = "data/Facility/ItemShop/VillageShop/ItemShopFukudamaPrizeUserData.user",
        0x75B36D1D = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShopFukudamaUserData {
        pub no_count_stop_param: Vec<ShopFukudamaUserDataParam>,
        pub count_stop_param: Vec<ShopFukudamaUserDataParam>,
    }
}

// snow.facility.itemShop.ItemLotFunc.LotType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
    pub enum ItemLotFuncLotType {
        Heal = 0,
        Trap = 1,
        Support = 2,
        Special = 3,
        Amiibo = 5,
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.LotUserData.Param",
        0xD55F86CA = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemShopLotUserDataParam {
        pub lot_type: ItemLotFuncLotType,
        pub rank_type: i32, // snow.facility.itemShop.RankTypes
        pub village: VillageProgress,
        pub hall: HallProgress,
        pub mr: MasterRankProgress,
        pub probability_list: [i32; 3],
        pub table_id_list: [u32; 3], // i32 in TDB but doesn't match table id type
    }
}

rsz_struct! {
    #[rsz("snow.facility.itemShop.LotUserData",
        path = "data/Facility/ItemShop/VillageShop/ItemShopLotData.user",
        0xC69CB246 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemShopLotUserData {
        pub param: Vec<ItemShopLotUserDataParam>
    }
}
