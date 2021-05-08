#![allow(clippy::unnecessary_wraps)]

use super::gen_quest::*;
use super::gen_website::{gen_multi_lang, head_common, navbar};
use super::pedia::*;
use crate::msg::*;
use crate::rsz::*;
use anyhow::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::write;
use std::path::*;
use typed_html::{dom::*, elements::*, html, text};

fn gen_extractive_type(extractive_type: ExtractiveType) -> Result<Box<span<String>>> {
    match extractive_type {
        ExtractiveType::Red => Ok(html!(<span class="mh-extract-red">"Red"</span>)),
        ExtractiveType::White => Ok(html!(<span class="mh-extract-white">"White"</span>)),
        ExtractiveType::Orange => Ok(html!(<span class="mh-extract-orange">"Orange"</span>)),
        ExtractiveType::None => Ok(html!(<span class="mh-extract-unknown">"None"</span>)),
    }
}

fn safe_float(v: f32) -> String {
    let normal = format!("{}", v);
    if normal.len() < 5 {
        normal
    } else {
        format!("{:e}", v)
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
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Paralyze"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset={}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_sleep(
    is_preset: bool,
    data: &SleepDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Sleep"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_stun(
    is_preset: bool,
    data: &StunDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Stun"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_stamina(
    is_preset: bool,
    data: &StaminaDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Exhaust"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Stamina reduction = {}, Preset={}", data.sub_stamina, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_flash(
    is_preset: bool,
    data: &FlashDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
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

    let caption = if is_preset {
        "Flash (preset, broken?)"
    } else {
        "Flash (not preset)"
    };

    let content = html!(
        <tr class={gen_disabled(used, None).as_str()}>
            <td>{text!("{}", caption)}</td>
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
            {text!("Preset = {}", data.preset_type)}
            </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_poison(
    is_preset: bool,
    data: &PoisonDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Poison"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_blast(
    is_preset: bool,
    data: &BlastDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Blast"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Blast damage = {}, Preset = {}", data.blast_damage, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_ride(
    data: &MarionetteStartDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let use_data = match data.use_data {
        UseDataType::Common => "common",
        UseDataType::Unique => "unique",
    };
    let content = html!(
        <tr class={gen_disabled(used, None).as_str()}>
            <td>"Ride"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("{}, Nora first limit = {}", use_data, data.nora_first_limit)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_water(
    is_preset: bool,
    data: &WaterDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Water"</td>
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
            {text!("Preset = {}", data.preset_type)}
            </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fire(
    is_preset: bool,
    data: &FireDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Fire"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Hit-damage rate = {}, Preset = {}", data.hit_damage_rate, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_ice(
    is_preset: bool,
    data: &IceDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Ice"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Motion speed rate = {}, Preset = {}", data.motion_speed_rate, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_thunder(
    is_preset: bool,
    data: &ThunderDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Thunder"</td>
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
            {text!("Stun active limit = {}, Preset = {}",
                data.stun_active_limit, data.preset_type)}
            </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fall_trap(
    is_preset: bool,
    data: &FallTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Fall trap"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fall_quick_sand(
    is_preset: bool,
    data: &FallQuickSandDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Quick sand"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fall_otomo_trap(
    is_preset: bool,
    data: &FallOtomoTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Buddy fall trap"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Poison stacking = {}, Preset = {}",
                data.already_poison_stock_value, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_shock_trap(
    is_preset: bool,
    data: &ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Shock trap"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_shock_otomo_trap(
    is_preset: bool,
    data: &ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Buddy shock trap"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_capture(
    is_preset: bool,
    data: &CaptureDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Capture"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_dung(
    is_preset: bool,
    data: &KoyashiDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Dung"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_steel_fang(
    is_preset: bool,
    data: &SteelFangData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class={gen_disabled(used, Some(is_preset)).as_str()}>
            <td>"Steel fang"</td>
            { gen_condition_base(&data.base) }
            <td> {text!("Active limit = {}, Preset = {}, Unique target param = {}",
                data.active_limit_count, data.preset_type, data.is_unique_target_param)}
                <br />
                {text!("Distance = {} ~ {}, Angle = {}",
                data.min_distance, data.max_distance, data.angle)}
            </td>
        </tr>
    );
    Ok(content)
}

pub fn gen_monster(
    is_large: bool,
    monster: &Monster,
    monster_aliases: &Msg,
    condition_preset: &EnemyConditionPresetData,
    sizes: &HashMap<u32, &SizeInfo>,
    size_dists: &HashMap<i32, &[ScaleAndRateData]>,
    quests: &[Quest],
    pedia: &Pedia,
    folder: &Path,
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

    let monster_id = monster.id;
    let monster_sub_id = monster.sub_id;

    let quest_list = html!(
        <section class="section">
        <h2 class="subtitle">"Quests"</h2>
        <table>
            <thead><tr>
                <th>"Quest"</th>
                <th>"Size (?)"</th>
                <th>"HP"</th>
                <th>"Attack"</th>
                <th>"Parts"</th>
                <th>"Defense"</th>
                <th>"Element A"</th>
                <th>"Element B"</th>
                <th>"Stun"</th>
                <th>"Exhaust"</th>
                <th>"Ride"</th>
                <th>"Stamina"</th>
            </tr></thead>
            <tbody> {
                quests.iter().flat_map(|quest| {
                    quest.param.boss_em_type.iter().copied().enumerate().filter(
                        |&(i, em_type)|em_type == monster_id | (monster_sub_id << 8)
                    )
                    .map(move |(i, em_type)|{

                        let target_tag = if quest.param.tgt_em_type.contains(&em_type) {
                            html!(<span class="tag is-primary">"Target"</span>)
                        } else {
                            html!(<span />)
                        };

                        html!(<tr>
                            <td>
                                <span class="tag">{text!("{:?}-{:?}", quest.param.enemy_level, quest.param.quest_level)}</span>
                                <a href={format!("/quest/{:06}.html", quest.param.quest_no)}>
                                {quest.name.as_ref().map_or(
                                    html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                                    gen_multi_lang
                                )}
                                </a>
                                {target_tag}
                            </td>
                            { gen_quest_monster_data(&quest.enemy_param, em_type, i, sizes,size_dists, pedia) }
                        </tr>)
                    })
                })
            } </tbody>
        </table>
        </section>
    );

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monster {:03}_{:02} - MHRice", monster.id, monster.sub_id)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-monster-header">
                    <img src=icon />
                    <h1 class="title"> {
                        if is_large {
                            let name_name = format!("Alias_EnemyIndex{:03}",
                                monster.boss_init_set_data.as_ref()
                                .context(format!("Cannot found boss_init_set for monster {}_{}",
                                    monster.id, monster.sub_id))?
                                .enemy_type);
                            monster_aliases.get_entry(&name_name).map_or(
                                html!(<span>{text!("Monster {:03}_{:02}", monster.id, monster.sub_id)}</span>),
                                gen_multi_lang
                            )
                        } else {
                            html!(<span>{text!("Monster {:03}_{:02}", monster.id, monster.sub_id)}</span>)
                        }
                    }</h1>
                </div>
                <section class="section">
                <h2 class="subtitle">"Basic data"</h2>
                <p>{ text!("Base HP: {}", monster.data_tune.base_hp_vital) }</p>
                <p>{ text!("Limping threshold: (village) {}% / (LR) {}% / (HR) {}%",
                    monster.data_tune.dying_village_hp_vital_rate,
                    monster.data_tune.dying_low_level_hp_vital_rate,
                    monster.data_tune.dying_high_level_hp_vital_rate
                ) }</p>
                <p>{ text!("Capturing threshold: (village) {}% / (LR) {}% / (HR) {}%",
                    monster.data_tune.capture_village_hp_vital_rate,
                    monster.data_tune.capture_low_level_hp_vital_rate,
                    monster.data_tune.capture_high_level_hp_vital_rate
                ) }</p>
                <p>{ text!("Sleep recovering: {} seconds / recover {}% HP",
                    monster.data_tune.self_sleep_time,
                    monster.data_tune.self_sleep_recover_hp_vital_rate
                ) }</p>
                </section>

                { quest_list }

                <section class="section">
                <h2 class="subtitle">"Hitzone data"</h2>
                <img src=meat_figure />
                <div>
                    <input type="checkbox" onclick="onCheckDisplay(this, 'mh-invalid-meat', null)" id="mh-invalid-meat-check"/>
                    <label for="mh-invalid-meat-check">"Display invalid parts"</label>
                </div>
                <table>
                    <thead>
                    <tr>
                        <th>"Part"</th>
                        <th>"Phase"</th>
                        <th>"Slash"</th>
                        <th>"Impact"</th>
                        <th>"Shot"</th>
                        <th>"Fire"</th>
                        <th>"Water"</th>
                        <th>"Ice"</th>
                        <th>"Thunder"</th>
                        <th>"Dragon"</th>
                        <th>"Dizzy"</th>
                    </tr>
                    </thead>
                    <tbody>{
                        monster.meat_data.meat_container.iter()
                            .enumerate().flat_map(|(part, meats)| {

                            let part_name = if let Some(names) = collider_mapping.meat_map.get(&part) {
                                names.iter().map(|s|s.as_str()).collect::<Vec<&str>>().join(", ")
                            } else {
                                format!("{}", part)
                            };

                            let part_color = format!("mh-part-{}", part);

                            let span = meats.meat_group_info.len();
                            let mut part_common: Option<Vec<Box<td<String>>>> = Some(vec![
                                html!(<td rowspan={span}>
                                    <span class=part_color.as_str()>"■"</span>
                                    { text!("{}", part_name) }
                                </td>),
                            ]);

                            let invalid = &meats.meat_group_info == &[
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

                            meats.meat_group_info.iter().enumerate()
                                .map(move |(phase, group_info)| {
                                    let mut tds = part_common.take().unwrap_or_else(||vec![]);
                                    tds.extend(vec![
                                        html!(<td>{text!("{}", phase)}</td>),
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
                                    html!(<tr class=hidden.clone()> {tds} </tr>)
                                })
                        })
                    }</tbody>
                </table>
                </section>
                <section class="section">
                <h2 class="subtitle">
                    "Parts"
                </h2>
                <img src=parts_group_figure />
                <div>
                    <input type="checkbox" onclick="onCheckDisplay(this, 'mh-invalid-part', null)" id="mh-invalid-part-check"/>
                    <label for="mh-invalid-part-check">"Display invalid parts"</label>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>"Part"</th>
                            <th>"Stagger"</th>
                            <th>"Break"</th>
                            <th>"Sever"</th>
                            <th>"Extract"</th>
                        </tr>
                    </thead>
                    <tbody>{
                        monster.data_tune.enemy_parts_data.iter().enumerate().map(|(index, part)| {
                            let part_name = if let Some(names) = collider_mapping.part_map.get(&index) {
                                names.iter().map(|s|s.as_str()).collect::<Vec<&str>>().join(", ")
                            } else {
                                format!("{}", index)
                            };

                            let part_color = format!("mh-part-{}", index);

                            let hidden = if part.extractive_type == ExtractiveType::None {
                                "mh-invalid-part"
                            } else {
                                ""
                            };

                            let index_u16 = u16::try_from(index)?;

                            let mut part_break_iter = enemy_parts_break_data_list.iter()
                                .filter(|p| p.parts_group == index_u16);
                            let part_break = if let Some(part_break) = part_break_iter.next() {
                                if part_break_iter.next().is_some() {
                                    bail!("Duplicated part break data found");
                                }
                                part_break.parts_break_data_list.iter().map(
                                    |p| format!("({}) {}", p.break_level, p.vital)
                                ).collect::<Vec<_>>().join(" / ")
                            } else {
                                "".to_string()
                            };

                            let mut part_loss_iter = enemy_parts_loss_data_list.iter()
                                .filter(|p| p.parts_group == index_u16);
                            let part_loss = if let Some(part_loss) = part_loss_iter.next() {
                                if part_loss_iter.next().is_some() {
                                    bail!("Duplicated part loss data found");
                                }
                                let attr = match part_loss.parts_loss_data.permit_damage_attr {
                                    PermitDamageAttrEnum::Slash => "(Slash) ",
                                    PermitDamageAttrEnum::Strike => "(Impact) ",
                                    PermitDamageAttrEnum::All => "",
                                };
                                format!("{}{}", attr, part_loss.parts_loss_data.vital)
                            } else {
                                "".to_string()
                            };

                            Ok(html!(<tr class=hidden>
                                <td><span class=part_color.as_str()>"■"</span>{ text!("{}", part_name) }</td>
                                <td>{ text!("{}", part.vital) }</td>
                                <td>{ text!("{}", part_break) }</td>
                                <td>{ text!("{}", part_loss) }</td>
                                <td>{ gen_extractive_type(part.extractive_type) }</td>
                            </tr>))
                        }).collect::<Result<Vec<_>>>()?
                    }</tbody>
                </table>
                </section>

                <section>
                <h2 class="subtitle">
                    "Abnormal status"
                </h2>
                <div>
                    <input type="checkbox" onclick="onCheckDisplay(this, 'mh-ride-cond', 'mh-default-cond')" id="mh-ride-cond-check"/>
                    <label for="mh-ride-cond-check">"Display data for riding"</label>
                </div>
                <div>
                    <input type="checkbox" onclick="onCheckDisplay(this, 'mh-no-preset', 'mh-preset')" id="mh-preset-check"/>
                    <label for="mh-preset-check">"Don't override with preset data"</label>
                </div>
                <table>
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

                        {gen_condition_paralyze(true, monster.condition_damage_data.paralyze_data.or_preset(condition_preset)?, monster.condition_damage_data.use_paralyze)}
                        {gen_condition_sleep(true, monster.condition_damage_data.sleep_data.or_preset(condition_preset)?, monster.condition_damage_data.use_sleep)}
                        {gen_condition_stun(true, monster.condition_damage_data.stun_data.or_preset(condition_preset)?, monster.condition_damage_data.use_stun)}
                        {gen_condition_stamina(true, monster.condition_damage_data.stamina_data.or_preset(condition_preset)?, monster.condition_damage_data.use_stamina)}

                        {gen_condition_flash(false, &monster.condition_damage_data.flash_data, monster.condition_damage_data.use_flash)}
                        {gen_condition_flash(true, monster.condition_damage_data.flash_data.or_preset(condition_preset)?, monster.condition_damage_data.use_flash)}

                        {gen_condition_poison(false, &monster.condition_damage_data.poison_data, monster.condition_damage_data.use_poison)}
                        {gen_condition_blast(false, &monster.condition_damage_data.blast_data, monster.condition_damage_data.use_blast)}

                        {gen_condition_poison(true, monster.condition_damage_data.poison_data.or_preset(condition_preset)?, monster.condition_damage_data.use_poison)}
                        {gen_condition_blast(true, monster.condition_damage_data.blast_data.or_preset(condition_preset)?, monster.condition_damage_data.use_blast)}

                        {gen_condition_ride(&monster.condition_damage_data.marionette_data, monster.condition_damage_data.use_ride)}

                        {gen_condition_water(false, &monster.condition_damage_data.water_data, monster.condition_damage_data.use_water)}
                        {gen_condition_fire(false, &monster.condition_damage_data.fire_data, monster.condition_damage_data.use_fire)}
                        {gen_condition_ice(false, &monster.condition_damage_data.ice_data, monster.condition_damage_data.use_ice)}
                        {gen_condition_thunder(false, &monster.condition_damage_data.thunder_data, monster.condition_damage_data.use_thunder)}
                        {gen_condition_fall_trap(false, &monster.condition_damage_data.fall_trap_data, monster.condition_damage_data.use_fall_trap)}
                        {gen_condition_fall_quick_sand(false, &monster.condition_damage_data.fall_quick_sand_data, monster.condition_damage_data.use_fall_quick_sand)}
                        {gen_condition_fall_otomo_trap(false, &monster.condition_damage_data.fall_otomo_trap_data, monster.condition_damage_data.use_fall_otomo_trap)}
                        {gen_condition_shock_trap(false, &monster.condition_damage_data.shock_trap_data, monster.condition_damage_data.use_shock_trap)}
                        {gen_condition_shock_otomo_trap(false, &monster.condition_damage_data.shock_otomo_trap_data, monster.condition_damage_data.use_shock_otomo_trap)}
                        {gen_condition_capture(false, &monster.condition_damage_data.capture_data, monster.condition_damage_data.use_capture)}
                        {gen_condition_dung(false, &monster.condition_damage_data.koyashi_data, monster.condition_damage_data.use_dung)}
                        {gen_condition_steel_fang(false, &monster.condition_damage_data.steel_fang_data, monster.condition_damage_data.use_steel_fang)}

                        {gen_condition_water(true, monster.condition_damage_data.water_data.or_preset(condition_preset)?, monster.condition_damage_data.use_water)}
                        {gen_condition_fire(true, monster.condition_damage_data.fire_data.or_preset(condition_preset)?, monster.condition_damage_data.use_fire)}
                        {gen_condition_ice(true, monster.condition_damage_data.ice_data.or_preset(condition_preset)?, monster.condition_damage_data.use_ice)}
                        {gen_condition_thunder(true, monster.condition_damage_data.thunder_data.or_preset(condition_preset)?, monster.condition_damage_data.use_thunder)}
                        {gen_condition_fall_trap(true, monster.condition_damage_data.fall_trap_data.or_preset(condition_preset)?, monster.condition_damage_data.use_fall_trap)}
                        {gen_condition_fall_quick_sand(true, monster.condition_damage_data.fall_quick_sand_data.or_preset(condition_preset)?, monster.condition_damage_data.use_fall_quick_sand)}
                        {gen_condition_fall_otomo_trap(true, monster.condition_damage_data.fall_otomo_trap_data.or_preset(condition_preset)?, monster.condition_damage_data.use_fall_otomo_trap)}
                        {gen_condition_shock_trap(true, <ShockTrapDamageData as ConditionDamage<PresetShockTrapData>>::or_preset(&monster.condition_damage_data.shock_trap_data, condition_preset)?, monster.condition_damage_data.use_shock_trap)}
                        {gen_condition_shock_otomo_trap(true, <ShockTrapDamageData as ConditionDamage<PresetShockOtomoTrapData>>::or_preset(&monster.condition_damage_data.shock_trap_data, condition_preset)?, monster.condition_damage_data.use_shock_otomo_trap)}
                        {gen_condition_capture(true, monster.condition_damage_data.capture_data.or_preset(condition_preset)?, monster.condition_damage_data.use_capture)}
                        {gen_condition_dung(true, monster.condition_damage_data.koyashi_data.or_preset(condition_preset)?, monster.condition_damage_data.use_dung)}
                        {gen_condition_steel_fang(true, monster.condition_damage_data.steel_fang_data.or_preset(condition_preset)?, monster.condition_damage_data.use_steel_fang)}
                    </tbody>
                </table>
                </section>

                </div> </div> </main>
            </body>
        </html>: String
    );

    let file = PathBuf::from(folder).join(format!("{:03}_{:02}.html", monster.id, monster.sub_id));
    write(file, doc.to_string())?;
    Ok(())
}
