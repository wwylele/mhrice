//use super::gen_armor::*;
//use super::gen_hyakuryu_skill::*;
use super::gen_item::*;
//use super::gen_map::*;
use super::gen_monster::*;
//use super::gen_otomo::*;
use super::gen_quest::*;
//use super::gen_skill::*;
//use super::gen_weapon::*;
use super::pedia::*;
use super::sink::*;
use crate::msg::*;
use crate::part_color::*;
use crate::rsz::*;
use anyhow::Result;
use chrono::prelude::*;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text, types::*};

pub const LANGUAGE_MAP: [Option<(&str, &str)>; 32] = [
    Some(("Japanese", "ja")),
    Some(("English", "en")),
    Some(("French", "fr")),
    Some(("Italian", "it")),
    Some(("German", "de")),
    Some(("Spanish", "es")),
    Some(("Russian", "ru")),
    Some(("Polish", "pl")),
    None,
    None,
    Some(("Portuguese", "pt")),
    Some(("Korean", "ko")),
    Some(("Traditional Chinese", "zh-TW")),
    Some(("Simplified Chinese", "zh-CN")),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(("Arabic", "ar")),
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
        html!(<meta name="viewport" content="width=device-width, initial-scale=1" />),
        html!(<link rel="icon" type="image/png" href="/favicon.png" />),
        html!(<link rel="stylesheet" href="/mhrice.css" />),
        html!(<link rel="stylesheet" href="/part_color.css" />),
        html!(<link rel="stylesheet" href="/resources/item_color.css" />),
        html!(<link rel="stylesheet" href="/resources/rarity_color.css" />),
        html!(<script src="https://kit.fontawesome.com/ceb13a2ba1.js" crossorigin="anonymous" />),
        html!(<script src="/mhrice.js" crossorigin="anonymous" />),
        html!(<style id="mh-lang-style">".mh-lang:not(.lang-default) { display:none; }"</style>),
    ]
}

pub fn navbar() -> Box<nav<String>> {
    html!(<nav><div>
        <div class="navbar-brand">
            <a class="navbar-item" href="/index.html">
                <img alt="Logo" src="/favicon.png"/>
                <div class="mh-logo-text">"MHRice "</div>
                <i class="fas fa-search"/>
            </a>

            <a id="navbarBurger" class="navbar-burger" data-target="navbarMenu">
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

                // <div class="navbar-item has-dropdown is-hoverable">
                // <a class="navbar-link">
                //     "Skills"
                // </a>
                // <div class="navbar-dropdown">
                //     <a class="navbar-item" href="/skill.html">
                //         "Armor skills"
                //     </a>
                //     <a class="navbar-item" href="/hyakuryu_skill.html">
                //         "Ramp-up skills"
                //     </a>
                // </div>
                // </div>

                // <a class="navbar-item" href="/armor.html">
                //     "Armors"
                // </a>

                // <div class="navbar-item has-dropdown is-hoverable">
                // <a class="navbar-link">
                //     "Weapon"
                // </a>
                // <div class="navbar-dropdown">
                //     <a class="navbar-item" href="/weapon/great_sword.html">"Great sword"</a>
                //     <a class="navbar-item" href="/weapon/long_sword.html">"Long sword"</a>
                //     <a class="navbar-item" href="/weapon/short_sword.html">"Sword & shield"</a>
                //     <a class="navbar-item" href="/weapon/dual_blades.html">"Dual blades"</a>
                //     <a class="navbar-item" href="/weapon/hammer.html">"Hammer"</a>
                //     <a class="navbar-item" href="/weapon/horn.html">"Hunting horn"</a>
                //     <a class="navbar-item" href="/weapon/lance.html">"Lance"</a>
                //     <a class="navbar-item" href="/weapon/gun_lance.html">"Gunlance"</a>
                //     <a class="navbar-item" href="/weapon/slash_axe.html">"Switch axe"</a>
                //     <a class="navbar-item" href="/weapon/charge_axe.html">"Charge blade"</a>
                //     <a class="navbar-item" href="/weapon/insect_glaive.html">"Insect glaive"</a>
                //     <a class="navbar-item" href="/weapon/light_bowgun.html">"Light bowgun"</a>
                //     <a class="navbar-item" href="/weapon/heavy_bowgun.html">"Heavy bowgun"</a>
                //     <a class="navbar-item" href="/weapon/bow.html">"Bow"</a>
                // </div>
                // </div>

                // <div class="navbar-item has-dropdown is-hoverable">
                // <a class="navbar-link">
                //     "Buddy"
                // </a>
                // <div class="navbar-dropdown">
                //     <a class="navbar-item" href="/airou.html">"Palico equipment"</a>
                //     <a class="navbar-item" href="/dog.html">"Palamute equipment"</a>
                // </div>
                // </div>

                // <a class="navbar-item" href="/map.html">
                //     "Maps"
                // </a>

                <a class="navbar-item" href="/item.html">
                   "Items"
                </a>
                <a class="navbar-item">
                    "(More are coming soon...)"
                </a>
                <a class="navbar-item" href="/about.html">
                    "About"
                </a>
            </div>
            <div class="navbar-end">
                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                        "Language"
                    </a>
                    <div class="navbar-dropdown">{
                        (0..32).filter_map(|i| {
                            let (language_name, language_code) = LANGUAGE_MAP[i]?;
                            let id_string = format!("mh-lang-menu-{language_code}");
                            Some(html!{ <a class="navbar-item mh-lang-menu" id={id_string.as_str()}> {
                                text!("{}", language_name)
                            }</a>})
                        })
                    }
                    <hr class="navbar-divider"/>
                    <div class="navbar-item">
                        "Use cookie to save preference"
                    </div>
                    <div class="navbar-item">
                        <div class="buttons has-addons">
                            <button id="cookie-yes" class="button is-small">"Yes"</button>
                            <button id="cookie-no" class="button is-small is-selected is-danger">"No"</button>
                        </div>
                    </div>

                    </div>
                </div>
            </div>
        </div>
    </div></nav>)
}

pub fn translate_msg(content: &str) -> (Box<span<String>>, bool) {
    let mut msg = content;
    let mut has_warning = false;
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
            msg.len()
        };

        let raw = Node::Raw(&msg[0..next_stop]);
        if let Some(last) = stack.last_mut() {
            last.seq.nodes.push(raw);
        } else {
            root.nodes.push(raw);
        }

        if msg.len() == next_stop {
            if !stack.is_empty() {
                has_warning = true;
                //eprintln!("Unfinished tag: {}", content);
            }

            while let Some(stack_tag) = stack.pop() {
                let tag = Node::Tagged(stack_tag);
                if let Some(last) = stack.last_mut() {
                    last.seq.nodes.push(tag);
                } else {
                    root.nodes.push(tag);
                }
            }

            break;
        }

        msg = &msg[(next_stop + 1)..];

        let next_stop = if let Some(next_stop) = msg.find('>') {
            next_stop
        } else {
            if !stack.is_empty() {
                has_warning = true;
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
                has_warning = true;
                eprintln!("Parse error: {}", content);
                break;
            };
            if stack_tag.tag != tag {
                has_warning = true;
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

            if tag.tag == "COLS" {
                tag.tag = "COL";
                stack.push(tag);
            } else if matches!(
                tag.tag,
                "COLOR" | "COL" | "BSL" | "LEFT" | "FONT" | "TCU" | "size"
            ) {
                stack.push(tag);
            } else {
                let tag = Node::Tagged(tag);
                if let Some(last) = stack.last_mut() {
                    last.seq.nodes.push(tag);
                } else {
                    root.nodes.push(tag);
                }
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
                            "YEL" | "YELLOW" => "orange",
                            "GRAY" => "gray",
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
                        html!(<span>
                            <span class="mh-msg-place-holder">{text!("<{} {}>", t.tag, t.arg)}</span>
                            {inner}
                            <span class="mh-msg-place-holder">{text!("</{}>", t.tag)}</span>
                        </span>)
                    }
                }
            }
        }
    }

    let result = html!(<span> {root.nodes.iter().map(translate_rec)} </span>);
    (result, has_warning)
}

pub fn gen_multi_lang(msg: &MsgEntry) -> Box<span<String>> {
    html!(<span> {
        (0..32).filter_map(|i|{
            let class_string = if i == 1 {
                "mh-lang lang-default"
            } else {
                "mh-lang"
            };
            let (_, language_code) = LANGUAGE_MAP[i]?;
            let language_code: LanguageTag = language_code.parse().unwrap();
            let (msg, warning) = translate_msg(&msg.content[i]);
            let warning = warning.then(||html!(<span class="icon has-text-warning">
                <i class="fas fa-exclamation-triangle" title="There is a parsing error"/>
            </span>));
            Some(html! (<span class={class_string} lang={language_code} >
                {msg} {warning}
            </span>))
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

pub fn gen_search(output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Search"</h1></header>
                <div class="control has-icons-left">
                    <input class="input is-large" type="text" placeholder="Nargacuga" id="mh-search"/>
                    <span class="icon is-large is-left">
                        <i class="fas fa-search" />
                    </span>
                </div>
                <ul id="mh-search-result">
                </ul>
                </main>
            </body>
        </html>
    );

    output
        .create_html("index.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_about(output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"About MHRice"</h1></header>
                <section>
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
                </section>
                <section>
                <h2 >"Download data"</h2>
                <p>
                "MHRice data is also available in JSON format."
                </p>
                <p>
                <a class="button" href="/mhrice.json" download="mhrice.json">
                    <span class="icon">
                        <i class="fas fa-download"></i>
                    </span>
                    <span>"Download mhrice.json"</span>
                </a>
                </p>
                </section>
                <section>
                <h2 >"Build information"</h2>
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
                </main>
            </body>
        </html>
    );

    output
        .create_html("about.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_static(output: &impl Sink) -> Result<()> {
    output
        .create("mhrice.css")?
        .write_all(include_bytes!("static/mhrice.css"))?;

    output
        .create("mhrice.js")?
        .write_all(include_bytes!("static/mhrice.js"))?;

    output
        .create("favicon.png")?
        .write_all(include_bytes!("static/favicon.png"))?;
    output
        .create("error.html")?
        .write_all(include_bytes!("static/error.html"))?;
    Ok(())
}

pub fn gen_part_color_css(output: &impl Sink) -> Result<()> {
    let mut file = output.create("part_color.css")?;

    for (i, color) in PART_COLORS.iter().enumerate() {
        writeln!(file, ".mh-part-{} {{background-color: {}}}", i, color)?;
    }

    Ok(())
}

pub fn gen_website(pedia: &Pedia, pedia_ex: &PediaEx<'_>, output: &impl Sink) -> Result<()> {
    let mut toc = Toc::new();
    gen_quests(pedia, pedia_ex, output, &mut toc)?;
    gen_quest_list(&pedia_ex.quests, output)?;
    //gen_skills(pedia_ex, output, &mut toc)?;
    //gen_skill_list(&pedia_ex.skills, output)?;
    //gen_hyakuryu_skills(pedia_ex, output, &mut toc)?;
    //gen_hyakuryu_skill_list(&pedia_ex.hyakuryu_skills, output)?;
    //gen_armors(pedia_ex, output, &mut toc)?;
    //gen_armor_list(&pedia_ex.armors, output)?;
    gen_monsters(pedia, pedia_ex, output, &mut toc)?;
    gen_items(pedia, pedia_ex, output, &mut toc)?;
    gen_item_list(pedia_ex, output)?;
    //gen_weapons(pedia_ex, output, &mut toc)?;
    //gen_maps(pedia, pedia_ex, output, &mut toc)?;
    //gen_map_list(pedia, output)?;
    //gen_otomo_equips(pedia_ex, output, &mut toc)?;
    //gen_otomo_equip_list(pedia_ex, output)?;
    gen_about(output)?;
    gen_search(output)?;
    gen_static(output)?;
    gen_part_color_css(output)?;
    toc.finalize(&output.sub_sink("toc")?)?;
    Ok(())
}
