mod alchemy;
mod anger_data;
mod armor;
mod boss_init_set_data;
mod collision;
mod common;
mod condition_damage_data;
mod condition_damage_preset;
mod data_base;
mod data_tune;
mod item;
mod lot;
mod map;
mod meat_data;
mod monster_list;
mod otomo;
mod parts_break_data;
mod quest_data;
mod scene;
mod skill;
mod weapon;

pub use alchemy::*;
pub use anger_data::*;
pub use armor::*;
pub use boss_init_set_data::*;
pub use collision::*;
pub use common::*;
pub use condition_damage_data::*;
pub use condition_damage_preset::*;
pub use data_base::*;
pub use data_tune::*;
pub use item::*;
pub use lot::*;
pub use map::*;
pub use meat_data::*;
pub use monster_list::*;
pub use otomo::*;
pub use parts_break_data::*;
pub use quest_data::*;
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

0 = 3.6.1.0
1 = 3.6.1.1
2 = 3.9.0.0
3 = 3.9.1.0

42 = sunbreak demo (temporary)

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

    pub fn deserialize(&self) -> Result<Vec<AnyRsz>> {
        let mut node_buf: Vec<Option<AnyRsz>> = vec![None];
        let mut node_rc_buf: HashMap<u32, Rc<dyn Any>> = HashMap::new();
        let mut cursor = Cursor::new(&self.data);
        for (i, &TypeDescriptor { hash, crc }) in self.type_descriptors.iter().enumerate().skip(1) {
            if let Some(slot_extern) = self.extern_slots.get(&u32::try_from(i)?) {
                if slot_extern.hash != hash {
                    bail!("Extern hash mismatch")
                }
                node_buf.push(Some(AnyRsz::new_extern(slot_extern.path.clone())));
                continue;
            }

            let type_info = RSZ_TYPE_MAP.get(&hash).with_context(|| {
                let mut buffer = [0; 0x100];
                let read = cursor.read(&mut buffer).unwrap();
                format!(
                    "Unsupported type {:08X}: {:02X?}...",
                    hash,
                    &buffer[0..read]
                )
            })?;
            let version = *type_info.versions.get(&crc).with_context(|| {
                format!(
                    "Unknown type CRC {:08X} for type {:08X} ({})",
                    crc, hash, type_info.symbol
                )
            })?;
            let pos = cursor.tell().unwrap();
            let mut rsz_deserializer = RszDeserializer {
                node_buf: &mut node_buf,
                node_rc_buf: &mut node_rc_buf,
                cursor: &mut cursor,
                version,
            };
            let node =
                (type_info.deserializer)(&mut rsz_deserializer, type_info).with_context(|| {
                    format!(
                        "Error deserializing for type {} at {:08X}",
                        type_info.symbol, pos
                    )
                })?;
            node_buf.push(Some(node));
        }

        let result = self
            .roots
            .iter()
            .map(|&root| {
                node_buf
                    .get_mut(usize::try_from(root)?)
                    .context("Root index out of bound")?
                    .take()
                    .context("Empty root")
            })
            .collect::<Result<Vec<_>>>()?;

        if node_buf.into_iter().any(|node| node.is_some()) {
            bail!("Left over node");
        }

        let mut leftover = vec![];
        cursor.read_to_end(&mut leftover)?;
        if !leftover.is_empty() {
            bail!("Left over data");
        }

        Ok(result)
    }

    pub fn deserialize_single<T: 'static>(&self) -> Result<T> {
        let mut result = self.deserialize()?;
        if result.len() != 1 {
            bail!("Not a single-valued RSZ");
        }
        result.pop().unwrap().downcast().context("Type mismatch")
    }

    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    pub fn verify_crc(&self, crc_mismatches: &mut BTreeMap<&str, u32>) {
        for td in &self.type_descriptors {
            if let Some(type_info) = RSZ_TYPE_MAP.get(&td.hash) {
                if !type_info.versions.contains_key(&td.crc) {
                    crc_mismatches.insert(type_info.symbol, td.crc);
                }
            }
        }
    }
}

pub struct RszDeserializer<'a, 'b> {
    node_buf: &'a mut [Option<AnyRsz>],
    node_rc_buf: &'a mut HashMap<u32, Rc<dyn Any>>,
    cursor: &'a mut Cursor<&'b Vec<u8>>,
    version: u32,
}

impl<'a, 'b> RszDeserializer<'a, 'b> {
    fn get_child_inner<T: 'static>(&mut self, index: u32) -> Result<T> {
        let node = self
            .node_buf
            .get_mut(usize::try_from(index)?)
            .context("Child index out of bound")?
            .take()
            .context("None child")?
            .downcast()
            .context("Type mismatch")?;
        Ok(node)
    }

    pub fn get_child<T: 'static>(&mut self) -> Result<T> {
        let index = self.cursor.read_u32()?;
        self.get_child_inner(index)
    }

    pub fn get_child_opt<T: 'static>(&mut self) -> Result<Option<T>> {
        let index = self.cursor.read_u32()?;
        if index == 0 {
            return Ok(None);
        }
        Ok(Some(self.get_child_inner(index)?))
    }

    pub fn get_child_rc<T: 'static>(&mut self) -> Result<Rc<T>> {
        self.get_child_rc_opt()?.context("None child")
    }

    pub fn get_child_rc_opt<T: 'static>(&mut self) -> Result<Option<Rc<T>>> {
        let index = self.cursor.read_u32()?;
        if index == 0 {
            return Ok(None);
        }
        if let Some(child) = self.node_rc_buf.get(&index) {
            child
                .clone()
                .downcast()
                .map_err(|_| anyhow!("Type mismatch"))
                .map(Option::Some)
        } else {
            let child = Rc::new(self.get_child_inner::<T>(index)?);
            self.node_rc_buf.insert(index, child.clone());
            Ok(Some(child))
        }
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

pub struct AnyRsz {
    any: Box<dyn Any>,
    type_info: &'static RszTypeInfo,
}

#[derive(Debug, Serialize)]
pub struct ExternPath(String);

impl Debug for AnyRsz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.type_info.debug)(&*self.any, f)
    }
}

impl AnyRsz {
    pub fn new<T: Any + Serialize + Debug>(v: T, type_info: &'static RszTypeInfo) -> AnyRsz {
        let any = Box::new(v);
        AnyRsz { any, type_info }
    }

    pub fn new_extern(path: String) -> AnyRsz {
        Self::new(ExternPath(path), &*EXTERN_PATH_TYPE_INFO)
    }

    pub fn downcast<T: Any>(self) -> Result<T> {
        let symbol = self.type_info.symbol;
        match self.any.downcast() {
            Ok(b) => Ok(*b),
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
    type RszType: 'static;
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

#[derive(Debug, Serialize, Clone)]
pub enum ExternUser<T> {
    Path(Rc<ExternPath>),
    Loaded(T),
}

impl<T: 'static> ExternUser<T> {
    pub fn load<'a, 'b>(
        &'a mut self,
        pak: &'b mut crate::pak::PakReader<impl Read + Seek>,
    ) -> Result<&'a mut T> {
        match self {
            ExternUser::Path(path) => {
                let index = pak.find_file(&path.0)?;
                let file = pak.read_file(index)?;
                let user = crate::user::User::new(Cursor::new(file))?;
                *self = ExternUser::Loaded(user.rsz.deserialize_single()?);
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

fn extern_path_deserializer(
    _rsz: &mut RszDeserializer,
    _type_info: &'static RszTypeInfo,
) -> Result<AnyRsz> {
    unreachable!()
}

impl<T> FieldFromRsz for ExternUser<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let extern_path = rsz.get_child_rc()?;
        Ok(ExternUser::Path(extern_path))
    }
}

impl<T> FieldFromRsz for Option<ExternUser<T>> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let extern_path = rsz.get_child_rc_opt()?;
        Ok(extern_path.map(ExternUser::Path))
    }
}

static EXTERN_PATH_TYPE_INFO: Lazy<RszTypeInfo> = Lazy::new(|| RszTypeInfo {
    deserializer: extern_path_deserializer,
    to_json: rsz_to_json::<ExternPath>,
    debug: rsz_debug::<ExternPath>,
    versions: HashMap::new(),
    symbol: "FAKE_SYMBOL_ExternPath",
});

pub static RSZ_TYPE_MAP: Lazy<HashMap<u32, RszTypeInfo>> = Lazy::new(|| {
    let mut m = HashMap::new();

    fn register<T: 'static + FromRsz + Serialize + Debug>(m: &mut HashMap<u32, RszTypeInfo>) {
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

    r!(EnemyAngerSeparateData, EnemyAngerData);

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
        MonsterListBossData
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

    r!(ItemUserDataParam, ItemUserData);

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
    );

    r!(
        MaskSetting,
        GuiMapScaleDefineData,
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
    );

    m
});
