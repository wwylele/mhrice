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
            <main> <div class="container"> <div class="content">
            <div class="mh-title-icon">
            { gen_rared_icon(rarity, icon) }
            </div>
            <h1 class="title"> {
                gen_multi_lang(series.name)
            } </h1>

            <section class="section">
            <h2 class="title">"Description"</h2>
            <table>
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
            </table>
            </section>

            <section class="section">
            <h2 class="title">"Stat"</h2>
            <table>
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
            </table>
            </section>

            <section class="section">
            <h2 class="title">"Crafting"</h2>
            <table>
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
            </table>
            </section>

            // TODO: how to unlock one

            </div> </div> </main>
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
    ) -> Result<()> {
        let doc: DOMTree<String> = html!(
            <html>
                <head>
                    <title>{text!("{} - MHRice", title)}</title>
                    { head_common() }
                </head>
                <body>
                    { navbar() }
                    <main> <div class="container">
                    <h1 class="title">{text!("{}", title)}</h1>
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
                            let series_name = gen_multi_lang(series.name);
                            html!(
                                <li class="mh-armor-series-list" data-sort=sort_tag>
                                <a href={format!("/otomo/{}.html", id.to_tag())}>
                                <h2>{
                                    series_name
                                }</h2>
                                <ul class="mh-armor-list">
                                    {series.head.as_ref().map(|p|html!(<li class="mh-armor-list">{
                                        gen_atomo_armor_label(p)
                                    }</li>))}
                                    {series.chest.as_ref().map(|p|html!(<li class="mh-armor-list">{
                                        gen_atomo_armor_label(p)
                                    }</li>))}
                                    {series.weapon.as_ref().map(|p|html!(<li class="mh-armor-list">{
                                        gen_atomo_weapon_label(p)
                                    }</li>))}
                                </ul>
                                </a></li>
                            )
                        })
                    }</ul>
                    </div></main>
                </body>
            </html>
        );
        output
            .create_html(file_name)?
            .write_all(doc.to_string().as_bytes())?;

        Ok(())
    }

    gen_list(
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Airou(_)),
        "Palico equipement",
        "airou.html",
    )?;
    gen_list(
        pedia_ex,
        output,
        |id| matches!(id, OtEquipSeriesId::Dog(_)),
        "Palamute equipement",
        "dog.html",
    )?;

    Ok(())
}
