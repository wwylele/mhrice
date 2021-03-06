use super::gen_website::*;
use super::pedia::*;
use crate::rsz::*;
use anyhow::*;
use std::fs::{create_dir, write};
use std::path::*;
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
            <span>{gen_multi_lang(&item.name)}</span>
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

pub fn gen_item(item: &Item, pedia_ex: &PediaEx<'_>, path: &Path) -> Result<()> {
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

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Item - MHRice"</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    {gen_item_icon(item)}
                </div>
                <h1 class="title">
                    {gen_multi_lang(&item.name)}
                </h1>

                <section class="section"><p>
                    {gen_multi_lang(&item.explain)}
                </p></section>

                <section class="section">
                <h2 class="title">"Basic data"</h2>
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

                </div></div></main>
            </body>
        </html>
    );
    write(&path, doc.to_string())?;

    Ok(())
}

pub fn gen_item_list(pedia_ex: &PediaEx<'_>, root: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Items - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Item"</h1>
                <ul class="mh-list-skill">
                {
                    pedia_ex.items.iter().map(|(&id, item)|{
                        let link = format!("/item/{}", item_page(id));
                        let icon = format!("/resources/item/{:03}", item.param.icon_chara);
                        html!(<li class="mh-list-skill">
                            {gen_item_label(&item)}
                        </li>)
                    })
                }
                </ul>
                </div></main>
            </body>
        </html>
    );
    let quests_path = root.join("item.html");
    write(&quests_path, doc.to_string())?;

    Ok(())
}

pub fn gen_items(pedia_ex: &PediaEx, root: &Path) -> Result<()> {
    let item_path = root.join("item");
    create_dir(&item_path)?;
    for (&id, item) in &pedia_ex.items {
        let path = item_path.join(item_page(id));
        gen_item(item, pedia_ex, &path)?
    }
    Ok(())
}
