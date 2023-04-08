use super::gen_armor::*;
use super::gen_common::*;
use super::gen_item::*;
use super::gen_otomo::*;
use super::gen_skill::*;
use super::gen_weapon::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_dlc_label(dlc: &Dlc) -> Box<a<String>> {
    let link = format!("/dlc/{}.html", dlc.data.dlc_id);
    html!(<a href={link}>
        {if let Some(name) = dlc.name {
            gen_multi_lang(name)
        } else {
            html!(<span>{text!("Unknown DLC {}", dlc.data.dlc_id)}</span>)
        }}
    </a>)
}

pub fn gen_slc_label(slc: &SaveLinkContents) -> Box<a<String>> {
    let link = format!("/dlc/slc_{}.html", slc.into_raw());
    html!(<a href={link}>
        {text!("{}", slc.display())}
    </a>)
}

pub fn gen_dlc_list(hash_store: &HashStore, pedia_ex: &PediaEx, output: &impl Sink) -> Result<()> {
    let mut sections = vec![];

    let mut dlcs: Vec<_> = pedia_ex.dlc.values().collect();
    dlcs.sort_by_key(|dlc| (dlc.data.sort_id, dlc.data.dlc_id));

    sections.push(Section {
        title: "Save link content".to_owned(),
        content: html!(
            <section id="s-slc">
            <h2>"Save link content"</h2>
            <ul class="mh-quest-list">
            {
                pedia_ex.slc.keys().map(|slc| {
                    html!(<li>{gen_slc_label(slc)}</li>)
                })
            }
            </ul>
            </section>
        ),
    });

    sections.push(Section {
        title: "DLC".to_owned(),
        content: html!(
            <section id="s-dlc">
            <h2 >"DLC"</h2>
            <ul class="mh-quest-list" id="slist-dlc">
            {
                dlcs.into_iter().map(|dlc| {
                    html!(<li>{gen_dlc_label(dlc)}</li>)
                })
            }
            </ul>
            </section>
        ),
    });

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("DLC - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header><h1>"DLC"</h1></header>

                { sections.into_iter().map(|s|s.content) }

                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("dlc.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_add(add: &AddDataInfo, pedia_ex: &PediaEx, sections: &mut Vec<Section>) {
    macro_rules! check_weapon {
            ($weapon:ident) => {{
                let weapons = &pedia_ex.$weapon;
                weapons
                    .weapons.iter()
                    .filter(|(k, _)|add.pl_weapon_list.contains(k))
                    .map(|(_, w)| {
                        html!(<li>{gen_weapon_label(w)}</li>)
                    })
            }}
        }

    macro_rules! check_weapon2 {
            ($weapon:ident) => {{
                let weapons = &pedia_ex.$weapon;
                weapons
                    .weapons.values()
                    .filter(|w|
                        if let Some(ow) = w.overwear{
                            add.pl_overwear_weapon_id_list
                                .0.as_deref()
                                .unwrap_or_default()
                                .contains(&ow.id)
                        } else {
                            false
                        }
                    )
                    .map(|w| {
                        html!(<li>{gen_weapon_label(w)}
                            <span class="tag">"Layered"</span></li>)
                    })
            }}
        }

    sections.push(Section {
        title: "Unlock".to_owned(),
        content: html!(
            <section id="s-unlock">
            <h2 >"Unlock"</h2>
            <ul class="mh-item-list">
                {check_weapon!(great_sword)}
                {check_weapon!(short_sword)}
                {check_weapon!(hammer)}
                {check_weapon!(lance)}
                {check_weapon!(long_sword)}
                {check_weapon!(slash_axe)}
                {check_weapon!(gun_lance)}
                {check_weapon!(dual_blades)}
                {check_weapon!(horn)}
                {check_weapon!(insect_glaive)}
                {check_weapon!(charge_axe)}
                {check_weapon!(light_bowgun)}
                {check_weapon!(heavy_bowgun)}
                {check_weapon!(bow)}

                {check_weapon2!(great_sword)}
                {check_weapon2!(short_sword)}
                {check_weapon2!(hammer)}
                {check_weapon2!(lance)}
                {check_weapon2!(long_sword)}
                {check_weapon2!(slash_axe)}
                {check_weapon2!(gun_lance)}
                {check_weapon2!(dual_blades)}
                {check_weapon2!(horn)}
                {check_weapon2!(insect_glaive)}
                {check_weapon2!(charge_axe)}
                {check_weapon2!(light_bowgun)}
                {check_weapon2!(heavy_bowgun)}
                {check_weapon2!(bow)}

                {pedia_ex.armors.values().flat_map(|a|&a.pieces).flatten().filter(|p| {
                    add.pl_armor_list.contains(&p.data.pl_armor_id)
                }).map(|p|html!(<li>
                    <a class="il" href={format!("/armor/{:03}.html", p.data.series.0)}>
                    {gen_armor_label(Some(p))}
                    </a>
                </li>))}

                {pedia_ex.armors.values().flat_map(|a|&a.pieces).flatten().filter(|p| {
                    if let Some(ow) = p.overwear {
                        add.pl_overwear_id_list.contains(&ow.id)
                    } else {
                        false
                    }
                }).map(|p|html!(<li>
                    <a class="il" href={format!("/armor/{:03}.html", p.data.series.0)}>
                    {gen_armor_label(Some(p))}
                    </a>
                    <span class="tag">"Layered"</span>
                </li>))}

                {pedia_ex.ot_equip.values().flat_map(|a|a.head.iter().chain(a.chest.iter())).filter(|p| {
                    if let Some(ow) = p.overwear {
                        add.ot_overwear_id_list.contains(&ow.id)
                    } else {
                        false
                    }
                }).map(|p|html!(<li>
                    <a class="il" href={format!("/otomo/{}.html", p.param.series_id.to_tag())}>
                    {gen_atomo_armor_label(p)}
                    </a>
                    <span class="tag">"Layered"</span>
                </li>))}
            </ul>
            </section>
        ),
    });

    if add
        .pl_talisman_list
        .iter()
        .any(|talisman| talisman.id_type != EquipmentInventoryDataIdTypes::Empty)
    {
        sections.push(Section {
            title: "Talisman".to_owned(),
            content: html!(
                <section id="s-talisman">
                <h2 >"Talisman"</h2>
                {add.pl_talisman_list.iter()
                    .filter(|talisman| talisman.id_type != EquipmentInventoryDataIdTypes::Empty)
                    .map(|talisman| if talisman.id_type == EquipmentInventoryDataIdTypes::Talisman {
                        html!(<div>
                            <div>"Slots: "{gen_slot(&talisman.talisman_deco_slot_num_list, false)}</div>
                            <div>"Skills: "
                            <ul>{
                                talisman.talisman_skill_id_list.iter().zip(&talisman.talisman_skill_level_list)
                                .filter(|&(&id, _)|id != PlEquipSkillId::None)
                                .map(|(&id, &lv)|gen_skill_lv_label(pedia_ex, id, lv as i32))
                            }</ul></div>
                        </div>)
                    } else {
                        html!(<div>"[Unsuppoerted data]"</div>)
                    })
                }
                </section>)
        })
    }
}

fn gen_dlc(
    hash_store: &HashStore,
    dlc: &Dlc,
    _pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    if let Some(name) = dlc.name {
        toc_sink.add(name);
    }

    let mut sections = vec![];

    if let Some(explain) = dlc.explain {
        sections.push(Section {
            title: "Description".to_owned(),
            content: html!(
                <section id="s-description">
                <h2 >"Description"</h2>
                <pre>{
                    gen_multi_lang(explain)
                }</pre>
                </section>
            ),
        });
    }

    if let Some(add) = dlc.add {
        gen_add(add, pedia_ex, &mut sections)
    }

    if let Some(item_pack) = dlc.item_pack {
        sections.push(Section {
            title: "Item".to_owned(),
            content: html!(
                <section id="s-item">
                <h2 >"Item"</h2>
                <ul class="mh-item-list">
                {item_pack.item_info.iter().map(|item| {
                    html!(<li>
                        {text!("{}x ", item.num)}
                        <div class="il">{gen_item_label_from_id(item.item, pedia_ex)}</div>
                    </li>)
                })}
                </ul>
                </section>
            ),
        })
    }

    let plain_title = format!("DLC {:03}", dlc.data.dlc_id);
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{}", plain_title)}</title>
                { head_common(hash_store) }
                { dlc.name.map(title_multi_lang).unwrap_or_default()}
                { open_graph(dlc.name, &plain_title,
                    dlc.explain, "", None, toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header>
                    <h1> { if let Some(name) = dlc.name {
                        gen_multi_lang(name)
                    } else {
                        html!(<span>{text!("{}", plain_title)}</span>)
                    } } </h1>
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

fn gen_slc(
    hash_store: &HashStore,
    slc_id: SaveLinkContents,
    slc: &Slc,
    _pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    mut output: impl Write,
    mut _toc_sink: TocSink<'_>,
) -> Result<()> {
    let mut sections = vec![];

    if let Some(add) = slc.add {
        gen_add(add, pedia_ex, &mut sections)
    }

    if let Some(item_pack) = slc.item_pack {
        sections.push(Section {
            title: "Item".to_owned(),
            content: html!(
                <section id="s-item">
                <h2 >"Item"</h2>
                <ul class="mh-item-list">
                {item_pack.item_info.iter().map(|item| {
                    html!(<li>
                        {text!("{}x ", item.num)}
                        <div class="il">{gen_item_label_from_id(item.item, pedia_ex)}</div>
                    </li>)
                })}
                </ul>
                </section>
            ),
        })
    }

    let plain_title = format!("Save link content: {}", slc_id.display());
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{}", plain_title)}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header>
                    <h1><span>{text!("{}", plain_title)}</span></h1>
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

pub fn gen_dlcs(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let dlc_path = output.sub_sink("dlc")?;
    for dlc in pedia_ex.dlc.values() {
        let (path, toc_sink) =
            dlc_path.create_html_with_toc(&format!("{}.html", dlc.data.dlc_id), toc)?;
        gen_dlc(hash_store, dlc, pedia, pedia_ex, config, path, toc_sink)?;
    }

    for (&id, slc) in &pedia_ex.slc {
        let (path, toc_sink) =
            dlc_path.create_html_with_toc(&format!("slc_{}.html", id.into_raw()), toc)?;
        gen_slc(hash_store, id, slc, pedia, pedia_ex, path, toc_sink)?;
    }

    Ok(())
}
