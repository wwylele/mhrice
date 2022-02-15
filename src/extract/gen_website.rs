use super::gen_armor::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_quest::*;
use super::gen_skill::*;
use super::gen_weapon::*;
use super::pedia::*;
use crate::msg::*;
use crate::part_color::*;
use crate::rsz::*;
use anyhow::Result;
use chrono::prelude::*;
use std::convert::TryInto;
use std::fs::{create_dir, remove_dir_all, write, File};
use std::io::Write;
use std::path::*;
use typed_html::{dom::*, elements::*, html, text, types::*};

const LANGUAGE_MAP: [Option<&str>; 32] = [
    Some("Japanese"),
    Some("English"),
    Some("French"),
    Some("Italian"),
    Some("German"),
    Some("Spanish"),
    Some("Russian"),
    Some("Polish"),
    None,
    None,
    Some("Portuguese"),
    Some("Korean"),
    Some("Traditional Chinese"),
    Some("Simplified Chinese"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("Arabic"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

pub fn head_common() -> Vec<Box<dyn MetadataContent<String>>> {
    vec![
        html!(<meta charset="UTF-8" />),
        html!(<link rel="icon" type="image/png" href="/favicon.png" />),
        html!(<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css" />),
        html!(<link rel="stylesheet" href="/mhrice.css" />),
        html!(<link rel="stylesheet" href="/part_color.css" />),
        html!(<link rel="stylesheet" href="/resources/item_color.css" />),
        html!(<link rel="stylesheet" href="/resources/rarity_color.css" />),
        html!(<script src="https://kit.fontawesome.com/ceb13a2ba1.js" crossorigin="anonymous" />),
        html!(<script src="/mhrice.js" crossorigin="anonymous" />),
    ]
}

pub fn navbar() -> Box<div<String>> {
    html!(<div>
        <nav class="navbar is-primary" role="navigation"> <div class="container">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <img src="/favicon.png"/>
                    <div class="mh-logo-text">"MHRice"</div>
                </a>

                <a id="navbarBurger" class="navbar-burger" data-target="navbarMenu" onclick="onToggleNavbarMenu()">
                    <span></span>
                    <span></span>
                    <span></span>
                </a>
            </div>

            <div id="navbarMenu" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/monster.html">
                        "Monsters"
                    </a>
                    <a class="navbar-item" href="/quest.html">
                        "Quests"
                    </a>
                    <a class="navbar-item" href="/skill.html">
                        "Skills"
                    </a>
                    <a class="navbar-item" href="/armor.html">
                        "Armors"
                    </a>

                    <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                        "Weapon"
                    </a>
                    <div class="navbar-dropdown">
                        <a class="navbar-item" href="/weapon/great_sword.html">"Great sword"</a>
                        <a class="navbar-item" href="/weapon/long_sword.html">"Long sword"</a>
                        <a class="navbar-item" href="/weapon/short_sword.html">"Sword & shield"</a>
                        <a class="navbar-item" href="/weapon/dual_blades.html">"Dual blades"</a>
                        <a class="navbar-item" href="/weapon/hammer.html">"Hammer"</a>
                        <a class="navbar-item" href="/weapon/horn.html">"Hunting horn"</a>
                        <a class="navbar-item" href="/weapon/lance.html">"Lance"</a>
                        <a class="navbar-item" href="/weapon/gun_lance.html">"Gunlance"</a>
                        <a class="navbar-item" href="/weapon/slash_axe.html">"Switch axe"</a>
                        <a class="navbar-item" href="/weapon/charge_axe.html">"Charge blade"</a>
                        <a class="navbar-item" href="/weapon/insect_glaive.html">"Insect glaive"</a>
                        <a class="navbar-item" href="/weapon/light_bowgun.html">"Light bowgun"</a>
                        <a class="navbar-item" href="/weapon/heavy_bowgun.html">"Heavy bowgun"</a>
                        <a class="navbar-item" href="/weapon/bow.html">"Bow"</a>
                    </div>
                    </div>

                    <a class="navbar-item" href="/item.html">
                        "Items"
                    </a>
                    <a class="navbar-item" href="/about.html">
                        "About"
                    </a>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            "Data language"
                        </a>
                        <div class="navbar-dropdown">{
                            (0..32).filter_map(|i| {
                                let language_name = LANGUAGE_MAP[i]?;
                                let onclick = format!("selectLanguage({})", i);
                                let class_string = format!("navbar-item mh-lang-menu-{}", i);
                                let c: SpacedSet<Class> = class_string.as_str().try_into().unwrap();
                                Some(html!{ <a class=c onclick=onclick> {
                                    text!("{}", language_name)
                                }</a>: String})
                            })
                        }
                        </div>
                    </div>
                </div>
            </div>
        </div> </nav>
    </div>: String)
}

pub fn translate_msg(content: &str) -> Box<span<String>> {
    let mut msg = content;
    struct Tag<'a> {
        tag: &'a str,
        arg: &'a str,
        seq: Seq<'a>,
    }
    enum Node<'a> {
        Raw(&'a str),
        Tagged(Tag<'a>),
    }

    struct Seq<'a> {
        nodes: Vec<Node<'a>>,
    }

    let mut root = Seq { nodes: vec![] };
    let mut stack: Vec<Tag> = vec![];

    loop {
        let next_stop = if let Some(next_stop) = msg.find('<') {
            next_stop
        } else {
            if !stack.is_empty() {
                eprintln!("Parse error: {}", content);
            }
            root.nodes.push(Node::Raw(msg));
            break;
        };

        let raw = Node::Raw(&msg[0..next_stop]);
        msg = &msg[(next_stop + 1)..];
        if let Some(last) = stack.last_mut() {
            last.seq.nodes.push(raw);
        } else {
            root.nodes.push(raw);
        }

        let next_stop = if let Some(next_stop) = msg.find('>') {
            next_stop
        } else {
            if !stack.is_empty() {
                eprintln!("Parse error: {}", content);
            }
            break;
        };
        let tag = &msg[0..next_stop];
        msg = &msg[(next_stop + 1)..];

        if let Some(tag) = tag.strip_prefix('/') {
            let stack_tag = if let Some(stack_tag) = stack.pop() {
                stack_tag
            } else {
                eprintln!("Parse error: {}", content);
                break;
            };
            if stack_tag.tag != tag {
                eprintln!("Parse error: {}", content);
            }
            let tag = Node::Tagged(stack_tag);
            if let Some(last) = stack.last_mut() {
                last.seq.nodes.push(tag);
            } else {
                root.nodes.push(tag);
            }
        } else {
            let (tag, arg) = if let Some(space) = tag.find(' ') {
                (&tag[0..space], &tag[(space + 1)..])
            } else {
                (tag, "")
            };
            let mut tag = Tag {
                tag,
                arg,
                seq: Seq { nodes: vec![] },
            };
            if matches!(tag.tag, "LSNR" | "PL" | "ПУСТО") {
                let tag = Node::Tagged(tag);
                if let Some(last) = stack.last_mut() {
                    last.seq.nodes.push(tag);
                } else {
                    root.nodes.push(tag);
                }
            } else if tag.tag == "COLS" {
                tag.tag = "COL";
                stack.push(tag);
            } else {
                stack.push(tag);
            }
        }
    }

    fn translate_rec(node: &Node<'_>) -> Box<dyn PhrasingContent<String>> {
        match node {
            Node::Raw(s) => Box::new(TextNode::<String>::new(*s)),
            Node::Tagged(t) => {
                let inner = t.seq.nodes.iter().map(translate_rec);
                match t.tag {
                    "COLOR" => {
                        let style = format!("color: #{};", t.arg);
                        html!(<span style={style}> {inner} </span>)
                    }
                    "COL" => {
                        let color = match t.arg {
                            "RED" => "red",
                            "YEL" => "yellow",
                            _ => {
                                eprintln!("Unknown color: {}", t.arg);
                                "black"
                            }
                        };
                        let style = format!("color: {};", color);
                        html!(<span style={style}> {inner} </span>)
                    }
                    "LSNR" => {
                        // Gender selector
                        html!(<span class="mh-msg-place-holder"> {text!("{}", t.arg)} </span>)
                    }
                    "BSL" => {
                        // Text direction change?
                        html!(<span> {inner} </span>)
                    }
                    "PL" => {
                        html!(<span class="mh-msg-place-holder"> "{Player}" </span>)
                    }
                    "ПУСТО" => {
                        html!(<span> "<ПУСТО>" </span>)
                    }
                    _ => {
                        eprintln!("Unknown tag: {}", t.tag);
                        html!(<span> {inner} </span>)
                    }
                }
            }
        }
    }

    html!(<span> {root.nodes.iter().map(translate_rec)} </span>)
}

pub fn gen_multi_lang(msg: &MsgEntry) -> Box<span<String>> {
    html!(<span> {
        (0..32).filter(|&i|LANGUAGE_MAP[i].is_some()).map(|i|{
            let hidden = if i == 1 {
                ""
            } else {
                " mh-hidden"
            };
            html! (<span class={format!("mh-lang-{}{}", i, hidden).as_str()}>
                {translate_msg(&msg.content[i])}
            </span>)
        })
    } </span>)
}

pub fn gen_colored_icon(color: i32, icon: &str, addons: &[&str]) -> Box<div<String>> {
    let color_class = format!("mh-item-color-{}", color);
    gen_colored_icon_inner(&color_class, icon, addons)
}

pub fn gen_rared_icon(rarity: RareTypes, icon: &str) -> Box<div<String>> {
    let color_class = format!("mh-rarity-color-{}", rarity.0);
    gen_colored_icon_inner(&color_class, icon, &[])
}

fn gen_colored_icon_inner(color_class: &str, icon: &str, addons: &[&str]) -> Box<div<String>> {
    let image_r_base = format!("url('{}.r.png')", icon);
    let image_a_base = format!("url('{}.a.png')", icon);
    let image_r = format!("mask-image: {0}; -webkit-mask-image: {0};", image_r_base);
    let image_a = format!("mask-image: {0}; -webkit-mask-image: {0};", image_a_base);
    html!(<div class="mh-colored-icon">
        <div style={image_r.as_str()} class={color_class}/>
        <div style={image_a.as_str()}/>
        <div>{ addons.iter().map(|&addon| html!(
            <div class=addon/>
        )) }</div>
    </div>)
}

pub fn gen_monsters(pedia: &Pedia, pedia_ex: &PediaEx<'_>, root: &Path) -> Result<()> {
    let monsters_path = root.join("monster.html");

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Monsters"</h1>
                <section class="section">
                <h2 class="title">"Large monsters"</h2>
                <div class="select"><select id="scombo-monster" onchange="onChangeSort(this)">
                    <option value="0">"Sort by internal ID"</option>
                    <option value="1">"Sort by in-game order"</option>
                </select></div>
                <ul class="mh-list-monster" id="slist-monster">{
                    pedia.monsters.iter().filter_map(|monster| {
                        let icon_path = format!("/resources/em{0:03}_{1:02}_icon.png", monster.id, monster.sub_id);
                        let name_name = format!("EnemyIndex{:03}", monster.enemy_type?);

                        let name_entry = pedia.monster_names.get_entry(&name_name)?;
                        let order = pedia_ex.monster_order.get(&EmTypes::Em(monster.id | (monster.sub_id << 8)))
                            .cloned().unwrap_or(0);
                        let sort_tag = format!("{},{}", monster.id << 16 | monster.sub_id, order);
                        Some(html!{<li class="mh-list-monster" data-sort=sort_tag>
                            <a href={format!("/monster/{:03}_{:02}.html", monster.id, monster.sub_id)}>
                                <img class="mh-list-monster-icon" src=icon_path />
                                <div>{gen_multi_lang(name_entry)}</div>
                            </a>
                        </li>})
                    }).collect::<Vec<_>>()
                }</ul>
                </section>
                <section class="section">
                <h2 class="title">"Small monsters"</h2>
                <ul class="mh-list-monster">{
                    pedia.small_monsters.iter().filter(|monster|monster.sub_id == 0) // sub small monsters are b0rked
                    .map(|monster| {
                        let icon_path = format!("/resources/ems{0:03}_{1:02}_icon.png", monster.id, monster.sub_id);
                        html!{<li class="mh-list-monster">
                            <a href={format!("/small-monster/{:03}_{:02}.html", monster.id, monster.sub_id)}>
                                <img class="mh-list-monster-icon" src=icon_path />
                                <div>{
                                    text!("Small monster {:03}_{:02}", monster.id, monster.sub_id)
                                }</div>
                            </a>
                        </li>}
                    })
                }</ul>
                </section>
                </div> </main>
            </body>
        </html>: String
    );

    write(&monsters_path, doc.to_string())?;

    let monster_path = root.join("monster");
    create_dir(&monster_path)?;
    for monster in &pedia.monsters {
        gen_monster(true, monster, pedia, pedia_ex, &monster_path)?;
    }

    let monster_path = root.join("small-monster");
    create_dir(&monster_path)?;
    for monster in &pedia.small_monsters {
        gen_monster(false, monster, pedia, pedia_ex, &monster_path)?;
    }
    Ok(())
}

pub fn gen_about(root: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"About MHRice"</h1>
                <p>
                "MHRice is an information site for Monster Hunter Rise, displaying data extracted from the game."
                </p>
                <p>
                "MHRice website is generated from the open source MHRice project."
                </p>
                <p>
                <a class="button" href="https://github.com/wwylele/mhrice" target="_blank" rel=["noopener", "noreferrer"]>
                    <span class="icon">
                        <i class="fab fa-github"></i>
                    </span>
                    <span>"Visit MHRice on Github"</span>
                </a>
                </p>
                <section class="section">
                <h2 class="title">"Build information"</h2>
                <ul>
                    <li>"Git hash: " <span class="is-family-monospace">{
                        text!("{}{}",
                            crate::built_info::GIT_COMMIT_HASH.unwrap_or("unknown"),
                            if crate::built_info::GIT_DIRTY == Some(true) {
                                "-dirty"
                            } else {
                                ""
                            }
                        )
                    }</span></li>
                    <li>{text!("Update time: {}", Utc::now())}</li>
                </ul>
                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    let about_path = root.join("about.html");
    write(&about_path, doc.to_string())?;

    Ok(())
}

pub fn gen_static(root: &Path) -> Result<()> {
    write(
        root.to_path_buf().join("mhrice.css"),
        include_bytes!("static/mhrice.css"),
    )?;
    write(
        root.to_path_buf().join("mhrice.js"),
        include_bytes!("static/mhrice.js"),
    )?;
    write(
        root.to_path_buf().join("favicon.png"),
        include_bytes!("static/favicon.png"),
    )?;
    Ok(())
}

pub fn gen_part_color_css(root: &Path) -> Result<()> {
    let mut file = File::create(root.to_path_buf().join("part_color.css"))?;

    for (i, color) in PART_COLORS.iter().enumerate() {
        writeln!(file, ".mh-part-{} {{color: {}}}", i, color)?;
    }

    Ok(())
}

pub fn gen_website(pedia: &Pedia, pedia_ex: &PediaEx<'_>, output: &str) -> Result<()> {
    let root = PathBuf::from(output);
    if root.exists() {
        remove_dir_all(&root)?;
    }
    create_dir(&root)?;

    gen_quests(pedia, pedia_ex, &root)?;
    gen_skills(pedia_ex, &root)?;
    gen_skill_list(&pedia_ex.skills, &root)?;
    gen_armors(pedia_ex, &root)?;
    gen_armor_list(&pedia_ex.armors, &root)?;
    gen_monsters(pedia, pedia_ex, &root)?;
    gen_quest_list(&pedia_ex.quests, &root)?;
    gen_items(pedia, pedia_ex, &root)?;
    gen_item_list(pedia_ex, &root)?;
    gen_weapons(pedia_ex, &root)?;
    gen_about(&root)?;
    gen_static(&root)?;
    gen_part_color_css(&root)?;
    Ok(())
}
