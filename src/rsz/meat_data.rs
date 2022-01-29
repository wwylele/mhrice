use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer.MeatGroupInfo",
        0xd256a6ba = 0,
    )]
    #[derive(Debug, Serialize, PartialEq, Eq)]
    pub struct MeatGroupInfo {
        pub slash: u16,
        pub strike: u16,
        pub shell: u16,
        pub fire: u16,
        pub water: u16,
        pub ice: u16,
        pub elect: u16,
        pub dragon: u16,
        pub piyo: u16,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer",
        0x8a1f3742 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatContainer {
        pub meat_group_info: Vec<MeatGroupInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatData",
        0x6ad65290 = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatData {
        pub meat_container: Vec<EnemyMeatContainer>,
    }
}
