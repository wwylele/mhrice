mod alchemy;
mod anger_data;
mod armor;
mod award;
mod block_move;
mod boss_init_set_data;
mod collision;
mod common;
mod condition_damage_data;
mod condition_damage_preset;
mod custom_buildup;
mod data_base;
mod data_tune;
mod dlc;
mod ecological;
mod facility;
mod item;
mod lot;
mod map;
mod meat_data;
mod monster_list;
mod mystery;
mod otomo;
mod parts_break_data;
mod pos;
mod quest_data;
mod random_quest;
mod scene;
mod skill;
mod weapon;

pub use alchemy::*;
pub use anger_data::*;
pub use armor::*;
pub use award::*;
pub use block_move::*;
pub use boss_init_set_data::*;
pub use collision::*;
pub use common::*;
pub use condition_damage_data::*;
pub use condition_damage_preset::*;
pub use custom_buildup::*;
pub use data_base::*;
pub use data_tune::*;
pub use dlc::*;
pub use ecological::*;
pub use facility::*;
pub use item::*;
pub use lot::*;
pub use map::*;
pub use meat_data::*;
pub use monster_list::*;
pub use mystery::*;
pub use otomo::*;
pub use parts_break_data::*;
pub use pos::*;
pub use quest_data::*;
pub use random_quest::*;
pub use scene::*;
pub use skill::*;
pub use weapon::*;

use crate::file_ext::*;
use crate::hash::*;
use anyhow::{anyhow, bail, Context, Result};
use bitflags::*;
use once_cell::sync::Lazy;
use serde::*;
use std::any::*;
use std::collections::{BTreeMap, HashMap};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::ops::Deref;
use std::rc::*;

/****

Version list:

0        = 3.6.1.0
10_00_02 = 10.0.2.0
10_00_03 = 10.0.3.0
11_00_01 = 11.0.1.0
11_00_02 = 11.0.2.0
12_00_00 = 12.0.0.0
12_00_01 = 12.0.1.1
13_00_00 = 13.0.0.0
-        = 13.0.0.1

****/

#[derive(Debug)]
pub struct Extern {
    pub hash: u32,
    pub path: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeDescriptor {
    pub hash: u32,
    pub crc: u32,
}

#[derive(Debug)]
pub struct Rsz {
    pub roots: Vec<u32>,
    pub extern_slots: HashMap<u32, Extern>,
    pub type_descriptors: Vec<TypeDescriptor>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
enum NodeSlot {
    None,
    Extern(String),
    Instance(AnyRsz),
}

impl NodeSlot {
    fn get_extern(&self) -> Result<&str> {
        match self {
            NodeSlot::Extern(path) => Ok(path),
            _ => bail!("The node slot doesn't contain extern: {:?}", self),
        }
    }

    fn get_instance(&self) -> Result<&AnyRsz> {
        match self {
            NodeSlot::Instance(rsz) => Ok(rsz),
            _ => bail!("The node slot doesn't contain instance: {:?}", self),
        }
    }

    fn take_instance(&mut self) -> Result<AnyRsz> {
        if matches!(self, NodeSlot::Instance(_)) {
            let NodeSlot::Instance(rsz) = std::mem::replace(self, NodeSlot::None) else {
                unreachable!()
            };
            Ok(rsz)
        } else {
            bail!("The node slot doesn't contain instance: {:?}", self)
        }
    }
}

impl Rsz {
    pub fn new<F: Read + Seek>(mut file: F, base: u64) -> Result<Rsz> {
        file.seek(SeekFrom::Start(base))?;
        let magic = file.read_magic()?;
        if &magic != b"RSZ\0" {
            bail!("Wrong magic for RSZ block");
        }

        let version = file.read_u32()?;
        if version != 0x10 {
            bail!("Unexpected RSZ version {}", version);
        }

        let root_count = file.read_u32()?;
        let type_descriptor_count = file.read_u32()?;
        let extern_count = file.read_u32()?;
        let padding = file.read_u32()?;
        if padding != 0 {
            bail!("Unexpected non-zero padding C: {}", padding);
        }
        let type_descriptor_offset = file.read_u64()?;
        let data_offset = file.read_u64()?;
        let string_table_offset = file.read_u64()?;

        let roots = (0..root_count)
            .map(|_| file.read_u32())
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(base + type_descriptor_offset)
            .context("Undiscovered data before type descriptor")?;

        let type_descriptors = (0..type_descriptor_count)
            .map(|_| {
                let hash = file.read_u32()?;
                let crc = file.read_u32()?;
                Ok(TypeDescriptor { hash, crc })
            })
            .collect::<Result<Vec<_>>>()?;

        if type_descriptors.get(0) != Some(&TypeDescriptor { hash: 0, crc: 0 }) {
            bail!("The first type descriptor should be 0")
        }

        file.seek_assert_align_up(base + string_table_offset, 16)
            .context("Undiscovered data before string table")?;

        let extern_slot_info = (0..extern_count)
            .map(|_| {
                let slot = file.read_u32()?;
                let hash = file.read_u32()?;
                let offset = file.read_u64()?;
                Ok((slot, hash, offset))
            })
            .collect::<Result<Vec<_>>>()?;

        let extern_slots = extern_slot_info
            .into_iter()
            .map(|(slot, hash, offset)| {
                file.seek_noop(base + offset)
                    .context("Undiscovered data in string table")?;
                let path = file.read_u16str()?;
                if !path.ends_with(".user") {
                    bail!("Non-USER slot string");
                }
                if hash
                    != type_descriptors
                        .get(usize::try_from(slot)?)
                        .context("slot out of bound")?
                        .hash
                {
                    bail!("slot hash mismatch")
                }
                Ok((slot, Extern { hash, path }))
            })
            .collect::<Result<HashMap<u32, Extern>>>()?;

        file.seek_assert_align_up(base + data_offset, 16)
            .context("Undiscovered data before data")?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        Ok(Rsz {
            roots,
            extern_slots,
            type_descriptors,
            data,
        })
    }

    pub fn deserialize(&self, version_hint: Option<u32>) -> Result<Vec<AnyRsz>> {
        let mut node_buf: Vec<NodeSlot> = vec![NodeSlot::None];
        let mut cursor = Cursor::new(&self.data);
        for (i, &TypeDescriptor { hash, crc }) in self.type_descriptors.iter().enumerate().skip(1) {
            if let Some(slot_extern) = self.extern_slots.get(&u32::try_from(i)?) {
                if slot_extern.hash != hash {
                    bail!("Extern hash mismatch")
                }
                node_buf.push(NodeSlot::Extern(slot_extern.path.clone()));
                continue;
            }

            let pos = cursor.tell().unwrap();
            let type_info = RSZ_TYPE_MAP.get(&hash).with_context(|| {
                let mut buffer = [0; 0x100];
                let read = cursor.read(&mut buffer).unwrap();
                format!(
                    "Unsupported type {:08X} at {:08X}: {:02X?}...",
                    hash,
                    pos,
                    &buffer[0..read]
                )
            })?;
            let version = if type_info.versions.is_empty() {
                version_hint.unwrap_or(0)
            } else {
                *type_info.versions.get(&crc).with_context(|| {
                    format!(
                        "Unknown type CRC {:08X} for type {:08X} ({}) at {:08X}",
                        crc, hash, type_info.symbol, pos
                    )
                })?
            };
            let mut rsz_deserializer = RszDeserializer {
                node_buf: &mut node_buf,
                cursor: &mut cursor,
                version,
            };
            let node =
                (type_info.deserializer)(&mut rsz_deserializer, type_info).with_context(|| {
                    format!(
                        "Error deserializing for type {} at {:08X}, index {}",
                        type_info.symbol, pos, i
                    )
                })?;
            node_buf.push(NodeSlot::Instance(node));
        }

        let result = self
            .roots
            .iter()
            .map(|&root| {
                node_buf
                    .get_mut(usize::try_from(root)?)
                    .context("Root index out of bound")?
                    .take_instance()
            })
            .collect::<Result<Vec<_>>>()?;

        for (i, node) in node_buf.into_iter().enumerate() {
            if let NodeSlot::Instance(node) = node {
                if Rc::strong_count(&node.any) == 1 {
                    bail!("Left over node {} ({})", i, node.symbol())
                }
            }
        }

        let mut leftover = vec![];
        cursor.read_to_end(&mut leftover)?;
        if !leftover.is_empty() {
            bail!("Left over data {leftover:?}");
        }

        Ok(result)
    }

    pub fn deserialize_single_any(&self, version_hint: Option<u32>) -> Result<AnyRsz> {
        let mut result = self.deserialize(version_hint)?;
        if result.len() != 1 {
            bail!("Not a single-valued RSZ");
        }
        Ok(result.pop().unwrap())
    }

    pub fn deserialize_single<T: FromUser>(&self, version_hint: Option<u32>) -> Result<T> {
        let mut result = self.deserialize(version_hint)?;
        if result.len() != 1 {
            bail!("Not a single-valued RSZ");
        }
        FromUser::from_any(result.pop().unwrap())
    }

    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    pub fn verify_crc(&self, crc_mismatches: &mut BTreeMap<&str, u32>, print_all: bool) {
        for td in &self.type_descriptors {
            if let Some(type_info) = RSZ_TYPE_MAP.get(&td.hash) {
                if print_all
                    || (!type_info.versions.contains_key(&td.crc) && !type_info.versions.is_empty())
                {
                    crc_mismatches.insert(type_info.symbol, td.crc);
                }
            }
        }
    }
}

pub struct RszDeserializer<'a, 'b> {
    node_buf: &'a mut [NodeSlot],
    cursor: &'a mut Cursor<&'b Vec<u8>>,
    version: u32,
}

impl<'a, 'b> RszDeserializer<'a, 'b> {
    pub fn get_extern_opt(&mut self) -> Result<Option<&str>> {
        let index = self.cursor.read_u32()?;
        if index == 0 {
            return Ok(None);
        }
        let slot = self
            .node_buf
            .get(usize::try_from(index)?)
            .context("Child index out of bound")?;
        Ok(Some(slot.get_extern()?))
    }

    pub fn get_extern(&mut self) -> Result<&str> {
        self.get_extern_opt()?.context("Null extern")
    }

    pub fn get_child_opt<T: 'static>(&mut self) -> Result<Option<T>> {
        let index = self.cursor.read_u32()?;
        if index == 0 {
            return Ok(None);
        }
        let slot = self
            .node_buf
            .get_mut(usize::try_from(index)?)
            .context("Child index out of bound")?;

        let slot_inner = slot.get_instance()?;
        if Rc::strong_count(&slot_inner.any) != 1 {
            bail!("Shared node")
        }
        let node: Rc<T> = slot_inner.clone().downcast()?;
        slot.take_instance()?;
        Ok(Some(Rc::try_unwrap(node).map_err(|_| ()).unwrap()))
    }

    pub fn get_child<T: 'static>(&mut self) -> Result<T> {
        self.get_child_opt()?.context("Null child")
    }

    pub fn get_child_any_opt(&mut self) -> Result<Option<AnyRsz>> {
        let index = self.cursor.read_u32()?;
        if index == 0 {
            return Ok(None);
        }
        let node = self
            .node_buf
            .get_mut(usize::try_from(index)?)
            .context("Child index out of bound")?
            .get_instance()?
            .clone();
        Ok(Some(node))
    }

    pub fn get_child_any(&mut self) -> Result<AnyRsz> {
        self.get_child_any_opt()?.context("Null child")
    }

    pub fn get_child_rc_opt<T: 'static>(&mut self) -> Result<Option<Rc<T>>> {
        if let Some(child) = self.get_child_any_opt()? {
            Ok(Some(child.downcast()?))
        } else {
            Ok(None)
        }
    }

    pub fn get_child_rc<T: 'static>(&mut self) -> Result<Rc<T>> {
        self.get_child_any()?.downcast()
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

impl<'a, 'b> Read for RszDeserializer<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

#[derive(Clone)]
pub struct AnyRsz {
    any: Rc<dyn Any>,
    type_info: &'static RszTypeInfo,
}

impl Debug for AnyRsz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.type_info.debug)(&*self.any, f)
    }
}

impl AnyRsz {
    pub fn new<T: Any + Serialize + Debug>(v: T, type_info: &'static RszTypeInfo) -> AnyRsz {
        let any = Rc::new(v);
        AnyRsz { any, type_info }
    }

    pub fn downcast<T: Any>(self) -> Result<Rc<T>> {
        let symbol = self.type_info.symbol;
        match self.any.downcast() {
            Ok(b) => Ok(b),
            Err(_) => {
                bail!("Expected {}, found {}", type_name::<T>(), symbol)
            }
        }
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.any.downcast_ref()
    }

    pub fn to_json(&self) -> Result<String> {
        (self.type_info.to_json)(&*self.any)
    }

    pub fn symbol(&self) -> &'static str {
        self.type_info.symbol
    }
}

pub trait FromRsz: Sized {
    fn from_rsz(rsz: &mut RszDeserializer) -> Result<Self>;
    const SYMBOL: &'static str;
    const VERSIONS: &'static [(u32, u32)];
    fn type_hash() -> u32 {
        hash_as_utf8(Self::SYMBOL)
    }
}

pub trait SingletonUser: Sized {
    const PATH: &'static str;
    type RszType: FromUser;
    fn from_rsz(value: Self::RszType) -> Self;
}

trait FieldFromRsz: Sized {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self>;
}

pub struct RszTypeInfo {
    deserializer: fn(&mut RszDeserializer, type_info: &'static RszTypeInfo) -> Result<AnyRsz>,
    to_json: fn(&dyn Any) -> Result<String>,
    debug: fn(&dyn Any, &mut std::fmt::Formatter) -> std::fmt::Result,
    versions: HashMap<u32, u32>,
    pub symbol: &'static str,
}

fn rsz_deserializer<T: 'static + FromRsz + Serialize + Debug>(
    rsz: &mut RszDeserializer,
    type_info: &'static RszTypeInfo,
) -> Result<AnyRsz> {
    Ok(AnyRsz::new(T::from_rsz(rsz)?, type_info))
}

fn rsz_to_json<T: 'static + Serialize>(any: &dyn Any) -> Result<String> {
    serde_json::to_string_pretty(any.downcast_ref::<T>().unwrap())
        .context("Failed to convert to json")
}

fn rsz_debug<T: 'static + Debug>(any: &dyn Any, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(any.downcast_ref::<T>().unwrap(), f)
}

pub trait FromUser: Sized {
    fn from_any(any: AnyRsz) -> Result<Self>;
}

impl<T: 'static + FromRsz> FromUser for T {
    fn from_any(any: AnyRsz) -> Result<Self> {
        Rc::try_unwrap(any.downcast()?).map_err(|_| anyhow!("Shared user node"))
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum ExternUser<T> {
    Path(String),
    Loaded(T),
}

impl<T: FromUser> ExternUser<T> {
    pub fn load<'a>(
        &'a mut self,
        pak: &'_ mut crate::pak::PakReader<impl Read + Seek>,
        version_hint: Option<u32>,
    ) -> Result<&'a mut T> {
        match self {
            ExternUser::Path(path) => {
                let index = pak.find_file(path)?;
                let file = pak.read_file(index)?;
                let user = crate::user::User::new(Cursor::new(file))?;
                *self = ExternUser::Loaded(user.rsz.deserialize_single(version_hint)?);
                if let ExternUser::Loaded(t) = self {
                    Ok(t)
                } else {
                    unreachable!()
                }
            }
            ExternUser::Loaded(t) => Ok(t),
        }
    }

    pub fn unwrap(&self) -> &T {
        match self {
            ExternUser::Path(_) => {
                panic!("ExternUser not loaded")
            }
            ExternUser::Loaded(t) => t,
        }
    }
}

impl<T> FieldFromRsz for ExternUser<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let extern_path = rsz.get_extern()?.to_owned();
        Ok(ExternUser::Path(extern_path))
    }
}

impl<T> FieldFromRsz for Option<ExternUser<T>> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let extern_path = rsz.get_extern_opt()?;
        Ok(extern_path.map(|p| ExternUser::Path(p.to_owned())))
    }
}

pub fn register<T: 'static + FromRsz + Serialize + Debug>(m: &mut HashMap<u32, RszTypeInfo>) {
    let hash = T::type_hash();

    let package = RszTypeInfo {
        deserializer: rsz_deserializer::<T>,
        to_json: rsz_to_json::<T>,
        debug: rsz_debug::<T>,
        versions: T::VERSIONS.iter().copied().collect(),
        symbol: T::SYMBOL,
    };

    let old = m.insert(hash, package);
    if old.is_some() {
        panic!("Multiple type reigstered for the same hash")
    }
}

pub static RSZ_TYPE_MAP: Lazy<HashMap<u32, RszTypeInfo>> = Lazy::new(|| {
    let mut m = HashMap::new();

    macro_rules! r {
        ($($t:ty),*$(,)?) => {
            $(register::<$t>(&mut m);)*
        };
    }

    r!(MeatGroupInfo, EnemyMeatContainer, EnemyMeatData);

    r!(
        StockData,
        ParalyzeDamageData,
        SleepDamageData,
        StunDamageData,
        StaminaDamageData,
        FlashDamageLvData,
        FlashDamageData,
        PoisonDamageData,
        BlastDamageData,
        MarionetteStartDamageData,
        AdjustMeatDownData,
        WaterDamageData,
        FireDamageData,
        IceDamageData,
        ThunderAdjustParamData,
        ThunderDamageData,
        FallTrapDamageData,
        FallQuickSandDamageData,
        FallOtomoTrapDamageData,
        ShockTrapDamageData,
        CaptureDamageData,
        KoyashiDamageData,
        SteelFangData,
        EnemyConditionDamageData,
    );

    r!(EnemyDataBase);

    r!(
        EnemyAngerSeparateData,
        EnemyAngerData,
        EnemyStaminaPointData,
        EnemyStaminaSeparateData,
        NikuEatInfo,
        SetMeatInfo,
        PredatorData,
        EnemyStaminaData
    );

    r!(
        PartsLockParam,
        PartsBreakData,
        ConditionPartsBreakData,
        PartsBreakGroupData,
        PartsLossData,
        ConditionPartsLossData,
        PartsLossGroupData,
        EnemyPartsBreakData
    );

    r!(
        EnemyPartsData,
        DataTunePartsBreakData,
        DataTuneEnemyPartsBreakData,
        DataTunePartsLossData,
        DataTuneEnemyPartsLossData,
        EnablePartsGroup,
        MultiPartsVital,
        EnemyMultiPartsSystemVitalData,
        EnemyMultiPartsVitalData,
        EnemyGimmickVitalData,
        EnemyMarionetteVitalData,
        CharacterContollerTune,
        EnemyDataTune,
    );

    r!(LotInfo, SetInfo, StageInfo, EnemyBossInitSetData);

    r!(
        RequestSetColliderUserData,
        PhysicsUserData,
        EmHitDamageRsData,
        EmHitDamageShapeData,
        Em135_00HitDamageShapeUniqueData,
        DummyHitAttackShapeData,
        EmHitAttackShapeData,
        EmHitAttackRsData,
        EmShellHitAttackRsData,
        HitAttackAppendShapeData,
    );

    r!(
        PresetParalyzeData,
        PresetSleepData,
        PresetStunData,
        PresetFlashData,
        PresetBlastData,
        PresetStaminaData,
        PresetPoison,
        PresetFireData,
        PresetWater,
        PresetIceData,
        PresetThunderData,
        PresetFallTrapData,
        PresetFallQuickSandData,
        PresetFallOtomoTrapData,
        PresetShockTrapData,
        PresetShockOtomoTrapData,
        PresetCaptureData,
        PresetKoyashiData,
        PresetSteelFangData,
        EnemyConditionPresetData,
        BindWireTotalNumParam,
        BindWireStrength,
        BindStartPullAdjustParam,
        MarionetteWireGaugeParam,
        SystemMarionetteStartDamageData,
        HitStopInfo,
        AdjustValueByDirection,
        EnemyMarionetteAttackAdjustInfo,
        EnemyMarionetteAttackRate,
        EnemyMarionetteAttackModeRate,
        MarionetteModePower,
        MarionetteModeReward,
        SystemMarionetteUserData,
    );

    r!(
        NormalQuestDataParam,
        NormalQuestData,
        NormalQuestDataForEnemyParam,
        NormalQuestDataForEnemy,
        VitalRateTableData,
        AttackRateTableData,
        PartsRateTableData,
        OtherRateTableData,
        MultiData,
        MultiRateTableData,
        SystemDifficultyRateData,
        ScaleAndRateData,
        RandomScaleTableData,
        EnemyBossRandomScaleData,
        SizeInfo,
        EnemySizeListData,
        DiscoverEmSetDataParam,
        DiscoverEmSetData,
        MainTargetRewardLotNumDefineUserDataParam,
        MainTargetRewardLotNumDefineUserData,
        QuestDataForRewardUserDataParam,
        QuestDataForRewardUserData,
        RewardIdLotTableUserDataParam,
        RewardIdLotTableUserData,
        HyakuryuQuestDataWaveData,
        HyakuryuQuestData,
        HyakuryuQuestDataTbl,
        MysteryRewardItemUserDataParam,
        MysteryRewardItemUserData,
        SelectQuestServantInfo,
        QuestServantData,
        QuestServantDataList,
        SupplyDataParam,
        SupplyData,
        ProgressCheckerUserDataParam,
        ProgressCheckerUserData,
        ArenaTalismanSkillData,
        ItemWork,
        ArenaPlayer,
        ArenaQuestDataParam,
        ArenaQuestData,
        QuestUnlockListGroup,
        QuestUnlockRelation,
        QuestUnlockByTalkFlag,
        UnlockQuestInfoList,
        QuestUnlockByQuestClear,
        RandomQuestList,
        RandomQuestUnlockByQuestClear,
        QuestUnlockByHuntEnemy,
        QuestUnlockRequestListUserData,
        RankData,
        TimeAttackRewardData,
        TimeAttackRewardUserData,
        QuestGroup,
        TalkConditionQuestListUserData,
        NPCMissionData,
        NPCMissionDataList,
    );

    r!(
        ArmorBaseUserDataParam,
        ArmorBaseUserData,
        ArmorSeriesUserDataParam,
        ArmorSeriesUserData,
        ArmorProductUserDataParam,
        ArmorProductUserData,
        PlOverwearBaseUserDataParam,
        PlOverwearBaseUserData,
        PlOverwearProductUserDataParam,
        PlOverwearProductUserData,
        ArmorBuildupTableUserDataParam,
        ArmorBuildupTableUserData,
        ArmorSeriesPairUserDataParam,
        ArmorSeriesPairUserData,
    );

    r!(
        PlEquipSkillBaseUserData,
        PlEquipSkillBaseUserDataParam,
        PlHyakuryuSkillBaseUserData,
        PlHyakuryuSkillBaseUserDataParam,
        PlHyakuryuSkillRecipeUserData,
        PlHyakuryuSkillRecipeUserDataParam,
        DecorationsBaseUserDataParam,
        DecorationsBaseUserData,
        DecorationsProductUserDataParam,
        DecorationsProductUserData,
        HyakuryuDecoBaseUserDataParam,
        HyakuryuDecoBaseUserData,
        HyakuryuDecoProductUserDataParam,
        HyakuryuDecoProductUserData,
    );

    r!(
        PartData,
        MarionetteData,
        BitSetFlagHabitatType,
        BossMonsterData,
        MonsterListBossData,
        RankInfo,
        EnemyRankData,
        EmSpeciesData,
        SystemEnemyDragonSpeciesData,
    );

    r!(
        AlchemyPatturnUserDataParam,
        AlchemyPatturnUserData,
        AlchemyPlSkillTableUserDataParam,
        AlchemyPlSkillTableUserData,
        GradeWorthTableUserDataParam,
        GradeWorthTableUserData,
        RareTypeTableUserDataParam,
        RareTypeTableUserData,
        SecondSkillLotRateTableUserDataParam,
        SecondSkillLotRateTableUserData,
        SkillGradeLotRateTableUserDataParam,
        SkillGradeLotRateTableUserData,
        SlotNumTableUserDataSkillParam,
        SlotNumTableUserDataSlotParam,
        SlotNumTableUserData,
        SlotWorthTableUserDataParam,
        SlotWorthTableUserData,
    );

    r!(
        ItemUserDataParam,
        ItemUserData,
        NormalLvBuffCageBaseUserDataParam,
        NormalLvBuffCageBaseUserData
    );

    r!(
        MonsterLotTableUserDataParam,
        MonsterLotTableUserData,
        EnemyDropItemInfo,
        EnemyDropItemTableData,
        EnemyDropItemInfoData,
        PartsBreakGroupConditionInfo,
        EnemyPartsBreakRewardInfo,
        EnemyPartsBreakRewardData,
        PartsTypeTextUserDataTextInfo,
        PartsTypeInfo,
        PartsTypeTextUserData,
        HagiPopParameter,
        EnemyPopParameterData,
    );

    r!(
        GreatSwordBaseUserDataParam,
        GreatSwordBaseUserData,
        ShortSwordBaseUserDataParam,
        ShortSwordBaseUserData,
        HammerBaseUserDataParam,
        HammerBaseUserData,
        LanceBaseUserDataParam,
        LanceBaseUserData,
        LongSwordBaseUserDataParam,
        LongSwordBaseUserData,
        SlashAxeBaseUserDataParam,
        SlashAxeBaseUserData,
        GunLanceBaseUserDataParam,
        GunLanceBaseUserData,
        DualBladesBaseUserDataParam,
        DualBladesBaseUserData,
        HornBaseUserDataParam,
        HornBaseUserData,
        InsectGlaiveBaseUserDataParam,
        InsectGlaiveBaseUserData,
        ChargeAxeBaseUserDataParam,
        ChargeAxeBaseUserData,
        LightBowgunBaseUserDataParam,
        LightBowgunBaseUserData,
        HeavyBowgunBaseUserDataParam,
        HeavyBowgunBaseUserData,
        BowBaseUserDataParam,
        BowBaseUserData,
        WeaponProcessUserDataParam,
        WeaponProcessUserData,
        WeaponProductUserDataParam,
        WeaponProductUserData,
        WeaponChangeUserDataParam,
        WeaponChangeUserData,
        WeaponUpdateTreeUserDataParam,
        WeaponUpdateTreeUserData,
        HyakuryuWeaponHyakuryuBuildupUserDataParam,
        HyakuryuWeaponHyakuryuBuildupUserData,
        OverwearWeaponBaseUserDataParam,
        OverwearWeaponBaseUserData,
        OverwearWeaponProductUserDataParam,
        OverwearWeaponProductUserData,
        WeaponChaosCriticalUserDataParam,
        WeaponChaosCriticalUserData,
    );

    r!(
        Folder,
        GameObject,
        Transform,
        WwiseMediaLoader,
        RequestSetGroup,
        RequestSetCollider,
        ViaGui,
        MaterialParam,
        ViaMesh,
        GuiControl,
        GuiPanel,
        Prefab,
        ObstacleFilterInfo,
        ObstacleFilterSet,
        NavigationSurface,
        MeshShape,
        PhysicsFilterInfo,
        Collider,
        Colliders,
        TreeLayer,
        MotionBank,
        DynamicMotionBank,
        Motion,
        CoreHandle,
        BehaviorTree,
        MotionFsm2Layer,
        MotionFsm2,
        CullingInfo,
        WwiseCullingTarget,
        SoundMotionSequencet,
        VolumeOccludee,
        TriggerData,
        WwiseEcTrigger,
        EnvironmentCreatureFindFlagSetter,
        HyakuryuArea,
        EnvironmentCreatureBase,
        CharacterController,
        ChainWind,
        Chain,
        JointParam,
        EyeJointParam,
        IkLookAt2,
        WwiseScreenTarget,
        AnimationCurve,
        AnimationCurve3d,
        StageObjectMaterialController,
        EnvironmentCreatureRemoteSync,
        GuideMessageInfoBase,
        NpcGuideChatInfo,
        HunterNoteInfo,
        StageGuideMessageRequester,
        TargetCameraTarget,
        EnvironmentCreatureSwitchMarionetteModeType,
        WwiseEc056,
        Primitive,
        WwiseEc050,
        Ec054Manager,
        EnvironmentCreatureWalkController,
        Ec054DetectZone,
        WwiseEc054,
        Ec051CirclePathContoroller,
        InteractionShapeOBB,
        AIMapEffector,
        WwiseSphere,
        WaterMesh,
    );

    r!(
        MaskSetting,
        GuiMapScaleDefineData,
        MapHyakuryuLayoutSetting,
        GuiMap07DefineData,
        GuiQuestStart,
        GuiQuestEnd,
        QuestUIManage,
        GuiHoldBoxChange,
        TrialNaviSignToTargetMonster,
        ObjectEffectManager,
        ItemPopLotTableUserDataParam,
        ItemPopLotTableUserData,
        RSCAPIWrapper,
        PopMaterialController,
        PlayerInfluencePopMarker,
        ItemPopBehavior,
        ItemPopVisualController,
        StageRestrictObserver,
        RelicNoteUnlock,
        GuiCommonNpcHeadMessage,
        AccessableDigree,
        NpcFacilityPopMarker,
        TentBehavior,
        CampFindCheck,
        SupplyBoxBehavior,
        WireLongJumpUnlock,
        EnvironmentEffectManager,
        EPVDataElementGroupInfo,
        EffectCustomExternParameter,
        GroupNameParameter,
        EffectManagerLODInfo,
        EPVStandardDataElement,
        EPVStandardData,
        EffectPlayerFadeByDepthParam,
        EffectPlayerFadeByDepthData,
        EnvironmentEffectManagerHelper,
        EPVStandard,
        UniqueBehaviorPop010,
        TentVisualController,
        GimmickPopMarker,
        StageFacilityPopMarker,
        FishingPoint,
        FishingPointBuoy,
        FishSpawnRate,
        FishSpawnGroupInfo,
        FishSpawnData,
        StageSceneLoader,
        StageGridRegister,
        M31IsletArrivalChecker,
        StageAppTagSetter,
        ItemPopIgnoreOtomoGathering,
        TargetScene,
        StageSceneStateController,
        KeyHash,
        StageDemoCameraSceneRequeterRequestData,
        StageDemoCameraSceneRequeter,
        StageAreaMoveSceneRequesterRequestData,
        StageAreaMoveSceneRequester,
        KillCameraConditionRegisterBlockNo,
        QuestAreaMoveRequest,
        QuestPhaseCondition,
        CountCondition,
        AreaMoveInfo,
        QuestAreaMovePopMarker,
        StageObjectStateControllerTargetObject,
        StageObjectStateController,
        OtomoReconSpot,
        StageObjectStateControllerMotionTarget,
        StageObjectMotionController,
        EffectFollowTarget,
        EffectKeyHash,
        StageObjectEffectController,
        WwiseContainerApp,
        MysteryItemPopIgnore,
        PhotoSubject,
        EnvironmentCreatureItem,
        EcPopBehavior,
        EnvironmentCreatureDrop,
        EcMotionSequenceCtrl,
        Ec024MaterialData,
        EcMaterialCurveController,
        WwiseEc025,
        RSCController,
        DamageReceiver,
        Ec009PartsCtrl,
        EnvironmentCreatureLvBuff,
        EcMaterialData,
        Ec009MaterialData,
        Ec009MaterialContoller,
        EcLinearMaterialData,
        EcLinearMaterialController,
        EnvironmentCreatureBuff,
        Ec006PartsCtrl,
        Ec021PartsCtrl,
        Ec001PartsCtrl,
        EnvironmentCreatureWireBuff,
        KakeriWingPartsFlap,
        EnvironmentCreatureTrap,
        Ec037PartsCtrl,
        EnvironmentCreaturePhoto,
        EcMovePathList,
        EnvironmentCreatureLongWire,
        Ec019Ref,
        JumpTypeBehavior,
        Ec019Trajectory,
        FollowTarget,
        StageObjectZoneController,
        EnvironmentCreatureActionControllerEc054Action,
        Ec055Manager,
        Ec055Group,
        EnvironmentCreatureActionControllerEc055Action,
        Ec055JointOffset,
        Ec055CollisionInfoRegister,
        EcMaterialCurve,
        EcMaterialCurveList,
        FieldZone,
        EcQuestSwitch,
        Ec052SearchZone,
        StageObjectTimerHolder,
        Ec053Manager,
        StageObjectPathMoveController,
        WwiseEc053,
        StageObjectPathHolderPathData,
        StageObjectPathHolder,
        ScrollSetting,
        BeltScrollController,
        FieldGimmickDelegate,
        WwiseFieldGimmickTriggerTriggerData,
        WwiseFieldGimmickTrigger,
        Fg023EnemyHitCounter,
        StageObjectCollisionController,
        FieldGimmickRemoteSync,
        Fg023StateMachine,
        WwiseFg023,
        StageObjectDelegate,
        WwiseGimmickHitOverWriter,
        StageEffectCullingController,
        StuckFloorZone,
        Fg028StateMachine,
        Ec054BiteArea,
        DamageStockHolder,
        Fg027StateMachineKeyHash,
        Fg027StateMachine,
        WwiseFieldGimmickStateTriggerStateTrigger,
        WwiseFieldGimmickStateTrigger,
        SnowWwiseGenerator,
        WwiseFg007,
        Fg003ECData,
        Fg003ECDataList,
        Fg003TableData,
        EnvironmentCreatureData,
        PropsBase,
        BreakableStatus,
        ObjectStateInfo,
        ObjectStateSettingsParam,
        ObjectStateSettings,
        PropsDelegate,
        DropObjectBehavior,
        WwiseBreakableObj,
        EnvCreatureLotteryRateInfoAnthill,
        EnvCreatureLotteryDataAnthill,
        EnvCreatureLotteryRateInfoBush,
        EnvCreatureLotteryDataBush,
        HyakuryuBarricadeBehavior,
        MapDetailIconListGPopData,
        GuiMapDetailIconListGPopData,
    );

    r!(
        OtAirouArmorBaseUserDataParam,
        OtAirouArmorBaseUserData,
        OtDogArmorBaseUserDataParam,
        OtDogArmorBaseUserData,
        OtArmorProductUserDataParam,
        OtArmorProductUserData,
        OtWeaponProductUserDataParam,
        OtWeaponProductUserData,
        OtWeaponBaseUserDataParam,
        OtWeaponBaseUserData,
        OtEquipSeriesUserDataParam,
        OtEquipSeriesUserData,
        OtOverwearBaseUserDataParam,
        OtOverwearBaseUserData,
        OtOverwearRecipeUserDataParam,
        OtOverwearRecipeUserData
    );

    r!(
        CustomBuildupBaseUserDataParam,
        CustomBuildupBaseUserData,
        CustomBuildupOpenUserDataParam,
        CustomBuildupOpenUserData,
        CustomBuildupArmorMaterialUserDataParam,
        CustomBuildupArmorMaterialUserData,
        CustomBuildupArmorLotUserDataParam,
        CustomBuildupArmorLotUserData,
        CustomBuildupArmorCategoryLotUserDataParam,
        CustomBuildupArmorCategoryLotUserData,
        CustomBuildupEquipSkillDetailUserDataParam,
        CustomBuildupEquipSkillDetailUserData,
        CustomBuildupWeaponMaterialUserDataParam,
        CustomBuildupWeaponMaterialUserData,
        CustomBuildupWepTableUserDataParam,
        CustomBuildupWepTableUserData,
        CustomBuildupSlotBonusUserDataParam,
        CustomBuildupSlotBonusUserData,
    );

    r!(
        RefTableData,
        RefDifficultyTable,
        RandomMysteryDifficultyRateKindData,
        RandomMysteryDifficultyRateListDataDifficultyDataKinds,
        RandomMysteryDifficultyRateListDataDifficultyData,
        RandomMysteryDifficultyRateListData,
        StageData,
        ShellScaleData,
        NGAppearanceData,
        LotEnemyData,
        RandomMysteryLotEnemyData,
        ReleaseDataParam,
        ReleaseData,
        RandomMysteryMonsterRankReleaseData,
        SpecialMysteryQuestData,
        RandomMysteryRewardBaseParam,
        RandomMysteryRewardBase,
        RandomMysteryRewardSubTargetParam,
        RandomMysteryRewardSubTarget,
        MysteryResearchPointUserDataParam,
        MysteryResearchPointUserData,
    );

    r!(
        ItemShopDisplayUserDataParam,
        ItemShopDisplayUserData,
        ShopFukudamaLotTableUserDataParam,
        ShopFukudamaLotTableUserDataTable,
        ShopFukudamaLotTableUserData,
        ShopFukudamaUserDataParam,
        ShopFukudamaUserData,
        ItemShopLotUserDataParam,
        ItemShopLotUserData,
        MysteryLaboTradeItemUserDataParam,
        MysteryLaboTradeItemUserData,
        ItemMixRecipeUserDataParam,
        ItemMixRecipeUserData,
        BbqConvertUserDataParam,
        BbqConvertUserData,
        ExchangeItemUserDataParam,
        ExchangeItemUserData,
        TradeDustUserDataParam,
        TradeDustUserData,
        TradeFeatureUserDataParam,
        TradeFeatureUserData,
        TradeRareUserDataParam,
        TradeRareUserData,
        TradeUserDataParam,
        TradeUserData,
        OtomoSpyUnitGridUserDataParam,
        OtomoSpyUnitGridUserData,
        OffcutsItemConvertTableParam,
        OffcutsItemConvertTable
    );

    r!(
        CustomBuildupResult,
        SymbolColorData,
        EquipmentInventoryData,
        AddDataInfo,
        DlcAddUserData,
        DlcData,
        DlcListUserData,
        ItemInfo,
        ItemPackParam,
        ItemPackUserData,
        SlcItemInfo,
        SlcItemPackParam,
        ItemPackSaveLinkUserData,
    );

    r!(
        MysteryCoreEffectSettingInfo,
        EnemyMysteryCorePartsData,
        EnemyMysteryMaximumActivityReleaseInfo,
        SystemMysteryUserDataAttackRate,
        SystemMysteryUserDataMotSpeedRate,
        EnemyUniqueMysteryDataConditionDamageData,
        EnemyCameraZoomParam,
        EnemyUniqueMysteryData,
        ShellInfo,
        SuperNovaSpiralShellPresetData,
        TimeSecPresetData,
        RatePresetData,
        PercentagePresetData,
        MaximumActivityReleaseInfoPresetData,
        MaximumToActivityNeedReleaseNumPresetData,
        AttackRatePresetData,
        MotSpeedRatePresetData,
        MysteryDebuffTimeRatePresetData,
        MRConditionDamageResistData,
        SystemMysteryUserData,
        StrengthLevelData,
        OverMysteryBurstData,
        EnemyUniqueOverMysteryData,
    );

    r!(
        AwardUserDataParam,
        AwardUserData,
        AchievementUserDataParam,
        AchievementUserData
    );

    r!(
        EnemyBlockMoveDataInsideMoveInfo,
        EnemyBlockMoveDataBlockBasicInfo,
        EnemyBlockMoveDataLotInfo,
        EnemyBlockMoveDataLotPrevBlockInfo,
        EnemyBlockMoveDataRouteBlockInfo,
        EnemyBlockMoveDataRouteInfo,
        EnemyBlockMoveDataStageInfo,
        EnemyBlockMoveDataCalcRotSpeed,
        EnemyBlockMoveData,
    );

    r!(EnemyEcologicalDataStageInfo, EnemyEcologicalData,);

    r!(
        BlockMovePosSetDataMovePosInfo,
        BlockMovePosSetData,
        EnemyInsideMoveInfo,
        InsideMovePosSetData,
        EnemyBossInitSetInfo,
        BossInitSetPosSetData,
    );

    m.extend(unique_mystery::unique_mystery_type_map());
    m.extend(ec::ec_type_map());
    m.extend(fg::fg_type_map());

    m
});
