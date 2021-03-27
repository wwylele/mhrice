use super::condition_damage_data::*;
use super::*;
use crate::rsz_struct;

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetParalyzeData")]
    #[derive(Debug, Serialize)]
    pub struct PresetParalyzeData {
        #[serde(flatten)]
        pub base: Flatten<ParalyzeDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetSleepData")]
    #[derive(Debug, Serialize)]
    pub struct PresetSleepData {
        #[serde(flatten)]
        pub base: Flatten<SleepDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetStunData")]
    #[derive(Debug, Serialize)]
    pub struct PresetStunData {
        #[serde(flatten)]
        pub base: Flatten<StunDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetStaminaData")]
    #[derive(Debug, Serialize)]
    pub struct PresetStaminaData {
        #[serde(flatten)]
        pub base: Flatten<StaminaDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFlashData")]
    #[derive(Debug, Serialize)]
    pub struct PresetFlashData {
        #[serde(flatten)]
        pub base: Flatten<FlashDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetPoison")]
    #[derive(Debug, Serialize)]
    pub struct PresetPoison {
        #[serde(flatten)]
        pub base: Flatten<PoisonDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetBlastData")]
    #[derive(Debug, Serialize)]
    pub struct PresetBlastData {
        #[serde(flatten)]
        pub base: Flatten<BlastDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetWater")]
    #[derive(Debug, Serialize)]
    pub struct PresetWater {
        #[serde(flatten)]
        pub base: Flatten<WaterDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFireData")]
    #[derive(Debug, Serialize)]
    pub struct PresetFireData {
        #[serde(flatten)]
        pub base: Flatten<FireDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetIceData")]
    #[derive(Debug, Serialize)]
    pub struct PresetIceData {
        #[serde(flatten)]
        pub base: Flatten<IceDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetThunderData")]
    #[derive(Debug, Serialize)]
    pub struct PresetThunderData {
        #[serde(flatten)]
        pub base: Flatten<ThunderDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallTrapData")]
    #[derive(Debug, Serialize)]
    pub struct PresetFallTrapData {
        #[serde(flatten)]
        pub base: Flatten<FallTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallQuickSandData")]
    #[derive(Debug, Serialize)]
    pub struct PresetFallQuickSandData {
        #[serde(flatten)]
        pub base: Flatten<FallQuickSandDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetFallOtomoTrapData")]
    #[derive(Debug, Serialize)]
    pub struct PresetFallOtomoTrapData {
        #[serde(flatten)]
        pub base: Flatten<FallOtomoTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetShockTrapData")]
    #[derive(Debug, Serialize)]
    pub struct PresetShockTrapData {
        #[serde(flatten)]
        pub base: Flatten<ShockTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetShockOtomoTrapData")]
    #[derive(Debug, Serialize)]
    pub struct PresetShockOtomoTrapData {
        #[serde(flatten)]
        pub base: Flatten<ShockTrapDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetCaptureData")]
    #[derive(Debug, Serialize)]
    pub struct PresetCaptureData {
        #[serde(flatten)]
        pub base: Flatten<CaptureDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetKoyashiData")]
    #[derive(Debug, Serialize)]
    pub struct PresetKoyashiData {
        #[serde(flatten)]
        pub base: Flatten<KoyashiDamageData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData.PresetSteelFangData")]
    #[derive(Debug, Serialize)]
    pub struct PresetSteelFangData {
        #[serde(flatten)]
        pub base: Flatten<SteelFangData>,
    }
}

rsz_struct! {
    #[rsz("snow.enemy.EnemyConditionPresetData")]
    #[derive(Debug, Serialize)]
    pub struct EnemyConditionPresetData {
        pub paralyze_data: Vec<PresetParalyzeData>,
        pub sleep_data: Vec<PresetSleepData>,
        pub stun_data: Vec<PresetStunData>,
        pub flash_data: Vec<PresetFlashData>,
        pub blast_data: Vec<PresetBlastData>,
        pub stamina_data: Vec<PresetStaminaData>,
        pub poison_data: Vec<PresetPoison>,
        pub fire_data: Vec<PresetFireData>,
        pub water_data: Vec<PresetWater>,
        pub ice_data: Vec<PresetIceData>,
        pub thunder_data: Vec<PresetThunderData>,
        pub fall_trap_data: Vec<PresetFallTrapData>,
        pub fall_quick_sand_data: Vec<PresetFallQuickSandData>,
        pub fall_otomo_trap_data: Vec<PresetFallOtomoTrapData>,
        pub shock_trap_data: Vec<PresetShockTrapData>,
        pub shock_otomo_trap_data: Vec<PresetShockOtomoTrapData>,
        pub capture_data: Vec<PresetCaptureData>,
        pub koyashi_data: Vec<PresetKoyashiData>,
        pub steel_fang_data: Vec<PresetSteelFangData>,
    }
}

pub trait ConditionDamage<ConditionPresetType>: Sized {
    const UNIQUE_INDEX: usize;

    fn preset_get_inner(preset: &ConditionPresetType) -> &Self;
    fn get_preset_index(&self) -> u32;
    fn preset_from_package(package: &EnemyConditionPresetData) -> &[ConditionPresetType];

    fn or_preset<'a>(&'a self, package: &'a EnemyConditionPresetData) -> Result<&'a Self>
    where
        ConditionPresetType: 'static,
    {
        let index = usize::try_from(self.get_preset_index())?;
        if index > Self::UNIQUE_INDEX {
            bail!("Unknown index");
        }
        Ok(Self::preset_from_package(package)
            .get(index)
            .map(|p| Self::preset_get_inner(p))
            .unwrap_or(self))
    }

    fn verify(package: &EnemyConditionPresetData) -> Result<()> {
        if Self::preset_from_package(package).len() != Self::UNIQUE_INDEX {
            bail!("UNIQUE_INDEX mismatch");
        }
        Ok(())
    }
}

macro_rules! cond {
    ($(($damage:ty, $preset:ty, $preset_member:ident, $unique:expr),)*) => {
        $(impl ConditionDamage<$preset> for $damage {
            const UNIQUE_INDEX: usize = $unique;

            fn preset_get_inner(preset: &$preset) -> &Self {
                &preset.base
            }
            fn get_preset_index(&self) -> u32 {
                self.preset_type
            }
            fn preset_from_package(package: &EnemyConditionPresetData) -> &[$preset] {
                &package.$preset_member
            }
        })*

        impl EnemyConditionPresetData {
            pub fn verify(&self) -> Result<()> {
                $(<$damage as ConditionDamage<$preset>>::verify(self).context(stringify!($damage))?;)*
                Ok(())
            }
        }
    };
}

cond! {
    (ParalyzeDamageData, PresetParalyzeData, paralyze_data, 6),
    (SleepDamageData, PresetSleepData, sleep_data, 4),
    (StunDamageData, PresetStunData, stun_data, 6),
    (FlashDamageData, PresetFlashData, flash_data, 4),
    (BlastDamageData, PresetBlastData, blast_data, 6),
    (StaminaDamageData, PresetStaminaData, stamina_data, 3),
    (PoisonDamageData, PresetPoison, poison_data, 7),
    (FireDamageData, PresetFireData, fire_data, 2),
    (WaterDamageData, PresetWater, water_data, 2),
    (IceDamageData, PresetIceData, ice_data, 2),
    (ThunderDamageData, PresetThunderData, thunder_data, 2),
    (FallTrapDamageData, PresetFallTrapData, fall_trap_data, 1),
    (FallQuickSandDamageData, PresetFallQuickSandData, fall_quick_sand_data, 1),
    (FallOtomoTrapDamageData, PresetFallOtomoTrapData, fall_otomo_trap_data, 1),
    (ShockTrapDamageData, PresetShockTrapData, shock_trap_data, 1),
    (ShockTrapDamageData, PresetShockOtomoTrapData, shock_otomo_trap_data, 1),
    (CaptureDamageData, PresetCaptureData, capture_data, 1),
    (KoyashiDamageData, PresetKoyashiData, koyashi_data, 1),
    (SteelFangData, PresetSteelFangData, steel_fang_data, 2),
}
