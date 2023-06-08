use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.ItemShopDisplayUserData.Param",
        0x6EACEE0E = 15_00_00,
        0xB22B10DF = 14_00_00,
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
        0xD5BEB420 = 15_00_00,
        0x7D4782AE = 14_00_00,
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

rsz_struct! {
    #[rsz("snow.facility.mysteryLabo.MysteryLaboTradeItemUserData.Param",
        0x1C96CB47 = 16_00_00,
        0xBA7ED270 = 15_00_00,
        0xC5DE7C0B = 14_00_00,
        0x6B818975 = 13_00_00,
        0x920575DD = 12_00_00,
        0x252C969B = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryLaboTradeItemUserDataParam {
        pub index_no: u32,
        pub item_id: ItemId,
        pub unlock_condition_mystery_research_lv: u32,
        pub unlock_condition_item_id: ItemId,
        pub unlock_condition_enemy_id_1: EmTypes,
        pub unlock_condition_enemy_id_2: EmTypes,
        pub unlock_condition_enemy_id_3: EmTypes,
        pub unlock_condition_enemy_id_4: EmTypes,
        pub unlock_condition_enemy_id_5: EmTypes,
        pub unlock_condition_enemy_hunt_count: u32,
        pub cost: u32,
        pub item_type: i32, // snow.facility.mysteryLabo.ItemFilterTypes
        pub sort_id: i32,
    }
}

rsz_struct! {
    #[rsz("snow.facility.mysteryLabo.MysteryLaboTradeItemUserData",
        path = "data/Facility/MysteryLabo/MysteryLaboTradeItemUserData.user",
        0x36F0016F = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryLaboTradeItemUserData {
        pub param: Vec<MysteryLaboTradeItemUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemMixRecipeUserData.Param",
        0x621839B3 = 15_00_00,
        0xC559BA8D = 14_00_00,
        0x10A910AE = 13_00_00,
        0xD4D26A8B = 12_00_00,
        0x57984480 = 11_00_01,
        0x841CBFD8 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemMixRecipeUserDataParam {
        pub recipe_no: u32,
        pub item_id_list: Vec<ItemId>,
        pub generated_item_id: ItemId,
        pub generated_item_num: u32,
        pub quest_type: i32, // snow.data.ItemMixData.QuestTypes
        pub default_open_flag: bool,
        pub auto_mix_enable_flag: bool,
        pub auto_mix_default: bool,
        pub recipe_tab_type: i32, // snow.data.itemMix.FromListFunc.RecipeTabTypes
    }
}

rsz_struct! {
    #[rsz("snow.data.ItemMixRecipeUserData",
        path = "data/Define/Player/System/ItemMix/ItemMixRecipeData.user",
        0xD5F19AF5 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ItemMixRecipeUserData {
        pub param: Vec<ItemMixRecipeUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.kitchen.BbqConvertUserData.Param",
        0xCF62F94A = 15_00_00,
        0x5D844F0B = 14_00_00,
        0x3C4D8DE4 = 13_00_00,
        0xD42FF9ED = 12_00_00,
        0xDF26D59B = 11_00_01,
        0xC7900744 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct BbqConvertUserDataParam {
        // base: snow.facility.kitchen.BbqConvertData

        pub item_id: ItemId,
        pub sort_id: u32,
        pub filter_type: i32, // snow.facility.kitchen.BbqFunc.FilterTypes
        pub money_cost: u32,
        pub point_cost: u32,
        pub bonus_point: u32,
        pub fix_out_item_id_list: Vec<ItemId>,
        pub fix_out_num_list: Vec<u32>,
        pub random_out_item_num: u32,
        pub table_id: u32,
    }
}

rsz_struct! {
    #[rsz("snow.facility.kitchen.BbqConvertUserData",
        path = "data/Define/Lobby/Facility/Kitchen/BbqConvertUserData.user",
        0xF324E28A = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct BbqConvertUserData {
        pub param: Vec<BbqConvertUserDataParam>
    }
}

// snow.facility.tradeCenter.ExchangeItemTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
    pub enum ExchangeItemTypes {
        Normal = 0,
        Special = 1,
        Random = 2,
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.ExchangeItemUserData.Param",
        0x6755A8B3 = 15_00_00,
        0x38075A13 = 14_00_00,
        0xBE9B9979 = 13_00_00,
        0x77FA345F = 12_00_00,
        0x867138F4 = 11_00_01,
        0x0A24E70A = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ExchangeItemUserDataParam {
        pub index_no: u32,
        pub item_type: ExchangeItemTypes, // snow.facility.tradeCenter.ExchangeItemTypes
        pub item_id: ItemId,
        pub unlock_flag_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_hall: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_mr_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub enemy_id: i32, // snow.enemy.EnemyDef.EnemyTypeIndex
        pub enemy_num: u32,
        pub quest_no: i32,
        pub cost: u32,
        pub rate: u32,
        pub item_num: u32,
        pub sort_id: i32,
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.ExchangeItemUserData",
        path = "data/Facility/TradeCenter/ExchangeItemUserData.user",
        0x6A90E840 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct ExchangeItemUserData {
        pub param: Vec<ExchangeItemUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeDustUserData.Param",
        0x598D33B3 = 15_00_00,
        0xBBC1A617 = 14_00_00,
        0x1A7475C4 = 13_00_00,
        0x933588FC = 12_00_00,
        0xE322B86C = 11_00_01,
        0x0DF264DE = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeDustUserDataParam {
        pub index_no: i32,
        pub rate: u32,
        pub item_id: ItemId,
        pub drop_num: u32,
        pub unlock_flag_mr_village: i32, // snow.data.DataDef.UnlockProgressTypes
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeDustUserData",
        path = "data/Facility/TradeCenter/TradeDustUserData.user",
        0x8FF3787B = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeDustUserData  {
        pub param: Vec<TradeDustUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeFeatureUserData.Param",
        0xEF4A0466 = 15_00_00,
        0x9B0A0BB3 = 14_00_00,
        0x9D884127 = 13_00_00,
        0xE17EBDE3 = 12_00_00,
        0x2FD79B85 = 11_00_01,
        0x084BDD9F = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeFeatureUserDataParam {
        pub index_no: i32,
        pub rate: u32,
        pub item_id: ItemId,
        pub drop_num: u32,
        pub unlock_flag_mr_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub check_have_item: bool,
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeFeatureUserData",
        path = "data/Facility/TradeCenter/TradeFeatureUserData.user",
        0xBF48F9C7 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeFeatureUserData {
        pub param: Vec<TradeFeatureUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeRareUserData.Param",
        0xC096923F = 15_00_00,
        0xF5981CE8 = 14_00_00,
        0xE30F6088 = 13_00_00,
        0x9C10FFDA = 12_00_00,
        0x60665179 = 11_00_01,
        0x3B68D3EB = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeRareUserDataParam {
        pub index_no: i32,
        pub item_id: ItemId,
        pub area: i32, // snow.facility.tradeCenter.TradeAreaTypes
        pub unlock_flag_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_hall: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_mr_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub rate: [u32; 3],
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeRareUserData",
        path = "data/Facility/TradeCenter/TradeRareUserData.user",
        0x17755770 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeRareUserData {
        pub param: Vec<TradeRareUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeUserData.Param",
        0x0D1D13FF = 15_00_00,
        0xE718A57E = 14_00_00,
        0x43C85A56 = 13_00_00,
        0x0B976739 = 12_00_00,
        0x0DAF0A26 = 11_00_01,
        0xB23E7D08 = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeUserDataParam {
        pub index_no: i32,
        pub item_id: ItemId,
        pub area: i32, // snow.facility.tradeCenter.TradeAreaTypes
        pub num: u32,
        pub range: u32,
        pub add_num: [u32; 4],
        pub add_range: [u32; 4],
        pub membership_range: Vec<u32>,
        pub feature_add_rate: f32,
        pub unlock_flag_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_hall: i32, // snow.data.DataDef.UnlockProgressTypes
        pub unlock_flag_mr_village: i32, // snow.data.DataDef.UnlockProgressTypes
        pub sort_id: i32,
    }
}

// snow.data.OtomoSpyUnitDefine.GridIcon
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
    pub enum GridIcon {
        Gathering(i32) = 0..=5,
        GatheringRare(i32) = 6..=11,
        Monster(i32) = 12..=1000
    }
}

rsz_struct! {
    #[rsz("snow.facility.tradeCenter.TradeUserData",
        path = "data/Facility/TradeCenter/TradeUserData.user",
        0x590BC77F = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct TradeUserData {
        pub param: Vec<TradeUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.OtomoSpyUnitGridUserData.Param",
        0x06954A1D = 15_00_00,
        0xDBC63D9D = 14_00_00,
        0xB95F8A6B = 10_00_02,
        0x8F614474 = 11_00_01,
        0xB520EF5C = 12_00_00,
        0x02B74274 = 13_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtomoSpyUnitGridUserDataParam {
        pub unique_id: u32,
        pub grid_id: String,
        pub map_name: i32, // snow.QuestMapManager.MapNoType
        pub rank: RankTypes,
        pub icon: GridIcon,
        pub unlock_village: VillageProgress,
        pub unlock_hall: HallProgress,
        pub unlock_mr_progress: MasterRankProgress,
        pub step_rate: [u32; 5],
        pub item_id: Vec<ItemId>,
        pub item_num: Vec<u32>,
        pub item_rate: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtomoSpyUnitGridUserData",
        path = "data/Facility/OtomoSpyUnit/OtomoSpyUnitGrid.user",
        0x1E8546BF = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtomoSpyUnitGridUserData {
        pub param: Vec<OtomoSpyUnitGridUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.OffcutsItemConvertTable.Param",
        0x1A24C554 = 15_00_00,
        0x21AE25FA = 14_00_00,
        0x5D6924F0 = 10_00_02,
        0x3F6FDC44 = 11_00_01,
        0xE0D3C5D6 = 12_00_00,
        0xE498D44D = 13_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct OffcutsItemConvertTableParam {
        pub base_item_id: ItemId,
        pub convert_item_id: ItemId,
        pub num: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.OffcutsItemConvertTable",
        path = "data/Define/Lobby/Facility/OtomoSmithy/OffcutsItemConvertTable.user",
        0x26EE9DDF = 10_00_02,
    )]
    #[derive(Debug, Serialize)]
    pub struct OffcutsItemConvertTable {
        pub param: Vec<OffcutsItemConvertTableParam>
    }
}
