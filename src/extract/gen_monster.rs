use super::gen_common::*;
use super::gen_item::*;
use super::gen_map::*;
use super::gen_quest::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::part_color::PART_COLORS;
use crate::rsz::*;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

// snow.enemy.....MeatType or MeatChangeNo or MeatGroup
const MEAT_TYPES: &[(u32, &[&str])] = &[
    (4, &["Normal", "Glowing", "Broken", "Broken & glowing"]),
    (19, &["Normal", "Guarding", "Broken"]),
    (25, &["Normal", "Stealth"]),
    (27, &["Normal", "Flame", "Blast"]),
    (44, &["Normal", "Mud", "MR normal", "MR mud"]),
    (57, &["Normal", "Charged", "Charged & enraged"]),
    (81, &["Normal", "Charged", "Broken"]),
    (82, &["Normal", "Enraged", "Broken", "Wet", "Wet & enraged"]),
    (
        86,
        &[
            "Normal",
            "Dragon energy",
            "Enraged",
            "Dragon energy & enraged",
        ],
    ),
    (
        89, // only apply to 89_05 but 89_00 doesn't have multi phase anyway
        &[
            "Normal",
            "Concentrated hellfire (1 pole)",
            "Raging hellfire (2 pole)",
        ],
    ),
    (100, &["Normal", "Broken"]),
    (108, &["Normal", "Mud"]),
];

const SPECIFIC_MEAT_TYPES: &[((u32, usize), &[&str])] = &[((81, 3), &["Normal", "Broken"])];

static MEAT_TYPE_MAP: Lazy<HashMap<u32, &[&str]>> =
    Lazy::new(|| HashMap::from_iter(MEAT_TYPES.iter().cloned()));
static SPECIFIC_MEAT_TYPE_MAP: Lazy<HashMap<(u32, usize), &[&str]>> =
    Lazy::new(|| HashMap::from_iter(SPECIFIC_MEAT_TYPES.iter().cloned()));

pub fn gen_mystery_tag(mystery_type: Option<EnemyIndividualType>) -> Option<Box<span<String>>> {
    match mystery_type {
        None | Some(EnemyIndividualType::Normal) => None,
        Some(EnemyIndividualType::Mystery) => {
            Some(html!(<span class="tag is-danger">"Afflicted"</span>))
        }
        Some(EnemyIndividualType::OverMysteryStrengthDefault) => {
            Some(html!(<span class="tag is-danger">"Risen"</span>))
        }
        Some(EnemyIndividualType::OverMysteryStrengthLv1) => {
            Some(html!(<span class="tag is-danger">"Risen (hard)"</span>))
        }
        Some(EnemyIndividualType::OverMysteryStrengthLv2) => {
            Some(html!(<span class="tag is-danger">"Risen lv2"</span>))
        }
        Some(EnemyIndividualType::OverMysteryStrengthLv3) => {
            Some(html!(<span class="tag is-danger">"Risen lv3"</span>))
        }
    }
}

pub fn gen_sub_type_tag(em_type: EmTypes, sub_type: Option<u8>) -> Option<Box<span<String>>> {
    let text = match (em_type, sub_type) {
        (_, None | Some(0)) => None,

        (EmTypes::Em(23), Some(1)) => Some("Sleeping".to_owned()), // observed in game but tdb says always enraged...
        (EmTypes::Em(23), Some(2)) => Some("Special wave boss".to_owned()),
        (EmTypes::Em(23), Some(3)) => Some("Enraged".to_owned()),

        (EmTypes::Em(24), Some(1)) => Some("vs allmother".to_owned()),
        (EmTypes::Em(27), Some(1)) => Some("vs allmother".to_owned()),

        (EmTypes::Em(549 /*37_02*/), Some(1)) => Some("High level".to_owned()),

        (EmTypes::Em(57), Some(1)) => Some("Charged".to_owned()),

        (EmTypes::Em(58), Some(1)) => Some("Emergency".to_owned()),
        (EmTypes::Em(58), Some(2)) => Some("High level".to_owned()),

        (EmTypes::Em(594 /*82_02*/), Some(1)) => Some("High level".to_owned()),

        (EmTypes::Em(89), Some(1)) => Some("vs allmother".to_owned()),

        (EmTypes::Em(96), Some(1)) => Some("ExMultiPartsType".to_owned()),

        (EmTypes::Em(99), Some(1)) => Some("ExStart".to_owned()),
        (EmTypes::Em(99), Some(2)) => Some("Debug".to_owned()),

        (EmTypes::Em(124), Some(1)) => Some("High level".to_owned()),

        (EmTypes::Em(132), Some(1)) => Some("vs allmother".to_owned()),

        (EmTypes::Em(133), Some(1)) => Some("Half afflicted".to_owned()),

        (EmTypes::Em(134), Some(1)) => Some("QuickGoApe".to_owned()), // what

        (EmTypes::Em(136 | 392), Some(1)) => Some("Sleeping".to_owned()),
        (EmTypes::Em(392), Some(2)) => Some("High level".to_owned()),
        (EmTypes::Em(392), Some(3)) => Some("High level sleeping".to_owned()),

        (_, Some(x)) => Some(format!("type{x}")),
    };
    text.map(|t| html!(<span class="tag">{text!("{}", t)}</span>))
}

pub fn gen_monster_tag(
    pedia_ex: &PediaEx,
    em_type: EmTypes,
    is_target: bool,
    short: bool,
    mystery_type: Option<EnemyIndividualType>,
    sub_type: Option<u8>,
) -> Box<div<String>> {
    let (id, is_large) = match em_type {
        EmTypes::Em(id) => (id, true),
        EmTypes::Ems(id) => (id, false),
    };

    let monster_name = (!short).then(|| {
        (|| {
            let name = pedia_ex.monsters[&em_type].name?;
            Some(gen_multi_lang(name))
        })()
        .unwrap_or(html!(<span>{text!("Monster {0:03}_{1:02}",
                                id & 0xFF, id >> 8)}</span>))
    });

    let icon_path = format!(
        "/resources/{}{:03}_{:02}_icon.png",
        if is_large { "em" } else { "ems" },
        id & 0xFF,
        id >> 8
    );

    html!(<div class="mh-quest-monster">
        <a href={format!("/{}/{:03}_{:02}.html",
            if is_large { "monster" } else { "small-monster" }, id & 0xFF, id >> 8)}>
            <img alt="Monster icon" class="mh-quest-list-monster-icon" src=icon_path />
            <span>
                {monster_name}
            </span>
        </a>

        {is_target.then(||html!(<span class="tag is-primary">"Target"</span>))}
        {gen_mystery_tag(mystery_type)}
        {gen_sub_type_tag(em_type, sub_type)}
    </div>)
}

fn gen_extractive_type(extractive_type: ExtractiveType) -> Box<span<String>> {
    match extractive_type {
        ExtractiveType::Red => html!(<span><span class="mh-extract-red"/>"Red"</span>),
        ExtractiveType::White => html!(<span><span class="mh-extract-white"/>"White"</span>),
        ExtractiveType::Orange => html!(<span><span class="mh-extract-orange"/>"Orange"</span>),
        ExtractiveType::None => html!(<span><span class="mh-extract-unknown"/>"None"</span>),
    }
}

fn gen_extractive_type_tag(extractive_type: ExtractiveType) -> &'static str {
    match extractive_type {
        ExtractiveType::Red => "red",
        ExtractiveType::White => "white",
        ExtractiveType::Orange => "orange",
        ExtractiveType::None => "none",
    }
}

fn safe_float(v: f32) -> String {
    let normal = format!("{v}");
    if normal.len() < 5 {
        normal
    } else {
        format!("{v:e}")
    }
}

fn gen_condition_base(data: &ConditionDamageDataBase) -> Vec<Box<dyn TableColumnContent<String>>> {
    vec![
        html!(<td>
            <span class="mh-default-cond">{text!("{} (+{}) → {}",
                data.default_stock.default_limit, data.default_stock.add_limit, data.default_stock.max_limit)}
            </span>
            <span class="mh-ride-cond">{text!("{} (+{}) → {}",
                data.ride_stock.default_limit, data.ride_stock.add_limit, data.ride_stock.max_limit)}
            </span>
        </td>),
        html!(<td>
            <span class="mh-default-cond">{text!("{} / {} sec",
                data.default_stock.sub_value, data.default_stock.sub_interval)}</span>
            <span class="mh-ride-cond">{text!("{} / {} sec",
                data.ride_stock.sub_value, data.ride_stock.sub_interval)}</span>
        </td>),
        html!(<td>{text!("{}", data.max_stock)}</td>),
        html!(<td>{text!("{} sec (-{} sec) → {} sec",
            safe_float(data.active_time), data.sub_active_time, data.min_active_time)}</td>),
        html!(<td>{text!("+{} sec", data.add_tired_time)}</td>),
        html!(<td>{text!("{} / {} sec", data.damage, data.damage_interval)}</td>),
    ]
}

fn gen_disabled(used: ConditionDamageDataUsed, is_preset: Option<bool>) -> String {
    match used {
        ConditionDamageDataUsed::Use => "",
        ConditionDamageDataUsed::NotUse => "mh-disabled ",
    }
    .to_string()
        + match is_preset {
            None => "",
            Some(false) => "mh-no-preset",
            Some(true) => "mh-preset",
        }
}

fn gen_condition_paralyze(
    is_preset: bool,
    data: &ParalyzeDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/para.png" alt="Paralyze" class="mh-small-icon"/>"Paralyze"</td>
            { gen_condition_base(&data.base) }
            <td>  </td>
        </tr>
    )
}

fn gen_condition_sleep(
    is_preset: bool,
    data: &SleepDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/sleep.png" alt="Sleep" class="mh-small-icon"/>"Sleep"</td>
            { gen_condition_base(&data.base) }
            <td>  </td>
        </tr>
    )
}

fn gen_condition_stun(
    is_preset: bool,
    data: &StunDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/stun.png" alt="Stun" class="mh-small-icon"/>"Stun"</td>
            { gen_condition_base(&data.base) }
            <td>  </td>
        </tr>
    )
}

fn gen_condition_stamina(
    is_preset: bool,
    data: &StaminaDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/exhaust.png" alt="Exhaust" class="mh-small-icon"/>"Exhaust"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Stamina reduction = {}", data.sub_stamina)} </td>
        </tr>
    )
}

fn gen_condition_flash(
    pedia_ex: &PediaEx,
    is_preset: bool,
    is_mystery: bool,
    data: &FlashDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    let mut ignore_refresh_stance = vec![];
    if data
        .ignore_refresh_stance
        .contains(StanceStatusFlags::STAND)
    {
        ignore_refresh_stance.push("stand");
    }

    if data.ignore_refresh_stance.contains(StanceStatusFlags::FLY) {
        ignore_refresh_stance.push("fly");
    }

    if data
        .ignore_refresh_stance
        .contains(StanceStatusFlags::DIVING)
    {
        ignore_refresh_stance.push("diving");
    }

    if data.ignore_refresh_stance.contains(StanceStatusFlags::WALL) {
        ignore_refresh_stance.push("wall");
    }

    if data
        .ignore_refresh_stance
        .contains(StanceStatusFlags::CEILING)
    {
        ignore_refresh_stance.push("ceiling");
    }

    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {pedia_ex.items.get(&ItemId::Normal(163)).map(|item| gen_item_icon(item, true))}
            "Flash"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td>
            { data.damage_lvs.iter().map(|lv| {
                html!(<div> {
                    text!("Activate count = {}, Active time = {}",
                    lv.activate_count, lv.active_time)
                } </div>)
            }) }
            <br />
            {text!("Ignore refresh stance = {}", ignore_refresh_stance.join(", "))}
            <br />
            {text!("Distance = {} ~ {}, Angle = {}", data.min_distance, data.max_distance, data.angle)}
            <br />
            </td>
        </tr>
    )
}

fn gen_condition_poison(
    is_preset: bool,
    data: &PoisonDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/poison.png" alt="Poison" class="mh-small-icon"/>"Poison"</td>
            { gen_condition_base(&data.base) }
            <td>  </td>
        </tr>
    )
}

fn gen_condition_blast(
    is_preset: bool,
    data: &BlastDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/blast.png" alt="Blast" class="mh-small-icon"/>"Blast"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Blast damage = {}", data.blast_damage)} </td>
        </tr>
    )
}

fn gen_condition_ride(
    pedia_ex: &PediaEx,
    is_preset: bool,
    data: &MarionetteStartDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {pedia_ex.items.get(&ItemId::Normal(1062)).map(|item| gen_item_icon(item, true))}
            "Ride"
            </td>
            { gen_condition_base(&data.base) }
            <td> {text!("Non-target monster first ride limit = {}", data.nora_first_limit)} </td>
        </tr>
    )
}

fn gen_condition_water(
    is_preset: bool,
    data: &WaterDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/water.png" alt="Water" class="mh-small-icon"/>"Water"</td>
            { gen_condition_base(&data.base) }
            <td>
            {text!("Melee hzv adjust: hard = {}, soft = {}, judge = {}",
                data.slash_strike_adjust.hard_meat_adjust_value,
                data.slash_strike_adjust.soft_meat_adjust_value,
                data.slash_strike_adjust.judge_meat_value
            )}
            <br />
            {text!("Shot hzv adjust: hard = {}, soft = {}, judge = {}",
                data.shell_adjust.hard_meat_adjust_value,
                data.shell_adjust.soft_meat_adjust_value,
                data.shell_adjust.judge_meat_value
            )}
            <br />
            </td>
        </tr>
    )
}

fn gen_condition_fire(
    is_preset: bool,
    data: &FireDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/fire.png" alt="Fire" class="mh-small-icon"/>"Fire"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Hit-damage rate = {}", data.hit_damage_rate)} </td>
        </tr>
    )
}

fn gen_condition_ice(
    is_preset: bool,
    data: &IceDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/ice.png" alt="Ice" class="mh-small-icon"/>"Ice"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Motion speed rate = {}", data.motion_speed_rate)} </td>
        </tr>
    )
}

fn gen_condition_thunder(
    is_preset: bool,
    data: &ThunderDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/thunder.png" alt="Thunder" class="mh-small-icon"/>"Thunder"</td>
            { gen_condition_base(&data.base) }
            <td>
            {text!("Stun hzv adjust: rate = {}, min = {}, max = {}, default = {}",
                data.stun_meat_adjust.hit_damage_to_stun_rate,
                data.stun_meat_adjust.hit_damage_to_stun_min,
                data.stun_meat_adjust.hit_damage_to_stun_max,
                data.stun_meat_adjust.default_stun_damage_rate
            )}
            <br />
            {text!("Normal hzv adjust: rate = {}, min = {}, max = {}, default = {}",
                data.normal_meat_adjust.hit_damage_to_stun_rate,
                data.normal_meat_adjust.hit_damage_to_stun_min,
                data.normal_meat_adjust.hit_damage_to_stun_max,
                data.normal_meat_adjust.default_stun_damage_rate
            )}
            <br />
            {text!("Stun active limit = {}", data.stun_active_limit)}
            </td>
        </tr>
    )
}

fn gen_condition_fall_trap(
    pedia_ex: &PediaEx,
    is_preset: bool,
    is_mystery: bool,
    data: &FallTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {pedia_ex.items.get(&ItemId::Normal(123)).map(|item| gen_item_icon(item, true))}
            "Pitfall trap"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td> </td>
        </tr>
    )
}

fn gen_condition_fall_quick_sand(
    is_preset: bool,
    is_mystery: bool,
    data: &FallQuickSandDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {gen_colored_icon(5, "/resources/item/113", [], true)}
            "Quicksand"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td> </td>
        </tr>
    )
}

fn gen_condition_fall_otomo_trap(
    is_preset: bool,
    is_mystery: bool,
    data: &FallOtomoTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {gen_colored_icon(9, "/resources/item/156", [], true)}
            "Poison purr-ison"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td> {text!("Poison stacking = {}", data.already_poison_stock_value)} </td>
        </tr>
    )
}

fn gen_condition_shock_trap(
    pedia_ex: &PediaEx,
    is_preset: bool,
    is_mystery: bool,
    data: &ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {pedia_ex.items.get(&ItemId::Normal(4)).map(|item| gen_item_icon(item, true))}
            "Shock trap"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td> </td>
        </tr>
    )
}

fn gen_condition_shock_otomo_trap(
    is_preset: bool,
    is_mystery: bool,
    data: &ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {gen_colored_icon(4, "/resources/item/156", [], true)}
            "Shock purr-ison"
            {is_mystery.then(||html!(<img src="/resources/afflicted.png" alt="Afflicted" class="mh-small-icon"/>))}
            </td>
            { gen_condition_base(&data.base) }
            <td></td>
        </tr>
    )
}

fn gen_condition_capture(
    pedia_ex: &PediaEx,
    is_preset: bool,
    data: &CaptureDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>
            {pedia_ex.items.get(&ItemId::Normal(494)).map(|item| gen_item_icon(item, true))}
            "Capture"</td>
            { gen_condition_base(&data.base) }
            <td> </td>
        </tr>
    )
}

fn gen_condition_dung(
    is_preset: bool,
    data: &KoyashiDamageData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/dung.png" alt="Dung" class="mh-small-icon"/>"Dung"</td>
            { gen_condition_base(&data.base) }
            <td></td>
        </tr>
    )
}

fn gen_condition_steel_fang(
    is_preset: bool,
    data: &SteelFangData,
    used: ConditionDamageDataUsed,
) -> Box<tr<String>> {
    html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td><img src="/resources/steelfang.png" alt="Steel fang" class="mh-small-icon"/>"Steel fang"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Active limit = {}, Unique target param = {}",
                data.active_limit_count, data.is_unique_target_param)}
                <br />
                {text!("Distance = {} ~ {}, Angle = {}",
                data.min_distance, data.max_distance, data.angle)}
            </td>
        </tr>
    )
}

fn gen_grouped_reward_table<'a>(
    pedia_ex: &'a PediaEx,
    drop_dictionary: &'a HashMap<EnemyRewardPopTypes, Vec<String>>,
    reward_type: &'a [EnemyRewardPopTypes],
    item: &'a [ItemId],
    num: &'a [u32],
    probability: &'a [u32],
) -> impl Iterator<Item = Box<tr<String>>> + 'a {
    reward_type
        .iter()
        .zip(item.chunks(10))
        .zip(num.chunks(10))
        .zip(probability.chunks(10))
        .filter(|&(((&reward_type, _), _), _)| reward_type != EnemyRewardPopTypes::None)
        .flat_map(move |(((&reward_type, item), num), probability)| {
            let count = item.iter().filter(|&&item| item != ItemId::None).count();
            item.iter()
                .zip(num)
                .zip(probability)
                .filter(|&((&item, _), _)| item != ItemId::None)
                .enumerate()
                .map(move |(i, ((&item, &num), &probability))| {
                    let reward_type: Vec<_> = drop_dictionary
                        .get(&reward_type)
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|name| html!(<div>{text!("{}", name)}</div>))
                        .collect();

                    let group = (i == 0).then(|| html!(<td rowspan={count}>{reward_type}</td>));

                    html!(<tr>
                        {group}
                        <td>{text!("{}x ", num)}
                        <div class="il">{gen_item_label_from_id(item, pedia_ex)}</div></td>
                        <td>{text!("{}%", probability)}</td>
                    </tr>)
                })
        })
}

pub fn gen_lot(
    monster: &Monster,
    em_type: EmTypes,
    rank: QuestRank,
    pedia_ex: &PediaEx<'_>,
) -> Option<Box<section<String>>> {
    let lot = if let Some(lot) = pedia_ex.monster_lot.get(&(em_type, rank)) {
        *lot
    } else {
        return None;
    };

    // TODO: find how to use other pop parameter
    let main_body_tag = monster
        .pop_parameter
        .system_pop_parameters
        .iter()
        .find(|p| p.pop_id == 0)
        .map_or_else(
            || "Main body".to_string(),
            |p| format!("Main body (x{})", p.base_max_hagi_count),
        );

    let mut drop_dictionary = HashMap::new();
    drop_dictionary.insert(EnemyRewardPopTypes::MainBody, vec![main_body_tag]);
    drop_dictionary.insert(
        EnemyRewardPopTypes::PartsLoss1,
        vec!["Severed part A".to_string()],
    );
    drop_dictionary.insert(
        EnemyRewardPopTypes::PartsLoss2,
        vec!["Severed part B".to_string()],
    );
    drop_dictionary.insert(EnemyRewardPopTypes::Unique1, vec!["Special".to_string()]);

    drop_dictionary
        .entry(monster.drop_item.marionette_rewad_pop_type)
        .or_default()
        .push("Riding".to_string());

    for (i, entry) in monster
        .drop_item
        .enemy_drop_item_table_data_tbl
        .iter()
        .enumerate()
    {
        for drop in &entry.enemy_drop_item_info_list {
            drop_dictionary
                .entry(drop.enemy_reward_pop_type)
                .or_default()
                .push(format!("Drop {} - {}%", i, drop.percentage))
        }
    }

    let (header, id) = match rank {
        QuestRank::Low => ("Low rank reward", "s-reward-lr"),
        QuestRank::High => ("High rank reward", "s-reward-hr"),
        QuestRank::Master => ("Master rank reward", "s-reward-mr"),
    };

    Some(html!(<section id={id}>
        <h2 >{text!("{}", header)}</h2>
        <div class="mh-reward-tables">

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Target rewards"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                gen_reward_table(pedia_ex,
                    &lot.target_reward_item_id_list,
                    &lot.target_reward_num_list,
                    &lot.target_reward_probability_list)
            } </tbody>
        </table></div>
        </div>

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Part"</th>
                <th>"Carves"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                gen_grouped_reward_table(pedia_ex,
                    &drop_dictionary,
                    &lot.enemy_reward_type_list,
                    &lot.hagitory_reward_item_id_list,
                    &lot.hagitory_reward_num_list,
                    &lot.hagitory_reward_probability_list)
            } </tbody>
        </table></div>
        </div>

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Capture rewards"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                gen_reward_table(pedia_ex,
                    &lot.capture_reward_item_id_list,
                    &lot.capture_reward_num_list,
                    &lot.capture_reward_probability_list)
            } </tbody>
        </table></div>
        </div>

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Part"</th>
                <th>"Broken part rewards"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                lot.parts_break_list.iter()
                    .zip(lot.parts_break_reward_item_id_list.chunks(10))
                    .zip(lot.parts_break_reward_num_list.chunks(10))
                    .zip(lot.parts_break_reward_probability_list.chunks(10))
                    .filter(|&(((&part, _), _), _)| part != BrokenPartsTypes::None)
                    .flat_map(|(((&part, item), num), probability)| {
                        let count = item.iter().filter(|&&item|item != ItemId::None).count();
                        item.iter().zip(num).zip(probability)
                            .filter(|&((&item, _), _)|item != ItemId::None)
                            .enumerate()
                            .map(move |(i, ((&item, &num), &probability))|{
                                let part_name = if let Some(name) =
                                    pedia_ex.parts_dictionary.get(&(em_type, part)) {
                                    gen_multi_lang(name)
                                } else {
                                    html!(<span>{text!("Unknown {:?}", part)}</span>)
                                };

                                let parts_list = html!(<div class="mh-part-rule"> {
                                    monster.parts_break_reward.iter()
                                        .flat_map(|data|&data.enemy_parts_break_reward_infos)
                                        .filter(|pbr|pbr.broken_parts_type == part)
                                        .map(|pbr| {
                                            let conds = pbr.parts_break_condition_list.iter()
                                                .map(|cond| {
                                                    let part_color = format!("mh-part-group mh-part-{}", cond.parts_group);
                                                    html!(<li>
                                                        <span class=part_color.as_str() />
                                                        {text!("[{}]", cond.parts_group)}
                                                        {text!("(x{})", cond.parts_break_level)}
                                                    </li>)
                                                });
                                            let operator = match pbr.condition_type {
                                                EnemyPartsBreakRewardDataConditionType::All => "All of:",
                                                EnemyPartsBreakRewardDataConditionType::Other => "Any of:"
                                            };
                                            html!(<div>
                                                {text!("{}", operator)}
                                                <ul>
                                                {conds}
                                                </ul>
                                            </div>)
                                        })
                                }</div>);

                                let group = (i == 0).then(|| {
                                    html!(<td rowspan={count}>
                                        {part_name}
                                        {parts_list}
                                    </td>)
                                });

                                html!(<tr>
                                    {group}
                                    <td>{text!("{}x ", num)}
                                    <div class="il">{gen_item_label_from_id(item, pedia_ex)}</div></td>
                                    <td>{text!("{}%", probability)}</td>
                                </tr>)
                            })
                    })
            } </tbody>
        </table></div>
        </div>

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Part"</th>
                <th>"Dropped materials"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                gen_grouped_reward_table(pedia_ex,
                    &drop_dictionary,
                    &lot.drop_reward_type_list,
                    &lot.drop_reward_item_id_list,
                    &lot.drop_reward_num_list,
                    &lot.drop_reward_probability_list)
            } </tbody>
        </table></div>
        </div>

        <div class="mh-reward-box">
        <div class="mh-table"><table>
            <thead><tr>
                <th>"From buddy"</th>
                <th>"Probability"</th>
            </tr></thead>
            <tbody> {
                gen_reward_table(pedia_ex,
                    &lot.otomo_reward_item_id_list,
                    &lot.otomo_reward_num_list,
                    &lot.otomo_reward_probability_list)
            } </tbody>
        </table></div>
        </div>

        </div>
    </section>))
}

pub fn gen_multipart<'a>(
    multipart: impl IntoIterator<Item = (/*is_system*/ bool, usize, &'a EnemyMultiPartsVitalData)>,
) -> Box<div<String>> {
    html!(<div class="mh-table"><table>
    <thead><tr>
        <th>"Index"</th>
        <th>"Part"</th>
        <th>"priority"</th>
        //<th>"enable_last_attack_parts"</th>
        <th>"Attributes"</th>
        //<th>"prio_damage_catagory_flag"</th>
        <th>"HP"</th>
        //<th>"enable_parts_names"</th>
        //<th>"enable_parts_values"</th>
    </tr></thead>
    <tbody>
    {
        multipart.into_iter().map(|(is_system, i, m)| html!(<tr>
            <td> {
                match (is_system, i) {
                    (true, 0) => text!("Apex shutdown (rampage)"),
                    (true, 1) => text!("Hellfire knockdown"),
                    (true, 2) => text!("Apex shutdown (normal quest)"),
                    (true, _) => unreachable!(),
                    (false, i) => text!("Unique{}", i),
                }
            } </td>
            <td>
            {
                if m.enable_parts_data[0].enable_parts == [true; 16] {
                    html!(<span>"All"</span>)
                } else {
                    let parts = ||m.enable_parts_data[0].enable_parts.iter()
                        .enumerate().filter(|&(_, &p)| p)
                        .map(|(part, _)| part);
                    html!(<span>
                        {parts().map(|part|{let part_color = format!("mh-part-group mh-part-{part}");
                            html!(<span class=part_color.as_str()/>)})}
                        {parts().map(|part|html!(<span>{text!("[{}]", part)}</span>))}
                    </span>)
                }
            }</td>
            <td>{text!("{}", m.priority)}</td>
            //<td>{text!("{:?}", m.enable_last_attack_parts)}</td>
            <td>
                <span class="tag">{text!("{:?}", m.use_type)}</span>
                {m.is_enable_hyakuryu.then(||html!(<span class="tag">"Rampage enable"</span>))}
                {m.is_enable_overwrite_down.then(||html!(<span class="tag">"Overwrite topple"</span>))}
                //{m.is_prio_damage_customize.then(||html!(<span class="tag">"Prio damage customize"</span>))}
                {m.is_not_use_difficulty_rate.then(||html!(<span class="tag">"No difficulty rate"</span>))}
                {m.is_multi_rate_ex.then(||html!(<span class="tag">"Multi rate EX"</span>))}
            </td>
            //<td>{text!("{:?}", m.prio_damage_catagory_flag)}</td>
            <td>{
                let s = m.multi_parts_vital_data.iter().map(
                    |p| if p.master_vital == -1 {
                        format!("{}", p.vital)
                    } else {
                        format!("(LR/HR) {}, (MR) {}", p.vital, p.master_vital)
                    }
                ).collect::<Vec<_>>().join(" / ");
                text!("{}", s)
            }</td>
            //<td>{text!("{:?}", m.enable_parts_names)}</td>
            //<td>{text!("{:?}", m.enable_parts_values)}</td>
        </tr>))
    }
    </tbody>
    </table></div>)
}

#[allow(clippy::too_many_arguments)]
pub fn gen_monster(
    hash_store: &HashStore,
    is_large: bool,
    monster: &Monster,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let collider_mapping = &monster.collider_mapping;
    let enemy_parts_break_data_list = &monster.data_tune.enemy_parts_break_data_list;
    let enemy_parts_loss_data_list = &monster.data_tune.enemy_parts_loss_data_list;
    let meat_figure = format!(
        "/resources/{}{:03}_{:02}_meat.png",
        if is_large { "em" } else { "ems" },
        monster.id,
        monster.sub_id,
    );
    let parts_group_figure = format!(
        "/resources/{}{:03}_{:02}_parts_group.png",
        if is_large { "em" } else { "ems" },
        monster.id,
        monster.sub_id,
    );
    let icon = format!(
        "/resources/{}{:03}_{:02}_icon.png",
        if is_large { "em" } else { "ems" },
        monster.id,
        monster.sub_id,
    );

    let monster_em_type = monster.em_type;
    let monster_ex = &pedia_ex.monsters[&monster_em_type];
    let condition_preset = &pedia.condition_preset;

    let explain1 = monster_ex
        .explain1
        .map(|m| html!(<pre> {gen_multi_lang(m)} </pre>));
    let explain2 = monster_ex
        .explain2
        .map(|m| html!(<pre> {gen_multi_lang(m)} </pre>));

    let monster_alias = monster_ex.alias;
    let phase_map = MEAT_TYPE_MAP.get(&monster.id).copied();
    let size_range = pedia_ex.sizes.get(&monster.em_type).copied();

    let mut sections = vec![];
    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            { explain1 }
            { explain2 }
            </section>
        ),
    });

    sections.push(Section { title: "Basic data".to_owned(), content: html!(
        <section id="s-basic">
        <h2 >"Basic data"</h2>
        <div class="mh-kvlist mh-wide">
        <p class="mh-kv"><span>"Base HP"</span>
            <span>{ text!("(LR/HR) {}, (MR) {}", monster.data_tune.base_hp_vital,
            monster.data_tune.master_hp_vital) }</span></p>
        <p class="mh-kv"><span>"Limping threshold"</span>
            <span>{ text!("(village) {}% / (LR) {}% / (HR) {}% / (MR) {}%",
            monster.data_tune.dying_village_hp_vital_rate,
            monster.data_tune.dying_low_level_hp_vital_rate,
            monster.data_tune.dying_high_level_hp_vital_rate,
            monster.data_tune.dying_master_class_hp_vital_rate
        ) }</span></p>
        <p class="mh-kv"><span>"Capturing threshold"</span>
            <span>{ text!("(village) {}% / (LR) {}% / (HR) {}% / (MR) {}%",
            monster.data_tune.capture_village_hp_vital_rate,
            monster.data_tune.capture_low_level_hp_vital_rate,
            monster.data_tune.capture_high_level_hp_vital_rate,
            monster.data_tune.capture_master_level_hp_vital_rate
        ) }</span></p>
        <p class="mh-kv"><span>"Sleep recovering"</span>
            <span>{ text!("{} seconds / recover {}% HP",
            monster.data_tune.self_sleep_time,
            monster.data_tune.self_sleep_recover_hp_vital_rate
        ) }</span></p>
        {size_range.map(|size_range| html!(<p class="mh-kv"><span>"Size"</span>
            <span>
                {text!("{}", size_range.base_size)}
                {(!size_range.no_size_scale).then(||html!(<span>
                    " ("
                    <img class="mh-crown-icon" alt="Small crown" src="/resources/small_crown.png" />
                    {text!("{}, ", size_range.base_size * size_range.small_boarder)}
                    <img class="mh-crown-icon" alt="Silver large crown" src="/resources/large_crown.png" />
                    {text!("{}, ", size_range.base_size * size_range.big_boarder)}
                    <img class="mh-crown-icon" alt="Large crown" src="/resources/king_crown.png" />
                    {text!("{})", size_range.base_size * size_range.king_boarder)}
                </span>))}
            </span>
        </p>))}
        <p class="mh-kv"><span>"Threat level"</span>
        <span> {
            if let Some(rank) = monster_ex.rank {
                text!("{}", rank)
            } else {
                text!("-")
            }
        } </span>
        </p>
        <p class="mh-kv"><span>"Type"</span>
        <span>
        {if let Some(family) = monster_ex.family {
            gen_multi_lang(family)
        } else {
            html!(<span>"-"</span>)
        }}
        {
            if let Some(species) = monster_ex.species {
                let base = if species.is_fang_beast_species {
                    "Fanged beast"
                } else {
                    match species.em_dragon_species {
                        EmDragonSpecies::BirdDragon => "Bird wyvern",
                        EmDragonSpecies::FlyingDragon => "Flying wyvern",
                        EmDragonSpecies::BeastDragon => "Brute wyvern",
                        EmDragonSpecies::SeaDragon => "Leviathan",
                        EmDragonSpecies::FishDragon => "Piscine wyvern",
                        EmDragonSpecies::FangDragon => "Fanged wyvern",
                        EmDragonSpecies::Max => "Max",
                        EmDragonSpecies::Invalid => "Other",
                    }
                };
                let habitat = match species.em_habitat_species {
                    EmHabitatSpecies::Arial => ", Arial",
                    EmHabitatSpecies::Aquatic => ", Aquatic",
                    EmHabitatSpecies::Max => "Max",
                    EmHabitatSpecies::Invalid => ""
                };
                text!(", (internal){}{}", base, habitat)
            } else {
                text!(", (internal)-", )
            }
        } </span>
        </p>
        <p class="mh-kv"><span>"GimmickVital"</span>
            <span>{text!("(S) {} / (M) {} / (L) {} / (KB) {}",
                monster.data_tune.gimmick_vital_data.vital_s,
                monster.data_tune.gimmick_vital_data.vital_m,
                monster.data_tune.gimmick_vital_data.vital_l,
                monster.data_tune.gimmick_vital_data.vital_knock_back
            )}</span>
        </p>
        <p class="mh-kv"><span>"Riding HP"</span>
            <span>{text!("(S) {} / (M) {} / (L) {}",
                monster.data_tune.marionette_vital_data.vital_s,
                monster.data_tune.marionette_vital_data.vital_m,
                monster.data_tune.marionette_vital_data.vital_l
            )}</span>
        </p>
        <p class="mh-kv"><span>"Weight"</span>
            <span>{text!("{:?}", monster.data_tune.weight)}</span>
        </p>
        <p class="mh-kv"><span>"Caution to combat timer"</span>
            <span>{text!("{}", monster.data_base.caution_to_combat_vision_timer)}</span>
        </p>
        <p class="mh-kv"><span>"Caution to normal timer"</span>
            <span>{text!("{}", monster.data_base.caution_to_non_combat_timer)}</span>
        </p>
        <p class="mh-kv"><span>"Combat to normal timer"</span>
            <span>{text!("{}", monster.data_base.combat_to_non_combat_timer)}</span>
        </p>
        <p class="mh-kv"><span>"Enrage threshold"</span>
            <span>{text!("(LR) {} / (HR) {} / (Rampage) {} / (MR) {}",
                monster.anger_data.data_info[0].val,
                monster.anger_data.data_info[1].val,
                monster.anger_data.data_info[2].val,
                monster.anger_data.data_info[3].val)}</span>
        </p>
        <p class="mh-kv"><span>"State time"</span>
            <span>{text!("(enraged) {}sec / (tired) {}sec", monster.anger_data.timer, monster.stamina_data.tired_sec)}</span>
        </p>
        <p class="mh-kv"><span>"State time (rampage)"</span>
            <span>{text!("(enraged) {}sec / (tired) {}sec", monster.anger_data.hyakuryu_cool_timer, monster.stamina_data.hyakuryu_tired_sec)}</span>
        </p>
        <p class="mh-kv"><span>"Motion"</span>
            <span>{text!("(enraged) x{} / (tired) x{}", monster.anger_data.mot_rate, monster.stamina_data.mot_rate)}</span>
        </p>
        <p class="mh-kv"><span>"Attack"</span>
            <span>{text!("(enraged) x{}", monster.anger_data.atk_rate)}</span>
        </p>
        <p class="mh-kv"><span>"Defense"</span>
            <span>{text!("(enraged) x{}", monster.anger_data.def_rate)}</span>
        </p>
        // <p class="mh-kv"><span>"Enrage compensation rate"</span>
        //     <span>{text!("{:?}", monster.anger_data.compensation_rate)}</span>
        // </p>
        // <p class="mh-kv"><span>"Enrage compensation rate (rampage)"</span>
        //     <span>{text!("{:?}", monster.anger_data.hyakuryu_compensation_rate)}</span>
        // </p>
        <p class="mh-kv"><span>"Enrage add staying time"</span>
            <span>{text!("{}sec", monster.anger_data.anger_stay_add_sec)}</span>
        </p>
        <p class="mh-kv"><span>"life_area_timer_rate"</span>
            <span>{text!("{}", monster.anger_data.life_area_timer_rate)}</span>
        </p>
        </div>
        </section>
    )});

    sections.push(Section { title: "Quests".to_owned(), content: html!(
        <section id="s-quest">
        <h2 >"Quests"</h2>
        <div>
            <input type="checkbox" id="mh-non-target-check"/>
            <label for="mh-non-target-check">"Display non-target"</label>
        </div>
        <div>
            <input type="checkbox" id="mh-quest-detail-check"/>
            <label for="mh-quest-detail-check">"More detailed stat"</label>
        </div>
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Quest"</th>
                <th>"Size"</th>
                <th>"HP"</th>
                <th>"Attack"</th>
                <th>"Parts"</th>
                <th class="mh-quest-detail">"Defense"</th>
                <th class="mh-quest-detail">"Element"</th>
                <th class="mh-quest-detail">"Stun"</th>
                <th class="mh-quest-detail">"Exhaust"</th>
                <th class="mh-quest-detail">"Ride"</th>
                <th class="mh-quest-detail">"Paralyze"</th>
                <th class="mh-quest-detail">"Sleep"</th>
                <th class="mh-quest-detail">"Stamina"</th>
            </tr></thead>
            <tbody> {
                pedia_ex.quests.values().flat_map(|quest| {
                    quest.param.boss_em_type.iter().copied().enumerate().filter(
                        |&(_, em_type)|em_type == monster_em_type
                    )
                    .map(move |(i, em_type)|{
                        let is_target = quest.param.has_target(em_type);
                        let mystery = quest.enemy_param
                            .and_then(|p|p.individual_type.get(i).cloned());
                        let class = if !is_target {
                            "mh-non-target"
                        } else {
                            ""
                        };
                        let sub_type = quest.enemy_param.and_then(|p|p.sub_type(i));
                        let sub_type = gen_sub_type_tag(em_type, sub_type);
                        html!(<tr class={class}>
                            <td> {
                                gen_quest_tag(quest, true, is_target, mystery, sub_type)
                            } </td>
                            { gen_quest_monster_data(quest.enemy_param, Some(em_type), i, &pedia.difficulty_rate, pedia_ex) }
                        </tr>)
                    })
                })
            }
            {
                if let Some(discovery) = monster_ex.discovery {
                    vec![
                        html!(<tr><td>{text!("Village tour ({})",
                            discovery.cond_village.display().unwrap_or_default())}
                            {gen_sub_type_tag(monster_em_type, Some(discovery.sub_type[0]))}
                            </td>{
                            gen_quest_monster_data(Some(discovery),
                                Some(monster_em_type), 0, &pedia.difficulty_rate, pedia_ex)
                        }</tr>),
                        html!(<tr><td>{text!("Low rank tour ({})",
                            discovery.cond_low.display().unwrap_or_default())}
                            {gen_sub_type_tag(monster_em_type, Some(discovery.sub_type[1]))}
                            </td>{
                            gen_quest_monster_data(Some(discovery),
                                Some(monster_em_type), 1, &pedia.difficulty_rate, pedia_ex)
                        }</tr>),
                        html!(<tr><td>{text!("High rank tour ({})",
                            discovery.cond_high.display().unwrap_or_default())}
                            {gen_sub_type_tag(monster_em_type, Some(discovery.sub_type[2]))}
                            </td>{
                            gen_quest_monster_data(Some(discovery),
                                Some(monster_em_type), 2, &pedia.difficulty_rate, pedia_ex)
                        }</tr>),
                        html!(<tr><td>{text!("Master rank tour ({})",
                            discovery.cond_master.display().unwrap_or_default())}
                            {gen_sub_type_tag(monster_em_type, Some(discovery.sub_type[3]))}
                            </td>{
                            gen_quest_monster_data(Some(discovery),
                                Some(monster_em_type), 3, &pedia.difficulty_rate, pedia_ex)
                        }</tr>)
                    ]
                } else {
                    vec![]
                }
            }
            </tbody>
        </table></div>
        </section>
    ) });

    if let (Some(random_quest), Some(rank_release)) =
        (monster_ex.random_quest, &pedia.random_mystery_rank_release)
    {
        let diff_table = format!(
            "/quest/anomaly_difficulty_0_{}.html",
            random_quest.difficulty_table_type
        );
        let extra_diff_table = format!(
            "/quest/anomaly_difficulty_1_{}.html",
            random_quest.difficulty_table_type_extra
        );
        sections.push(Section {
            title: "Anomaly investigation".to_owned(),
            content: html!(
            <section id="s-anomaly">
            <h2>"Anomaly investigation"</h2>
            <div class="mh-kvlist">
            <p class="mh-kv"><span>"Appear as afflicted at"</span>
            {
                let release = rank_release.release_level_data[0].param_data.iter().find(
                    |p|p.monster_rank == random_quest.mystery_rank
                ).map(|p|format!("(Lv{})", p.release_level)).unwrap_or_default();
                let rank = (random_quest.mystery_rank.0 != 12).then(||
                    text!("A{} {}", random_quest.mystery_rank.0, release));
                let level = (random_quest.release_level != -1).then(||
                    text!("Lv{}", random_quest.release_level));
                let or = (rank.is_some() && level.is_some()).then(||text!(" Or "));
                let none = (rank.is_none() && level.is_none()).then(||text!("None"));
                html!(<span>{rank}{or}{level}{none}</span>)
            }
            </p>
            <p class="mh-kv"><span>"Appear as normal at"</span>
            {
                let release = rank_release.release_level_data[1].param_data.iter().find(
                    |p|p.monster_rank == random_quest.normal_rank
                ).map(|p|format!("(Lv{})", p.release_level)).unwrap_or_default();
                let rank = (random_quest.normal_rank.0 != 12).then(||
                    text!("\"A{}\" {}", random_quest.normal_rank.0, release));
                let level = (random_quest.release_level_normal != -1).then(||
                    text!("Lv{}", random_quest.release_level_normal));
                let or = (rank.is_some() && level.is_some()).then(||text!(" Or "));
                let none = (rank.is_none() && level.is_none()).then(||text!("None"));
                html!(<span>{rank}{or}{level}{none}</span>)
            }
            </p>
            <p class="mh-kv"><span>"Main target"</span>
                <span>{text!("{}", random_quest.is_mystery)}</span>
            </p>
            <p class="mh-kv"><span>"Sub target"</span>
                <span>{text!("{}", random_quest.is_enable_sub)}</span>
            </p>
            <p class="mh-kv"><span>"Extra monster"</span>
                <span>{text!("{}", random_quest.is_enable_extra)}</span>
            </p>
            <p class="mh-kv"><span>"Intrusion"</span>
                <span>{text!("{}", random_quest.is_intrusion)}</span>
            </p>
            <p class="mh-kv"><span>"Stats table as target"</span>
                <span><a href={diff_table.as_str()}>{
                text!("Table {}", random_quest.difficulty_table_type)}</a></span>
            </p>
            <p class="mh-kv"><span>"Stats table as extra"</span>
                <span><a href={extra_diff_table.as_str()}>{
                text!("Table {}", random_quest.difficulty_table_type_extra)}</a></span>
            </p>
            <p class="mh-kv"><span>"Base research point"</span>
                <span>{
                    let s: Vec<_> = monster_ex.random_mystery_reward.iter().map(|p|format!("(A{}) {}", p.rank, p.base)).collect();
                    text!("{}", s.join(" / "))
                }</span>
            </p>
            <p class="mh-kv"><span>"Subtarget research point adjust"</span>
            { if let Some(p) = monster_ex.random_mystery_subtarget_reward {
                html!(<span>{text!("x{}", p.adjust)}</span>)
            } else {
                html!(<span>"-"</span>)
            }}
            </p>
            </div>

            <div class="mh-anomaly-maps"> <h3>"Allowed map"</h3>
            <ul class="mh-item-list">{
                let mut ids = vec![];
                let stages = &random_quest.stage_data;
                if stages.is_map_01 { ids.push(1); }
                if stages.is_map_02 { ids.push(2); }
                if stages.is_map_03 { ids.push(3); }
                if stages.is_map_04 { ids.push(4); }
                if stages.is_map_05 { ids.push(5); }
                if stages.is_map_31 { ids.push(12); }
                if stages.is_map_32 { ids.push(13); }
                if stages.is_map_09 { ids.push(9); }
                if stages.is_map_10 { ids.push(10); }
                if stages.is_map_41 { ids.push(14); }
                ids.into_iter().map(|id|
                    html!(<li> {gen_map_label(id, pedia)} </li>)
                )
            }</ul>
            </div>
            </section>),
        });
    }

    sections.push(Section {
        title: "Hitzone data".to_owned(),
        content: html!(
            <section id="s-hitzone">
            <h2 >"Hitzone data"</h2>
            <div class="mh-color-diagram">
                <img id="mh-hitzone-img" class="mh-color-diagram-img" alt="Monster hitzone diagram" src=meat_figure />
                <canvas id="mh-hitzone-canvas" width=1 height=1 />
            </div>
            <div>
                <input type="checkbox" id="mh-invalid-meat-check"/>
                <label for="mh-invalid-meat-check">"Display invalid parts"</label>
            </div>
            <div>
                <input type="checkbox" id="mh-hitzone-internal-check"/>
                <label for="mh-hitzone-internal-check">"Display internal name"</label>
            </div>
            <div class="mh-table"><table>
                <thead>
                <tr>
                    <th>"Hitzone"</th>
                    <th>"Phase"</th>
                    <th>"Name"</th>
                    <th>"Slash"</th>
                    <th>"Impact"</th>
                    <th>"Shot"</th>
                    <th><img src="/resources/fire.png" alt="Fire" class="mh-small-icon"/>"Fire"</th>
                    <th><img src="/resources/water.png" alt="Water" class="mh-small-icon"/>"Water"</th>
                    <th><img src="/resources/ice.png" alt="Ice" class="mh-small-icon"/>"Ice"</th>
                    <th><img src="/resources/thunder.png" alt="Thunder" class="mh-small-icon"/>"Thunder"</th>
                    <th><img src="/resources/dragon.png" alt="Dragon" class="mh-small-icon"/>"Dragon"</th>
                    <th><img src="/resources/stun.png" alt="Stun" class="mh-small-icon"/>"Dizzy"</th>
                </tr>
                </thead>
                {
                    monster.meat_data.meat_container.iter()
                        .enumerate().map(|(part, meats)| {

                        let part_name = if let Some(names) = collider_mapping.meat_map.get(&part) {
                            names.iter().map(|s|s.as_str()).collect::<Vec<&str>>().join(", ")
                        } else {
                            "".to_owned()
                        };

                        let part_color = format!("mh-part mh-part-{part}");

                        let span = meats.meat_group_info.len();
                        let mut part_common: Option<Vec<Box<td<String>>>> = Some(vec![
                            html!(<td rowspan={span}>
                                <span class=part_color.as_str()/>
                                {text!("[{}]", part)}
                                <span lang="ja" class="mh-hitzone-internal">{ text!("{}", part_name) }</span>
                            </td>),
                        ]);

                        let invalid = meats.meat_group_info == [
                            MeatGroupInfo {
                                slash: 0,
                                strike: 0,
                                shell: 0,
                                fire: 0,
                                water: 0,
                                ice: 0,
                                elect: 0,
                                dragon: 0,
                                piyo: 0,
                            }
                        ];

                        let hidden = if invalid {
                            "mh-invalid-meat"
                        } else {
                            ""
                        };

                        let id = format!("mh-hitzone-dt-{part}");

                        html!(<tbody id={id.as_str()} class="mh-color-diagram-switch"
                            data-color={ PART_COLORS[part] } data-diagram="mh-hitzone"> {

                            meats.meat_group_info.iter().enumerate()
                            .map(move |(phase, group_info)| {
                                let names = pedia_ex.meat_names.get(&MeatKey {
                                    em_type: monster_em_type,
                                    part,
                                    phase
                                }).map(|v|v.as_slice()).unwrap_or_default();

                                let mut tds = part_common.take().unwrap_or_default();

                                let phase_text = if let Some(phase_text) = SPECIFIC_MEAT_TYPE_MAP
                                    .get(&(monster.id, part)).copied().and_then(|m|m.get(phase)) {

                                    phase_text.to_string()
                                } else if let Some(phase_text) =
                                    phase_map.and_then(|m|m.get(phase)) {

                                    phase_text.to_string()
                                } else {
                                    format!("{phase}")
                                };

                                tds.extend(vec![
                                    html!(<td>{text!("{}", phase_text)}</td>),
                                    html!(<td>{
                                        names.iter().enumerate().map(|(i, n)| html!(<span>
                                            { text!("{}", if i == 0 {""} else {", "}) }
                                            { gen_multi_lang(n) }
                                        </span>))
                                    }</td>),
                                    html!(<td>{text!("{}", group_info.slash)}</td>),
                                    html!(<td>{text!("{}", group_info.strike)}</td>),
                                    html!(<td>{text!("{}", group_info.shell)}</td>),
                                    html!(<td>{text!("{}", group_info.fire)}</td>),
                                    html!(<td>{text!("{}", group_info.water)}</td>),
                                    html!(<td>{text!("{}", group_info.ice)}</td>),
                                    html!(<td>{text!("{}", group_info.elect)}</td>),
                                    html!(<td>{text!("{}", group_info.dragon)}</td>),
                                    html!(<td>{text!("{}", group_info.piyo)}</td>),
                                ]);
                                html!(<tr class=hidden> {tds} </tr>)
                            })
                        } </tbody>)
                    })
                }
            </table></div>
            </section>
        ),
    });

    sections.push(Section {
        title: "Parts".to_owned(),
        content: html!(
        <section id="s-parts">
        <h2 >
            "Parts"
        </h2>
        <div class="mh-color-diagram">
            <img id="mh-part-img" class="mh-color-diagram-img" alt="Monster parts diagram" src=parts_group_figure />
            <canvas id="mh-part-canvas" width=1 height=1 />
        </div>
        <div>
            <input type="checkbox" id="mh-invalid-part-check"/>
            <label for="mh-invalid-part-check">"Display invalid parts"</label>
        </div>
        <div>
            <input type="checkbox" id="mh-part-internal-check"/>
            <label for="mh-part-internal-check">"Display internal name"</label>
        </div>
        <div class="mh-table"><table>
            <thead>
                <tr>
                    <th>"Part"</th>
                    <th>"Stagger"</th>
                    <th>"Break"</th>
                    <th>"Sever"</th>
                    <th class="mh-color-diagram-switch" id="mh-part-dt-extract" data-diagram="mh-part">"Extract"</th>
                    <th>"Anomaly cores" {
                        monster.unique_mystery.as_ref().map(|m| text!(" ({}x active)", m.base.maximum_activity_core_num))
                    } </th>
                </tr>
            </thead>
            <tbody>{
                monster.data_tune.enemy_parts_data.iter().enumerate().map(|(index, part)| {
                    let part_name = if let Some(names) = collider_mapping.part_map.get(&index) {
                        names.iter().map(|s|s.as_str()).collect::<Vec<&str>>().join(", ")
                    } else {
                        "".to_owned()
                    };

                    let part_color = format!("mh-part-group mh-part-{index}");

                    let class_str = if part.extractive_type == ExtractiveType::None {
                        "mh-invalid-part mh-color-diagram-switch mh-extractive-color"
                    } else {
                        "mh-color-diagram-switch mh-extractive-color"
                    };

                    let index_u16 = u16::try_from(index);

                    let part_break = enemy_parts_break_data_list.iter()
                        .filter(|p| Ok(p.parts_group) == index_u16)
                        .map(|part_break|{
                            part_break.parts_break_data_list.iter().map(
                                |p| if p.master_vital == -1 {
                                    format!("(x{}) {}", p.break_level, p.vital)
                                } else {
                                    format!("(x{}) (LR/HR) {}, (MR) {}", p.break_level, p.vital, p.master_vital)
                                }
                            ).collect::<Vec<_>>().join(" / ")
                        }).collect::<Vec<_>>().join(" , ");

                    let part_loss = enemy_parts_loss_data_list.iter()
                        .filter(|p| Ok(p.parts_group) == index_u16)
                        .map(|part_loss| {
                            let attr = match part_loss.parts_loss_data.permit_damage_attr {
                                PermitDamageAttrEnum::Slash => "(Slash) ",
                                PermitDamageAttrEnum::Strike => "(Impact) ",
                                PermitDamageAttrEnum::All => "",
                            };
                            if part_loss.parts_loss_data.master_vital == -1 {
                                format!("{} {}", attr, part_loss.parts_loss_data.vital)
                            } else {
                                format!("{} (LR/HR) {}, (MR) {}", attr, part_loss.parts_loss_data.vital,
                                    part_loss.parts_loss_data.master_vital)
                            }
                        }).collect::<Vec<_>>().join(" , ");

                    let id = format!("mh-part-dt-{index}");
                    let extractive_tag = gen_extractive_type_tag(part.extractive_type);
                    html!(<tr id = {id.as_str()} class=class_str data-color={ PART_COLORS[index] }
                        data-diagram="mh-part" data-extractcolor={extractive_tag}>
                        <td>
                            <span class=part_color.as_str()/>
                            { text!("[{}]", index) }
                            <span lang="ja" class="mh-part-internal">{ text!("{}", part_name) }</span>
                        </td>
                        <td>{ if part.master_vital == -1 {
                            text!("{}", part.vital)
                        } else {
                            text!("(LR/HR) {}, (MR) {}", part.vital, part.master_vital)
                        }}</td>
                        <td>{ text!("{}", part_break) }</td>
                        <td>{ text!("{}", part_loss) }</td>
                        <td>{ gen_extractive_type(part.extractive_type) }</td>
                        {
                            if let Some(m) = &monster.unique_mystery {
                                html!(<td>
                                {if let Some(core @ EnemyMysteryCorePartsData{is_core_candidate: true, ..}) = m.base.mystery_core_parts_data.get(index) {
                                    html!(<div>
                                        <div>{text!("{}% probability, {} HP", core.activate_percentage, core.maximum_activity_vital)}</div>
                                        {(!core.link_parts_list.is_empty()).then(|| html!(<div>"Linked parts "
                                            {core.link_parts_list.iter().map(|part|{let part_color = format!("mh-part-group mh-part-{part}");
                                                html!(<span class=part_color.as_str()/>)})}
                                            {core.link_parts_list.iter().map(|part|text!("[{}]", part))}
                                        </div>))}
                                        {(core.prohibit_same_apply_group.0.is_some() && core.prohibit_same_apply_group.0 != Some(6)).then(||
                                            html!(<div>{text!("Mutual exclusive group {}", core.prohibit_same_apply_group.0.unwrap())}</div>)
                                        )}
                                    </div>)
                                } else {
                                    html!(<div>"-"</div>)
                                }}
                                { (Ok(m.base.maximum_activity_release_last_attack_parts) == index_u16).then(||
                                    html!(<span class="tag is-primary">"Final part to release"</span>)
                                )}
                                </td>)
                            } else {
                                html!(<td/>)
                            }
                        }
                    </tr>)
                }).collect::<Vec<_>>()
            }</tbody>
        </table></div>
        </section>
    )});

    sections.push(Section {
        title: "Multi-part vital".to_owned(),
        content: html!(
            <section id="s-multipart">
            <h2>"Multi-part vital"</h2>
            {
                // TODO: "Common" data should be overridden by system preset. This however only appears for 89_00/05
                let system = monster.data_tune.enemy_multi_parts_vital_system_data
                    .iter().enumerate().map(|(i, m)|(true, i, &m.base.0));
                let additional = monster.data_tune.enemy_multi_parts_vital_data_list
                    .iter().enumerate().map(|(i, m)|(false, i, m));
                gen_multipart(system.chain(additional))
            }
            </section>
        ),
    });

    sections.push(Section{title: "Abnormal status".to_owned(), content: html!(
        <section id="s-status">
        <h2 >
            "Abnormal status"
        </h2>
        <div>
            <input type="checkbox" id="mh-ride-cond-check"/>
            <label for="mh-ride-cond-check">"Display data for riding"</label>
        </div>
        <div>
            <input type="checkbox" id="mh-preset-check"/>
            <label for="mh-preset-check">"Don't override with preset data"</label>
        </div>
        <div class="mh-table"><table>
            <thead>
                <tr>
                    <th></th>
                    <th>"Threshold"</th>
                    <th>"Decay"</th>
                    <th>"Max stock"</th>
                    <th>"Active time"</th>
                    <th>"Add tired time"</th>
                    <th>"Damage"</th>
                    <th>"Additional information"</th>
                </tr>
            </thead>
            <tbody>
                {gen_condition_paralyze(false, &monster.condition_damage_data.paralyze_data, monster.condition_damage_data.use_paralyze)}
                {gen_condition_sleep(false, &monster.condition_damage_data.sleep_data, monster.condition_damage_data.use_sleep)}
                {gen_condition_stun(false, &monster.condition_damage_data.stun_data, monster.condition_damage_data.use_stun)}
                {gen_condition_stamina(false, &monster.condition_damage_data.stamina_data, monster.condition_damage_data.use_stamina)}

                {gen_condition_paralyze(true, monster.condition_damage_data.paralyze_data.or_preset(condition_preset), monster.condition_damage_data.use_paralyze)}
                {gen_condition_sleep(true, monster.condition_damage_data.sleep_data.or_preset(condition_preset), monster.condition_damage_data.use_sleep)}
                {gen_condition_stun(true, monster.condition_damage_data.stun_data.or_preset(condition_preset), monster.condition_damage_data.use_stun)}
                {gen_condition_stamina(true, monster.condition_damage_data.stamina_data.or_preset(condition_preset), monster.condition_damage_data.use_stamina)}

                {gen_condition_flash(pedia_ex, false, false, &monster.condition_damage_data.flash_data, monster.condition_damage_data.use_flash)}
                {gen_condition_flash(pedia_ex, true, false, monster.condition_damage_data.flash_data.or_preset(condition_preset), monster.condition_damage_data.use_flash)}
                {monster.unique_mystery.as_ref().into_iter().flat_map(|m| &m.base.condition_damage_data).map(|c| {
                    let value = usize::try_from(c.flash_damage_use_preset_type).ok().and_then(|v|pedia.system_mystery.flash_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.flash_damage_data);
                    gen_condition_flash(pedia_ex, true, true, value, monster.condition_damage_data.use_flash)
                })}

                {gen_condition_poison(false, &monster.condition_damage_data.poison_data, monster.condition_damage_data.use_poison)}
                {gen_condition_blast(false, &monster.condition_damage_data.blast_data, monster.condition_damage_data.use_blast)}

                {gen_condition_poison(true, monster.condition_damage_data.poison_data.or_preset(condition_preset), monster.condition_damage_data.use_poison)}
                {gen_condition_blast(true, monster.condition_damage_data.blast_data.or_preset(condition_preset), monster.condition_damage_data.use_blast)}

                {gen_condition_ride(pedia_ex, false, &monster.condition_damage_data.marionette_data, monster.condition_damage_data.use_ride)}
                {
                    let marionette_data = match monster.condition_damage_data.marionette_data.use_data {
                        UseDataType::Common => &pedia.system_mario.marionette_start_damage_data.base,
                        UseDataType::Unique => &monster.condition_damage_data.marionette_data
                    };
                    gen_condition_ride(pedia_ex, true, marionette_data, monster.condition_damage_data.use_ride)
                }

                {gen_condition_water(false, &monster.condition_damage_data.water_data, monster.condition_damage_data.use_water)}
                {gen_condition_fire(false, &monster.condition_damage_data.fire_data, monster.condition_damage_data.use_fire)}
                {gen_condition_ice(false, &monster.condition_damage_data.ice_data, monster.condition_damage_data.use_ice)}
                {gen_condition_thunder(false, &monster.condition_damage_data.thunder_data, monster.condition_damage_data.use_thunder)}
                {gen_condition_fall_trap(pedia_ex, false, false, &monster.condition_damage_data.fall_trap_data, monster.condition_damage_data.use_fall_trap)}
                {gen_condition_fall_quick_sand(false, false, &monster.condition_damage_data.fall_quick_sand_data, monster.condition_damage_data.use_fall_quick_sand)}
                {gen_condition_fall_otomo_trap(false, false, &monster.condition_damage_data.fall_otomo_trap_data, monster.condition_damage_data.use_fall_otomo_trap)}
                {gen_condition_shock_trap(pedia_ex, false, false, &monster.condition_damage_data.shock_trap_data, monster.condition_damage_data.use_shock_trap)}
                {gen_condition_shock_otomo_trap(false, false, &monster.condition_damage_data.shock_otomo_trap_data, monster.condition_damage_data.use_shock_otomo_trap)}
                {gen_condition_capture(pedia_ex, false, &monster.condition_damage_data.capture_data, monster.condition_damage_data.use_capture)}
                {gen_condition_dung(false, &monster.condition_damage_data.koyashi_data, monster.condition_damage_data.use_dung)}
                {gen_condition_steel_fang(false, &monster.condition_damage_data.steel_fang_data, monster.condition_damage_data.use_steel_fang)}

                {gen_condition_water(true, monster.condition_damage_data.water_data.or_preset(condition_preset), monster.condition_damage_data.use_water)}
                {gen_condition_fire(true, monster.condition_damage_data.fire_data.or_preset(condition_preset), monster.condition_damage_data.use_fire)}
                {gen_condition_ice(true, monster.condition_damage_data.ice_data.or_preset(condition_preset), monster.condition_damage_data.use_ice)}
                {gen_condition_thunder(true, monster.condition_damage_data.thunder_data.or_preset(condition_preset), monster.condition_damage_data.use_thunder)}
                {gen_condition_fall_trap(pedia_ex, true, false, monster.condition_damage_data.fall_trap_data.or_preset(condition_preset), monster.condition_damage_data.use_fall_trap)}
                {gen_condition_fall_quick_sand(true, false, monster.condition_damage_data.fall_quick_sand_data.or_preset(condition_preset), monster.condition_damage_data.use_fall_quick_sand)}
                {gen_condition_fall_otomo_trap(true, false, monster.condition_damage_data.fall_otomo_trap_data.or_preset(condition_preset), monster.condition_damage_data.use_fall_otomo_trap)}
                {gen_condition_shock_trap(pedia_ex, true, false, <ShockTrapDamageData as ConditionDamage<PresetShockTrapData>>::or_preset(&monster.condition_damage_data.shock_trap_data, condition_preset), monster.condition_damage_data.use_shock_trap)}
                {gen_condition_shock_otomo_trap(true, false, <ShockTrapDamageData as ConditionDamage<PresetShockOtomoTrapData>>::or_preset(&monster.condition_damage_data.shock_trap_data, condition_preset), monster.condition_damage_data.use_shock_otomo_trap)}
                {monster.unique_mystery.as_ref().into_iter().flat_map(|m| &m.base.condition_damage_data).flat_map(|c| {
                    let value = usize::try_from(c.fall_trap_use_preset_type).ok().and_then(|v|pedia.system_mystery.fall_trap_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.fall_trap_data);
                    let fall_trap = gen_condition_fall_trap(pedia_ex, true, true, value, monster.condition_damage_data.use_fall_trap);
                    let value = usize::try_from(c.fall_quick_sand_use_preset_type).ok().and_then(|v|pedia.system_mystery.fall_quick_sand_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.fall_quick_sand_data);
                    let fall_quick_sand = gen_condition_fall_quick_sand(true, true, value, monster.condition_damage_data.use_fall_quick_sand);
                    let value = usize::try_from(c.fall_otomo_trap_use_preset_type).ok().and_then(|v|pedia.system_mystery.fall_otomo_trap_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.fall_otomo_trap_data);
                    let fall_otomo_trap = gen_condition_fall_otomo_trap(true, true, value, monster.condition_damage_data.use_fall_otomo_trap);
                    let value = usize::try_from(c.shock_trap_use_preset_type).ok().and_then(|v|pedia.system_mystery.shock_trap_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.shock_trap_data);
                    let shock_trap = gen_condition_shock_trap(pedia_ex, true, true, value, monster.condition_damage_data.use_shock_trap);
                    let value = usize::try_from(c.shock_otomo_trap_use_preset_type).ok().and_then(|v|pedia.system_mystery.shock_otomo_trap_data.get(v))
                        .map(|v|&v.base.0).unwrap_or(&c.shock_otomo_trap_data);
                    let shock_otomo_trap = gen_condition_shock_otomo_trap(true, true, value, monster.condition_damage_data.use_shock_otomo_trap);

                    [fall_trap, fall_quick_sand, fall_otomo_trap, shock_trap, shock_otomo_trap]
                })}
                {gen_condition_capture(pedia_ex, true, monster.condition_damage_data.capture_data.or_preset(condition_preset), monster.condition_damage_data.use_capture)}
                {gen_condition_dung(true, monster.condition_damage_data.koyashi_data.or_preset(condition_preset), monster.condition_damage_data.use_dung)}
                {gen_condition_steel_fang(true, monster.condition_damage_data.steel_fang_data.or_preset(condition_preset), monster.condition_damage_data.use_steel_fang)}
            </tbody>
        </table></div>
        </section>
    )});

    if let Some(m) = &monster.unique_mystery {
        let base = &m.base;
        let system = &pedia.system_mystery;
        sections.push(Section {
            title: "Afflicted stats".to_owned(),
            content: html!(<section id="s-afflicted">
                <h2>"Afflicted stats"</h2>
                <div class="mh-kvlist mh-wide">
                <p class="mh-kv"><span>"Return to great activity time"</span>
                    <span>{
                        let value = usize::try_from(base.return_to_great_activity_time_use_data_type).ok()
                            .and_then(|i|system.return_to_great_activity_time_sec_preset_data_list.get(i))
                            .map(|v|v.time_sec)
                            .unwrap_or(base.return_to_great_activity_time_sec);
                        text!("{} sec", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Damage to release"</span>
                <span>{
                    let value = usize::try_from(base.maximum_activity_release_info_use_data_type).ok()
                        .and_then(|i|system.maximum_activity_release_info_preset_data_list.get(i))
                        .map(|v|&v.maximum_activity_release_info_list[..])
                        .unwrap_or(&base.maximum_activity_release_info[..]);
                    let display: Vec<_> = value.iter().map(|v| format!("x{} (carry over x{})",
                        v.release_damage_rate, v.carry_over_limit_rate)).collect();
                    text!("{}", display.join(" / "))
                }</span>
                </p>
                <p class="mh-kv"><span>"Release count to topple"</span>
                <span>{
                    let value = usize::try_from(base.maximum_to_activity_need_release_num_use_data_type).ok()
                        .and_then(|i|system.maximum_to_activity_need_release_num_preset_data_list.get(i))
                        .map(|v|&v.num_list[..])
                        .unwrap_or(&base.maximum_to_activity_need_release_num[..]);
                    let display: Vec<_> = value.iter().map(|v| format!("{v}")).collect();
                    text!("{}", display.join(" / "))
                }</span>
                </p>
                <p class="mh-kv"><span>"Maximum activity min continue time"</span>
                    <span>{
                        let value = usize::try_from(base.maxiimum_activity_min_continue_time_use_data_type).ok()
                            .and_then(|i|system.maxiimum_activity_min_continue_time_sec_preset_data_list.get(i))
                            .map(|v|v.time_sec)
                            .unwrap_or(base.maxiimum_activity_min_continue_time_sec);
                        text!("{} sec", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Anomaly burst time"</span>
                    <span>{
                        let value = usize::try_from(base.maxiimum_activity_failed_end_time_use_data_type).ok()
                            .and_then(|i|system.maxiimum_activity_failed_end_time_sec_preset_data_list.get(i))
                            .map(|v|v.time_sec)
                            .unwrap_or(base.maxiimum_activity_failed_end_time_sec);
                        text!("{} sec", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Add enrage"</span>
                    <span>{
                        let value = usize::try_from(base.add_anger_rate_use_data_type).ok()
                            .and_then(|i|system.add_anger_rate_preset_data_list.get(i))
                            .map(|v|v.rate)
                            .unwrap_or(base.add_anger_rage);
                        text!("x{}", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Core break damage"</span>
                    <span>{
                        let value = usize::try_from(base.mystery_core_break_damage_rate_use_data_type).ok()
                            .and_then(|i|system.mystery_core_break_damage_rate_preset_data_list.get(i))
                            .map(|v|v.percentage)
                            .unwrap_or(base.mystery_core_break_damage_rate);
                        text!("{}%", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Core break part break"</span>
                    <span>{
                        let value = usize::try_from(base.mystery_core_break_parts_damage_rate_use_data_type).ok()
                            .and_then(|i|system.mystery_core_break_parts_damage_rate_preset_data_list.get(i))
                            .map(|v|v.percentage)
                            .unwrap_or(base.mystery_core_break_parts_damage_rate);
                        text!("{}%", value)
                    }</span>
                </p>
                <p class="mh-kv"><span>"Attack"</span>
                    <span>{
                        let value = usize::try_from(base.attack_rate_use_data_type).ok()
                            .and_then(|i|system.attack_rate_preset_data_list.get(i))
                            .map(|v|&v.attack_rate_list)
                            .unwrap_or(&base.attack_rate);
                        let display: Vec<_> = value.iter().map(|v| format!("(normal) x{} / (great) x{} / (max) x{}",
                            v.activity, v.great_activity, v.maximum_activity
                        )).collect();
                        text!("{}", display.join(" || "))
                    }</span>
                </p>
                <p class="mh-kv"><span>"Motion"</span>
                    <span>{
                        let value = usize::try_from(base.mot_speed_rate_use_data_type).ok()
                            .and_then(|i|system.mot_speed_rate_preset_data_list.get(i))
                            .map(|v|&v.mot_speed_rate_list)
                            .unwrap_or(&base.mot_speed_rate);
                        let display: Vec<_> = value.iter().map(|v| format!("(normal) x{} / (great) x{} / (max) x{}",
                            v.activity, v.great_activity, v.maximum_activity
                        )).collect();
                        text!("{}", display.join(" || "))
                    }</span>
                </p>
                <p class="mh-kv"><span>"Bloodblight time"</span>
                <span>{
                    let value = usize::try_from(base.mystery_debuff_time_rate_use_data_type).ok()
                        .and_then(|i|system.mystery_debuff_time_rate_preset_data_list.get(i))
                        .map(|v|&v.rate_list[..])
                        .unwrap_or(&base.mystery_debuff_time_rate[..]);
                    let display: Vec<_> = value.iter().map(|v| format!("x{v}")).collect();
                    text!("{}", display.join(" / "))
                }</span>
                </p>
                {base.special_mystery_quest_hp_tbl_no.0
                    .zip(base.special_mystery_quest_attack_rate.0)
                    .zip(base.special_mystery_quest_mot_speed_rate.0)
                    .zip(pedia.difficulty_rate_anomaly.as_ref()).map(|(((hp, attack), speed), dr)| {
                    let hp = if hp == -1 {
                        "Default".to_owned()
                    } else {
                        dr.vital_rate_table_list
                        .get(hp as usize)
                        .map_or_else(|| format!("~ {hp}"), |r| format!("x{}", r.vital_rate))
                    };
                    html!(<p class="mh-kv"><span>"Special anomaly investigation"</span>
                    <span>{text!("(HP) {} / (attack) x{} / (motion) x{}", hp, attack, speed)}</span>
                    </p>)
                })}
                </div>
            </section>),
        });
    }

    if let Some(over_mystery) = &monster.unique_over_mystery {
        struct Combined<'a> {
            level_data: Option<&'a StrengthLevelData>,
            burst_data: Option<&'a OverMysteryBurstData>,
        }
        let mut combineds: Vec<_> = over_mystery
            .strength_level_list
            .iter()
            .map(|level| Combined {
                level_data: Some(level),
                burst_data: None,
            })
            .collect();
        let mut next = 0;
        for burst in &over_mystery.over_mystery_burst_list {
            loop {
                if next == combineds.len() {
                    combineds.push(Combined {
                        level_data: None,
                        burst_data: Some(burst),
                    });
                    next += 1;
                } else {
                    match combineds[next]
                        .level_data
                        .unwrap()
                        .need_research_level
                        .cmp(&burst.need_research_level)
                    {
                        std::cmp::Ordering::Less => {
                            next += 1;
                            continue;
                        }
                        std::cmp::Ordering::Equal => {
                            combineds[next].burst_data = Some(burst);
                            next += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            combineds.insert(
                                next,
                                Combined {
                                    level_data: None,
                                    burst_data: Some(burst),
                                },
                            );
                            next += 1;
                        }
                    }
                }
                break;
            }
        }

        sections.push(Section {
            title: "Risen mode stats".to_owned(),
            content: html!(<section id="s-risen">
            <h2> "Risen mode stats" </h2>
            <div class="mh-table"><table>
            <thead>
            <tr>
                <th>"Research level"</th>
                <th>"Level"</th>
                <th>"Trigger health"</th>
                <th>"Release health"</th>
                <th>"Motion"</th>
                <th>"Attack"</th>
            </tr>
            </thead>
            {combineds.into_iter().map(|combined| html!(<tr>
                <td> {
                    let lva = combined.level_data.map(|d|d.need_research_level);
                    let lvb = combined.burst_data.map(|d|d.need_research_level);
                    if lva.is_some() && lvb.is_some() {
                        assert!(lva == lvb)
                    }
                    text!("{}", lva.or(lvb).unwrap())
                }</td>
                <td>{combined.level_data.map(|d| match d.strength_level {
                    OverMysteryStrengthLevel::Default => text!("Normal"),
                    OverMysteryStrengthLevel::Lv1 => text!("Hard"),
                    OverMysteryStrengthLevel::Lv2 => text!("lv2"),
                    OverMysteryStrengthLevel::Lv3 => text!("lv3"),
                })}</td>
                <td>{combined.burst_data.map(|d|text!("x{}", d.enable_vital_rate))}</td>
                <td>{combined.burst_data.map(|d|text!("x{}", d.release_vital_rate))}</td>
                <td>{combined.burst_data.map(|d|text!("x{}", d.mot_speed_rate))}</td>
                <td>{combined.burst_data.map(|d|text!("x{}", d.attack_rate))}</td>
            </tr>))}
            <tbody>
            </tbody>
            </table></div>
            <div class="mh-kvlist mh-wide">
            {over_mystery.special_mystery_quest_hp_tbl_no.0
                .zip(over_mystery.special_mystery_quest_attack_rate.0)
                .zip(over_mystery.special_mystery_quest_mot_speed_rate.0)
                .zip(pedia.difficulty_rate_anomaly.as_ref()).map(|(((hp, attack), speed), dr)| {
                let hp = if hp == -1 {
                    "Default".to_owned()
                } else {
                    dr.vital_rate_table_list
                    .get(hp as usize)
                    .map_or_else(|| format!("~ {hp}"), |r| format!("x{}", r.vital_rate))
                };
                html!(<p class="mh-kv"><span>"Special anomaly investigation"</span>
                <span>{text!("(HP) {} / (attack) x{} / (motion) x{}", hp, attack, speed)}</span>
                </p>)
            })}
            </div>
            </section>),
        });
    }

    if let Some(lot) = gen_lot(monster, monster_em_type, QuestRank::Low, pedia_ex) {
        sections.push(Section {
            title: "Low rank reward".to_owned(),
            content: lot,
        });
    }

    if let Some(lot) = gen_lot(monster, monster_em_type, QuestRank::High, pedia_ex) {
        sections.push(Section {
            title: "High rank reward".to_owned(),
            content: lot,
        });
    }

    if let Some(lot) = gen_lot(monster, monster_em_type, QuestRank::Master, pedia_ex) {
        sections.push(Section {
            title: "Master rank reward".to_owned(),
            content: lot,
        });
    }

    for (index, reward) in monster_ex.mystery_reward.iter().enumerate() {
        let title = if reward.lv_lower_limit == 0 && reward.lv_upper_limit == 0 {
            "Anomaly quest reward".to_owned()
        } else if reward.is_special {
            "Special anomaly investigation reward".to_owned()
        } else {
            format!(
                "Anomaly investigation reward (lv{} ~ lv{})",
                reward.lv_lower_limit, reward.lv_upper_limit
            )
        };
        let id = format!("s-mystery-reward-{index}");

        sections.push(Section {
            content: html!(<section id={id.as_str()}>
                <h2>{ text!("{}", title) }</h2>
                <div class="mh-reward-tables">
                <div class="mh-reward-box"><div class="mh-table"><table>
                    <thead><tr>
                        <th>"Carve & part break"</th>
                        <th>"Probability"</th>
                    </tr></thead>
                    <tbody> {
                        gen_reward_table(pedia_ex,
                            &[reward.reward_item],
                            &[reward.item_num],
                            &[reward.hagibui_probability])
                    } </tbody>
                </table></div></div>
                {reward.quest_reward.map(|r| html!(
                    <div class="mh-reward-box"><div class="mh-table"><table>
                        <thead><tr>
                            <th>"Quest rewards"<br/>
                            {translate_rule(r.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &r.item_id_list,
                                &r.num_list,
                                &r.probability_list)
                        } </tbody>
                    </table></div></div>
                ))}
                {reward.additional_quest_reward.iter().map(|r| html!(
                    <div class="mh-reward-box"><div class="mh-table"><table>
                        <thead><tr>
                            <th>"Quest bonus rewards"<br/>
                            {translate_rule(r.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &r.item_id_list,
                                &r.num_list,
                                &r.probability_list)
                        } </tbody>
                    </table></div></div>
                ))}
                {reward.special_quest_reward.map(|r| html!(
                    <div class="mh-reward-box"><div class="mh-table"><table>
                        <thead><tr>
                            <th>"Special quest rewards"<br/>
                            {translate_rule(r.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &r.item_id_list,
                                &r.num_list,
                                &r.probability_list)
                        } </tbody>
                    </table></div></div>
                ))}
                {reward.multiple_target_reward.map(|r| html!(
                    <div class="mh-reward-box"><div class="mh-table"><table>
                        <thead><tr>
                            <th>"Multi-target quest rewards"<br/>
                            {translate_rule(r.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &r.item_id_list,
                                &r.num_list,
                                &r.probability_list)
                        } </tbody>
                    </table></div></div>
                ))}
                {reward.multiple_fix_reward.map(|r| html!(
                    <div class="mh-reward-box"><div class="mh-table"><table>
                        <thead><tr>
                            <th>"Multi-target fixed rewards"<br/>
                            {translate_rule(r.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &r.item_id_list,
                                &r.num_list,
                                &r.probability_list)
                        } </tbody>
                    </table></div></div>
                ))}
                </div>
            </section>),
            title,
        });
    }

    sections.push(Section {
        title: "Move set".to_owned(),
        content: html!(<section id="s-moveset">
        <h2>"Move set"</h2>
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Internal name"</th>
                <th>"Damage"</th>
                <th>"Status"</th>
                <th>"Guardable"</th>
                <th>"Power"</th>
                <th>"Type"</th>
                <th>"To object"</th>
                <th>"To other monster"</th>
                <th>"Flags"</th>
                //<th>"Riding"</th>
            </tr></thead>
            <tbody> {
                monster.atk_colliders.iter().map(|atk| {
                    let mut damages = vec![];
                    if atk.data.base_damage != 0 {
                        damages.push(html!(<li>{text!("Physical {}", atk.data.base_damage)}</li>))
                    }
                    if atk.data.base_attack_element_value != 0 || atk.data.base_attack_element != AttackElement::None {
                        let image = match atk.data.base_attack_element {
                            AttackElement::None => None,
                            AttackElement::Fire => Some(("fire", "Fire")),
                            AttackElement::Thunder => Some(("thunder", "Thunder")),
                            AttackElement::Water => Some(("water", "Water")),
                            AttackElement::Ice => Some(("ice", "Ice")),
                            AttackElement::Dragon => Some(("dragon", "Dragon")),
                            AttackElement::Heal => Some(("heal", "Heal")),
                        };
                        let image = image.map(|(file, alt)| {
                            let path = format!("/resources/{file}.png");
                            html!(<img src={path.as_str()} class="mh-small-icon" alt={alt}/>)
                        });
                        damages.push(html!(<li>{image}
                            {(atk.data.base_attack_element == AttackElement::None).then(||text!("Null-elem "))}
                            {text!("{}", atk.data.base_attack_element_value)}</li>))
                    }

                    let mut statuss = vec![];

                    if atk.data.base_piyo_value != 0 {
                        statuss.push(html!(<li>
                            <img src="/resources/stun.png" class="mh-small-icon" alt="Stun"/>
                            {text!("{}", atk.data.base_piyo_value)}</li>))
                    }

                    let mut add_debuff = |t: DebuffType, v: u8, s: MeqF32| {
                        if t != DebuffType::None || v != 0 || s.0 != 0.0 {
                            let (image, text): (&[(&str, &str)], &str) = match t {
                                DebuffType::None => (&[], ""),
                                DebuffType::Fire => (&[("fire", "Fire")], ""),
                                DebuffType::Thunder =>(&[("thunder", "Thunder")], ""),
                                DebuffType::Water => (&[("water", "Water")], ""),
                                DebuffType::Ice => (&[("ice", "Ice")], ""),
                                DebuffType::Dragon => (&[("dragon", "Dragon")], ""),
                                DebuffType::Sleep => (&[("sleep", "Sleep")], ""),
                                DebuffType::Paralyze => (&[("para", "Paralyze")], ""),
                                DebuffType::Poison => (&[("poison", "Poison")], ""),
                                DebuffType::NoxiousPoison => (&[("noxious", "Venom")], ""),
                                DebuffType::Bomb => (&[("blast", "Blast")], ""),
                                DebuffType::BubbleS => (&[("bubble", "Bubble")], ""),
                                DebuffType::BubbleRedS => (&[("bubble", "Bubble"), ("attackup", "Attack up")], ""),
                                DebuffType::RedS => (&[("attackup", "Attack up")], ""),
                                DebuffType::BubbleL => (&[("bubblel", "Bubble L")], ""),
                                DebuffType::DefenceDown => (&[("defencedown", "Defense down")], ""),
                                DebuffType::ResistanceDown =>(&[("resdown", "Resistance down")], ""),
                                DebuffType::Stink => (&[("dung", "Stink")], ""),
                                DebuffType::Capture => (&[("capture", "Capture")], ""),
                                DebuffType::OniBomb => (&[("oni", "Hellfire")], ""),
                                DebuffType::Kijin => (&[], "Kijin"), // TODO
                                DebuffType::Kouka => (&[], "Kouka"), // TODO
                                DebuffType::Bleeding => (&[("bleed", "Bleed")], ""),
                                DebuffType::ParalyzeShort => (&[("para", "Paralyze")], "(Short)"),
                                DebuffType::Virus => (&[("frenzy", "Frenzy")], ""),
                            };

                            statuss.push(html!(<li>
                                { image.iter().map(|&(file, alt)| {
                                    let path = format!("/resources/{file}.png");
                                    html!(<img src={path.as_str()} class="mh-small-icon" alt={alt}/>)
                                }) }
                                { text!("{}", text) }
                                { (v != 0).then(||text!("{}", v)) }
                                { (s.0 != 0.0).then(||text!(" {}sec", s)) }
                            </li>))
                        }
                    };

                    add_debuff(atk.data.base_debuff_type, atk.data.base_debuff_value, atk.data.base_debuff_sec);
                    add_debuff(atk.data.base_debuff_type2, atk.data.base_debuff_value2, atk.data.base_debuff_sec2);
                    add_debuff(atk.data.base_debuff_type3, atk.data.base_debuff_value3, atk.data.base_debuff_sec3);

                    if atk.data.is_mystery_debuff {
                        statuss.push(html!(<li>
                            <img src="/resources/blood.png" class="mh-small-icon" alt="BloodBlight"/>
                            {text!("{}sec", atk.data.mystery_debuff_sec)}</li>))
                    }

                    let mut flags = vec![];
                    if atk.is_shell {
                        flags.push(html!(<span class="tag">"Shell"</span>));
                    }

                    if atk.data.hit_attr.contains(HitAttr::CALC_HIT_DIRECTION) {
                        flags.push(html!(<span class="tag">"CalcHitDirection"</span>));
                    }
                    if atk.data.hit_attr.contains(HitAttr::CALC_HIT_DIRECTION_BASED_ROOT_POS) {
                        flags.push(html!(<span class="tag">"CalcHitDirectionBasedRootPos"</span>));
                    }
                    if atk.data.hit_attr.contains(HitAttr::ALL_DIR_GUARDABLE) {
                        flags.push(html!(<span class="tag">"AllDirGuardable"</span>));
                    }
                    if atk.data.hit_attr.contains(HitAttr::USE_HIT_STOP) {
                        flags.push(html!(<span class="tag">"HitStop"</span>));
                    }
                    if atk.data.hit_attr.contains(HitAttr::USE_HIT_SLOW) {
                        flags.push(html!(<span class="tag">"HitSlow"</span>));
                    }
                    if atk.data.hit_attr.contains(HitAttr::ABS_HIT_STOP) {
                        flags.push(html!(<span class="tag">"AbsHitStop"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::USE_CYCLE_HIT) {
                        flags.push(html!(<span class="tag">"CycleHit"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::CHECK_RAY_CAST) {
                        flags.push(html!(<span class="tag">"CheckRayCast"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::IGNORE_END_DELAY) {
                        flags.push(html!(<span class="tag">"IgnoreEndDelay"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::USE_DIRECTION_OBJECT) {
                        flags.push(html!(<span class="tag">"DirectionObject"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::OVERRIDE_COLLISION_RESULT_BY_RAY) {
                        flags.push(html!(<span class="tag">"OverrideCollisionResultByRay"</span>));

                    }
                    if atk.data.hit_attr.contains(HitAttr::HIT_POS_CORRECTION) {
                        flags.push(html!(<span class="tag">"HitPosCorrection"</span>));
                    }

                    if atk.data.base_attack_attr.contains(AttackAttr::ELEMENT_S) {
                        flags.push(html!(<span class="tag">"ElementS"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::BOMB) {
                        flags.push(html!(<span class="tag">"Bomb"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::ENSURE_DEBUFF) {
                        flags.push(html!(<span class="tag">"EnsureDebuff"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::TRIGGER_MARIONETTE_START) {
                        flags.push(html!(<span class="tag">"TriggerRidingStart"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::FORCE_KILL) {
                        flags.push(html!(<span class="tag">"ForceKill"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::KOYASHI) {
                        flags.push(html!(<span class="tag">"Kayoshi"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::ALLOW_DISABLED) {
                        flags.push(html!(<span class="tag">"AllowDisabled"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::PREVENT_CHEEP_TECH) {
                        flags.push(html!(<span class="tag">"PreventCheepTech"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::HYAKURYU_BOSS_ANGER_END_SP_STOP) {
                        flags.push(html!(<span class="tag">"RampageBossAngerEndSpStop"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::FORCE_PARTS_LOSS_PERMIT_DAMAGE_ATTR_SLASH) {
                        flags.push(html!(<span class="tag">"ForcePartsLossPermitDamageAttrSlash"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::KEEP_RED_DAMAGE) {
                        flags.push(html!(<span class="tag">"KeepRedDamage"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::CREATURE_NET_SEND_DAMAGE) {
                        flags.push(html!(<span class="tag">"CreatureNetSendDamage"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::EM_HP_STOP1) {
                        flags.push(html!(<span class="tag">"EmHpStop1"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::RESTRAINT_PL_ONLY) {
                        flags.push(html!(<span class="tag">"RestraintPlOnly"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::RESTRAINT_ALL) {
                        flags.push(html!(<span class="tag">"RestraintAll"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::SUCK_BLOOD) {
                        flags.push(html!(<span class="tag">"SuckBlood"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::FORCE_PARTS_LOSS_PERMIT_DAMAGE_ATTR_STRIKE) {
                        flags.push(html!(<span class="tag">"ForcePartsLossPermistDamageAttrStrike"</span>))
                    }
                    if atk.data.base_attack_attr.contains(AttackAttr::FROZEN_ZAKO_EM) {
                        flags.push(html!(<span class="tag">"Frozen small monster"</span>))
                    }

                    html!(<tr>
                        <td lang="ja">{text!("{}", atk.data.name)}</td>
                        <td><ul class="mh-damages">{damages}</ul></td>
                        <td><ul class="mh-damages">{statuss}</ul></td>
                        <td>{text!("{}", atk.data.guardable_type.display())}</td>
                        <td>{text!("{}", atk.data.power)}</td>
                        <td>{text!("{}", atk.data.damage_type.display(atk.data.damage_type_value))}</td>
                        <td>{text!("{:?}", atk.data.object_break_type)}</td>
                        <td>{text!("{:?}", atk.data.base_em2em_damage_type)}</td>
                        <td>{flags}</td>
                        /*<td>{text!("{:?} {}//{}/{}/{}//{:?}",
                            atk.data.marionette_enemy_damage_type,
                            atk.data.marionette_enemy_base_damage,
                            atk.data.marionette_enemy_base_damage_s,
                            atk.data.marionette_enemy_base_damage_m,
                            atk.data.marionette_enemy_base_damage_l,
                            atk.data.marionette_unique_damage_list
                        )}</td>*/
                    </tr>)
                })
            } </tbody>
        </table></div>
        </section>),
    });

    let plain_title = format!("Monster {:03}_{:02} - MHRice", monster.id, monster.sub_id);

    let (mut output, mut toc_sink) = output.create_html_with_toc(
        &format!("{:03}_{:02}.html", monster.id, monster.sub_id),
        toc,
    )?;

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{}", plain_title)}</title>
                { head_common(hash_store) }
                { monster_alias.iter().flat_map(|&alias|title_multi_lang(alias)) }
                { open_graph(monster_alias, &plain_title,
                    monster_ex.explain1, "", Some(&icon), toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header class="mh-monster-header">
                    <img alt="Monster icon" src=icon />
                    <h1> {
                        if let Some(monster_alias) = monster_alias {
                            gen_multi_lang(monster_alias)
                        } else if monster.id == 131 {
                            html!(<span>"Toadversary"</span>)
                        } else {
                            html!(<span>{text!("Monster {:03}_{:02}", monster.id, monster.sub_id)}</span>)
                        }
                    }</h1>
                </header>

                { sections.into_iter().map(|s|s.content) }

                </main>
                { right_aside() }
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    if let Some(monster_alias) = monster_alias {
        toc_sink.add(monster_alias);
    }

    Ok(())
}

pub fn gen_monsters(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let mut monsters_path = output.create_html("monster.html")?;

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Monsters - MHRice - Monster Hunter Rise Database")}</title>
                { head_common(hash_store) }
                <meta name="description" content="List of monsters. Monster Hunter Rise Database" />
            </head>
            <body>
                { navbar() }

                <main>
                <header><h1>"Monsters"</h1></header>
                <section>
                <h2 >"Large monsters"</h2>
                <div class="select"><select id="scombo-monster" class="mh-scombo">
                    <option value="0">"Sort by internal ID"</option>
                    <option value="1">"Sort by in-game order"</option>
                </select></div>
                <ul class="mh-list-monster" id="slist-monster">{
                    pedia.monsters.iter().map(|monster| {
                        let icon_path = format!("/resources/em{0:03}_{1:02}_icon.png", monster.id, monster.sub_id);

                        let monster_ex = &pedia_ex.monsters[&monster.em_type];
                        let name_entry = if let Some(entry) = monster_ex.name {
                            gen_multi_lang(entry)
                        } else if monster.id == 131 {
                            html!(<span>"Toadversary"</span>)
                        } else {
                            html!(<span>{text!("Monster {:03}_{:02}", monster.id, monster.sub_id)}</span>)
                        };

                        let order = pedia_ex.monster_order.get(&EmTypes::Em(monster.id | (monster.sub_id << 8)))
                            .cloned().unwrap_or(i32::MAX as usize);
                        let sort_tag = format!("{},{}", monster.id << 16 | monster.sub_id, order);
                        html!{<li data-sort=sort_tag>
                            <a href={format!("/monster/{:03}_{:02}.html", monster.id, monster.sub_id)}>
                                <img alt="Monster icon" class="mh-list-monster-icon" src=icon_path />
                                <div>{name_entry}</div>
                            </a>
                        </li>}
                    }).collect::<Vec<_>>()
                }</ul>
                </section>
                <section>
                <h2 >"Small monsters"</h2>
                <ul class="mh-list-monster">{
                    pedia.small_monsters.iter().filter(|monster|monster.sub_id == 0) // sub small monsters are b0rked
                    .map(|monster| {
                        let icon_path = format!("/resources/ems{0:03}_{1:02}_icon.png", monster.id, monster.sub_id);

                        let monster_ex = &pedia_ex.monsters[&monster.em_type];
                        let name = if let Some(entry) = monster_ex.name {
                            gen_multi_lang(entry)
                        } else {
                            html!(<span>{text!("Monster {:03}_{:02}", monster.id, monster.sub_id)}</span>)
                        };

                        html!{<li>
                            <a href={format!("/small-monster/{:03}_{:02}.html", monster.id, monster.sub_id)}>
                                <img alt="Monster icon" class="mh-list-monster-icon" src=icon_path />
                                <div>{ name }</div>
                            </a>
                        </li>}
                    })
                }</ul>
                </section>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    monsters_path.write_all(doc.to_string().as_bytes())?;

    let monster_path = output.sub_sink("monster")?;
    for monster in &pedia.monsters {
        gen_monster(
            hash_store,
            true,
            monster,
            pedia,
            pedia_ex,
            config,
            &monster_path,
            toc,
        )?;
    }

    let monster_path = output.sub_sink("small-monster")?;
    for monster in &pedia.small_monsters {
        gen_monster(
            hash_store,
            false,
            monster,
            pedia,
            pedia_ex,
            config,
            &monster_path,
            toc,
        )?;
    }
    Ok(())
}
