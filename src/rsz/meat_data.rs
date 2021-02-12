use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer.MeatGroupInfo")]
    #[derive(Debug, Serialize, PartialEq, Eq)]
    pub struct MeatGroupInfo {
        pub slash: u16,
        pub strike: u16,
        pub shell: u16,
        pub fire: u16,
        pub water: u16,
        pub thunder: u16,
        pub ice: u16,
        pub dragon: u16,
        pub piyo: u16,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatContainer {
        pub meat_group_info: Vec<MeatGroupInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatData {
        pub meat_container: Vec<EnemyMeatContainer>,
    }
}
