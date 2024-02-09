use super::gen_common::*;
use super::gen_dlc::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_atomo_armor_label(piece: &OtArmor) -> Box<div<String>> {
    let icon = format!("resources/equip/{:03}", piece.param.id.icon_index());
    html!(<div class="mh-icon-text">
        { gen_rared_icon(piece.param.rare_type, &icon, [], false) }
        <span>{ gen_multi_lang(piece.name) }</span>
    </div>)
}

pub fn gen_atomo_weapon_label(piece: &OtWeapon) -> Box<div<String>> {
    let icon_index = match (piece.param.id, piece.param.atk_type) {
        (OtWeaponId::Airou(_), OtAtkTypes::Smash) => 11,
        (OtWeaponId::Airou(_), OtAtkTypes::Blow) => 12,
        (OtWeaponId::Dog(_), OtAtkTypes::Smash) => 32,
        (OtWeaponId::Dog(_), OtAtkTypes::Blow) => 33,
        (OtWeaponId::None, _) => 9,
    };

    let icon = format!("resources/equip/{icon_index:03}");
    html!(<div class="mh-icon-text">
        { gen_rared_icon(piece.param.rare_type, &icon, [], false) }
        <span>{ gen_multi_lang(piece.name) }</span>
    </div>)
}

fn gen_otomo_equip(
    hash_store: &HashStore,
    series: &OtEquipSeries,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    otomo_path: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let (mut output, mut toc_sink) =
        otomo_path.create_html_with_toc(&format!("{}.html", series.series.id.to_tag()), toc)?;

    toc_sink.add(series.name);
    let mut rarity = RareTypes(1);
    if let Some(head) = &series.head {
        toc_sink.add(head.name);
        rarity = head.param.rare_type;
    }
    if let Some(chest) = &series.chest {
        toc_sink.add(chest.name);
        rarity = chest.param.rare_type;
    }
    if let Some(weapon) = &series.weapon {
        toc_sink.add(weapon.name);
        rarity = weapon.param.rare_type;
    }

    let icon = match series.series.id {
        OtEquipSeriesId::Airou(_) => "resources/equip/010",
        OtEquipSeriesId::Dog(_) => "resources/equip/031",
    };

    let gen_armor_stat = |armor: &Option<OtArmor>| -> Option<Box<tr<String>>> {
        let armor = armor.as_ref()?;
        Some(html!(<tr>
            <td>{gen_atomo_armor_label(armor)}</td>
            <td>{text!("{}z", armor.param.sell_value)}</td>
            <td>{text!("Defense: {}", armor.param.def)}</td>
            <td>"Defense"
                <ul class="mh-buddy-gear-stat">
                    <li><img alt="Fire" src="resources/fire.png" class="mh-small-icon"/>
                        {text!("Fire: {}", armor.param.element_regist_list[0])}</li>
                    <li><img alt="Water" src="resources/water.png" class="mh-small-icon"/>
                        {text!("Water: {}", armor.param.element_regist_list[1])}</li>
                    <li><img alt="Thunder" src="resources/thunder.png" class="mh-small-icon"/>
                        {text!("Thunder: {}", armor.param.element_regist_list[2])}</li>
                    <li><img alt="Ice" src="resources/ice.png" class="mh-small-icon"/>
                        {text!("Ice: {}", armor.param.element_regist_list[3])}</li>
                    <li><img alt="Dragon" src="resources/dragon.png" class="mh-small-icon"/>
                        {text!("Dragon: {}", armor.param.element_regist_list[4])}</li>
                </ul>
            </td>
        </tr>))
    };

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Description"</th>
                </tr></thead>
                <tbody>
                    {series.head.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td><pre>{gen_multi_lang(p.explain)}</pre></td>
                    </tr>)})}
                    {series.chest.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td><pre>{gen_multi_lang(p.explain)}</pre></td>
                    </tr>)})}
                    {series.weapon.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_weapon_label(p)}</td>
                        <td><pre>{gen_multi_lang(p.explain)}</pre></td>
                    </tr>)})}
                </tbody>
            </table></div>
            </section>
        ),
    });

    sections.push(Section {
        title: "Stat".to_owned(),
        content: html!(
            <section id="s-stat">
            <h2 >"Stat"</h2>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Sell value"</th>
                    <th>"Physical"</th>
                    <th>"Element"</th>
                </tr></thead>
                <tbody>
                { gen_armor_stat(&series.head) }
                { gen_armor_stat(&series.chest) }
                { series.weapon.as_ref().map(|weapon|{
                    let atk_type = match weapon.param.atk_type {
                        OtAtkTypes::Smash => "Slash",
                        OtAtkTypes::Blow => "Impact"
                    };
                    let specialize = match weapon.param.specilize_type {
                        OtSpecializeTypes::Short => "Short range",
                        OtSpecializeTypes::Balance => "Balanced",
                        OtSpecializeTypes::Long => "Long range"
                    };
                    html!(<tr>
                    <td>
                        {gen_atomo_weapon_label(weapon)}
                        <span class="tag is-primary">{text!("{}", atk_type)}</span>
                        <span class="tag is-primary">{text!("{}", specialize)}</span>
                    </td>
                    <td>{text!("{}z", weapon.param.sell_value)}</td>
                    <td>
                        <div>{text!("Short: {} (Aff. {}%)",
                            weapon.param.atk_val_list[0], weapon.param.critical_rate_list[0])}</div>
                        <div>{text!("Long: {} (Aff. {}%)",
                            weapon.param.atk_val_list[1], weapon.param.critical_rate_list[1])}</div>
                        <div>{text!("Def bonus: {}", weapon.param.def_bonus)}</div>
                    </td>
                    { (weapon.param.element_type != ElementType::None).then(|| {
                        let (img, text) = match weapon.param.element_type {
                            ElementType::None => unreachable!(),
                            ElementType::Fire => ("fire", "Fire"),
                            ElementType::Water => ("water", "Water"),
                            ElementType::Thunder => ("thunder", "Thunder"),
                            ElementType::Ice => ("ice", "Ice"),
                            ElementType::Dragon => ("dragon", "Dragon"),
                            ElementType::Poison => ("poison", "Poison"),
                            ElementType::Sleep => ("sleep", "Sleep"),
                            ElementType::Paralyze => ("para", "Paralyze"),
                            ElementType::Bomb => ("blast", "Blast"),
                        };
                        let img = format!("resources/{img}.png");
                        html!(<td><div>
                            <img alt={text} src={img.as_str()} class="mh-small-icon"/>
                            {text!("{}", text)}
                        </div>
                        <div>{text!("Short: {}", weapon.param.element_val_list[0])}</div>
                        <div>{text!("Long: {}", weapon.param.element_val_list[1])}</div>
                        </td>)
                    })}

                    {(weapon.param.element_type == ElementType::None).then(||
                        html!(<td>"None"</td>)
                    )}
                </tr>)})}
                </tbody>
            </table></div>
            </section>
        ),
    });

    let dlc_add = pedia_ex
        .dlc
        .values()
        .filter_map(|dlc| dlc.add.map(|add| (gen_dlc_label(dlc), add)));

    let slc_add = pedia_ex
        .slc
        .iter()
        .filter_map(|(id, slc)| slc.add.map(|add| (gen_slc_label(id), add)));

    let dlc: Vec<Box<a<String>>> = dlc_add
        .chain(slc_add)
        .filter_map(|(label, add)| {
            if let Some(id) = series.head.as_ref().and_then(|p| p.overwear).map(|p| p.id) {
                if add.ot_overwear_id_list.contains(&id) {
                    return Some(label);
                }
            }
            if let Some(id) = series.chest.as_ref().and_then(|p| p.overwear).map(|p| p.id) {
                if add.ot_overwear_id_list.contains(&id) {
                    return Some(label);
                }
            }
            None
        })
        .collect();

    if !dlc.is_empty() {
        sections.push(Section {
            title: "DLC".to_owned(),
            content: html!(<section id="s-dlc">
            <h2 >"DLC"</h2>
            <ul class="mh-item-list">
            {dlc.into_iter().map(|dlc| html!(<li>
                {dlc}
                <span class="tag">"Layered"</span>
            </li>))}
            </ul>
            </section>),
        })
    }

    #[allow(clippy::get_first)]
    let three_item_condition = |ty: EvaluationTypeFor3Argument, item: &[ItemId]| {
        let item0 = item.get(0).filter(|&&i| i != ItemId::None);
        let item1 = item.get(1).filter(|&&i| i != ItemId::None);
        let item2 = item.get(2).filter(|&&i| i != ItemId::None);
        let item0 = if let Some(&item) = item0 {
            gen_item_label_from_id(item, pedia_ex)
        } else {
            text!("None")
        };
        let item1 = if let Some(&item) = item1 {
            gen_item_label_from_id(item, pedia_ex)
        } else {
            text!("None")
        };
        let item2 = if let Some(&item) = item2 {
            gen_item_label_from_id(item, pedia_ex)
        } else {
            text!("None")
        };

        match ty {
            EvaluationTypeFor3Argument::AndAnd => {
                html!(<div>{item0} " and " {item1} " and " {item2}</div>)
            }
            EvaluationTypeFor3Argument::OrOr => {
                html!(<div>{item0} " or " {item1} " or " {item2}</div>)
            }
            EvaluationTypeFor3Argument::AndOr => {
                html!(<div>"("{item0} " and " {item1} ") or " {item2}</div>)
            }
            EvaluationTypeFor3Argument::OrAnd => {
                html!(<div>"("{item0} " or " {item1} ") and " {item2}</div>)
            }
        }
    };

    sections.push(Section {
        title: "Crafting".to_owned(),
        content: html!(
            <section id="s-crafting">
            <h2 >"Crafting"</h2>
            {(series.series.unlock_progress != 0).then(
                ||html!(<div><span class="has-text-weight-bold">"Unlock at: "</span> {
                    gen_progress(series.series.unlock_progress, pedia_ex)
                }</div>)
            )}
            {(series.series.unlock_enemy != EmTypes::Em(0)).then(
                ||html!(<div><span class="has-text-weight-bold">"Key monster: "</span> {
                    gen_monster_tag(pedia_ex, series.series.unlock_enemy, false, false, None, None)
                }</div>)
            )}
            {(series.series.unlock_item.iter().any(|&i|i != ItemId::None)).then(
                ||html!(<div><span class="has-text-weight-bold">"Key item: "</span> {
                    three_item_condition(series.series.evaluation, &series.series.unlock_item)
                }</div>)
            )}
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Cost"</th>
                    <th>"Material"</th>
                </tr></thead>
                <tbody>
                    {series.head.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td>{text!("{}z", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, &series.series.unlock_item)
                        } else {
                            html!(<td>"-"</td>)
                        }}
                    </tr>)})}
                    {series.chest.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td>{text!("{}z", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, &series.series.unlock_item)
                        } else {
                            html!(<td>"-"</td>)
                        }}

                    </tr>)})}
                    {series.weapon.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_weapon_label(p)}</td>
                        <td>{text!("{}z", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, &series.series.unlock_item)
                        } else {
                            html!(<td>"-"</td>)
                        }}
                    </tr>)})}
                </tbody>
            </table></div>
            </section>
        )
    });

    #[allow(clippy::question_mark)] // clippy is drunk
    let layered_row = |piece: &Option<OtArmor>| {
        let Some(piece) = piece else {
            return None;
        };
        piece.overwear.map(|overwear| {
            html!(<tr>
                <td>{gen_atomo_armor_label(piece)}</td>
                <td>{text!("{}z", overwear.sell_value * 3 / 2)}</td>
                {
                    if let Some(recipe) = piece.overwear_recipe {
                        [
                            gen_materials(pedia_ex, &recipe.required_item, &recipe.required_num, &recipe.unlock_id),
                            html!(<td>{(recipe.unlock_enemy != EmTypes::Em(0)).then(
                                ||gen_monster_tag(pedia_ex, recipe.unlock_enemy, false, false, None, None))}</td>),
                            html!(<td>{(recipe.unlock_id.iter().any(|&i|i != ItemId::None)).then(
                                ||three_item_condition(recipe.evaluation, &recipe.unlock_id))}</td>),
                            html!(<td>{(recipe.unlock_progress != 0).then(
                                ||gen_progress(recipe.unlock_progress, pedia_ex))}
                                {recipe.hr_limit_flag.then(||html!(<span class="tag">"HR limit"</span>))}
                                {recipe.mystery_flag.then(||html!(<span class="tag">"Anomaly"</span>))}
                            </td>),
                        ]
                    } else {
                        [
                            html!(<td>"-"</td>),
                            html!(<td>"-"</td>),
                            html!(<td>"-"</td>),
                            html!(<td>"-"</td>),
                        ]
                    }
                }
            </tr>)
        })
    };

    sections.push(Section {
        title: "Layered".to_owned(),
        content: html!(
            <section id="s-layered">
            <h2 >"Layered"</h2>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Cost"</th>
                    <th>"Material"</th>
                    <th>"Key monster"</th>
                    <th>"Key item"</th>
                    <th>"Unlock"</th>
                </tr></thead>
                <tbody>
                    {layered_row(&series.head)}
                    {layered_row(&series.chest)}
                </tbody>
            </table></div>
            </section>
        ),
    });

    let doc: DOMTree<String> = html!(<html lang="en">
        <head itemscope=true>
            <title>{text!("Buddy equipment")}</title>
            { head_common(hash_store, otomo_path) }
            { title_multi_lang(series.name) }
            { open_graph(Some(series.name), "",
                None, "", None, toc_sink.path(), config) }
        </head>
        <body>
            { navbar() }
            { gen_menu(&sections, toc_sink.path()) }
            <main>
            <header>
                <div class="mh-title-icon">
                { gen_rared_icon(rarity, icon, [], false) }
                </div>
                <h1> {gen_multi_lang(series.name)} </h1>
            </header>

            { sections.into_iter().map(|s|s.content) }

            </main>
            { right_aside() }
        </body>

    </html>);

    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

pub fn gen_otomo_equips(
    hash_store: &HashStore,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let otomo_path = output.sub_sink("otomo")?;
    for series in pedia_ex.ot_equip.values() {
        gen_otomo_equip(hash_store, series, pedia_ex, config, &otomo_path, toc)?
    }
    Ok(())
}

pub fn gen_otomo_equip_list(
    hash_store: &HashStore,
    pedia_ex: &PediaEx<'_>,
    output: &impl Sink,
) -> Result<()> {
    fn gen_list(
        hash_store: &HashStore,
        pedia_ex: &PediaEx<'_>,
        output: &impl Sink,
        filter: impl Fn(OtEquipSeriesId) -> bool,
        title: &str,
        file_name: &str,
        interlink: Box<div<String>>,
    ) -> Result<()> {
        let doc: DOMTree<String> = html!(
            <html lang="en">
                <head itemscope=true>
                    <title>{text!("{} - MHRice", title)}</title>
                    { head_common(hash_store, output) }
                    <style id="mh-armor-list-style">""</style>
                </head>
                <body>
                    { navbar() }
                    <main>
                    <header><h1>{text!("{}", title)}</h1></header>
                    { interlink }
                    <div class="mh-filters"><ul>
                        <li id="mh-armor-filter-button-all" class="is-active mh-armor-filter-button">
                            <a>"All"</a></li>
                        <li id="mh-armor-filter-button-lr" class="mh-armor-filter-button">
                            <a>"Low rank"</a></li>
                        <li id="mh-armor-filter-button-hr" class="mh-armor-filter-button">
                            <a>"High rank"</a></li>
                        <li id="mh-armor-filter-button-mr" class="mh-armor-filter-button">
                            <a>"Master rank"</a></li>
                        <li id="mh-armor-filter-button-layered" class="mh-armor-filter-button">
                            <a>"Layered"</a></li>
                    </ul></div>
                    <div class="select"><select id="scombo-armor" class="mh-scombo">
                        <option value="0">"Sort by internal ID"</option>
                        <option value="1">"Sort by in-game order"</option>
                    </select></div>
                    <ul class="mh-armor-series-list" id="slist-armor">{
                        pedia_ex.ot_equip.iter().filter(|(id, _)|filter(**id)).enumerate().map(|(index, (id, series))| {
                            let sort_id = if series.series.sort_id == 0 {
                                u32::MAX // collab seems to have sort_id == 0 but they should be at the bottom
                            } else {
                                series.series.sort_id
                            };
                            let sort_tag = format!("{index},{sort_id}");
                            let mut filter = match series.series.rank {
                                OtRankTypes::Lower => "lr",
                                OtRankTypes::Upper => "hr",
                                OtRankTypes::Master => "mr",
                            }.to_owned();
                            if (series.head.is_some() && series.head.as_ref().unwrap().overwear.is_some())
                                || (series.chest.is_some() && series.chest.as_ref().unwrap().overwear.is_some()) {
                                filter += " layered";
                            }
                            let series_name = gen_multi_lang(series.name);
                            html!(
                                <li class="mh-armor-filter-item" data-sort=sort_tag data-filter={filter}>
                                <a href={format!("otomo/{}.html", id.to_tag())}>
                                <h2>{
                                    series_name
                                }</h2>
                                <ul>
                                    {series.head.as_ref().map(|p|html!(<li>{
                                        gen_atomo_armor_label(p)
                                    }</li>))}
                                    {series.chest.as_ref().map(|p|html!(<li>{
                                        gen_atomo_armor_label(p)
                                    }</li>))}
                                    {series.weapon.as_ref().map(|p|html!(<li>{
                                        gen_atomo_weapon_label(p)
                                    }</li>))}
                                </ul>
                                </a></li>
                            )
                        })
                    }</ul>
                    </main>
                    { right_aside() }
                </body>
            </html>
        );
        output
            .create_html(file_name)?
            .write_all(doc.to_string().as_bytes())?;

        Ok(())
    }

    let dog_link = html!(<div>
        <a href="dog.html"><span class="icon-text">
        <span class="icon">
          <i class="fas fa-arrow-right"></i>
        </span>
        <span>"go to palamute equipment"</span>
        </span></a>
    </div>);

    let airou_link = html!(<div>
        <a href="airou.html"><span class="icon-text">
        <span class="icon">
          <i class="fas fa-arrow-right"></i>
        </span>
        <span>"go to palico equipment"</span>
        </span></a>
    </div>);

    gen_list(
        hash_store,
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Airou(_)),
        "Palico equipment",
        "airou.html",
        dog_link,
    )?;
    gen_list(
        hash_store,
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Dog(_)),
        "Palamute equipment",
        "dog.html",
        airou_link,
    )?;

    Ok(())
}
