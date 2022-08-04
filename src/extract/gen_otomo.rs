use super::gen_item::*;
use super::gen_website::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_atomo_armor_label(piece: &OtArmor) -> Box<div<String>> {
    let icon = format!("/resources/equip/{:03}", piece.param.id.icon_index());
    html!(<div class="mh-icon-text">
        { gen_rared_icon(piece.param.rare_type, &icon) }
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

    let icon = format!("/resources/equip/{:03}", icon_index);
    html!(<div class="mh-icon-text">
        { gen_rared_icon(piece.param.rare_type, &icon) }
        <span>{ gen_multi_lang(piece.name) }</span>
    </div>)
}

fn gen_otomo_equip(
    series: &OtEquipSeries,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
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
        OtEquipSeriesId::Airou(_) => "/resources/equip/010",
        OtEquipSeriesId::Dog(_) => "/resources/equip/031",
    };

    let gen_armor_stat = |armor: &Option<OtArmor>| -> Option<Box<tr<String>>> {
        let armor = armor.as_ref()?;
        Some(html!(<tr>
            <td>{gen_atomo_armor_label(armor)}</td>
            <td>{text!("{}", armor.param.sell_value)}</td>
            <td>{text!("Defense: {}", armor.param.def)}</td>
            <td>"Defense"
                <ul>
                    <li>{text!("Fire: {}", armor.param.element_regist_list[0])}</li>
                    <li>{text!("Water: {}", armor.param.element_regist_list[1])}</li>
                    <li>{text!("Thunder: {}", armor.param.element_regist_list[2])}</li>
                    <li>{text!("Ice: {}", armor.param.element_regist_list[3])}</li>
                    <li>{text!("Dragon: {}", armor.param.element_regist_list[4])}</li>
                </ul>
            </td>
        </tr>))
    };

    let doc: DOMTree<String> = html!(<html>
        <head>
            <title>{text!("Buddy equipment")}</title>
            { head_common() }
        </head>
        <body>
            { navbar() }
            <main>
            <header>
                <div class="mh-title-icon">
                { gen_rared_icon(rarity, icon) }
                </div>
                <h1> {gen_multi_lang(series.name)} </h1>
            </header>

            <section>
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

            <section>
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
                    <td>{text!("{}", weapon.param.sell_value)}</td>
                    <td><ul>
                        <li>{text!("Short: {} (Aff. {}%)",
                            weapon.param.atk_val_list[0], weapon.param.critical_rate_list[0])}</li>
                        <li>{text!("Long: {} (Aff. {}%)",
                            weapon.param.atk_val_list[1], weapon.param.critical_rate_list[1])}</li>
                        <li>{text!("Def bonus: {}", weapon.param.def_bonus)}</li>
                    </ul></td>
                    <td><ul>
                        <li>{text!("{:?}", weapon.param.element_type)}</li>
                        <li>{text!("Short: {}", weapon.param.element_val_list[0])}</li>
                        <li>{text!("Long: {}", weapon.param.element_val_list[1])}</li>
                    </ul></td>
                </tr>)})}
                </tbody>
            </table></div>
            </section>

            <section>
            <h2 >"Crafting"</h2>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Cost"</th>
                    <th>"Material"</th>
                </tr></thead>
                <tbody>
                    {series.head.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td>{text!("{}", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, ItemId::None)
                        } else {
                            html!(<td>"-"</td>)
                        }}
                    </tr>)})}
                    {series.chest.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_armor_label(p)}</td>
                        <td>{text!("{}", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, ItemId::None)
                        } else {
                            html!(<td>"-"</td>)
                        }}

                    </tr>)})}
                    {series.weapon.as_ref().map(|p|{html!(<tr>
                        <td>{gen_atomo_weapon_label(p)}</td>
                        <td>{text!("{}", p.param.sell_value * 3 / 2)}</td>
                        {if let Some(product) = &p.product {
                            gen_materials(pedia_ex, &product.item_list, &product.item_num, ItemId::None)
                        } else {
                            html!(<td>"-"</td>)
                        }}
                    </tr>)})}


                </tbody>
            </table></div>
            </section>

            // TODO: how to unlock one

            </main>
        </body>

    </html>);

    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

pub fn gen_otomo_equips(pedia_ex: &PediaEx<'_>, output: &impl Sink, toc: &mut Toc) -> Result<()> {
    let otomo_path = output.sub_sink("otomo")?;
    for (id, series) in &pedia_ex.ot_equip {
        let (output, toc_sink) =
            otomo_path.create_html_with_toc(&format!("{}.html", id.to_tag()), toc)?;
        gen_otomo_equip(series, pedia_ex, output, toc_sink)?
    }
    Ok(())
}

pub fn gen_otomo_equip_list(pedia_ex: &PediaEx<'_>, output: &impl Sink) -> Result<()> {
    fn gen_list(
        pedia_ex: &PediaEx<'_>,
        output: &impl Sink,
        filter: impl Fn(OtEquipSeriesId) -> bool,
        title: &str,
        file_name: &str,
        interlink: Box<div<String>>,
    ) -> Result<()> {
        let doc: DOMTree<String> = html!(
            <html>
                <head>
                    <title>{text!("{} - MHRice", title)}</title>
                    { head_common() }
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
                            let sort_tag = format!("{},{}", index, sort_id);
                            let filter = match series.series.rank {
                                OtRankTypes::Lower => "lr",
                                OtRankTypes::Upper => "hr",
                                OtRankTypes::Master => "mr",
                            };
                            let series_name = gen_multi_lang(series.name);
                            html!(
                                <li class="mh-armor-filter-item" data-sort=sort_tag data-filter={filter}>
                                <a href={format!("/otomo/{}.html", id.to_tag())}>
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
                </body>
            </html>
        );
        output
            .create_html(file_name)?
            .write_all(doc.to_string().as_bytes())?;

        Ok(())
    }

    let dog_link = html!(<div>
        <a href="/dog.html"><span class="icon-text">
        <span class="icon">
          <i class="fas fa-arrow-right"></i>
        </span>
        <span>"go to palamute equipment"</span>
        </span></a>
    </div>);

    let airou_link = html!(<div>
        <a href="/airou.html"><span class="icon-text">
        <span class="icon">
          <i class="fas fa-arrow-right"></i>
        </span>
        <span>"go to palico equipment"</span>
        </span></a>
    </div>);

    gen_list(
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Airou(_)),
        "Palico equipment",
        "airou.html",
        dog_link,
    )?;
    gen_list(
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Dog(_)),
        "Palamute equipment",
        "dog.html",
        airou_link,
    )?;

    Ok(())
}
