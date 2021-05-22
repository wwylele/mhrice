use super::*;
use crate::rsz_struct;
use serde::*;

rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct ViaVec2 {
        #[serde(skip)]
        begin_align: Aligner<16>,
        x: f32,
        y: f32,
        #[serde(skip)]
        endn_align: Aligner<16>,
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData.PartData")]
    #[derive(Debug, Serialize)]
    pub struct PartData {
        pub part: i32,
        pub circle_size: i32,
        pub circle_pos: ViaVec2,
        pub em_meat: i32,
        pub em_meat_group_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.BitSetFlag`1<snow.data.monsterList.HabitatType>")]
    #[derive(Debug, Serialize)]
    pub struct BitSetFlagHabitatType {
        flag: Vec<u32>
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData")]
    #[derive(Debug, Serialize)]
    pub struct BossMonsterData {
        pub em_type: EmTypes,
        pub family_type: i32,
        pub habitat_area: BitSetFlagHabitatType,
        pub is_limit_open_lv: bool,
        pub part_table_data: Vec<PartData>,
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.MonsterListBossData")]
    #[derive(Debug, Serialize)]
    pub struct MonsterListBossData {
        pub data_list: Vec<BossMonsterData>
    }
}
