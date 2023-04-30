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
