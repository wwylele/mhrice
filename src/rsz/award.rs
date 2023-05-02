use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.AwardUserData.Param")]
    #[derive(Debug, Serialize, Clone)]
    pub struct AwardUserDataParam {
        pub name: String,
        pub explain: String,
    }
}

rsz_struct! {
    #[rsz("snow.data.AwardUserData",
        path = "data/Define/Common/HunterRecord/AwardUserDataAsset.user"
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct AwardUserData {
        pub param: Vec<AwardUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.AchievementUserData.Param")]
    #[derive(Debug, Serialize, Clone)]
    pub struct AchievementUserDataParam {
        pub name: String,
        pub explain: String,
        pub sort_id: i32,
        pub village_progress: VillageProgress,
        pub hr: u32,
        pub mr: u32,
        pub enemy_type: EmTypes,
        pub enemy_count: u32,
        pub clear_num: u32,
        pub servant_id: i32, // snow.ai.ServantDefine.ServantId
        pub other_condition: i32, // snow.data.AchievementDef.OtherConditions
        pub quest_no: i32,
    }
}

impl AchievementUserDataParam {
    pub fn condition_eq(&self, other: &Self) -> bool {
        self.village_progress == other.village_progress
            && self.hr == other.hr
            && self.mr == other.mr
            && self.enemy_type == other.enemy_type
            && self.enemy_count == other.enemy_count
            && self.clear_num == other.clear_num
            && self.servant_id == other.servant_id
            && self.other_condition == other.other_condition
            && self.quest_no == other.quest_no
    }
}

rsz_struct! {
    #[rsz("snow.data.AchievementUserData",
        path = "data/Define/Common/HunterRecord/AchievementUserDataAsset.user"
    )]
    #[derive(Debug, Serialize, Clone)]
    pub struct AchievementUserData {
        pub param: Vec<AchievementUserDataParam>
    }
}
