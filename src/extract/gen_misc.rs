use super::gen_common::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_quest::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
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
                    crate::rsz::ItemLotFuncLotType::Heal => "Recovery",
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

    let gen_lucky_prize_row = |param: &ShopFukudamaUserDataParam| {
        html!(<tr>
            <td>{text!("{}x ", param.item_num)}
            <div class="il">{gen_item_label_from_id(param.item_id, pedia_ex)}</div></td>
            <td>{text!("{}", param.fukudama_num)}</td>
        </tr>)
    };

    sections.push(Section {
        title: "Lucky prize".to_owned(),
        content: html!(<section id="s-lucky">
            <h2>"Lucky prize"</h2>
            <div class="mh-table"><table>
            <thead><tr>
            <th>"Item"</th><th>"Lucky ball points"</th>
            </tr></thead>
            <tbody>
            {pedia.fukudama.no_count_stop_param.iter().map(gen_lucky_prize_row)}
            <tr><td/><td>"Stop counting points at this point"</td></tr>
            {pedia.fukudama.count_stop_param.iter().map(gen_lucky_prize_row)}
            </tbody>
            </table>
            </div>
            </section>),
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

fn gen_lab(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let gen_em = |em| {
        (em != EmTypes::Em(0))
            .then(|| html!(<li>{gen_monster_tag(pedia_ex, em, false, true, None, None)}</li>))
    };

    let content = if let Some(lab) = &pedia.mystery_labo_trade_item {
        let mut lab: Vec<_> = lab
            .param
            .iter()
            .filter(|p| !matches!(p.item_id, ItemId::Null | ItemId::None))
            .collect();
        lab.sort_by_key(|p| p.sort_id);
        html!(<div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Price"</th>
            <th>"Unlock by research lv"</th>
            <th>"Unlock by item"</th>
            <th>"Unlock by monster"</th>
            <th>"Monster count"</th>
        </tr></thead>
        <tbody>
        { lab.iter().map(|param| {
            let unlock_item_label = if matches!(param.unlock_condition_item_id, ItemId::Null | ItemId::None) {
                html!(<td>"-"</td>)
            } else{
                html!(<td>{gen_item_label_from_id(param.unlock_condition_item_id, pedia_ex)}</td>)
            };
            html!(<tr>
                <td>{gen_item_label_from_id(param.item_id, pedia_ex)}</td>
                <td>{text!("{}", param.cost)}</td>
                <td>{text!("{}", param.unlock_condition_mystery_research_lv)}</td>
                {unlock_item_label}
                <td>
                <ul class="mh-rampage-em-list">
                {gen_em(param.unlock_condition_enemy_id_1)}
                {gen_em(param.unlock_condition_enemy_id_2)}
                {gen_em(param.unlock_condition_enemy_id_3)}
                {gen_em(param.unlock_condition_enemy_id_4)}
                {gen_em(param.unlock_condition_enemy_id_5)}
                </ul>
                </td>
                <td>{text!("x{}", param.unlock_condition_enemy_hunt_count)}</td>
            </tr>)
        }) }
        </tbody>
        </table></div>)
    } else {
        html!(<div>"Not open"</div>)
    };
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Anomaly research lab - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Anomaly research lab"</h1></header>
                {content}
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

fn gen_mix(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let content = html!(<div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Material"</th>
            <th>"Revealed"</th>
            <th>"Can auto"</th>
            <th>"Default auto"</th>
        </tr></thead>
        <tbody>
        { pedia.item_mix.param.iter().map(|p| {
            html!(<tr>
                <td>{text!("{}x ", p.generated_item_num)}
                {gen_item_label_from_id(p.generated_item_id, pedia_ex)}
                </td>
                <td><ul class="mh-armor-skill-list">
                {p.item_id_list.iter()
                    .filter(|i|!matches!(i, ItemId::Null | ItemId::None))
                    .map(|&i|html!(<li>{gen_item_label_from_id(i, pedia_ex)}</li>))}
                </ul></td>
                <td>{text!("{}", p.default_open_flag)}</td>
                <td>{text!("{}", p.auto_mix_enable_flag)}</td>
                <td>{text!("{}", p.auto_mix_default)}</td>
            </tr>)
        })}
        </tbody>
        </table></div>);

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Item crafting - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Item crafting"</h1></header>
                {content}
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

fn gen_bbq(hash_store: &HashStore, pedia_ex: &PediaEx, mut output: impl Write) -> Result<()> {
    let content = html!(<div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Money cost"</th>
            <th>"Point cost"</th>
            <th>"Bonus point"</th>
            <th>"Fixed output"</th>
            <th>"Random output"</th>
        </tr></thead>
        <tbody>
        { pedia_ex.bbq.iter().map(|bbq| {

            html!(<tr>
            <td>{gen_item_label_from_id(bbq.param.item_id, pedia_ex)}</td>
            <td>{text!("{}z", bbq.param.money_cost)}</td>
            <td>{text!("{}pts", bbq.param.point_cost)}</td>
            <td>{text!("{}", bbq.param.bonus_point)}</td>
            <td><ul class="mh-armor-skill-list">
            {bbq.param.fix_out_item_id_list.iter().zip(&bbq.param.fix_out_num_list)
                .filter(|&(&item, _)|!matches!(item, ItemId::Null | ItemId::None))
                .map(|(&item, &num)| {
                    html!(<li class="il">
                        {text!("{}x ", num)}
                        {gen_item_label_from_id(item, pedia_ex)}
                    </li>)
                })
            }
            </ul></td>
            <td> {
                bbq.table.map(|table| html!(
                <div class="mh-reward-box"><div class="mh-table"><table>
                    <thead><tr>
                        <th>{text!("{}x random choice", bbq.param.random_out_item_num)}
                        //<br/>{translate_rule(table.lot_rule)}
                        </th>
                        <th>"Probability"</th>
                    </tr></thead>
                    <tbody> {
                        gen_reward_table(pedia_ex,
                            &table.item_id_list,
                            &table.num_list,
                            &table.probability_list)
                    } </tbody>
                </table></div></div>)
            )}</td>
            </tr>)
        })}
        </tbody>
        </table></div>);

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Motley mix - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Motley mix"</h1></header>
                {content}
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
                <div class="mh-misc-list">
                <a href="/misc/petalace.html">"Petalace"</a>
                <a href="/misc/market.html">"Market"</a>
                <a href="/misc/lab.html">"Anomaly research lab"</a>
                <a href="/misc/mix.html">"Item crafting"</a>
                <a href="/misc/bbq.html">"Motley mix"</a>
                <a href="/misc/argosy.html">"Argosy"</a>
                </div>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_argosy(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let mut sections = vec![];

    let mut trade: Vec<_> = pedia.trade.param.iter().collect();
    trade.sort_by_key(|p| p.sort_id);
    sections.push(Section {
        title: "Trade request".to_owned(),
        content: html!(<section id="s-trade">
        <h1>"Trade request"</h1>
        <div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Category"</th>
            <th>"Count (base; bargain)"</th>
            <th>"Random add count (base; bargain)"</th>
            //<th>"feature_add_rate"</th>
            <th>"Unlock by village"</th>
            <th>"Unlock by hub"</th>
            <th>"Unlock by MR"</th>
        </tr></thead>
        <tbody>{trade.iter().map(|p| html!(<tr>
            <td>{gen_item_label_from_id(p.item_id, pedia_ex)}</td>
            <td>{text!("{}", p.area)}</td>
            <td>{text!("{0}; {1}/{2}/{0}/{3}/{4}/{0}",
                p.num, p.num + p.add_num[0], p.num + p.add_num[1], p.num + p.add_num[2], p.num + p.add_num[3])}</td>
            <td>{text!("{0}; {1}/{2}/0/{3}/{4}/0",
                p.range, p.add_range[0], p.add_range[1], p.add_range[2], p.add_range[3])}</td>
            //<td>{text!("{}", p.feature_add_rate)}</td>
            <td>{gen_progress(p.unlock_flag_village, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_hall, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_mr_village, pedia_ex)}</td>
        </tr>))}</tbody>
        </table>
        </div>
    </section>),
    });

    sections.push(Section {
        title: "Trade bonus".to_owned(),
        content: html!(<section id="s-rare">
        <h1>"Trade bonus"</h1>
        <div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Category"</th>
            <th>"Rate (base; bargain)"</th>
            <th>"Unlock by village"</th>
            <th>"Unlock by hub"</th>
            <th>"Unlock by MR"</th>
        </tr></thead>
        <tbody>{pedia.trade_rare.param.iter().map(|p| html!(<tr>
            <td>{gen_item_label_from_id(p.item_id, pedia_ex)}</td>
            <td>{text!("{}", p.area)}</td>
            <td>{text!("{0}; {0}/{0}/{1}/{0}/{0}/{2}", p.rate[0], p.rate[1], p.rate[2])}</td>
            <td>{gen_progress(p.unlock_flag_village, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_hall, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_mr_village, pedia_ex)}</td>
        </tr>))}</tbody>
        </table>
        </div>
        </section>),
    });

    sections.push(Section {
        title: "Backroom deals".to_owned(),
        content: html!(<section id="s-feature">
        <h1>"Backroom deals"</h1>
        <div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Rate"</th>
            <th>"Count"</th>
            <th>"Unlock by MR"</th>
            //<th>"Check have"</th>
        </tr></thead>
        <tbody>{pedia.trade_feature.param.iter().map(|p| html!(<tr>
            <td>{gen_item_label_from_id(p.item_id, pedia_ex)}</td>
            <td>{text!("{}", p.rate)}</td>
            <td>{text!("{}", p.drop_num)}</td>
            <td>{gen_progress(p.unlock_flag_mr_village, pedia_ex)}</td>
            //<td>{text!("{}", p.check_have_item)}</td>
        </tr>))}</tbody>
        </table>
        </div>
        </section>),
    });

    sections.push(Section {
        title: "Backroom deals junk".to_owned(),
        content: html!(<section id="s-dust">
        <h1>"Backroom deals junk"</h1>
        <div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Rate"</th>
            <th>"Count"</th>
            <th>"unlock MR"</th>
        </tr></thead>
        <tbody>{pedia.trade_dust.param.iter().map(|p| html!(<tr>
            <td>{gen_item_label_from_id(p.item_id, pedia_ex)}</td>
            <td>{text!("{}", p.rate)}</td>
            <td>{text!("{}", p.drop_num)}</td>
            <td>{gen_progress(p.unlock_flag_mr_village, pedia_ex)}</td>
        </tr>))}</tbody>
        </table>
        </div>
        </section>),
    });

    let mut exchange: Vec<_> = pedia.exchange_item.param.iter().collect();
    exchange.sort_by_key(|p| (p.item_type, p.sort_id));

    sections.push(Section {
        title: "Exchange for items".to_owned(),
        content: html!(<section id="s-exchange">
        <h1>"Exchange for items"</h1>
        <div class="mh-table"><table>
        <thead><tr>
            <th>"Item"</th>
            <th>"Type"</th>
            <th>"Cost"</th>
            <th>"Rate"</th>
            <th>"Count"</th>
            <th>"Unlock by village"</th>
            <th>"Unlock by hub"</th>
            <th>"Unlock by MR"</th>
            <th>"Unlock by monster"</th>
            <th>"Monster count"</th>
            <th>"Unlock by quest"</th>
        </tr></thead>
        <tbody>{exchange.iter().map(|p| html!(<tr>
            <td>{gen_item_label_from_id(p.item_id, pedia_ex)}</td>
            <td>{text!("{}", match p.item_type {
                ExchangeItemTypes::Normal => "Trade goods",
                ExchangeItemTypes::Special => "Special goods",
                ExchangeItemTypes::Random => "Rare Finds",
            })}
            </td>
            <td>{text!("{}pts", p.cost)}</td>
            <td>{text!("{}", p.rate)}</td>
            <td>{text!("{}", p.item_num)}</td>
            <td>{gen_progress(p.unlock_flag_village, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_hall, pedia_ex)}</td>
            <td>{gen_progress(p.unlock_flag_mr_village, pedia_ex)}</td>
            { if p.enemy_num != 0 {
                // TODO: better searching
                let em = pedia_ex.monsters.iter()
                    .find(|(_, monster)|monster.data.enemy_type == Some(p.enemy_id))
                    .map(|(em, _)|*em);
                if let Some(em) = em {
                    html!(<td>{gen_monster_tag(pedia_ex, em, false, false, None, None)}</td>)
                } else {
                    html!(<td>{text!("Unknown monster {}", p.enemy_id)}</td>)
                }
            } else {
                html!(<td/>)
            }}
            <td>{if p.enemy_num != 0 {text!("x{}", p.enemy_num)} else {text!("")}}</td>
            {if p.quest_no != 0 {if let Some(quest) = pedia_ex.quests.get(&p.quest_no) {
                html!(<td>{gen_quest_tag(quest, false, false, None, None)}</td>)
            } else {
                html!(<td>{text!("Unknown quest {}", p.quest_no)}</td>)
            }} else {
                html!(<td/>)
            }}
        </tr>))}</tbody>
        </table>
        </div>
        </section>),
    });

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Argosy - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header><h1>"Argosy"</h1></header>
                { sections.into_iter().map(|s|s.content) }
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

    let path = folder.create_html("lab.html")?;
    gen_lab(hash_store, pedia, pedia_ex, path)?;

    let path = folder.create_html("mix.html")?;
    gen_mix(hash_store, pedia, pedia_ex, path)?;

    let path = folder.create_html("bbq.html")?;
    gen_bbq(hash_store, pedia_ex, path)?;

    let path = folder.create_html("argosy.html")?;
    gen_argosy(hash_store, pedia, pedia_ex, path)?;

    gen_misc_page(hash_store, output.create_html("misc.html")?)?;

    Ok(())
}
