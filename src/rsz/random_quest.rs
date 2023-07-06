use super::*;
use crate::{rsz_newtype, rsz_struct};
use serde::*;

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateKindData.RefTableData",
        0x56E82E4A = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RefTableData {
        pub vital_tbl_no: u16,
        pub attack_tbl_no: u16,
        pub parts_tbl_no: u16,
        pub core_tbl_no: u16,
        pub other_tbl_no: u16,
        pub multi_tbl_no: u16,
    }
}

impl EnemyParam for RefTableData {
    fn vital_tbl(&self, _: usize) -> Option<u16> {
        Some(self.vital_tbl_no)
    }

    fn attack_tbl(&self, _: usize) -> Option<u16> {
        Some(self.attack_tbl_no)
    }

    fn parts_tbl(&self, _: usize) -> Option<u16> {
        Some(self.parts_tbl_no)
    }

    fn other_tbl(&self, _: usize) -> Option<u16> {
        Some(self.other_tbl_no)
    }

    fn stamina_tbl(&self, _: usize) -> Option<u8> {
        None
    }

    fn scale(&self, _: usize) -> Option<u8> {
        None
    }

    fn scale_tbl(&self, _: usize) -> Option<i32> {
        None
    }

    fn difficulty(&self, _: usize) -> Option<NandoYuragi> {
        None
    }

    fn boss_multi(&self, _: usize) -> Option<u16> {
        Some(self.multi_tbl_no)
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateKindData.RefDifficultyTable",
        0xCA57CD6D = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RefDifficultyTable {
        pub ref_rate_table: Vec<RefTableData>
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateKindData",
        0xF436E0EE = 15_00_00,
        0xE9DE3A6F = 14_00_00,
        0x81573281 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryDifficultyRateKindData {
        pub ref_table: RefDifficultyTable,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateListData.DifficultyDataKinds",
        0xE7D86AF8 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryDifficultyRateListDataDifficultyDataKinds {
        pub nando_ref_table: ExternUser<RandomMysteryDifficultyRateKindData>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateListData.DifficultyData",
        0x6C80604A = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryDifficultyRateListDataDifficultyData {
        pub nand_kinds_data: Vec<RandomMysteryDifficultyRateListDataDifficultyDataKinds>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryDifficultyRateListData",
        path = "Quest/RandomMystery/RandomMysteryDifficultyRefData.user",
        0xF385B2AE = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryDifficultyRateListData {
        pub sub_nand_adjust_param: i32,
        pub nand_data: [RandomMysteryDifficultyRateListDataDifficultyData; 2]
    }
}

// snow.enemy.EnemyDef.MysteryRank
rsz_newtype! {
    #[rsz_offset(1)]
    #[derive(Debug, Serialize, PartialEq, Eq)]
    #[serde(transparent)]
    pub struct MysteryRank(pub i32);
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData.StageData",
        0x381880F8 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct StageData {
        pub is_map_01: bool,
        pub is_map_02: bool,
        pub is_map_03: bool,
        pub is_map_04: bool,
        pub is_map_05: bool,
        pub is_map_31: bool,
        pub is_map_32: bool,
        pub is_map_09: bool,
        pub is_map_10: bool,
        pub is_map_41: bool,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData.ShellScaleData",
        0x7ee657be = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct ShellScaleData {
        pub release_level: u32,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData.NGAppearanceData",
        0x5B615FA8 = 16_00_00,
        0xAB8FD59A = 15_00_00,
        0x0405560B = 14_00_00,
        0xCE7A98B5 = 13_00_00,
        0xe609a59a = 11_00_01,
        0x36F97D99 = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct NGAppearanceData {
        pub ng_stage_no: i32, // snow.QuestMapManager.MapNoType
        pub ng_em_type: EmTypes,

    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData.SpecialMysteryQuestData")]
    #[derive(Debug, Serialize)]
    pub struct SpecialMysteryQuestData {
        pub mystery_boss_scale_tbl: i32, // snow.enemy.EnemyDef.BossScaleTblType
        pub normal_boss_scale_tbl: i32, // snow.enemy.EnemyDef.BossScaleTblType
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData.LotEnemyData",
        0x391D8DE5 = 16_00_00,
        0x8EB6D943 = 15_00_00,
        0x4DD6D4AF = 14_00_00,
        0x9D4D71DE = 13_00_00,
        0xBA8468A5 = 11_00_01,
        0xFDCAC2BE = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct LotEnemyData {
        pub em_type: EmTypes,
        pub mystery_rank: MysteryRank,
        pub release_level: i32,
        pub normal_rank: MysteryRank,
        pub release_level_normal: i32,
        pub stage_data: StageData,
        pub is_mystery: bool,
        pub is_enable_sub: bool,
        pub is_enable_extra: bool,
        pub is_intrusion: bool,
        pub difficulty_table_type: i32,
        pub difficulty_table_type_extra: i32,
        pub step_scale_table_type: Versioned<i32, 15_00_00>, // snow.enemy.EnemyDef.BossScaleTblType
        pub special_mystery_data: Versioned<SpecialMysteryQuestData, 15_00_00>,
        pub shell_scale_data_list: Vec<ShellScaleData>,
        pub ng_data_list: Vec<NGAppearanceData>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryLotEnemyData",
        path = "Quest/RandomMystery/RandomMysteryLotEnemyListData.user",
        0x65E2357E = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryLotEnemyData {
        pub lot_enemy_list: Vec<LotEnemyData>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryRankReleaseData.Param",
        0x225A508D = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct ReleaseDataParam {
        pub monster_rank: MysteryRank,
        pub release_level: u32,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryRankReleaseData.ReleaseData",
        0x1C52CB2A = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct ReleaseData {
        pub param_data: Vec<ReleaseDataParam>,
    }
}

rsz_struct! {
    #[rsz("snow.quest.RandomMysteryRankReleaseData",
        path = "Quest/RandomMystery/RandomMysteryMonsterRankReleaseData.user",
        0xD758E497 = 11_00_01,
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryMonsterRankReleaseData {
        pub release_level_data: [ReleaseData; 2],
    }
}

rsz_struct! {
    #[rsz("snow.data.RandomMysteryRewardBase.Param")]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryRewardBaseParam {
        pub em_type: EmTypes,
        pub rank: i32, // snow.quest.MysteryQuestRank
        pub base: i32,
    }
}

rsz_struct! {
    #[rsz("snow.data.RandomMysteryRewardBase",
        path = "Quest/RandomMystery/RandomMysteryRewardBase.user",
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryRewardBase {
        pub param_data: Vec<RandomMysteryRewardBaseParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.RandomMysteryRewardSubTarget.Param")]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryRewardSubTargetParam {
        pub em_type: EmTypes,
        pub adjust: f32,
    }
}

rsz_struct! {
    #[rsz("snow.data.RandomMysteryRewardSubTarget",
        path = "Quest/RandomMystery/RandomMysteryRewardSubTarget.user",
    )]
    #[derive(Debug, Serialize)]
    pub struct RandomMysteryRewardSubTarget {
        pub param_data: Vec<RandomMysteryRewardSubTargetParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.MysteryResearchPointUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct MysteryResearchPointUserDataParam {
        pub em_type: EmTypes,
        pub quest_level: QuestLevel,
        pub point: i32,
        pub adjust: f32,
        pub participation_adjust: f32,
    }
}

rsz_struct! {
    #[rsz("snow.data.MysteryResearchPointUserData",
        path = "Quest/RandomMystery/MysteryResearchPointUserData.user",
    )]
    #[derive(Debug, Serialize)]
    pub struct MysteryResearchPointUserData {
        pub param_data: Vec<MysteryResearchPointUserDataParam>
    }
}
