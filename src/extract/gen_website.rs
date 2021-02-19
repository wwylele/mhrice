use super::pedia::*;
use crate::msg::*;
use crate::rsz::*;
use anyhow::*;
use chrono::prelude::*;
use std::convert::TryInto;
use std::fs::{create_dir, remove_dir_all, write};
use std::path::*;
use typed_html::{dom::*, elements::*, html, text, types::*};

fn head_common() -> Vec<Box<dyn MetadataContent<String>>> {
    vec![
        html!(<meta charset="UTF-8" />),
        html!(<link rel="icon" type="image/png" href="/favicon.png" />),
        html!(<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css" />),
        html!(<link rel="stylesheet" href="/mhrice.css" />),
        html!(<script src="https://kit.fontawesome.com/ceb13a2ba1.js" crossorigin="anonymous" />),
        html!(<script src="/mhrice.js" crossorigin="anonymous" />),
    ]
}

fn navbar() -> Box<dyn FlowContent<String>> {
    html!(<div>
        <nav class="navbar is-primary" role="navigation"> <div class="container">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    "MHRice 🍚"
                </a>

                <a class="navbar-item" href="/monster.html">
                    "Monsters"
                </a>

                <a class="navbar-item" href="/about.html">
                    "About"
                </a>

                <a class="navbar-burger" data-target="navbarBasicExample">
                    <span></span>
                    <span></span>
                    <span></span>
                </a>
            </div>
        </div> </nav>
    </div>)
}

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

fn gen_condition_base(data: ConditionDamageDataBase) -> Vec<Box<dyn TableColumnContent<String>>> {
    vec![
        html!(<td>
            <span class="mh-default-cond">{text!("{} (+{}) → {}",
                data.default_stock.default_limit, data.default_stock.add_limit, data.default_stock.max_limit)}
            </span>
            <span class="mh-ride-cond mh-hidden">{text!("{} (+{}) → {}",
                data.ride_stock.default_limit, data.ride_stock.add_limit, data.ride_stock.max_limit)}
            </span>
        </td>),
        html!(<td>
            <span class="mh-default-cond">{text!("{} / {} sec",
                data.default_stock.sub_value, data.default_stock.sub_interval)}</span>
            <span class="mh-ride-cond mh-hidden">{text!("{} / {} sec",
                data.ride_stock.sub_value, data.ride_stock.sub_interval)}</span>
        </td>),
        html!(<td>{text!("{}", data.max_stock)}</td>),
        html!(<td>{text!("{} sec (-{} sec) → {} sec",
            safe_float(data.active_time), data.sub_active_time, data.min_active_time)}</td>),
        html!(<td>{text!("+{} sec", data.add_tired_time)}</td>),
        html!(<td>{text!("{} / {} sec", data.damage, data.damage_interval)}</td>),
    ]
}

fn gen_disabled(used: ConditionDamageDataUsed) -> &'static str {
    match used {
        ConditionDamageDataUsed::Use => "",
        ConditionDamageDataUsed::NotUse => "mh-disabled",
    }
}

fn gen_condition_paralyze(
    data: ParalyzeDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Paralyze"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset={}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_sleep(
    data: SleepDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Sleep"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_stun(
    data: StunDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Stun"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_stamina(
    data: StaminaDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Exhaust"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Stamina reduction = {}, Preset={}", data.sub_stamina, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_flash(
    data: FlashDamageData,
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

    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Flash"</td>
            { gen_condition_base(data.base) }
            <td>
            { data.damage_lvs.into_iter().map(|lv| {
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
    data: PoisonDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Poison"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_blast(
    data: BlastDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Blast"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Blast damage = {}, Preset = {}", data.blast_damage, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_ride(
    data: MarionetteStartDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let use_data = match data.use_data {
        UseDataType::Common => "common",
        UseDataType::Unique => "unique",
    };
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Ride"</td>
            { gen_condition_base(data.base) }
            <td> {text!("{}, Nora first limit = {}", use_data, data.nora_first_limit)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_water(
    data: WaterDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Water"</td>
            { gen_condition_base(data.base) }
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
    data: FireDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Fire"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Hit-damage rate = {}, Preset = {}", data.hit_damage_rate, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_ice(
    data: IceDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Ice"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Motion speed rate = {}, Preset = {}", data.motion_speed_rate, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_thunder(
    data: ThunderDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Thunder"</td>
            { gen_condition_base(data.base) }
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
    data: FallTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Fall trap"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fall_quick_sand(
    data: FallQuickSandDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td class="mh-spoiler">"Quick sand"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_fall_otomo_trap(
    data: FallOtomoTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td class="mh-spoiler">"Buddy fall trap"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Poison stacking = {}, Preset = {}",
                data.already_poison_stock_value, data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_shock_trap(
    data: ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Shock trap"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_shock_otomo_trap(
    data: ShockTrapDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td class="mh-spoiler">"Buddy shock trap"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_capture(
    data: CaptureDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Capture"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_dung(
    data: KoyashiDamageData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td>"Dung"</td>
            { gen_condition_base(data.base) }
            <td> {text!("Preset = {}", data.preset_type)} </td>
        </tr>
    );
    Ok(content)
}

fn gen_condition_steel_fang(
    data: SteelFangData,
    used: ConditionDamageDataUsed,
) -> Result<Box<tr<String>>> {
    let content = html!(
        <tr class=gen_disabled(used)>
            <td class="mh-spoiler">"\"Steel fang\""</td>
            { gen_condition_base(data.base) }
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

fn gen_monster(
    is_large: bool,
    monster: Monster,
    monster_aliases: &Msg,
    folder: &Path,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monster {:03} - MHRice", monster.id)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">{
                    if is_large {
                        let name_name = format!("Alias_EnemyIndex{:03}",
                            monster.boss_init_set_data.as_ref()
                            .context(format!("Cannot found boss_init_set for monster {}", monster.id))?
                            .enemy_type);
                        let name = &monster_aliases.get_entry(&name_name)
                            .context(format!("Cannot found name for monster {}", monster.id))?
                            .content[1];
                        text!("{}", name)
                    } else {
                        text!("Monster {:03}", monster.id)
                    }
                }</h1>
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

                <section class="section">
                <h2 class="subtitle">"Hitzone data"</h2>
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
                        monster.meat_data.meat_container.into_iter()
                            .enumerate().flat_map(|(part, meats)| {

                            let span = meats.meat_group_info.len();
                            let mut part_common: Option<Vec<Box<td<String>>>> = Some(vec![
                                html!(<td rowspan={span}>{ text!("{}", part) }</td>),
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

                            let hidden: SpacedSet<Class> = if invalid {
                                "mh-invalid-meat mh-hidden"
                            } else {
                                ""
                            }.try_into().unwrap();

                            meats.meat_group_info.into_iter().enumerate()
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
                <div>
                    <input type="checkbox" onclick="onCheckDisplay(this, 'mh-invalid-part', null)" id="mh-invalid-part-check"/>
                    <label for="mh-invalid-part-check">"Display invalid parts"</label>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>"Part"</th>
                            <th>"Stagger"</th>
                            <th>"Extract"</th>
                        </tr>
                    </thead>
                    <tbody>{
                        monster.data_tune.enemy_parts_data.into_iter().enumerate().map(|(index, part)| {
                            let hidden: SpacedSet<Class> = if part.extractive_type == ExtractiveType::None {
                                "mh-invalid-part mh-hidden"
                            } else {
                                ""
                            }.try_into().unwrap();
                            html!(<tr class=hidden>
                                <td>{ text!("{}", index) }</td>
                                <td>{ text!("{}", part.vital) }</td>
                                <td>{ gen_extractive_type(part.extractive_type) }</td>
                            </tr>)
                        })
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
                        {gen_condition_paralyze(monster.condition_damage_data.paralyze_data, monster.condition_damage_data.use_paralyze)}
                        {gen_condition_sleep(monster.condition_damage_data.sleep_data, monster.condition_damage_data.use_sleep)}
                        {gen_condition_stun(monster.condition_damage_data.stun_data, monster.condition_damage_data.use_stun)}
                        {gen_condition_stamina(monster.condition_damage_data.stamina_data, monster.condition_damage_data.use_stamina)}
                        {gen_condition_flash(monster.condition_damage_data.flash_data, monster.condition_damage_data.use_flash)}
                        {gen_condition_poison(monster.condition_damage_data.poison_data, monster.condition_damage_data.use_poison)}
                        {gen_condition_blast(monster.condition_damage_data.blast_data, monster.condition_damage_data.use_blast)}
                        {gen_condition_ride(monster.condition_damage_data.marionette_data, monster.condition_damage_data.use_ride)}
                        {gen_condition_water(monster.condition_damage_data.water_data, monster.condition_damage_data.use_water)}
                        {gen_condition_fire(monster.condition_damage_data.fire_data, monster.condition_damage_data.use_fire)}
                        {gen_condition_ice(monster.condition_damage_data.ice_data, monster.condition_damage_data.use_ice)}
                        {gen_condition_thunder(monster.condition_damage_data.thunder_data, monster.condition_damage_data.use_thunder)}
                        {gen_condition_fall_trap(monster.condition_damage_data.fall_trap_data, monster.condition_damage_data.use_fall_trap)}
                        {gen_condition_fall_quick_sand(monster.condition_damage_data.fall_quick_sand_data, monster.condition_damage_data.use_fall_quick_sand)}
                        {gen_condition_fall_otomo_trap(monster.condition_damage_data.fall_otomo_trap_data, monster.condition_damage_data.use_fall_otomo_trap)}
                        {gen_condition_shock_trap(monster.condition_damage_data.shock_trap_data, monster.condition_damage_data.use_shock_trap)}
                        {gen_condition_shock_otomo_trap(monster.condition_damage_data.shock_otomo_trap_data, monster.condition_damage_data.use_shock_otomo_trap)}
                        {gen_condition_capture(monster.condition_damage_data.capture_data, monster.condition_damage_data.use_capture)}
                        {gen_condition_dung(monster.condition_damage_data.koyashi_data, monster.condition_damage_data.use_dung)}
                        {gen_condition_steel_fang(monster.condition_damage_data.steel_fang_data, monster.condition_damage_data.use_steel_fang)}
                    </tbody>
                </table>
                </section>

                </div> </div> </main>
            </body>
        </html>: String
    );

    let file = PathBuf::from(folder).join(format!("{:03}.html", monster.id));
    write(file, doc.to_string())?;
    Ok(())
}

pub fn gen_monsters(
    monsters: Vec<Monster>,
    small_monsters: Vec<Monster>,
    monster_names: &Msg,
    monster_aliases: &Msg,
    root: &Path,
) -> Result<()> {
    let monsters_path = root.join("monster.html");

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"Monsters"</h1>
                <section class="section">
                <h2 class="subtitle">"Large monsters"</h2>
                <ul>{
                    monsters.iter().map(|monster| {
                        Ok(html!{<li>
                            <a href={format!("/monster/{:03}.html", monster.id)}>{
                                let name_name = format!("EnemyIndex{:03}",
                                    monster.boss_init_set_data.as_ref()
                                    .context(format!("Cannot found boss_init_set for monster {}", monster.id))?
                                    .enemy_type);
                                let name = &monster_names.get_entry(&name_name)
                                    .context(format!("Cannot found name for monster {}", monster.id))?
                                    .content[1];
                                text!("{}", name)
                            }</a>
                        </li>})
                    }).collect::<Result<Vec<_>>>()?
                }</ul>
                </section>
                <section class="section">
                <h2 class="subtitle">"Small monsters"</h2>
                <ul>{
                    small_monsters.iter().map(|monster| {
                        html!{<li>
                            <a href={format!("/small-monster/{:03}.html", monster.id)}>{
                                text!("Small monster {:03}", monster.id)
                            }</a>
                        </li>}
                    })
                }</ul>
                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    write(&monsters_path, doc.to_string())?;

    let monster_path = root.join("monster");
    create_dir(&monster_path)?;
    for monster in monsters {
        gen_monster(true, monster, &monster_aliases, &monster_path)?;
    }

    let monster_path = root.join("small-monster");
    create_dir(&monster_path)?;
    for monster in small_monsters {
        gen_monster(false, monster, &monster_aliases, &monster_path)?;
    }
    Ok(())
}

pub fn gen_about(root: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"About MHRice"</h1>
                <p>
                "MHRice is an information site for Monster Hunter Rise, displaying data extracted from the game."
                </p>
                <p>
                "MHRice website is generated from the open source MHRice project."
                </p>
                <p>
                <a class="button" href="https://github.com/wwylele/mhrice" target="_blank" rel=["noopener", "noreferrer"]>
                    <span class="icon">
                        <i class="fab fa-github"></i>
                    </span>
                    <span>"Visit MHRice on Github"</span>
                </a>
                </p>
                <section class="section">
                <h2 class="subtitle">"Build information"</h2>
                <ul>
                    <li>"Git hash: " <span class="is-family-monospace">{
                        text!("{}{}",
                            crate::built_info::GIT_COMMIT_HASH.unwrap_or("unknown"),
                            if crate::built_info::GIT_DIRTY == Some(true) {
                                "-dirty"
                            } else {
                                ""
                            }
                        )
                    }</span></li>
                    <li>{text!("Update time: {}", Utc::now())}</li>
                </ul>
                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    let about_path = root.join("about.html");
    write(&about_path, doc.to_string())?;

    Ok(())
}

pub fn gen_static(root: &Path) -> Result<()> {
    write(
        root.to_path_buf().join("mhrice.css"),
        include_bytes!("static/mhrice.css"),
    )?;
    write(
        root.to_path_buf().join("mhrice.js"),
        include_bytes!("static/mhrice.js"),
    )?;
    write(
        root.to_path_buf().join("favicon.png"),
        include_bytes!("static/favicon.png"),
    )?;
    Ok(())
}

pub fn gen_website(pedia: Pedia, output: &str) -> Result<()> {
    let root = PathBuf::from(output);
    if root.exists() {
        remove_dir_all(&root)?;
    }
    create_dir(&root)?;

    gen_monsters(
        pedia.monsters,
        pedia.small_monsters,
        &pedia.monster_names,
        &pedia.monster_aliases,
        &root,
    )?;
    gen_about(&root)?;
    gen_static(&root)?;
    Ok(())
}
