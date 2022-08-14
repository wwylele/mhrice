use super::gen_common::*;
use super::gen_item::*;
use super::gen_skill::*;
use super::gen_website::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_armor_label(piece: Option<&Armor>) -> Box<div<String>> {
    let piece_name = if let Some(piece) = piece {
        let icon = format!(
            "/resources/equip/{:03}",
            piece.data.pl_armor_id.icon_index()
        );
        html!(<div class="mh-icon-text">
            { gen_rared_icon(piece.data.rare, &icon) }
            <span>{ gen_multi_lang(piece.name) }</span>
        </div>)
    } else {
        html!(<div class="mh-icon-text">
            <div class="mh-colored-icon"/>
            <span>"-"</span>
        </div>)
    };
    html!(<div>
        { piece_name }
    </div>)
}

pub fn gen_armor_list(serieses: &[ArmorSeries], output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Armors - MHRice")}</title>
                { head_common() }
                <style id="mh-armor-list-style">""</style>
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Armors"</h1></header>
                <div class="mh-filters"><ul>
                    <li id="mh-armor-filter-button-all" class="is-active mh-armor-filter-button">
                        <a>"All armors"</a></li>
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
                    serieses.iter().map(|series|{
                        let sort = if series.series.index == 0 {
                            // index 0 looks like invalid. Put to the end
                            u32::MAX
                        } else {
                            series.series.index
                        };
                        let sort_tag = format!("{},{}",
                            series.series.armor_series.0, sort);

                        let filter = match series.series.difficulty_group {
                            EquipDifficultyGroup::Lower => "lr",
                            EquipDifficultyGroup::Upper => "hr",
                            EquipDifficultyGroup::Master => "mr",
                        };
                        let series_name = gen_multi_lang(series.name);
                        html!(
                            <li class="mh-armor-filter-item" data-sort=sort_tag data-filter={filter}>
                            <a href={format!("/armor/{:03}.html", series.series.armor_series.0)}>
                            <h2>{
                                series_name
                            }</h2>
                            <ul> {
                                series.pieces.iter().take(5).map(|piece| {
                                    html!(<li>
                                        { gen_armor_label(piece.as_ref()) }
                                    </li>)
                                })
                            } </ul>
                            </a></li>
                        )
                    })
                }</ul>
                </main>
            </body>
        </html>
    );

    output
        .create_html("armor.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_armor(
    series: &ArmorSeries,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    toc_sink.add(series.name);

    for piece in &series.pieces {
        if let Some(piece) = piece.as_ref() {
            toc_sink.add(piece.name);
        }
    }

    let gen_explain = |pieces: &[Option<Armor<'_>>]| {
        html!(<div class="mh-table"><table>
            <thead><tr>
                <th>"Name"</th>
                <th>"Description"</th>
            </tr></thead>
            <tbody> {
                pieces.iter().map(|piece| {
                    let piece = if let Some(piece) = piece {
                        piece
                    } else {
                        return html!(<tr><td colspan="2">"-"</td></tr>)
                    };
                    html!(<tr>
                        <td>{gen_armor_label(Some(piece))}</td>
                        <td><pre>{gen_multi_lang(piece.explain)}</pre></td>
                    </tr>)
                })
            } </tbody>
        </table></div>)
    };

    let gen_stat = |pieces: &[Option<Armor<'_>>]| {
        html!(<div class="mh-table"><table>
            <thead><tr>
                <th>"Name"</th>
                <th>"Buying cost"</th>
                <th>"Defense"</th>
                <th>"Fire"</th>
                <th>"Water"</th>
                <th>"Ice"</th>
                <th>"Thunder"</th>
                <th>"Dragon"</th>
                <th>"Slots"</th>
                <th>"Skills"</th>
            </tr></thead>
            <tbody> {
                pieces.iter().map(|piece| {
                    let piece = if let Some(piece) = piece {
                        piece
                    } else {
                        return html!(<tr><td colspan="10">"-"</td></tr>)
                    };

                    let slots = gen_slot(&piece.data.decorations_num_list, false);

                    let skills = html!(<ul class="mh-armor-skill-list"> {
                        piece.data.skill_list.iter().zip(piece.data.skill_lv_list.iter())
                            .filter(|&(&skill, _)| skill != PlEquipSkillId::None)
                            .map(|(&skill, lv)| {
                            let name = if let Some(skill_data) = pedia_ex.skills.get(&skill) {
                                html!(<span><a href={format!("/skill/{}", skill_page(skill))}
                                    class="mh-icon-text">
                                    {gen_colored_icon(skill_data.icon_color, "/resources/skill", &[])}
                                    {gen_multi_lang(skill_data.name)}
                                </a></span>)
                            } else {
                                html!(<span>"<UNKNOWN>"</span>)
                            };
                            html!(<li>
                                {name}
                                {text!(" + {}", lv)}
                            </li>)
                        })
                    } </ul>);

                    html!(<tr>
                        <td>{gen_armor_label(Some(piece))}</td>
                        <td>{text!("{}z", piece.data.buy_value)}</td>
                        <td>{text!("{}", piece.data.def_val)}</td>
                        <td>{text!("{}", piece.data.fire_reg_val)}</td>
                        <td>{text!("{}", piece.data.water_reg_val)}</td>
                        <td>{text!("{}", piece.data.ice_reg_val)}</td>
                        <td>{text!("{}", piece.data.thunder_reg_val)}</td>
                        <td>{text!("{}", piece.data.dragon_reg_val)}</td>
                        <td>{slots}</td>
                        <td>{skills}</td>
                    </tr>)
                })
            } </tbody>
        </table></div>)
    };

    let gen_buildup = |pieces: &[Option<Armor<'_>>]| {
        let mut buildup_tables: Vec<(i32, Vec<&Armor>)> = vec![];
        'outer: for piece in pieces.iter().flatten() {
            for table in &mut buildup_tables {
                if table.0 == piece.data.buildup_table {
                    table.1.push(piece);
                    continue 'outer;
                }
            }
            buildup_tables.push((piece.data.buildup_table, vec![piece]))
        }

        buildup_tables
            .into_iter()
            .map(|table| {
                let table_out = if let Some(table) = pedia_ex.armor_buildup.get(&table.0) {
                    html!(<div class="mh-table"><table>
                    <thead><tr>
                        <th>"Levels"</th>
                        <th>"Unlock at"</th>
                        <th>"+Def / level"</th>
                        <th>"Armor sphere"</th>
                        <th>"Cost increase / level"</th>
                    </tr></thead>
                    <tbody> {table.iter().map(|upgrade|{
                        let progress = [upgrade.village_progress.display(),
                            upgrade.hub_progress.display(),
                            upgrade.master_rank_progress.display()];
                        let progress = html!(<ul class="mh-progress">{
                            progress.into_iter().flatten().map(|p|html!(<li>{text!("{}", p)}</li>))
                        }</ul>);
                        html!(<tr>
                            <td>{text!("Up to {}", upgrade.limit_lv)}</td>
                            <td>{progress}</td>
                            <td>{text!("{}", upgrade.up_val)}</td>
                            <td>{text!("{}", upgrade.lv_up_rate)}</td>
                            <td>{text!("{}z", upgrade.cost)}</td>
                        </tr>)
                    })}
                    </tbody>
                    </table></div>)
                } else {
                    html!(<div>{text!("Unknown upgrade table {}", table.0)}</div>)
                };
                html!(<div>
                    <ul class="mh-item-list">{table.1.into_iter().map(|piece|
                        html!(<li>{gen_armor_label(Some(piece))}</li>)
                    )}</ul>
                {table_out}
                </div>)
            })
            .collect::<Vec<_>>()
    };

    #[derive(PartialEq, Eq)]
    struct CbDedupKey {
        custom_table_no: u32,
        custom_cost: u32,
        rare: RareTypes,
    }

    let gen_custom_buildup = |pieces: &[Option<Armor<'_>>]| {
        let mut buildup_tables: Vec<(CbDedupKey, Vec<&Armor>)> = vec![];
        'outer: for piece in pieces.iter().flatten() {
            if piece.data.custom_table_no == 0 {
                continue;
            }
            let key = CbDedupKey {
                custom_table_no: piece.data.custom_table_no,
                custom_cost: piece.data.custom_cost,
                rare: piece.data.rare,
            };
            for table in &mut buildup_tables {
                if table.0 == key {
                    table.1.push(piece);
                    continue 'outer;
                }
            }
            buildup_tables.push((key, vec![piece]))
        }
        if buildup_tables.is_empty() {
            return None;
        }
        Some(html!(<section>
            <h2>"Qurious crafting"</h2>
            {buildup_tables.into_iter().map(|(key, armor)| {
                let table_out = if let Some(table) =
                    pedia_ex.armor_custom_buildup.get(&key.custom_table_no) {
                    html!(<div class="mh-table"><table>
                    <thead><tr>
                        <th>"Category"</th>
                        <th>"Category probability"</th>
                        <th>"Probability"</th>
                        //<th>"Level"</th>
                        //<th>"Color"</th>
                        <th>{text!("Points (credit: {})", key.custom_cost)}</th>
                        <th>"Bonus"</th>
                    </tr></thead>
                    <tbody> {
                        table.categories.iter().flat_map(|(&category_id, category)| {
                            let rowspan = category.pieces.len();
                            let category_name = match category_id {
                                13 => text!("Defense"),
                                14 => text!("Element resistance"),
                                19 => text!("Slot"),
                                20 => text!("Skill"),
                                c => text!("{}", c)
                            };
                            let mut category_cell = Some(html!(<td rowspan={rowspan}>
                                { category_name }
                            </td>));
                            let mut probability_cell = Some(html!(<td rowspan={rowspan}>
                                { text!("{}%", category.lot)}
                            </td>));
                            category.pieces.iter().map(move |(_, piece)| {
                                html!(<tr>
                                    {category_cell.take()}
                                    {probability_cell.take()}
                                    <td>{ text!("{}%", piece.lot) }</td>
                                    //<td>{ text!("{}", piece.data.lv) }</td>
                                    //<td>{ text!("{}", piece.data.icon_color) }</td>
                                    <td>{ text!("{:+}", piece.data.cost) }</td>
                                    <td>
                                    { (category_id == 20 && piece.data.cost > 0).then(||{
                                        let class=format!("tag mh-cb-lv{}", piece.data.cost);
                                        html!(<span class={class.as_str()}>
                                            {text!("Pt{} skill", piece.data.cost)}
                                        </span>)}
                                    ) }
                                    <ul class="mh-custom-lot"> {
                                        piece.data.value_table.iter().zip(&piece.data.lot_table)
                                        .filter(|(_, lot)| **lot != 0)
                                        .map(|(value, lot)| html!(<li> {
                                            text!("{:+}, {}%", value, lot)
                                        } </li>))
                                    } </ul>
                                    </td>
                                </tr>)
                            })
                        })
                    } </tbody>
                    </table>
                    </div>)
                } else {
                    html!(<div>{text!("Unknown custom upgrade table {}", key.custom_table_no)}</div>)
                };
                html!(
                <div>
                    <ul class="mh-item-list">{armor.into_iter().map(|piece|
                        html!(<li>{gen_armor_label(Some(piece))}</li>)
                    )}</ul>
                    <div class="mh-table"><table>
                    <thead><tr>
                        <th>""</th>
                        <th>"Cost"</th>
                        <th>"Categorized Material"</th>
                        <th>"Material"</th>
                    </tr></thead>
                    <tbody>
                    { pedia.custom_buildup_armor_open.as_ref().and_then(
                        |m|m.param.iter().find(|m|m.rare == key.rare).map(|m|html!(<tr>
                        <td>"Enable"</td>
                        <td>{text!("{}z", m.price)}</td>
                        {gen_category(pedia_ex, m.material_category, m.material_category_num)}
                        {gen_materials(pedia_ex, &m.item, &m.item_num, ItemId::Null)}
                    </tr>))) }
                    { pedia.custom_buildup_armor_material.as_ref().and_then(
                        |m|m.param.iter().find(|m|m.rare == key.rare).map(|m|html!(<tr>
                        <td>"Augment"</td>
                        <td>{text!("{}z", m.price)}</td>
                        {gen_category(pedia_ex, m.material_category, m.material_category_num)}
                        <td></td>
                    </tr>))) }
                    </tbody>
                    </table>
                    </div>
                    {table_out}
                </div>)
            })}
        </section>))
    };

    let rarity = series
        .pieces
        .iter()
        .find_map(|p| p.as_ref())
        .map_or(RareTypes(1), |p| p.data.rare);

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Armor {:03}", series.series.armor_series.0)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header>
                    <div class="mh-title-icon"> {
                        gen_rared_icon(rarity, "/resources/equip/006")
                    } </div>
                    <h1> {
                        gen_multi_lang(series.name)
                    } </h1>
                </header>

                <section>
                <h2 >"Description"</h2>
                { gen_explain(&series.pieces[0..5])}
                </section>

                <section>
                <h2 >"Stat"</h2>
                { gen_stat(&series.pieces[0..5])}
                </section>


                {
                    series.pieces[5..10].iter().any(|p|p.is_some()).then(||{
                        [
                            html!(<section>
                                <h2 >"EX Description"</h2>
                                { gen_explain(&series.pieces[5..10])}
                                </section>
                            ),
                            html!(<section>
                                <h2 >"EX Stat"</h2>
                                { gen_stat(&series.pieces[5..10])}
                                </section>
                            ),
                        ]
                    }).into_iter().flatten()
                }

                <section>
                <h2 >"Crafting"</h2>
                <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Name"</th>
                        <th>"Cost"</th>
                        <th>"Categorized Material"</th>
                        <th>"Material"</th>
                        <th>"Output"</th>
                    </tr></thead>
                    <tbody> {
                        series.pieces.iter().take(5).map(|piece| {
                            let product = if let Some(Armor{product: Some(product), ..}) = &piece {
                                product
                            } else {
                                return html!(<tr><td colspan="4">"-"</td></tr>)
                            };
                            let armor = piece.as_ref().unwrap();

                            let category = gen_category(pedia_ex, product.material_category,
                                product.material_category_num);

                            let materials = gen_materials(pedia_ex, &product.item,
                                &product.item_num, product.item_flag);

                            let output = gen_materials(pedia_ex, &product.output_item,
                                &product.output_item_num, ItemId::None);

                            html!(<tr>
                                <td>{gen_armor_label(Some(armor))}</td>
                                <td>{text!("{}z", armor.data.value)}</td>
                                {category}
                                {materials}
                                {output}
                            </tr>)
                        })
                    } </tbody>
                </table></div>

                </section>

                <section>
                <h2 >"Layered crafting"</h2>
                <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Name"</th>
                        <th>"Categorized Material"</th>
                        <th>"Material"</th>
                    </tr></thead>
                    <tbody> {
                        series.pieces.iter().take(5).map(|piece| {
                            let product = if let Some(Armor{overwear_product: Some(product), ..}) = &piece {
                                product
                            } else {
                                return html!(<tr><td colspan="3">"-"</td></tr>)
                            };
                            let armor = piece.as_ref().unwrap();

                            let category = gen_category(pedia_ex, product.material_category,
                                product.material_category_num);

                            let materials = gen_materials(pedia_ex, &product.item,
                                &product.item_num, product.item_flag);

                            html!(<tr>
                                <td>{gen_armor_label(Some(armor))}</td>
                                {category}
                                {materials}
                            </tr>)
                        })
                    } </tbody>
                </table></div>

                </section>

                <section>
                <h2 >"Upgrades"</h2>
                { gen_buildup(&series.pieces[0..5])}
                </section>

                {series.pieces[5..10].iter().any(|p|p.is_some()).then(||{
                    html!(<section>
                        <h2 >"EX Upgrades"</h2>
                        { gen_buildup(&series.pieces[5..10])}
                        </section>
                    )
                })}

                {gen_custom_buildup(&series.pieces)}

                </main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

pub fn gen_armors(
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let armor_path = output.sub_sink("armor")?;
    for series in &pedia_ex.armors {
        let (output, toc_sink) = armor_path
            .create_html_with_toc(&format!("{:03}.html", series.series.armor_series.0), toc)?;
        gen_armor(series, pedia, pedia_ex, output, toc_sink)?
    }
    Ok(())
}
