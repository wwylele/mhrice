use crate::file_ext::*;
use crate::rsz::FromRsz;
use anyhow::*;
use serde::*;

#[derive(Debug, Serialize)]
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

impl FromRsz for MeatGroupInfo {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatContainer.MeatGroupInfo";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(2)?;
        Ok(MeatGroupInfo {
            slash: rsz.read_u16()?,
            impact: rsz.read_u16()?,
            shot: rsz.read_u16()?,
            fire: rsz.read_u16()?,
            water: rsz.read_u16()?,
            thunder: rsz.read_u16()?,
            ice: rsz.read_u16()?,
            dragon: rsz.read_u16()?,
            dizzy: rsz.read_u16()?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct EnemyMeatContainer {
    pub meat_group_infos: Vec<MeatGroupInfo>,
}

impl FromRsz for EnemyMeatContainer {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatContainer";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let meat_group_infos = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(EnemyMeatContainer { meat_group_infos })
    }
}

#[derive(Debug, Serialize)]
pub struct EnemyMeatData {
    pub meat_containers: Vec<EnemyMeatContainer>,
}

impl FromRsz for EnemyMeatData {
    const SYMBOL: &'static str = "snow.enemy.EnemyMeatData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        rsz.align_up(4)?;
        let count = rsz.read_u32()?;
        let meat_containers = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        Ok(EnemyMeatData { meat_containers })
    }
}

#[derive(Debug, Serialize)]
pub struct StockData {
    default_limit: f32,
    add_limit: f32,
    max_limit: f32,
    sub_value: f32,
    sub_interval: f32,
}

impl FromRsz for StockData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.StockData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_limit = rsz.read_f32()?;
        let add_limit = rsz.read_f32()?;
        let max_limit = rsz.read_f32()?;
        let sub_value = rsz.read_f32()?;
        let sub_interval = rsz.read_f32()?;
        Ok(StockData {
            default_limit,
            add_limit,
            max_limit,
            sub_value,
            sub_interval,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ParalyzeDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for ParalyzeDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.ParalyzeDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(ParalyzeDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SleepDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for SleepDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.SleepDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(SleepDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct StunDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for StunDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.StunDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(StunDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct StaminaDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub sub_stamina: f32,
    pub preset_type: u32,
}

impl FromRsz for StaminaDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.StaminaDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let sub_stamina = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(StaminaDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            sub_stamina,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FlashDamageLvData {
    activate_count: u32,
    active_time: f32,
}

impl FromRsz for FlashDamageLvData {
    const SYMBOL: &'static str = "snow.enemy.EnemyFlashDamageParam.DamageLvData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let activate_count = rsz.read_u32()?;
        let active_time = rsz.read_f32()?;
        Ok(FlashDamageLvData {
            activate_count,
            active_time,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FlashDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub damage_lvs: Vec<FlashDamageLvData>,
    pub ignore_refresh_stance: u32,
    pub max_distance: f32,
    pub min_distance: f32,
    pub angle: f32,
    pub preset_type: u32,
}

impl FromRsz for FlashDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.FlashDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let count = rsz.read_u32()?;
        let damage_lvs = (0..count)
            .map(|_| rsz.get_child())
            .collect::<Result<Vec<_>>>()?;
        let ignore_refresh_stance = rsz.read_u32()?;
        let max_distance = rsz.read_f32()?;
        let min_distance = rsz.read_f32()?;
        let angle = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(FlashDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            damage_lvs,
            ignore_refresh_stance,
            max_distance,
            min_distance,
            angle,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct PoisonDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for PoisonDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.PoisonDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(PoisonDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct BlastDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub blast_damage: f32,
    pub preset_type: u32,
}

impl FromRsz for BlastDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.BlastDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let blast_damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(BlastDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            blast_damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MarionetteStartDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub use_data: u32,
    pub nora_first_limit: f32,
}

impl FromRsz for MarionetteStartDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.MarionetteStartDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let use_data = rsz.read_u32()?;
        let nora_first_limit = rsz.read_f32()?;
        Ok(MarionetteStartDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            use_data,
            nora_first_limit,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct AdjustMeatDownData {
    hard_meat_adjust_value: f32,
    soft_meat_adjust_value: f32,
    judge_meat_value: f32,
}

impl FromRsz for AdjustMeatDownData {
    const SYMBOL: &'static str =
        "snow.enemy.EnemyConditionDamageData.WaterDamageData.AdjustMeatDownData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let hard_meat_adjust_value = rsz.read_f32()?;
        let soft_meat_adjust_value = rsz.read_f32()?;
        let judge_meat_value = rsz.read_f32()?;
        Ok(AdjustMeatDownData {
            hard_meat_adjust_value,
            soft_meat_adjust_value,
            judge_meat_value,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct WaterDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub melee_adjust: AdjustMeatDownData,
    pub shot_adjust: AdjustMeatDownData,
    pub preset_type: u32,
}

impl FromRsz for WaterDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.WaterDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let melee_adjust = rsz.get_child()?;
        let shot_adjust = rsz.get_child()?;
        let preset_type = rsz.read_u32()?;
        Ok(WaterDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            melee_adjust,
            shot_adjust,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FireDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub hit_damage_rate: f32,
    pub preset_type: u32,
}

impl FromRsz for FireDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.FireDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let hit_damage_rate = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(FireDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            hit_damage_rate,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct IceDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub motion_speed_rate: f32,
    pub preset_type: u32,
}

impl FromRsz for IceDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.IceDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let motion_speed_rate = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(IceDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            motion_speed_rate,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ThunderAdjustParamData {
    pub hit_damage_to_stun_rate: f32,
    pub hit_damage_to_stun_max: f32,
    pub hit_damage_to_stun_min: f32,
    pub default_stun_damage_rate: f32,
}

impl FromRsz for ThunderAdjustParamData {
    const SYMBOL: &'static str = "snow.enemy.EnemyThunderDamageParam.AdjustParamData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let hit_damage_to_stun_rate = rsz.read_f32()?;
        let hit_damage_to_stun_max = rsz.read_f32()?;
        let hit_damage_to_stun_min = rsz.read_f32()?;
        let default_stun_damage_rate = rsz.read_f32()?;
        Ok(ThunderAdjustParamData {
            hit_damage_to_stun_rate,
            hit_damage_to_stun_max,
            hit_damage_to_stun_min,
            default_stun_damage_rate,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ThunderDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub stun_meat_adjust: ThunderAdjustParamData,
    pub normal_meat_adjust: ThunderAdjustParamData,
    pub stun_active_limit: u32,
    pub preset_type: u32,
}

impl FromRsz for ThunderDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.ThunderDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let stun_meat_adjust = rsz.get_child()?;
        let normal_meat_adjust = rsz.get_child()?;
        let stun_active_limit = rsz.read_u32()?;
        let preset_type = rsz.read_u32()?;
        Ok(ThunderDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            stun_meat_adjust,
            normal_meat_adjust,
            stun_active_limit,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FallTrapDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub render_offset_y: f32,
    pub render_offset_speed: f32,
    pub render_offset_reset_time: f32,
    pub preset_type: u32,
}

impl FromRsz for FallTrapDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.FallTrapDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let render_offset_y = rsz.read_f32()?;
        let render_offset_speed = rsz.read_f32()?;
        let render_offset_reset_time = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(FallTrapDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            render_offset_y,
            render_offset_speed,
            render_offset_reset_time,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FallQuickSandDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub render_offset_y: f32,
    pub render_offset_speed: f32,
    pub render_offset_reset_time: f32,
    pub preset_type: u32,
}

impl FromRsz for FallQuickSandDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.FallQuickSandDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let render_offset_y = rsz.read_f32()?;
        let render_offset_speed = rsz.read_f32()?;
        let render_offset_reset_time = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(FallQuickSandDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            render_offset_y,
            render_offset_speed,
            render_offset_reset_time,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FallOtomoTrapDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub already_poison_stock_value: f32,
    pub preset_type: u32,
}

impl FromRsz for FallOtomoTrapDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.FallOtomoTrapDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let already_poison_stock_value = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(FallOtomoTrapDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            already_poison_stock_value,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ShockTrapDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for ShockTrapDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.ShockTrapDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(ShockTrapDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CaptureDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for CaptureDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.CaptureDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(CaptureDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct KoyashiDamageData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub preset_type: u32,
}

impl FromRsz for KoyashiDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.KoyashiDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let preset_type = rsz.read_u32()?;
        Ok(KoyashiDamageData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            preset_type,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SteelFangData {
    pub default_stock: StockData,
    pub ride_stock: StockData,
    pub max_stock: f32,
    pub active_time: f32,
    pub sub_active_time: f32,
    pub min_active_time: f32,
    pub add_tired_time: f32,
    pub damage_interval: f32,
    pub damage: f32,
    pub active_limit_count: u32,
    pub preset_type: u32,
    pub is_unique_target_param: u32,
    pub max_distance: f32,
    pub min_distance: f32,
    pub angle: f32,
}

impl FromRsz for SteelFangData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData.SteelFangData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let default_stock = rsz.get_child()?;
        let ride_stock = rsz.get_child()?;
        let max_stock = rsz.read_f32()?;
        let active_time = rsz.read_f32()?;
        let sub_active_time = rsz.read_f32()?;
        let min_active_time = rsz.read_f32()?;
        let add_tired_time = rsz.read_f32()?;
        let damage_interval = rsz.read_f32()?;
        let damage = rsz.read_f32()?;
        let active_limit_count = rsz.read_u32()?;
        let preset_type = rsz.read_u32()?;
        let is_unique_target_param = rsz.read_u32()?;
        let max_distance = rsz.read_f32()?;
        let min_distance = rsz.read_f32()?;
        let angle = rsz.read_f32()?;
        Ok(SteelFangData {
            default_stock,
            ride_stock,
            max_stock,
            active_time,
            sub_active_time,
            min_active_time,
            add_tired_time,
            damage_interval,
            damage,
            active_limit_count,
            preset_type,
            is_unique_target_param,
            max_distance,
            min_distance,
            angle,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct EnemyConditionDamageData {
    pub paralyze: ParalyzeDamageData,
    pub sleep: SleepDamageData,
    pub stun: StunDamageData,
    pub stamina: StaminaDamageData,
    pub flash: FlashDamageData,
    pub poison: PoisonDamageData,
    pub blast: BlastDamageData,
    pub ride: MarionetteStartDamageData,
    pub water: WaterDamageData,
    pub fire: FireDamageData,
    pub ice: IceDamageData,
    pub thunder: ThunderDamageData,
    pub fall_trap: FallTrapDamageData,
    pub fall_quick_sand: FallQuickSandDamageData,
    pub fall_otomo_trap: FallOtomoTrapDamageData,
    pub shock_trap: ShockTrapDamageData,
    pub shock_otomo_trap: ShockTrapDamageData,
    pub capture: CaptureDamageData,
    pub dung: KoyashiDamageData,
    pub steel_fang: SteelFangData,
    pub use_paralyze: u8,
    pub use_sleep: u8,
    pub use_stun: u8,
    pub use_stamina: u8,
    pub use_flash: u8,
    pub use_poison: u8,
    pub use_blast: u8,
    pub use_ride: u8,
    pub use_water: u8,
    pub use_fire: u8,
    pub use_ice: u8,
    pub use_thunder: u8,
    pub use_fall_trap: u8,
    pub use_fall_quick_sand: u8,
    pub use_fall_otomo_trap: u8,
    pub use_shock_trap: u8,
    pub use_shock_otomo_trap: u8,
    pub use_capture: u8,
    pub use_dung: u8,
    pub use_steel_fang: u8,
}

impl FromRsz for EnemyConditionDamageData {
    const SYMBOL: &'static str = "snow.enemy.EnemyConditionDamageData";
    fn from_rsz(rsz: &mut crate::rsz::RszDeserializer) -> Result<Self> {
        let paralyze = rsz.get_child()?;
        let sleep = rsz.get_child()?;
        let stun = rsz.get_child()?;
        let stamina = rsz.get_child()?;
        let flash = rsz.get_child()?;
        let poison = rsz.get_child()?;
        let blast = rsz.get_child()?;
        let ride = rsz.get_child()?;
        let water = rsz.get_child()?;
        let fire = rsz.get_child()?;
        let ice = rsz.get_child()?;
        let thunder = rsz.get_child()?;
        let fall_trap = rsz.get_child()?;
        let fall_quick_sand = rsz.get_child()?;
        let fall_otomo_trap = rsz.get_child()?;
        let shock_trap = rsz.get_child()?;
        let shock_otomo_trap = rsz.get_child()?;
        let capture = rsz.get_child()?;
        let dung = rsz.get_child()?;
        let steel_fang = rsz.get_child()?;
        let use_paralyze = rsz.read_u8()?;
        let use_sleep = rsz.read_u8()?;
        let use_stun = rsz.read_u8()?;
        let use_stamina = rsz.read_u8()?;
        let use_flash = rsz.read_u8()?;
        let use_poison = rsz.read_u8()?;
        let use_blast = rsz.read_u8()?;
        let use_ride = rsz.read_u8()?;
        let use_water = rsz.read_u8()?;
        let use_fire = rsz.read_u8()?;
        let use_ice = rsz.read_u8()?;
        let use_thunder = rsz.read_u8()?;
        let use_fall_trap = rsz.read_u8()?;
        let use_fall_quick_sand = rsz.read_u8()?;
        let use_fall_otomo_trap = rsz.read_u8()?;
        let use_shock_trap = rsz.read_u8()?;
        let use_shock_otomo_trap = rsz.read_u8()?;
        let use_capture = rsz.read_u8()?;
        let use_dung = rsz.read_u8()?;
        let use_steel_fang = rsz.read_u8()?;
        Ok(EnemyConditionDamageData {
            paralyze,
            sleep,
            stun,
            stamina,
            flash,
            poison,
            blast,
            ride,
            water,
            fire,
            ice,
            thunder,
            fall_trap,
            fall_quick_sand,
            fall_otomo_trap,
            shock_trap,
            shock_otomo_trap,
            capture,
            dung,
            steel_fang,
            use_paralyze,
            use_sleep,
            use_stun,
            use_stamina,
            use_flash,
            use_poison,
            use_blast,
            use_ride,
            use_water,
            use_fire,
            use_ice,
            use_thunder,
            use_fall_trap,
            use_fall_quick_sand,
            use_fall_otomo_trap,
            use_shock_trap,
            use_shock_otomo_trap,
            use_capture,
            use_dung,
            use_steel_fang,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Monster {
    pub id: u32,
    pub meat_data: EnemyMeatData,
    pub condition_damage_data: EnemyConditionDamageData,
}

#[derive(Debug, Serialize)]
pub struct Pedia {
    pub monsters: Vec<Monster>,
}
