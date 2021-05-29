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
mod meat_data;
mod monster_list;
mod parts_break_data;
mod quest_data;
mod skill;

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
pub use meat_data::*;
pub use monster_list::*;
pub use parts_break_data::*;
pub use quest_data::*;
pub use skill::*;

use crate::file_ext::*;
use crate::hash::*;
use anyhow::*;
use bitflags::*;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::any::*;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::ops::Deref;
use std::rc::*;

#[derive(Debug)]
pub struct SlotString {
    pub slot: u32,
    pub hash: u32,
    pub string: String,
}

#[derive(Debug)]
pub struct Rsz {
    pub roots: Vec<u32>,
    pub slot_strings: Vec<SlotString>,
    pub type_descriptors: Vec<u64>,
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
        let string_count = file.read_u32()?;
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
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        if type_descriptors.get(0) != Some(&0) {
            bail!("The first type descriptor should be 0")
        }

        file.seek_assert_align_up(base + string_table_offset, 16)
            .context("Undiscovered data before string table")?;

        let string_info = (0..string_count)
            .map(|_| {
                let slot = file.read_u32()?;
                let hash = file.read_u32()?;
                let offset = file.read_u64()?;
                Ok((slot, hash, offset))
            })
            .collect::<Result<Vec<_>>>()?;

        let slot_strings = string_info
            .into_iter()
            .map(|(slot, hash, offset)| {
                file.seek_noop(base + offset)
                    .context("Undiscovered data in string table")?;
                let string = file.read_u16str()?;
                if !string.ends_with(".user") {
                    bail!("Non-USER slot string");
                }
                if u64::from(hash)
                    != 0xFFFFFFFF
                        & *type_descriptors
                            .get(usize::try_from(slot)?)
                            .context("slot out of bound")?
                {
                    bail!("slot hash mismatch")
                }
                Ok(SlotString { slot, hash, string })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(base + data_offset, 16)
            .context("Undiscovered data before data")?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        Ok(Rsz {
            roots,
            slot_strings,
            type_descriptors,
            data,
        })
    }

    pub fn deserialize(&self) -> Result<Vec<Box<dyn Any>>> {
        let mut node_buf: Vec<Option<Box<dyn Any>>> = vec![None];
        let mut node_rc_buf: HashMap<u32, Rc<dyn Any>> = HashMap::new();
        let mut cursor = Cursor::new(&self.data);
        for &td in self.type_descriptors.iter().skip(1) {
            let hash = u32::try_from(td & 0xFFFFFFFF)?;
            let (deserializer, versions) = RSZ_TYPE_MAP
                .get(&hash)
                .with_context(|| format!("Unsupported type {:08X}", hash))?;
            let pos = cursor.tell().unwrap();
            let mut rsz_deserializer = RszDeserializer {
                node_buf: &mut node_buf,
                node_rc_buf: &mut node_rc_buf,
                cursor: &mut cursor,
                version: versions.get(&u32::try_from(td >> 32)?).cloned(),
            };
            let node = deserializer(&mut rsz_deserializer).with_context(|| {
                format!("Error deserializing for type {:016X} at {:08X}", td, pos)
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
        Ok(*result
            .pop()
            .unwrap()
            .downcast()
            .map_err(|_| anyhow!("Type mismatch"))?)
    }
}

pub struct RszDeserializer<'a, 'b> {
    node_buf: &'a mut [Option<Box<dyn Any>>],
    node_rc_buf: &'a mut HashMap<u32, Rc<dyn Any>>,
    cursor: &'a mut Cursor<&'b Vec<u8>>,
    version: Option<u32>,
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
            .map_err(|_| anyhow!("Type mismatch"))?;
        Ok(*node)
    }

    pub fn get_child<T: 'static>(&mut self) -> Result<T> {
        let index = self.cursor.read_u32()?;
        self.get_child_inner(index)
    }

    pub fn get_child_rc<T: 'static>(&mut self) -> Result<Rc<T>> {
        let index = self.cursor.read_u32()?;
        if let Some(child) = self.node_rc_buf.get(&index) {
            child
                .clone()
                .downcast()
                .map_err(|_| anyhow!("Type mismatch"))
        } else {
            let child = Rc::new(self.get_child_inner::<T>(index)?);
            self.node_rc_buf.insert(index, child.clone());
            Ok(child)
        }
    }

    pub fn version(&self) -> Option<u32> {
        self.version
    }
}

impl<'a, 'b> Read for RszDeserializer<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
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

trait FieldFromRsz: Sized {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self>;
}

impl FieldFromRsz for bool {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        match rsz.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => bail!("Invalid bool"),
        }
    }
}

impl FieldFromRsz for u8 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.read_u8()
    }
}

impl FieldFromRsz for u16 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(2)?;
        rsz.read_u16()
    }
}

impl FieldFromRsz for u32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_u32()
    }
}

impl FieldFromRsz for u64 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(8)?;
        rsz.read_u64()
    }
}

impl FieldFromRsz for i8 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.read_i8()
    }
}

impl FieldFromRsz for i16 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(2)?;
        rsz.read_i16()
    }
}

impl FieldFromRsz for i32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_i32()
    }
}

impl FieldFromRsz for i64 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(8)?;
        rsz.read_i64()
    }
}

impl FieldFromRsz for f32 {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.read_f32()
    }
}

#[derive(Debug, Serialize)]
pub struct Aligner<const ALIGN: u64>;

impl<const ALIGN: u64> FieldFromRsz for Aligner<ALIGN> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(ALIGN)?;
        Ok(Aligner)
    }
}

impl<T: FromRsz + 'static> FieldFromRsz for T {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.get_child()
    }
}

impl<T: FromRsz + 'static> FieldFromRsz for Rc<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        rsz.get_child_rc()
    }
}

impl<T: FieldFromRsz + 'static> FieldFromRsz for Vec<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let count = rsz.read_u32()?;
        (0..count)
            .map(|_| T::field_from_rsz(rsz))
            .collect::<Result<Vec<_>>>()
    }
}

impl<T: FieldFromRsz + 'static, const N: usize> FieldFromRsz for [T; N] {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        Vec::<T>::field_from_rsz(rsz)?
            .try_into()
            .map_err(|v: Vec<T>| anyhow!("Expected array size {}, found {}", N, v.len()))
    }
}

impl FieldFromRsz for String {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        rsz.cursor.seek_align_up(4)?;
        let count = rsz.read_u32()?;
        let mut utf16 = (0..count)
            .map(|_| rsz.read_u16())
            .collect::<Result<Vec<_>>>()?;
        if utf16.pop() != Some(0) {
            bail!("String not null-terminated");
        }
        Ok(String::from_utf16(&utf16)?)
    }
}

#[derive(Debug, Serialize)]
pub struct Flatten<T>(pub T);

impl<T: FromRsz> FieldFromRsz for Flatten<T> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        Ok(Flatten(T::from_rsz(rsz)?))
    }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Versioned<T, const MIN: u32, const MAX: u32>(pub Option<T>);

impl<T: FieldFromRsz, const MIN: u32, const MAX: u32> FieldFromRsz for Versioned<T, MIN, MAX> {
    fn field_from_rsz(rsz: &mut RszDeserializer) -> Result<Self> {
        let version = rsz.version().context("Unknown version")?;
        Ok(Versioned(if version >= MIN && version <= MAX {
            Some(T::field_from_rsz(rsz)?)
        } else {
            None
        }))
    }
}

impl<T> Deref for Flatten<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! rsz_inner {
    ($rsz:ident, $($field_name:ident : $field_type:ty,)*) => {
        Ok(Self {
            $(
                $field_name: <$field_type>::field_from_rsz($rsz).context(stringify!($field_name))?,
            )*
        })
    }
}

#[macro_export]
macro_rules! rsz_inner_trait {
    (rsz($symbol:literal $(,$vhash:literal=$version:literal)*), $struct_name:ident, $($field_name:ident : $field_type:ty,)*) => {
        impl crate::rsz::FromRsz for $struct_name {
            const SYMBOL: &'static str = $symbol;
            const VERSIONS: &'static [(u32, u32)] = &[$(($vhash, $version)),*];
            fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
                crate::rsz_inner!(rsz, $($field_name : $field_type,)*)
            }
        }
    };

    (rsz(), $struct_name:ident, $($field_name:ident : $field_type:ty,)*) => {
        impl crate::rsz::FieldFromRsz for $struct_name {
            fn field_from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
                crate::rsz_inner!(rsz, $($field_name : $field_type,)*)
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_struct {
    (
        #[rsz($($symbol:literal $(,$vhash:literal=$version:literal)* $(,)?)?)]
        $(#[rsz_version($a:literal, $b:literal)])*
        $(#[$outer_meta:meta])*
        $outer_vis:vis struct $struct_name:ident {
            $(
                $(#[$inner_meta:meta])*
                $inner_vis:vis $field_name:ident : $field_type:ty
            ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])*
        $outer_vis struct $struct_name {
            $(
                $(#[$inner_meta])*
                $inner_vis $field_name : $field_type,
            )*
        }

        crate::rsz_inner_trait!(rsz($($symbol $(,$vhash=$version)*)?), $struct_name, $($field_name : $field_type,)*);
    };
}

#[macro_export]
macro_rules! rsz_enum_arm {
    ($enum_name:ident, $variant:ident, $raw:ident, $value:literal, $end_value:literal) => {
        $enum_name::$variant($raw - $value)
    };
    ($enum_name:ident, $variant:ident, $raw:ident, $value:literal) => {
        $enum_name::$variant
    };
}

#[macro_export]
macro_rules! rsz_enum {
    (
        #[rsz($base:ty)]
        $(#[$outer_meta:meta])*
        $outer_vis:vis enum $enum_name:ident {
            $( $variant:ident $(($field:ty))? = $value:literal $(..= $end_value:literal)? ),*$(,)?
        }
    ) => {
        $(#[$outer_meta])*
        $outer_vis enum $enum_name {
            $( $variant $(($field))?, )*
        }

        impl crate::rsz::FieldFromRsz for $enum_name {
            fn field_from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
                let raw = <$base>::field_from_rsz(rsz)?;
                Ok(#[allow(unreachable_patterns)] match raw {
                    $(
                        $value $(..=$end_value)? =>
                        crate::rsz_enum_arm!($enum_name, $variant, raw, $value $(, $end_value)?),
                    )*
                    x => bail!("Unknown value {} for enum {}", x, stringify!($enum_name))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! rsz_bitflags {
    (
        $(#[$outer_meta:meta])*
        pub struct $name:ident : $base:ty {
            $( const $field_name:ident = $field_value:literal; )*
        }
    ) => {
        bitflags! {
            #[serde(into = "Vec<&'static str>")]
            $(#[$outer_meta])*
            pub struct $name : $base {
                $( const $field_name = $field_value; )*
            }
        }
        impl crate::rsz::FieldFromRsz for $name {
            fn field_from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
                let value = <$base>::field_from_rsz(rsz)?;
                <$name>::from_bits(value).with_context(|| {
                    format!("Unknown bit flag {:08X} for {}", value, stringify!($name))
                })
            }
        }

        impl From<$name> for Vec<&'static str> {
            fn from(v: $name) -> Vec<&'static str> {
                let mut result = vec![];
                $( if v.contains($name::$field_name) {
                    result.push(stringify!($field_name))
                } )*
                result
            }
        }
    }
}

#[macro_export]
macro_rules! rsz_newtype {
    (
        #[rsz_offset($offset:literal)]
        $(#[$outer_meta:meta])*
        $outer_vis:vis struct $name:ident($inner_vis:vis $base:ty);
    ) => (
        $(#[$outer_meta])*
        $outer_vis struct $name($inner_vis $base);

        impl crate::rsz::FieldFromRsz for $name {
            fn field_from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
                let raw = <$base>::field_from_rsz(rsz)?;
                Ok($name(raw + $offset))
            }
        }
    )
}

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum Zero {
        Zero = 0
    }
}

type RszDeserializerPackage = (
    fn(&mut RszDeserializer) -> Result<Box<dyn Any>>,
    HashMap<u32, u32>,
);

static RSZ_TYPE_MAP: Lazy<HashMap<u32, RszDeserializerPackage>> = Lazy::new(|| {
    let mut m = HashMap::new();

    fn register<T: 'static + FromRsz>(m: &mut HashMap<u32, RszDeserializerPackage>) {
        let hash = T::type_hash();
        let versions: HashMap<u32, u32> = T::VERSIONS.iter().copied().collect();
        let old = m.insert(
            hash,
            (
                |rsz| Ok(Box::new(T::from_rsz(rsz)?) as Box<dyn Any>),
                versions,
            ),
        );
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
    );

    r!(
        PartData,
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

    m
});
