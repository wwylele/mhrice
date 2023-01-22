use super::gen_common::*;
use super::gen_item::*;
use super::gen_quest::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, html, text};

fn gen_petalace(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut _toc_sink: TocSink<'_>,
) -> Result<()> {
    let mut petalace: Vec<_> = pedia_ex.buff_cage.values().collect();
    petalace.sort_unstable_by_key(|p| (p.data.sort_index, p.data.id));
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Petalace - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Petalace"</h1></header>
                <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Description"</th>
                    <th>"Health"</th>
                    <th>"Stamina"</th>
                    <th>"Attack"</th>
                    <th>"Defense"</th>
                </tr></thead>
                <tbody>
                { petalace.into_iter().map(|petalace| html!(<tr>
                    <td><div class="mh-icon-text">
                        {gen_rared_icon(petalace.data.rarity, "/resources/equip/030", [])}
                        <span>{gen_multi_lang(petalace.name)}</span>
                    </div></td>
                    <td><pre>{gen_multi_lang(petalace.explain)}</pre></td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[0], petalace.data.status_buff_limit[0])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[1], petalace.data.status_buff_limit[1])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[2], petalace.data.status_buff_limit[2])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[3], petalace.data.status_buff_limit[3])}</td>
                </tr>)) }
                </tbody>
                </table></div>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_market(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let mut sections = vec![];

    let mut item_shop: Vec<_> = pedia.item_shop.param.iter().collect();
    item_shop.sort_by_key(|item| (item.sort_id, item.id));

    sections.push(Section {
        title: "Items".to_owned(),
        content: html!(<section id="s-item">
            <h2 >"Items"</h2>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Item"</th>
                    <th>"Price"</th>
                    <th>"Unlock"</th>
                    //<th>"flag index"</th>
                </tr></thead>
                <tbody>
                {item_shop.into_iter().map(|item|{
                    let (item_label, price) = if let Some(item_data) = pedia_ex.items.get(&item.id) {
                        (
                            html!(<td>{gen_item_label(item_data)}</td>),
                            html!(<td>{text!("{}z", item_data.param.buy_price)}
                            { (!item.is_bargin_object).then(||text!(" (cannot be on sell)")) }
                            </td>)
                        )
                    } else {
                        (html!(<td>{text!("Unknown item {:?}", item)}</td>), html!(<td></td>))
                    };
                    html!(<tr>
                        {item_label}
                        {price}
                        <td>{ text!("{} {} {}",
                            item.village_progress.display().unwrap_or_default(),
                            item.hall_progress.display().unwrap_or_default(),
                            item.mr_progress.display().unwrap_or_default()) }
                            { item.is_unlock_after_alchemy.then(||text!("Needs to combine first")) }
                        </td>
                        //<td>{ text!("{}", item.flag_index) }</td>
                    </tr>)
                })}
                </tbody>
            </table></div>
        </section>),
    });

    sections.push(Section {
        title: "Lottery".to_owned(),
        content: html!(<section id="s-lottery">
            <h2 >"Lottery"</h2>
            {pedia_ex.item_shop_lot.iter().map(|lot| {
                let lot_type = match lot.data.lot_type {
                    crate::rsz::ItemLotFuncLotType::Heal => "Heal",
                    crate::rsz::ItemLotFuncLotType::Trap => "Traps",
                    crate::rsz::ItemLotFuncLotType::Support => "Support",
                    crate::rsz::ItemLotFuncLotType::Special => "Special goods",
                    crate::rsz::ItemLotFuncLotType::Amiibo => "Amiibo",
                };

                html!(<section>
                <h3>{text!("Type: {} - Rank {}", lot_type, lot.data.rank_type)}</h3>
                {text!("Unlock at: {} {} {}",
                    lot.data.village.display().unwrap_or_default(),
                    lot.data.hall.display().unwrap_or_default(),
                    lot.data.mr.display().unwrap_or_default()
                )}
                <div class="mh-reward-tables">
                {lot.reward_tables.iter().zip(&lot.data.probability_list).enumerate().map(|(i, (table, probability))| {
                    let grade = match i {
                        0 => "Jackpot",
                        1 => "Bingo",
                        2 => "Miss",
                        _ => unreachable!()
                    };
                    html!(
                    <div class="mh-reward-box">
                    <div class="mh-table"><table>
                    <thead><tr>
                        <th>{text!("{} ({}%)", grade, probability)}
                            <br/>{translate_rule(table.lot_rule)}</th>
                        <th>"Probability"</th>
                    </tr></thead>
                    <tbody> {
                        gen_reward_table(pedia_ex,
                            &table.item_id_list,
                            &table.num_list,
                            &table.probability_list)
                    } </tbody>
                    </table></div>
                    </div>
                )})}
                </div>
            </section>)})}
            </section>
        ),
    });

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Market - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header><h1>"Market"</h1></header>
                { sections.into_iter().map(|s|s.content) }
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

fn gen_misc_page(hash_store: &HashStore, mut output: impl Write) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Misc - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Miscellaneous"</h1></header>

                <a href="/misc/petalace.html">"Petalace"</a>
                <a href="/misc/market.html">"Market"</a>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_misc(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let folder = output.sub_sink("misc")?;
    let (path, toc_sink) = folder.create_html_with_toc("petalace.html", toc)?;
    gen_petalace(hash_store, pedia_ex, path, toc_sink)?;

    let path = folder.create_html("market.html")?;
    gen_market(hash_store, pedia, pedia_ex, path)?;

    gen_misc_page(hash_store, output.create_html("misc.html")?)?;

    Ok(())
}
