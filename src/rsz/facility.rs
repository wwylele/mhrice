use super::*;
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
