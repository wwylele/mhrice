use super::common::*;
use super::*;
use crate::{rsz_enum, rsz_struct};
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData.PartData",
        0xBA639AE8 = 13_00_00,
        0xD80AF230 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct PartData {
        pub part: i32, // snow.data.monsterList.PartType
        pub circle_size: i32,
        pub circle_pos: Vec2,
        pub em_meat: i32, // snow.enemy.EnemyDef.Meat
        pub em_meat_group_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.BitSetFlag`1<snow.data.monsterList.HabitatType>",
        0xd8e3d0dc = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct BitSetFlagHabitatType {
        flag: Vec<u32>
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData.MarionetteData",
        0xF54B0204 = 13_00_00,
        0xcae6b96a = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct MarionetteData {
        pub attack_type: i32, // snow.data.monsterList.AttackType
        pub is_button_repeatedly: bool,
        pub is_change_air: bool,
        pub description_id: Guid,

    }
}

// snow.data.monsterList.FamilyType
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum FamilyType {
        Species(i32) = -1..=15,
        MrSpecies(i32) = 16..=100,
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData",
        0xB640A8F6 = 16_00_00,
        0x0671091B = 15_00_00,
        0x4C446EFD = 14_00_00,
        0x4BC19206 = 13_00_00,
        0xf03fc40b = 10_00_02,
        0x32FFAC88 = 11_00_01,
        0x514FBB1C = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct BossMonsterData {
        pub em_type: EmTypes,
        pub family_type: FamilyType,
        pub habitat_area: BitSetFlagHabitatType,
        pub is_limit_open_lv: bool,
        pub part_table_data: Vec<PartData>,
        pub marionette_table_data: Vec<MarionetteData>,
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.MonsterListBossData",
        path = "data/Define/Common/HunterNote/MonsterListBossData_MR.user",
        0x4a9edb4f = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MonsterListBossData {
        pub data_list: Vec<BossMonsterData>
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyRankData.RankInfo",
        0x8C00C584 = 16_00_00,
        0xB8938B08 = 15_00_00,
        0x31838C1A = 14_00_00,
        0x4D959590 = 13_00_00,
        0x1A624800 = 10_00_02,
        0x70EF6657 = 11_00_01,
        0x53149CEF = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct RankInfo {
        pub em_type: EmTypes,
        pub rank: u8,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyRankData",
        path = "enemy/user_data/system_em_rank_data.user",
        0xCEB28157 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct EnemyRankData {
        pub rank_info_list: Vec<RankInfo>
    }
}

// snow.enemy.EnemyDef.EmDragonSpecies
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum EmDragonSpecies {
        BirdDragon = 0,
        FlyingDragon = 1,
        BeastDragon = 2,
        SeaDragon = 3,
        FishDragon = 4,
        FangDragon = 5,
        Max = 6,
        Invalid = 7,
    }
}

// snow.enemy.EnemyDef.EmHabitatSpecies
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
    pub enum EmHabitatSpecies {
        Arial = 0,
        Aquatic = 1,
        Max = 2,
        Invalid = 3,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemyDragonSpeciesData.EmSpeciesData",
        0xC5EC8B96 = 16_00_00,
        0x1EFFF543 = 15_00_00,
        0x5DE3E218 = 14_00_00,
        0x65A4BA42 = 13_00_00,
        0x657FF9F2 = 10_00_02,
        0xE444CBFD = 11_00_01,
        0x3815775C = 12_00_00,
    )]
    #[derive(Debug, Serialize)]
    pub struct EmSpeciesData {
        pub em_type: EmTypes,
        pub em_dragon_species: EmDragonSpecies,
        pub em_habitat_species: EmHabitatSpecies,
        pub is_fang_beast_species: bool
    }
}

rsz_struct! {
    #[rsz("snow.enemy.SystemEnemyDragonSpeciesData",
        path = "enemy/user_data/system_dragon_species_data.user",
        0xD7F46563 = 10_00_02
    )]
    #[derive(Debug, Serialize)]
    pub struct SystemEnemyDragonSpeciesData {
        pub em_species_list: Vec<EmSpeciesData>
    }
}
