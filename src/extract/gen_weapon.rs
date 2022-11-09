use super::gen_common::*;
use super::gen_hyakuryu_skill::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::HashSet;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_weapon_icon(weapon: &WeaponBaseData, white: bool) -> Box<div<String>> {
    let icon = format!("/resources/equip/{:03}", weapon.id.icon_index());
    let rare = if white {
        RareTypes(1)
    } else {
        weapon.rare_type
    };
    gen_rared_icon(rare, &icon)
}

pub fn gen_weapon_label<Param>(weapon: &Weapon<Param>) -> Box<a<String>>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let main = weapon.param.to_base();
    let link = format!("/weapon/{}.html", main.id.to_tag());
    html!(
        <a href={link} class="mh-icon-text">
            {gen_weapon_icon(main, false)}
            <span>{gen_multi_lang(weapon.name)}</span>
        </a>
    )
}

#[allow(unused_variables)]
fn no<T, Base>(t: &T) -> Option<&Base> {
    None
}

#[allow(clippy::unnecessary_wraps)]
fn yes<T, Base>(t: &T) -> Option<&Base>
where
    T: ToBase<Base>,
{
    Some(t.to_base())
}

fn gen_craft_row(
    pedia_ex: &PediaEx,
    label: Box<td<String>>,
    cost: Option<u32>,
    data: &WeaponCraftingData,
    output: Option<(&[ItemId], &[u32])>,
) -> Box<tr<String>> {
    let cost = if let Some(cost) = cost {
        html!(<td>{text!("{}z", cost)}</td>)
    } else {
        html!(<td></td>) // TODO: what this should be for layered?
    };
    let category = gen_category(pedia_ex, data.material_category, data.material_category_num);

    let materials = gen_materials(pedia_ex, &data.item, &data.item_num, data.item_flag);

    let output = if let Some((output_item, output_item_num)) = output {
        gen_materials(pedia_ex, output_item, output_item_num, ItemId::None)
    } else {
        html!(<td>"-"</td>)
    };

    html!(<tr>
        {label}
        <td>{gen_progress(data.progress_flag, pedia_ex)}</td>
        <td>{(data.enemy_flag != EmTypes::Em(0)).then(
            ||gen_monster_tag(pedia_ex, data.enemy_flag, false, false, false)
        )}</td>
        {cost}
        {category}
        {materials}
        {output}
    </tr>)
}

fn display_bullet_type(bullet: BulletType) -> &'static str {
    match bullet {
        BulletType::None => "<None>",
        BulletType::Normal1 => "Normal Ammo 1",
        BulletType::Normal2 => "Normal Ammo 2",
        BulletType::Normal3 => "Normal Ammo 3",
        BulletType::Kantsu1 => "Pierce Ammo 1",
        BulletType::Kantsu2 => "Pierce Ammo 2",
        BulletType::Kantsu3 => "Pierce Ammo 3",
        BulletType::SanW1 => "Spread Ammo 1",
        BulletType::SanW2 => "Spread Ammo 2",
        BulletType::SanW3 => "Spread Ammo 3",
        BulletType::SanO1 => "Shrapnel Ammo 1",
        BulletType::SanO2 => "Shrapnel Ammo 2",
        BulletType::SanO3 => "Shrapnel Ammo 3",
        BulletType::Tekko1 => "Sticky Ammo 1",
        BulletType::Tekko2 => "Sticky Ammo 2",
        BulletType::Tekko3 => "Sticky Ammo 3",
        BulletType::Kakusan1 => "Cluster Bomb 1",
        BulletType::Kakusan2 => "Cluster Bomb 2",
        BulletType::Kakusan3 => "Cluster Bomb 3",
        BulletType::Poison1 => "Poison Ammo 1",
        BulletType::Poison2 => "Poison Ammo 2",
        BulletType::Paralyze1 => "Paralysis Ammo 1",
        BulletType::Paralyze2 => "Paralysis Ammo 2",
        BulletType::Sleep1 => "Sleep Ammo 1",
        BulletType::Sleep2 => "Sleep Ammo 2",
        BulletType::Genki1 => "Exhaust Ammo 1",
        BulletType::Genki2 => "Exhaust Ammo 2",
        BulletType::Heal1 => "Recover Ammo 1",
        BulletType::Heal2 => "Recover Ammo 2",
        BulletType::Kijin => "Demon Ammo",
        BulletType::Kouka => "Amor Ammo",
        BulletType::Fire => "Flaming Ammo",
        BulletType::FireKantsu => "Piercing Fire Ammo",
        BulletType::Water => "Water Ammo",
        BulletType::WaterKantsu => "Piercing Water Ammo",
        BulletType::Ice => "Freeze Ammo",
        BulletType::IceKantsu => "Piercing Ice Ammo",
        BulletType::Thunder => "Thunder Ammo",
        BulletType::ThunderKantsu => "Piercing Thunder Ammo",
        BulletType::Dragon => "Dragon Ammo",
        BulletType::DragonKantsu => "Piercing Drago Ammo",
        BulletType::Zanretsu => "Slicing Ammo",
        BulletType::Ryugeki => "Wyvern Ammo",
        BulletType::Capture => "Tranq Ammo",
        BulletType::Setti => "<Setti>",
        BulletType::Gatling => "<Gatling>",
        BulletType::Snipe => "<Snipe>",
        BulletType::GatlingHeal => "<GatlingHeal>",
        BulletType::SnipeHeal => "<SnipeHeal>",
        BulletType::WireBullet => "<WireBullet>",
        BulletType::FullAuto => "<FullAuto>",
        BulletType::Max => "<Max>",
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn gen_weapon<Param>(
    hash_store: &HashStore,
    weapon: &Weapon<Param>,
    weapon_tree: &WeaponTree<'_, Param>,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
    has_element: fn(&Param) -> Option<&ElementWeaponBaseData>,
    has_second_element: fn(&Param) -> Option<&DualBladesBaseUserDataParam>,
    has_close_range: fn(&Param) -> Option<&CloseRangeWeaponBaseData>,
    has_horn: fn(&Param) -> Option<&HornBaseUserDataParam>,
    has_bullet: fn(&Param) -> Option<&BulletWeaponBaseUserDataParam>,
    has_lbg: fn(&Param) -> Option<&LightBowgunBaseUserDataParam>,
    has_bow: fn(&Param) -> Option<&BowBaseUserDataParam>,
    special: Option<fn(&Param) -> Vec<Box<p<String>>>>,
) -> Result<()>
where
    Param: ToBase<MainWeaponBaseData>,
{
    toc_sink.add(weapon.name);

    let param = weapon.param;
    let main = param.to_base();
    let first_element = has_element(param);
    let second_element = has_second_element(param);
    let close_range = has_close_range(param);
    let horn = has_horn(param);
    let bullet = has_bullet(param);
    let lbg = has_lbg(param);
    let bow = has_bow(param);

    let bowgun_param = bullet.into_iter().flat_map(|bullet| {
        [
            html!(<p class="mh-kv"><span>"Fluctuation"</span>
            <span>{ text!("{:?}", bullet.fluctuation) }</span></p>),
            html!(<p class="mh-kv"><span>"Reload"</span>
            <span>{ text!("{}", bullet.reload) }</span></p>),
            html!(<p class="mh-kv"><span>"Recoil"</span>
            <span>{ text!("{}", bullet.recoil) }</span></p>),
            html!(<p class="mh-kv"><span>"Kakusan type"</span>
            <span>{ text!("{:?}", bullet.kakusan_type) }</span></p>),
        ]
    });

    let bow_param = bow.into_iter().flat_map(|bow| {
        let charge_type: Vec<String> = bow
            .bow_charge_type_list
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();
        [
            html!(<p class="mh-kv"><span>"Default charge lv"</span>
            <span>{ text!("{}", bow.bow_default_charge_lv_limit.0) }</span></p>),
            html!(<p class="mh-kv"><span>"Charge type"</span>
            <span>{ text!("{}", charge_type.join("-")) }</span></p>),
            html!(<p class="mh-kv"><span>"Curve type"</span>
            <span>{ text!("{:?}", bow.bow_curve_type) }</span></p>),
        ]
    });

    let sharpness = close_range.map(|close_range| {
        let highest = close_range
            .sharpness_val_list
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &s)| s != 0)
            .map_or(0, |(i, _)| i);
        let mut sharpness_pos = 0;
        html!(
        <p class="mh-kv"><span>"Sharpness"</span>
        <span>
        <span class="mh-sharpness-bar">
            {
                close_range.sharpness_val_list.iter().enumerate().map(|(i, &s)|{
                    let pos = sharpness_pos as f32 * 0.25;
                    sharpness_pos += s;
                    let width = s as f32 * 0.25;
                    let class = format!("mh-sharpness mh-sharpness-color-{}", i);
                    let style = format!("left:{}%;width:{}%;", pos, width);
                    html!(<span class={class.as_str()} style={style.as_str()} />)
                })
            }
            {
                close_range.takumi_val_list.iter().enumerate().map(|(i, &s)|{
                    let pos = sharpness_pos as f32 * 0.25;
                    sharpness_pos += s;
                    let width = s as f32 * 0.25;
                    let class = format!("mh-sharpness-half mh-sharpness-color-{}", i + highest);
                    let style = format!("left:{}%;width:{}%;", pos, width);
                    html!(<span class={class.as_str()} style={style.as_str()} />)
                })
            }
        </span>
        </span></p>)
    });

    struct BowBottleMap {
        name: &'static str,
        power_up: Option<BottlePowerUpTypes>,
    }

    const BOW_BOTTLE_MAP: [BowBottleMap; 7] = [
        BowBottleMap {
            name: "Close range",
            power_up: Some(BottlePowerUpTypes::ShortRange),
        },
        BowBottleMap {
            name: "Power",
            power_up: None,
        },
        BowBottleMap {
            name: "Poison",
            power_up: Some(BottlePowerUpTypes::Poison),
        },
        BowBottleMap {
            name: "Paralyze",
            power_up: Some(BottlePowerUpTypes::Paralyze),
        },
        BowBottleMap {
            name: "Sleep",
            power_up: Some(BottlePowerUpTypes::Sleep),
        },
        BowBottleMap {
            name: "Blast",
            power_up: None,
        },
        BowBottleMap {
            name: "Exhaust",
            power_up: None,
        },
    ];

    let more_bullet: HashSet<BulletType> = main
        .hyakuryu_skill_id_list
        .iter()
        .flat_map(|skill| {
            pedia_ex
                .hyakuryu_skills
                .get(skill)
                .and_then(|skill| skill.data)
                .map(|skill| {
                    skill
                        .add_bullet_type_list
                        .iter()
                        .cloned()
                        .filter(|&bullet| bullet != BulletType::None)
                })
                .into_iter()
                .flatten()
        })
        .collect();

    let rapid = lbg.map_or(&[][..], |lbg| &lbg.rapid_shot_list[..]);

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            <pre>
            {weapon.explain.as_ref().map(|e|gen_multi_lang(e))}
            </pre></section>
        ),
    });

    sections.push(Section {
        title: "Stat".to_owned(),
        content: html!(<section id="s-stat">
        <h2 >"Stat"</h2>
        <div class="mh-kvlist">
        <p class="mh-kv"><span>"Attack"</span>
        <span>{text!("{}", main.atk)}</span></p>
        <p class="mh-kv"><span>"Affinity"</span>
        <span>{text!("{}%", main.critical_rate)}</span></p>
        <p class="mh-kv"><span>"Defense"</span>
        <span>{text!("{}", main.def_bonus)}</span></p>
        <p class="mh-kv"><span>"Slot"</span>
        <span>{gen_slot(&main.slot_num_list, false)}</span></p>
        <p class="mh-kv"><span>"Rampage Slot"</span>
        <span>{gen_slot(&main.hyakuryu_slot_num_list, true)}</span></p>

        {first_element.map(|first_element| html!(
            <p class="mh-kv"><span>"Element"</span>
            <span>
                <span>{text!("{:?} {}",
                    first_element.main_element_type,
                    first_element.main_element_val
                )}</span>
                {
                    second_element.map(|second_element| html!(
                        <span>{text!(" {:?} {}",
                            second_element.sub_element_type,
                            second_element.sub_element_val
                        )}</span>
                    ))
                }
            </span></p>
        ))}

        { sharpness }

        { bowgun_param }

        { bow_param }

        { special.map(|special|special(param)).into_iter().flatten() }

        </div>
        </section>),
    });

    sections.push(Section {
        title: "Rampage skills".to_owned(),
        content: html!(<section id="s-rampage">
        <h2 >"Rampage skills"</h2>
        <ul> {
            let main_list = main.hyakuryu_skill_id_list.iter()
                .zip(std::iter::repeat(None));
            let ex_list = weapon.hyakuryu_weapon_buildup.iter()
                .flat_map(|(&slot_type, param)| {
                    param.buildup_id_list.iter().zip(std::iter::repeat(Some(slot_type)))
                });

            main_list.chain(ex_list)
            .filter(|(&skill, _)|skill != PlHyakuryuSkillId::None)
            .map(|(skill, slot_type)|{
                let hyakuryu_tag = slot_type.map(|s|html!(
                    <span class="tag">{text!("Slot {}", s)}</span>
                ));
                if let Some(skill) = pedia_ex.hyakuryu_skills.get(skill) {
                    html!(<li> {
                        gen_hyakuryu_skill_label(skill)
                    } {hyakuryu_tag} </li>)
                } else {
                    html!(<li>{ text!("Unknown {:?}", skill) }</li>)
                }
            })
        } </ul>
        </section>),
    });

    if let Some(horn) = horn {
        sections.push(Section {
            title: "Melody".to_owned(),
            content: html!(<section id="s-melody">
            <h2 >"Melody"</h2>
            <ul> {
                horn.horn_melody_type_list.iter().map(|id| {
                    html!(<li> {
                        if let Some(name) = pedia_ex.horn_melody.get(id) {
                            gen_multi_lang(name)
                        } else {
                            html!(<span>{ text!("[{}]", id) }</span>)
                        }
                    } </li>)
                })
            } </ul>
            </section>),
        })
    }

    if let Some(bullet) = bullet {
        sections.push(Section {
            title: "Ammo list".to_owned(),
            content: html!(<section id="s-ammo">
            <h2 >"Ammo list"</h2>
            <div class="mh-table"><table>
            <thead><tr>
                <th>"Ammo Type"</th>
                <th>"Capacity"</th>
                <th>"Shot Type"</th>
            </tr></thead>
            <tbody> {
                bullet.bullet_equip_flag_list.iter()
                    .zip(bullet.bullet_num_list.iter())
                    .zip(bullet.bullet_type_list.iter())
                    .enumerate()
                    .map(|(bullet_type, ((flag, num), shoot_type))|
                        (BulletType::from_raw(bullet_type as u32).unwrap(), *flag, *num, *shoot_type)
                    )
                    .filter(|(bullet_type, flag, _, _)|*flag || more_bullet.contains(bullet_type))
                    .map(|(bullet_type, flag, num, shoot_type)| {
                        let class = if flag {
                            ""
                        } else {
                            "mh-disabled"
                        };
                        let mut shoot_types = vec![];
                        let shoot_type = shoot_type.to_flags();
                        if shoot_type.moving_shot {
                            shoot_types.push("Moving shot")
                        }
                        if shoot_type.moving_reload {
                            shoot_types.push("Moving reload")
                        }
                        if shoot_type.single_auto {
                            shoot_types.push("Single shot auto reload")
                        }
                        if rapid.contains(&bullet_type) {
                            shoot_types.push("Rapid shot")
                        }
                        let shoot_types = shoot_types.join(", ");
                        html!(<tr class={class}>
                            <td>{ text!("{}", display_bullet_type(bullet_type)) }</td>
                            <td>{ text!("{}", num) }</td>
                            <td>{ text!("{}", shoot_types) }</td>
                        </tr>)
                    })
            }
            { lbg.map(|lbg| {
                html!(<tr><td>{ text!("{}", display_bullet_type(lbg.unique_bullet)) }</td></tr>)
            }) }
            </tbody>
            </table></div>
            </section>),
        })
    }

    if let Some(bow) = bow {
        sections.push(Section {
            title: "Bottle".to_owned(),
            content: html!(<section id="s-bottle">
            <h2 >"Bottle"</h2>
            <ul> {
                BOW_BOTTLE_MAP.iter().enumerate().filter(|&(i,_)| {
                    bow.bow_bottle_equip_flag_list[i]
                }).map(|(_, bottle)| {
                    let power_up = if let Some(power_up) = bottle.power_up {
                        bow.bow_bottle_power_up_type_list.contains(&power_up)
                    } else {
                        false
                    }.then(|| html!(<span class="tag">"Power up"</span>));
                    html!(<li>{ text!("{}", bottle.name) }
                    {power_up}
                    </li>)
                })
            } </ul>
            </section>),
        })
    };

    sections.push(Section {
        title: "Crafting".to_owned(),
        content: html!(<section id="s-crafting">
        <h2 >"Crafting"</h2>
        { weapon.update.map(|update| {
            html!(<p>{text!("Unlock at: {} {} {}",
                update.village_progress.display().unwrap_or_default(),
                update.hall_progress.display().unwrap_or_default(),
                update.mr_progress.display().unwrap_or_default())}</p>)
        }) }
        <div class="mh-table"><table>
            <thead><tr>
                <th>""</th>
                <th>"Unlock at"</th>
                <th>"Key Monster"</th>
                <th>"Cost"</th>
                <th>"Categorized Material"</th>
                <th>"Material"</th>
                <th>"Output"</th>
            </tr></thead>
            <tbody>
                { (main.base.buy_val != 0).then(|| {
                    html!(<tr>
                        <td>"Buy"</td>
                        <td/>
                        <td/>
                        <td>{text!("{}z", main.base.buy_val)}</td>
                        <td/><td/><td/>
                    </tr>)
                })}
                {weapon.product.as_ref().map(|product| {
                    gen_craft_row(pedia_ex, html!(<td>"Forge"</td>), Some(main.base.base_val * 3 / 2),
                        &product.base, Some((&product.output_item, &product.output_item_num)))
                })}
                {weapon.process.as_ref().map(|process| {
                    let label = if let Some(parent) = weapon.parent {
                        let parent = weapon_tree.weapons.get(&parent).unwrap();
                        html!(<td>"Upgrade from " {gen_weapon_label(parent)}</td>)
                    } else {
                        html!(<td>"Upgrade from unknown"</td>)
                    };

                    gen_craft_row(pedia_ex, label, Some(main.base.base_val),
                        &process.base, Some((&process.output_item, &process.output_item_num)))
                })}
                {weapon.change.as_ref().map(|change| {
                    gen_craft_row(pedia_ex, html!(<td>"As layered (rampage weapon)"</td>), None,
                        &change.base, None)
                })}
                {weapon.overwear.as_ref().map(|data| {
                    let category = gen_category(pedia_ex, data.material_category, data.material_category_num);
                    let materials = gen_materials(pedia_ex, &data.item, &data.item_num, data.item_flag);
                    html!(<tr>
                        <td>"As layered"</td>
                        <td>{gen_progress(data.progress_flag, pedia_ex)}</td>
                        <td>{(data.enemy_flag != EmTypes::Em(0)).then(
                            ||gen_monster_tag(pedia_ex, data.enemy_flag, false, false, false)
                        )}</td>
                        <td>{text!("{}z", data.price)}</td>
                        {category}
                        {materials}
                        <td></td>
                    </tr>)
                })}
            </tbody>
        </table></div>
        </section>
    )});

    sections.push(Section {
        title: "Upgrade".to_owned(),
        content: html!(<section id="s-upgrade">
            <h2 >"Upgrade"</h2>
            <ul> {
                weapon.children.iter().map(|child| {
                    let weapon = weapon_tree.weapons.get(child).unwrap();
                    html!(<li>{gen_weapon_label(weapon)}</li>)
                })
            } </ul>
            </section>
        ),
    });

    if let (Some(table_no), Some(cost)) = (main.custom_table_no.0, main.custom_cost.0) {
        if table_no != 0 {
            let table = pedia_ex.weapon_custom_buildup.get(&table_no);
            let table = if let Some(table) = table {
                html!(<div class="mh-table"><table>
                <thead><tr>
                <th>"Category"</th>
                <th>"Level"</th>
                <th>{text!("Anomaly slots (available: {})", cost)}</th>
                <th>"Bonus"</th>
                <th>"Cost"</th>
                <th>"Categorized Material"</th>
                <th>"Material"</th>
                </tr></thead>
                <tbody>
                { pedia.custom_buildup_weapon_open.as_ref().and_then(
                    |m|m.param.iter().find(|m|m.rare == main.base.rare_type).map(|m|html!(<tr>
                    <td>"Enable"</td>
                    <td/>
                    <td/>
                    <td/>
                    <td>{text!("{}z", m.price)}</td>
                    {gen_category(pedia_ex, m.material_category, m.material_category_num)}
                    {gen_materials(pedia_ex, &m.item, &m.item_num, ItemId::Null)}
                </tr>))) }
                {
                table.categories.iter().flat_map(|(&category_id, category)| {
                    let rowspan = category.pieces.len();
                    let category_name = match category_id {
                        1 => text!("Attack boost"),
                        2 => text!("Affinity boost"),
                        3 => text!("Elemental boost"),
                        4 => text!("Status effect boost"),
                        5 => text!("Sharpness boost"),
                        6 => text!("Rampage slot upgrade"),
                        7 => text!("Add anomaly slot"),
                        8 => text!("Element/status boost"),
                        c => text!("{}", c)
                    };
                    let mut category_cell = Some(html!(<td rowspan={rowspan}>
                        { category_name }
                    </td>));
                    category.pieces.iter().map(move |(_, piece)| {
                        html!(<tr>
                        {category_cell.take()}
                        <td>{text!("{}", piece.data.lv)}</td>
                        <td>{text!("{}", piece.data.cost)}</td>
                        <td>
                        <ul class="mh-custom-lot"> {
                            piece.data.value_table.iter().zip(&piece.data.lot_table)
                            .filter(|(_, lot)| **lot != 0)
                            .map(|(value, &lot)| html!(<li> {
                                if lot != 100 {
                                    text!("{:+}, {}%", value, lot)
                                } else {
                                    text!("{:+}", value)
                                }
                            } </li>))
                        } </ul>
                        </td>
                        <td>{text!("{}z", piece.material.price)}</td>
                        {gen_category(pedia_ex, piece.material.material_category, piece.material.material_category_num)}
                        {gen_materials(pedia_ex, &piece.material.item, &piece.material.item_num, ItemId::Null)}

                        </tr>)
                    })
                })
                } </tbody>
                </table>
                </div>)
            } else {
                html!(<div>{text!("Unknown table {}", table_no)}</div>)
            };

            sections.push(Section {
                title: "Qurious crafting".to_owned(),
                content: html!(<section id="s-qurio">
                <h2>"Qurious crafting"</h2>
                {table}
                </section>),
            });
        }
    }

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>"Weapon - MHRice"</title>
                { head_common(hash_store) }
                { title_multi_lang(weapon.name) }
                { open_graph(Some(weapon.name), "",
                    weapon.explain, "", None, toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_weapon_icon(main, false)}
                    </div>
                    <h1> {gen_multi_lang(weapon.name)} </h1>
                </header>

                { sections.into_iter().map(|s|s.content) }

                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_tree_rec<Param>(weapon_tree: &WeaponTree<Param>, list: &[WeaponId]) -> Box<ul<String>>
where
    Param: ToBase<MainWeaponBaseData>,
{
    html!(<ul> {
        list.iter().map(|id| {
            let weapon = weapon_tree.weapons.get(id).unwrap();
            let mut filter_tags = vec![];
            if weapon.children.is_empty() {
                filter_tags.push("final");
            }
            if weapon.overwear.is_some() {
                filter_tags.push("layer");
            }
            if weapon.change.is_some() {
                filter_tags.push("rampage");
            }
            let filter = filter_tags.join(" ");
            html!(<li>
                <div class="mh-main-filter-item" data-filter={filter}>{
                    gen_weapon_label(weapon)
                }</div>
                { gen_tree_rec(weapon_tree, &weapon.children) }
            </li>)
        })
    } </ul>)
}

fn gen_tree<Param>(
    hash_store: &HashStore,
    weapon_tree: &WeaponTree<Param>,
    weapon_path: &impl Sink,
    tag: &str,
    name: &str,
) -> Result<()>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let mut list_path = weapon_path.create_html(&format!("{}.html", tag))?;

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{} - MHRice", name)}</title>
                { head_common(hash_store) }
                <style id="mh-main-list-style">""</style>
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1> {text!("{}", name)} </h1></header>
                <div>
                    <a href="/weapon.html"><span class="icon-text">
                    <span class="icon">
                    <i class="fas fa-arrow-right"></i>
                    </span>
                    <span>"go to other weapon classes"</span>
                    </span></a>
                </div>
                <div class="mh-filters"><ul>
                    <li id="mh-main-filter-button-all" class="is-active mh-main-filter-button">
                        <a>"All"</a></li>
                    <li id="mh-main-filter-button-final" class="mh-main-filter-button">
                        <a>"Final upgrade"</a></li>
                    <li id="mh-main-filter-button-layer" class="mh-main-filter-button">
                        <a>"Layered"</a></li>
                    <li id="mh-main-filter-button-rampage" class="mh-main-filter-button">
                        <a>"Layered for rampage"</a></li>
                </ul></div>
                <div class="mh-weapon-tree">
                { gen_tree_rec(weapon_tree, &weapon_tree.roots) }
                </div>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    list_path.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

#[allow(clippy::vec_box)]
fn slash_axe(param: &SlashAxeBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Bottle"</span>
    <span>{text!("{:?} {}", param.slash_axe_bottle_type,
            param.slash_axe_bottle_element_val)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn gun_lance(param: &GunLanceBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Type"</span>
    <span>{text!("{:?} {}", param.gun_lance_fire_type,
            param.gun_lance_fire_lv.0)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn insect_glaive(param: &InsectGlaiveBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Insect level"</span>
    <span>{text!("{}", param.insect_glaive_insect_lv.0)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn charge_axe(param: &ChargeAxeBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Bottle"</span>
    <span>{text!("{:?}", param.charge_axe_bottle_type)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn heavy_bowgun(param: &HeavyBowgunBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Unique bullet"</span>
    <span>{text!("{:?}", param.heavy_bowgun_unique_bullet_type)}</span>
    </p>)]
}

pub fn gen_weapons(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let path = output.sub_sink("weapon")?;

    let mut entry_label = vec![];

    macro_rules! weapon {
        ($label:ident, $name:expr,
            element:$element:ident,
            second_element:$second_element:ident,
            close_range:$close_range:ident,
            horn:$horn:ident,
            bullet:$bullet:ident,
            lbg:$lbg:ident,
            bow:$bow:ident,
            special:$special:expr
        ) => {{
            let entry_link = format!("/weapon/{}.html", stringify!($label));
            entry_label.push(html!(<li>
                <a href={entry_link.as_str()} class="mh-icon-text">
                    {
                        pedia_ex.$label.weapons.values().next().map(
                            |first|gen_weapon_icon(&first.param, true))
                    }
                    <span>{text!("{}", $name)}</span>
                </a>
            </li>));
            gen_tree(hash_store, &pedia_ex.$label, &path, stringify!($label), $name)?;
            for (weapon_id, weapon) in &pedia_ex.$label.weapons {
                let (file_path, toc_sink) =
                    path.create_html_with_toc(&format!("{}.html", weapon_id.to_tag()), toc)?;
                gen_weapon(
                    hash_store,
                    weapon,
                    &pedia_ex.$label,
                    pedia,
                    pedia_ex,
                    config,
                    file_path,
                    toc_sink,
                    $element,
                    $second_element,
                    $close_range,
                    $horn,
                    $bullet,
                    $lbg,
                    $bow,
                    $special,
                )?;
            }
        }};
    }

    weapon!(
        great_sword,
        "Great sword",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        short_sword,
        "Sword & shield",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        hammer,
        "Hammer",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        lance,
        "Lance",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        long_sword,
        "Long sword",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        slash_axe,
        "Switch axe",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(slash_axe)
    );
    weapon!(
        gun_lance,
        "Gunlance",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(gun_lance)
    );
    weapon!(
        dual_blades,
        "Dual Blades",
        element: yes,
        second_element: yes,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        horn,
        "Hunting horn",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: yes,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        insect_glaive,
        "Insect glaive",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(insect_glaive)
    );
    weapon!(
        charge_axe,
        "Charge blade",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(charge_axe)
    );
    weapon!(
        light_bowgun,
        "Light bowgun",
        element: no,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: yes,
        lbg: yes,
        bow: no,
        special: None
    );
    weapon!(
        heavy_bowgun,
        "Heavy bowgun",
        element: no,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: yes,
        lbg: no,
        bow: no,
        special: Some(heavy_bowgun)
    );
    weapon!(
        bow,
        "Bow",
        element: yes,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: no,
        lbg: no,
        bow: yes,
        special: None
    );

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Weapons - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1> "Weapons" </h1></header>
                <ul class="mh-item-list">
                {entry_label}
                </ul>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("weapon.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}
