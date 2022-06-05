use super::gen_armor::*;
use super::gen_hyakuryu_skill::*;
use super::gen_map::*;
use super::gen_monster::*;
use super::gen_otomo::*;
use super::gen_quest::*;
use super::gen_skill::*;
use super::gen_weapon::*;
use super::gen_website::*;
use super::pedia::*;
use super::prepare_map::MapPopKind;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn item_page(item: ItemId) -> String {
    match item {
        ItemId::Null => "null.html".to_string(),
        ItemId::None => "none.html".to_string(),
        ItemId::Normal(id) => format!("normal_{:04}.html", id),
        ItemId::Ec(id) => format!("ec_{:04}.html", id),
    }
}

fn gen_item_icon(item: &Item) -> Box<div<String>> {
    let icon = format!("/resources/item/{:03}", item.param.icon_chara);

    let mut addons = vec![];

    match item.param.icon_item_rank {
        IconRank::Great => addons.push("mh-addon-great"),
        IconRank::Lv1 => addons.push("mh-addon-lv1"),
        IconRank::Lv2 => addons.push("mh-addon-lv2"),
        IconRank::Lv3 => addons.push("mh-addon-lv3"),
        _ => (),
    }

    if item.param.supply {
        addons.push("mh-addon-supply");
    }
    gen_colored_icon(item.param.icon_color, &icon, &addons)
}

pub fn gen_item_label(item: &Item) -> Box<a<String>> {
    let link = format!("/item/{}", item_page(item.param.id));
    html!(
        <a href={link} class="mh-icon-text">
            {gen_item_icon(item)}
            <span>{gen_multi_lang(item.name)}</span>
        </a>
    )
}

pub fn gen_materials(
    pedia_ex: &PediaEx,
    item: &[ItemId],
    item_num: &[u32],
    item_flag: ItemId,
) -> Box<td<String>> {
    html!(<td><ul class="mh-armor-skill-list"> {
        item.iter().zip(item_num)
            .filter(|&(&item, _)| item != ItemId::None)
            .map(|(&item, num)|{
            let key = if item == item_flag {
                Some(html!(<span class="tag is-primary">"Key"</span>))
            } else {
                None
            };
            let item = if let Some(item) = pedia_ex.items.get(&item) {
                html!(<span>{gen_item_label(item)}</span>)
            } else {
                html!(<span>{text!("{:?}", item)}</span>)
            };
            html!(<li>
                {text!("{}x ", num)}
                {item}
                {key}
            </li>)
        })
    } </ul></td>)
}

pub fn gen_category(
    pedia_ex: &PediaEx,
    material_category: MaterialCategory,
    material_category_num: u32,
) -> Box<td<String>> {
    let category = if material_category == MaterialCategory(0) {
        return html!(<td>"-"</td>);
    } else if let Some(name) = pedia_ex.material_categories.get(&material_category) {
        html!(<span>{gen_multi_lang(name)}" "</span>)
    } else {
        html!(<span>{text!("{:?} ", material_category)}</span>)
    };

    html!(<td>{category}{text!("{} pt", material_category_num)}</td>)
}

fn gen_item_source_monster(
    item_id: ItemId,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
) -> Option<Box<div<String>>> {
    let mut em_types: Vec<EmTypes> = pedia_ex
        .monster_lot
        .iter()
        .filter(|(_, lot)| {
            lot.target_reward_item_id_list.contains(&item_id)
                || lot.hagitory_reward_item_id_list.contains(&item_id)
                || lot.capture_reward_item_id_list.contains(&item_id)
                || lot.parts_break_reward_item_id_list.contains(&item_id)
                || lot.drop_reward_item_id_list.contains(&item_id)
                || lot.otomo_reward_item_id_list.contains(&item_id)
        })
        .map(|((em_types, _), _)| *em_types)
        .collect();
    em_types.sort_unstable();
    em_types.dedup();
    if !em_types.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"From monsters, and their quests: "</h3>
            <ul class="mh-item-list">
                {
                    em_types.into_iter().map(|em_type|html!(<li>{
                        gen_monster_tag(pedia, em_type, false, false)
                    }</li>))
                }
            </ul></div>),
        )
    } else {
        None
    }
}

fn gen_item_source_quest(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let quests: Vec<_> = pedia_ex
        .quests
        .iter()
        .filter(|quest| {
            let reward = if let Some(reward) = &quest.reward {
                reward
            } else {
                return false;
            };
            if let Some(r) = reward.common_material_reward {
                if r.item_id_list.contains(&item_id) {
                    return true;
                }
            }

            if let Some(r) = reward.additional_target_reward {
                if r.item_id_list.contains(&item_id) {
                    return true;
                }
            }

            if let Some(r) = reward.cloth_ticket {
                if r.item_id_list.contains(&item_id) {
                    return true;
                }
            }

            for &r in &reward.additional_quest_reward {
                if r.item_id_list.contains(&item_id) {
                    return true;
                }
            }

            false
        })
        .collect();

    if !quests.is_empty() {
        Some(html!(<div class="mh-item-in-out"> <h3>"From quests: "</h3>
        <ul class="mh-item-list">{
            quests.into_iter().map(|quest| {
                html!(<li>{gen_quest_tag(quest, false)}</li>)
            })
        }</ul> </div>))
    } else {
        None
    }
}

fn gen_item_source_weapon(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];
    macro_rules! check_weapon {
        ($weapon:ident) => {
            let weapons = &pedia_ex.$weapon;
            for (_, weapon) in &weapons.weapons {
                let mut found = false;
                if let Some(product) = &weapon.product {
                    if product.output_item.contains(&item_id) {
                        found = true;
                    }
                }

                if let Some(process) = &weapon.process {
                    if process.output_item.contains(&item_id) {
                        found = true;
                    }
                }

                if found {
                    htmls.push(html!(<li>{
                        gen_weapon_label(weapon)
                    }</li>));
                }
            }
        };
    }

    check_weapon!(great_sword);
    check_weapon!(short_sword);
    check_weapon!(hammer);
    check_weapon!(lance);
    check_weapon!(long_sword);
    check_weapon!(slash_axe);
    check_weapon!(gun_lance);
    check_weapon!(dual_blades);
    check_weapon!(horn);
    check_weapon!(insect_glaive);
    check_weapon!(charge_axe);
    check_weapon!(light_bowgun);
    check_weapon!(heavy_bowgun);
    check_weapon!(bow);

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"From crafting / upgrading weapons: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_source_armor(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];

    for series in &pedia_ex.armors {
        for piece in series.pieces.iter().flatten() {
            if let Some(product) = piece.product {
                if product.output_item.contains(&item_id) {
                    htmls.push(html!(<li>
                        <a href={format!("/armor/{:03}.html", series.series.armor_series.0)}>
                            { gen_armor_label(Some(piece)) }
                        </a>
                    </li>))
                }
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"From crafting armors: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_usage_weapon(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];
    macro_rules! check_weapon {
        ($weapon:ident) => {
            let weapons = &pedia_ex.$weapon;
            for (_, weapon) in &weapons.weapons {
                let mut found = false;
                if let Some(product) = &weapon.product {
                    if product.base.item.contains(&item_id) {
                        found = true;
                    }
                }

                if let Some(process) = &weapon.process {
                    if process.base.item.contains(&item_id) {
                        found = true;
                    }
                }

                if let Some(change) = &weapon.change {
                    if change.base.item.contains(&item_id) {
                        found = true;
                    }
                }

                if found {
                    htmls.push(html!(<li>{
                        gen_weapon_label(weapon)
                    }</li>));
                }
            }
        };
    }

    check_weapon!(great_sword);
    check_weapon!(short_sword);
    check_weapon!(hammer);
    check_weapon!(lance);
    check_weapon!(long_sword);
    check_weapon!(slash_axe);
    check_weapon!(gun_lance);
    check_weapon!(dual_blades);
    check_weapon!(horn);
    check_weapon!(insect_glaive);
    check_weapon!(charge_axe);
    check_weapon!(light_bowgun);
    check_weapon!(heavy_bowgun);
    check_weapon!(bow);

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"For crafting / upgrading weapons: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_usage_armor(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];

    for series in &pedia_ex.armors {
        for piece in series.pieces.iter().flatten() {
            let mut found = false;
            if let Some(product) = piece.product {
                if product.item.contains(&item_id) {
                    found = true;
                }
            }
            if let Some(overwear_product) = piece.overwear_product {
                if overwear_product.item.contains(&item_id) {
                    found = true;
                }
            }
            if found {
                htmls.push(html!(<li>
                    <a href={format!("/armor/{:03}.html", series.series.armor_series.0)}>
                        { gen_armor_label(Some(piece)) }
                    </a>
                </li>))
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"For crafting armors: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_usage_otomo(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];

    for (id, series) in &pedia_ex.ot_equip {
        let href = format!("/otomo/{}.html", id.to_tag());

        for armor in [&series.head, &series.chest].into_iter().flatten() {
            if let piece @ OtArmor {
                product: Some(product),
                ..
            } = armor
            {
                if product.item_list.contains(&item_id) {
                    htmls.push(html!(<li>
                    <a href={&href}>
                        { gen_atomo_armor_label(piece) }
                    </a>
                </li>))
                }
            }
        }

        if let Some(
            piece @ OtWeapon {
                product: Some(product),
                ..
            },
        ) = &series.weapon
        {
            if product.item_list.contains(&item_id) {
                htmls.push(html!(<li>
                <a href={&href}>
                    { gen_atomo_weapon_label(piece) }
                </a>
            </li>))
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"For crafting buddy equipements: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_usage_hyakuryu(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];

    for skill in pedia_ex.hyakuryu_skills.values() {
        if let Some(reciepe) = skill.recipe {
            if reciepe.recipe_item_id_list.contains(&item_id) {
                htmls.push(html!(<li>
                    { gen_hyakuryu_skill_label(skill) }
                </li>))
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"For enabling ramp-up skills: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_usage_deco(item_id: ItemId, pedia_ex: &PediaEx) -> Option<Box<div<String>>> {
    let mut htmls = vec![];

    for (&id, skill) in &pedia_ex.skills {
        if let Some(deco) = &skill.deco {
            if deco.product.item_id_list.contains(&item_id) {
                htmls.push(html!(<li>
                    <a href={format!("/skill/{}", skill_page(id))}>
                    { gen_deco_label(deco) }
                    </a>
                </li>))
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<div class="mh-item-in-out"> <h3>"For crafting decorations: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>),
        )
    } else {
        None
    }
}

fn gen_item_source_map(
    item_id: ItemId,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
) -> Option<Box<div<String>>> {
    let mut htmls = vec![];
    for (&id, map) in &pedia.maps {
        let mut found = false;
        for pop in &map.pops {
            match &pop.kind {
                MapPopKind::Item { behavior, .. } => {
                    if let Some(lot) = pedia_ex
                        .item_pop
                        .get(&(behavior.pop_id, id))
                        .or_else(|| pedia_ex.item_pop.get(&(behavior.pop_id, -1)))
                    {
                        if lot.lower_id.contains(&item_id) || lot.upper_id.contains(&item_id) {
                            found = true;
                            break;
                        }
                    }
                }
                MapPopKind::FishingPoint { behavior } => {
                    let spawn = behavior.fish_spawn_data.unwrap();
                    let fishes = spawn
                        .spawn_group_list_info_low
                        .iter()
                        .chain(spawn.spawn_group_list_info_high.iter())
                        .flat_map(|f| f.fish_spawn_rate_list.iter());
                    for fish in fishes {
                        if get_fish_item_id(fish.fish_id) == Some(item_id) {
                            found = true;
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
        if found {
            htmls.push(html!(<li> {gen_map_label(id, pedia)} </li>));
        }
    }

    if !htmls.is_empty() {
        Some(html!(<div class="mh-item-in-out"> <h3>"From maps: "</h3>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div>))
    } else {
        None
    }
}

static ITEM_TYPES: Lazy<BTreeMap<ItemTypes, (&'static str, &'static str)>> = Lazy::new(|| {
    BTreeMap::from_iter([
        (ItemTypes::Consume, ("consume", "Consumable")),
        (ItemTypes::Tool, ("tool", "Tool")),
        (ItemTypes::Material, ("material", "Material")),
        (ItemTypes::OffcutsMaterial, ("offcuts", "Scrap")),
        (ItemTypes::Bullet, ("bullet", "Ammo")),
        (ItemTypes::Bottle, ("bottle", "Bottle")),
        (ItemTypes::Present, ("present", "Present")),
        (ItemTypes::PayOff, ("payoff", "Account")),
        (ItemTypes::CarryPayOff, ("carrypayoff", "Carrying Account")),
        (ItemTypes::Carry, ("carry", "Carrying")),
        (ItemTypes::Judge, ("judge", "Judge")),
        (ItemTypes::Antique, ("antique", "Antique")),
    ])
});

pub fn gen_item(
    item: &Item,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    let material_categories = item.param.material_category.iter().filter_map(|&category| {
        if category == MaterialCategory(0) {
            return None;
        }
        Some(
            if let Some(name) = pedia_ex.material_categories.get(&category) {
                html!(<span>{gen_multi_lang(name)}" "</span>)
            } else {
                html!(<span>{text!("{:?} ", category)}</span>)
            },
        )
    });

    toc_sink.add(item.name);

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Item - MHRice"</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_item_icon(item)}
                    </div>
                    <h1>{gen_multi_lang(item.name)}</h1>
                </header>

                <section><pre>
                    {gen_multi_lang(item.explain)}
                </pre></section>

                <section>
                <h2 >"Basic data"</h2>
                <div class="mh-kvlist">
                <p class="mh-kv"><span>"Carriable filter"</span>
                <span>{text!("{:?}", item.param.cariable_filter)}</span></p>
                <p class="mh-kv"><span>"Type"</span>
                <span>{text!("{:?}", item.param.type_)}</span></p>
                <p class="mh-kv"><span>"Rarity"</span>
                <span>{text!("{}", item.param.rare.0)}</span></p>
                <p class="mh-kv"><span>"Maximum carry"</span>
                <span>{text!("{}", item.param.pl_max_count)}</span></p>
                <p class="mh-kv"><span>"Maximum carry by buddy"</span>
                <span>{text!("{}", item.param.ot_max_count)}</span></p>
                <p class="mh-kv"><span>"In item bar"</span>
                <span>{text!("{}", item.param.show_item_window)}</span></p>
                <p class="mh-kv"><span>"In action bar"</span>
                <span>{text!("{}", item.param.show_action_window)}</span></p>
                <p class="mh-kv"><span>"Infinite"</span>
                <span>{text!("{}", item.param.infinite)}</span></p>
                <p class="mh-kv"><span>"Fixed item"</span>
                <span>{text!("{}", item.param.default)}</span></p>
                /*<p class="mh-kv"><span>"SE type"</span>
                <span>{text!("{:?}", item.param.se_type)}</span></p>*/
                <p class="mh-kv"><span>"Sell price"</span>
                <span>{text!("{}", item.param.sell_price)}</span></p>
                <p class="mh-kv"><span>"Buy price"</span>
                <span>{text!("{}", item.param.buy_price)}</span></p>
                /*<p class="mh-kv"><span>"Rank type"</span>
                <span>{text!("{:?}", item.param.rank_type)}</span></p>*/
                <p class="mh-kv"><span>"Item group"</span>
                <span>{text!("{:?}", item.param.item_group)}</span></p>
                <p class="mh-kv"><span>"Material category"</span>
                <span>
                    {material_categories}
                    {text!("{} pt", item.param.category_worth)}
                </span></p>
                <p class="mh-kv"><span>"Evaluation value"</span>
                <span>{text!("{}",item.param.evaluation_value)}</span></p>
                </div>
                </section>

                <section>
                <h2 >"Where to get"</h2>
                {gen_item_source_monster(item.param.id, pedia, pedia_ex)}
                {gen_item_source_quest(item.param.id, pedia_ex)}
                {gen_item_source_map(item.param.id, pedia, pedia_ex)}
                {gen_item_source_weapon(item.param.id, pedia_ex)}
                {gen_item_source_armor(item.param.id, pedia_ex)}
                </section>

                <section>
                <h2 >"Where to use"</h2>
                {gen_item_usage_weapon(item.param.id, pedia_ex)}
                {gen_item_usage_armor(item.param.id, pedia_ex)}
                {gen_item_usage_otomo(item.param.id, pedia_ex)}
                {gen_item_usage_deco(item.param.id, pedia_ex)}
                {gen_item_usage_hyakuryu(item.param.id, pedia_ex)}
                </section>

                </main>
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_item_list(pedia_ex: &PediaEx<'_>, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Items - MHRice")}</title>
                { head_common() }
                <style id="mh-item-list-style">""</style>
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Item"</h1></header>
                <div class="mh-filters"><ul>
                    <li id="mh-item-filter-button-all" class="is-active mh-item-filter-button">
                        <a>"All items"</a></li>
                    {
                        ITEM_TYPES.iter().map(|(_, (symbol, name))| {
                            let id = format!("mh-item-filter-button-{symbol}");
                            html!(<li id={id.as_str()} class="mh-item-filter-button">
                                <a>{text!("{}", name)}</a></li>)
                        })
                    }
                </ul></div>
                <div class="select"><select id="scombo-item" class="mh-scombo">
                    <option value="0">"Sort by internal ID"</option>
                    <option value="1">"Sort by in-game order"</option>
                </select></div>
                <ul class="mh-item-list" id="slist-item">
                {
                    pedia_ex.items.iter().map(|(i, item)|{
                        let sort_id = item.param.sort_id;
                        let sort_tag = format!("{},{}", i.into_raw(), sort_id);
                        let filter = ITEM_TYPES[&item.param.type_].0;
                        html!(<li class="mh-item-filter-item"
                            data-sort=sort_tag data-filter={filter}>
                            {gen_item_label(item)}
                        </li>)
                    })
                }
                </ul>
                </main>
            </body>
        </html>
    );
    output
        .create_html("item.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_items(
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let item_path = output.sub_sink("item")?;
    for (&id, item) in &pedia_ex.items {
        let (path, toc_sink) = item_path.create_html_with_toc(&item_page(id), toc)?;
        gen_item(item, pedia, pedia_ex, path, toc_sink)?
    }
    Ok(())
}

pub fn gen_reward_table<'a>(
    pedia_ex: &'a PediaEx,
    item: &'a [ItemId],
    num: &'a [u32],
    probability: &'a [u32],
) -> impl Iterator<Item = Box<tr<String>>> + 'a {
    item.iter()
        .zip(num)
        .zip(probability)
        .filter(|&((&item, _), _)| item != ItemId::None)
        .map(move |((&item, &num), probability)| {
            let item = if let Some(item) = pedia_ex.items.get(&item) {
                html!(<span>{gen_item_label(item)}</span>)
            } else {
                html!(<span>{text!("{:?}", item)}</span>)
            };

            html!(<tr>
                <td>{text!("{}x ", num)}{item}</td>
                <td>{text!("{}%", probability)}</td>
            </tr>)
        })
}
