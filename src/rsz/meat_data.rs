use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer.MeatGroupInfo")]
    #[derive(Debug, Serialize, PartialEq, Eq)]
    pub struct MeatGroupInfo {
        pub slash: u16,
        pub impact: u16,
        pub shot: u16,
        pub fire: u16,
        pub water: u16,
        pub thunder: u16,
        pub ice: u16,
        pub dragon: u16,
        pub dizzy: u16,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatContainer")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatContainer {
        pub meat_group_infos: Vec<MeatGroupInfo>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyMeatData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyMeatData {
        pub meat_containers: Vec<EnemyMeatContainer>,
    }
}
