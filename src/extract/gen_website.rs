use super::gen_armor::*;
use super::gen_common::*;
use super::gen_dlc::*;
use super::gen_hyakuryu_skill::*;
use super::gen_item::*;
use super::gen_map::*;
use super::gen_misc::*;
use super::gen_monster::*;
use super::gen_otomo::*;
use super::gen_quest::*;
use super::gen_skill::*;
use super::gen_weapon::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::msg::*;
use crate::part_color::*;
use crate::rsz::*;
use anyhow::Result;
use chrono::prelude::*;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text, types::*};

pub struct WebsiteConfig {
    pub origin: Option<String>, // e.g. https://mhrice.info
}

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

pub fn head_common(
    hash_store: &HashStore,
    parent: &impl Sink,
) -> Vec<Box<dyn MetadataContent<String>>> {
    let main_css = format!("mhrice.css?h={}", hash_store.get(FileTag::MainCss));
    let main_js = format!("mhrice.js?h={}", hash_store.get(FileTag::MainJs));
    let fa = format!(
        "fontawesome/fontawesome.min.js?h={}",
        hash_store.get(FileTag::Fa)
    );
    let fa_brand = format!(
        "fontawesome/brands.js?h={}",
        hash_store.get(FileTag::FaBrand)
    );
    let fa_solid = format!(
        "fontawesome/solid.js?h={}",
        hash_store.get(FileTag::FaSolid)
    );
    let part_color = format!("part_color.css?h={}", hash_store.get(FileTag::PartColor));
    vec![
        html!(<meta charset="UTF-8" />),
        html!(<base href={parent.home_path().as_str()}/>),
        html!(<meta name="viewport" content="width=device-width, initial-scale=1" />),
        html!(<meta name="keywords" content="Monster Hunter,Monster Hunter Rise,MHR,MHRise,Database,Guide,Hitzone,HZV"/>),
        html!(<link rel="icon" type="image/png" href="favicon.png" />),
        html!(<link rel="stylesheet" href={main_css} />),
        html!(<link rel="stylesheet" href={part_color} />),
        html!(<link rel="stylesheet" href="resources/item_color.css" />),
        html!(<link rel="stylesheet" href="resources/rarity_color.css" />),
        html!(<script src={main_js}/>),
        html!(<script defer=true src={fa_brand}/>),
        html!(<script defer=true src={fa_solid}/>),
        html!(<script defer=true src={fa}/>),
        html!(<style id="mh-lang-style">".mh-lang:not(.lang-default) { display:none; }"</style>),
    ]
}

pub fn navbar() -> Box<nav<String>> {
    html!(<nav><div>
        <div class="navbar-brand">
            <a class="navbar-item" href="index.html">
                <img alt="Logo" src="favicon.png"/>
                <div id="mh-logo-text">"MHRice "</div>
            </a>

            <div class="navbar-item nav-search-item">
                <div class="control has-icons-left">
                <input id="nav-search" class="input" type="search" placeholder="Search"/>
                <span class="icon is-small is-left">
                    <i class="fas fa-magnifying-glass" />
                </span>
                </div>
            </div>

            <a id="navbarBurger" class="navbar-burger" data-target="navbarMenu">
                <span></span>
                <span></span>
                <span></span>
            </a>
        </div>

        <div id="navbarMenu" class="navbar-menu">
            <div class="navbar-start">
                <a class="navbar-item" href="monster.html">
                    "Monsters"
                </a>

                <a class="navbar-item navbar-folded" href="quest.html">
                    "Quests"
                </a>
                <div class="navbar-item has-dropdown is-hoverable navbar-expanded">
                <a class="navbar-link" href="quest.html">
                    "Quests"
                </a>
                <div class="navbar-dropdown">
                    <a class="navbar-item" href="quest.html">"Main quests"</a>
                    <a class="navbar-item" href="villager_request.html">"Villager requests"</a>
                </div>
                </div>

                <a class="navbar-item navbar-folded" href="skill.html">
                    "Skills"
                </a>
                <div class="navbar-item has-dropdown is-hoverable navbar-expanded">
                <a class="navbar-link" href="skill.html">
                    "Skills"
                </a>
                <div class="navbar-dropdown">
                    <a class="navbar-item" href="skill.html">
                        "Armor skills"
                    </a>
                    <a class="navbar-item" href="hyakuryu_skill.html">
                        "Rampage skills"
                    </a>
                </div>
                </div>

                <a class="navbar-item" href="armor.html">
                    "Armors"
                </a>

                <a class="navbar-item navbar-folded" href="weapon.html">
                    "Weapons"
                </a>
                <div class="navbar-item has-dropdown is-hoverable navbar-expanded">
                <a class="navbar-link" href="weapon.html">
                    "Weapons"
                </a>
                <div class="navbar-dropdown">
                    <a class="navbar-item" href="weapon/great_sword.html">"Great sword"</a>
                    <a class="navbar-item" href="weapon/long_sword.html">"Long sword"</a>
                    <a class="navbar-item" href="weapon/short_sword.html">"Sword & shield"</a>
                    <a class="navbar-item" href="weapon/dual_blades.html">"Dual blades"</a>
                    <a class="navbar-item" href="weapon/hammer.html">"Hammer"</a>
                    <a class="navbar-item" href="weapon/horn.html">"Hunting horn"</a>
                    <a class="navbar-item" href="weapon/lance.html">"Lance"</a>
                    <a class="navbar-item" href="weapon/gun_lance.html">"Gunlance"</a>
                    <a class="navbar-item" href="weapon/slash_axe.html">"Switch axe"</a>
                    <a class="navbar-item" href="weapon/charge_axe.html">"Charge blade"</a>
                    <a class="navbar-item" href="weapon/insect_glaive.html">"Insect glaive"</a>
                    <a class="navbar-item" href="weapon/light_bowgun.html">"Light bowgun"</a>
                    <a class="navbar-item" href="weapon/heavy_bowgun.html">"Heavy bowgun"</a>
                    <a class="navbar-item" href="weapon/bow.html">"Bow"</a>
                </div>
                </div>

                <a class="navbar-item navbar-folded" href="airou.html">
                    "Buddy"
                </a>
                <div class="navbar-item has-dropdown is-hoverable navbar-expanded">
                <a class="navbar-link" href="airou.html">
                    "Buddy"
                </a>
                <div class="navbar-dropdown">
                    <a class="navbar-item" href="airou.html">"Palico equipment"</a>
                    <a class="navbar-item" href="dog.html">"Palamute equipment"</a>
                </div>
                </div>

                <a class="navbar-item" href="map.html">
                    "Maps"
                </a>

                <a class="navbar-item" href="item.html">
                   "Items"
                </a>

                <a class="navbar-item navbar-folded" href="misc.html">
                    "Misc."
                </a>
                <div class="navbar-item has-dropdown is-hoverable navbar-expanded">
                <a class="navbar-link" href="misc.html">
                    "Misc."
                </a>
                <div class="navbar-dropdown">
                    <a class="navbar-item" href="misc/petalace.html">"Petalace"</a>
                    <a class="navbar-item" href="misc/market.html">"Market"</a>
                    <a class="navbar-item" href="misc/lab.html">"Anomaly research lab"</a>
                    <a class="navbar-item" href="misc/mix.html">"Item crafting"</a>
                    <a class="navbar-item" href="misc/bbq.html">"Motley mix"</a>
                    <a class="navbar-item" href="misc/argosy.html">"Argosy"</a>
                    <a class="navbar-item" href="misc/meowcenaries.html">"Meowcenaries"</a>
                    <a class="navbar-item" href="misc/scraps.html">"Trade for scraps"</a>
                    <a class="navbar-item" href="dlc.html">"DLC"</a>
                    <a class="navbar-item" href="misc/award.html">"Awards"</a>
                    <a class="navbar-item" href="misc/achievement.html">"Guild card titles"</a>
                </div>
                </div>
            </div>
        </div>
    </div></nav>)
}

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

fn parse_msg(content: &str) -> (Seq, bool) {
    let mut msg = content;
    let mut has_warning = false;

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
                eprintln!("Parse error: {content}");
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
                eprintln!("Parse error: {content}");
                break;
            };
            if stack_tag.tag != tag {
                has_warning = true;
                eprintln!("Parse error: {content}");
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

    (root, has_warning)
}

pub fn translate_msg<'r, RefF>(
    content: &str,
    language_i: usize,
    reference: RefF,
) -> (Box<span<String>>, bool)
where
    RefF: Fn(&str) -> Option<&'r MsgEntry> + Clone,
{
    let (root, has_warning) = parse_msg(content);

    fn translate_rec<'r, RefF>(
        node: &Node<'_>,
        language_i: usize,
        reference: RefF,
    ) -> Box<dyn PhrasingContent<String>>
    where
        RefF: Fn(&str) -> Option<&'r MsgEntry> + Clone,
    {
        match node {
            Node::Raw(s) => Box::new(TextNode::<String>::new(*s)),
            Node::Tagged(t) => {
                let inner = t
                    .seq
                    .nodes
                    .iter()
                    .map(|n| translate_rec(n, language_i, reference.clone()));
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
                        let style = format!("color: {color};");
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
                    "REF" => {
                        if let Some(entry) = reference(t.arg) {
                            let (translated, _inner_has_warning) = translate_msg(
                                &entry.content[language_i],
                                language_i,
                                reference.clone(),
                            );
                            //has_warning |= _inner_has_warning; // TODO: ehh
                            translated
                        } else {
                            html!(<span class="mh-msg-place-holder">{text!("{{{} {}}}", t.tag, t.arg)}</span>)
                        }
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

    let result = html!(<span> {root.nodes.iter().map(|n|translate_rec(n, language_i, reference.clone()))} </span>);
    (result, has_warning)
}

pub fn translate_msg_plain(content: &str) -> String {
    let (root, _) = parse_msg(content);
    fn translate_rec(result: &mut String, node: &Node<'_>) {
        match node {
            Node::Raw(s) => *result += *s,
            Node::Tagged(t) => {
                for inner in &t.seq.nodes {
                    translate_rec(result, inner)
                }
            }
        }
    }
    let mut result = String::new();
    for node in &root.nodes {
        translate_rec(&mut result, node)
    }
    result
}

pub fn gen_multi_lang(msg: &MsgEntry) -> Box<span<String>> {
    gen_multi_lang_with_ref(msg, |_| None)
}

pub fn gen_multi_lang_with_ref<'r, RefF>(msg: &MsgEntry, reference: RefF) -> Box<span<String>>
where
    RefF: Fn(&str) -> Option<&'r MsgEntry> + Clone,
{
    html!(<span> {
        (0..32).filter_map(|i|{
            let class_string = if i == 1 {
                "mh-lang lang-default"
            } else {
                "mh-lang"
            };
            let (_, language_code) = LANGUAGE_MAP[i]?;
            let language_code: LanguageTag = language_code.parse().unwrap();
            let (msg, warning) = translate_msg(&msg.content[i], i, reference.clone());
            let warning = warning.then(||html!(<span class="icon has-text-warning">
                <i class="fas fa-triangle-exclamation" title="There is a parsing error"/>
            </span>));
            Some(html! (<span class={class_string} lang={language_code} >
                {msg} {warning}
            </span>))
        })
    } </span>)
}

#[allow(clippy::vec_box)]
pub fn title_multi_lang(msg: &MsgEntry) -> Vec<Box<meta<String>>> {
    LANGUAGE_MAP
        .iter()
        .zip(&msg.content)
        .filter_map(|(language, entry)| {
            let &(_, language_code) = if let Some(language) = language {
                language
            } else {
                return None;
            };
            let title = translate_msg_plain(entry);
            let itemprop = format!("title-{language_code}");
            Some(html!(<meta itemprop={itemprop} content={title}/>))
        })
        .collect()
}

pub fn gen_colored_icon<'a>(
    color: i32,
    icon: &str,
    addons: impl IntoIterator<Item = &'a str> + 'a,
    is_small: bool,
) -> Box<div<String>> {
    let color_class = format!("mh-item-color-{color}");
    gen_colored_icon_inner(&color_class, icon, addons, is_small)
}

pub fn gen_rared_icon<'a>(
    rarity: RareTypes,
    icon: &str,
    addons: impl IntoIterator<Item = &'a str> + 'a,
    is_small: bool,
) -> Box<div<String>> {
    let color_class = format!("mh-rarity-color-{}", rarity.0);
    gen_colored_icon_inner(&color_class, icon, addons, is_small)
}

pub fn gen_colored_icon_inner<'a>(
    color_class: &str,
    icon: &str,
    addons: impl IntoIterator<Item = &'a str> + 'a,
    is_small: bool,
) -> Box<div<String>> {
    let image_r_base = format!("url('{icon}.r.png')");
    let image_a_base = format!("url('{icon}.a.png')");
    let image_r = format!("mask-image: {image_r_base}; -webkit-mask-image: {image_r_base};");
    let image_a = format!("mask-image: {image_a_base}; -webkit-mask-image: {image_a_base};");
    let class = if is_small {
        "mh-colored-icon mh-small-colored-icon"
    } else {
        "mh-colored-icon"
    };
    let (color_class, color_style) = if color_class.starts_with('#') {
        ("", format!("background-color:{color_class};"))
    } else {
        (color_class, "".to_owned())
    };
    html!(<div class={class}>
        <div style={(image_r + &color_style).as_str()} class={color_class}/>
        <div style={image_a.as_str()}/>
        <div>{ addons.into_iter().map(|addon| html!(
            <div class=addon/>
        )) }</div>
    </div>)
}

pub fn gen_search(hash_store: &HashStore, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("MHRice - Monster Hunter Rise Database")}</title>
                { head_common(hash_store, output) }
                <meta name="description" content="Monster Hunter Rise Database" />
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Search"</h1></header>
                <div class="control has-icons-left">
                    <input class="input is-large" type="search" placeholder="Nargacuga" id="mh-search"/>
                    <span class="icon is-large is-left">
                        <i class="fas fa-magnifying-glass" />
                    </span>
                </div>
                <ul id="mh-search-result">
                </ul>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("index.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_about(hash_store: &HashStore, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common(hash_store, output) }
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
                <a class="button" href="mhrice.json" download="mhrice.json">
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
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("about.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_static(hash_store: &mut HashStore, output: &impl Sink) -> Result<()> {
    output
        .create_with_hash("mhrice.css", FileTag::MainCss, hash_store)?
        .write_all(include_bytes!("static/mhrice.css"))?;

    output
        .create_with_hash("mhrice.js", FileTag::MainJs, hash_store)?
        .write_all(include_bytes!("static/mhrice.js"))?;

    output
        .create_with_hash("masonry.pkgd.min.js", FileTag::Masonry, hash_store)?
        .write_all(include_bytes!("static/masonry.pkgd.min.js"))?;

    output
        .create("favicon.png")?
        .write_all(include_bytes!("static/favicon.png"))?;
    output
        .create("error.html")?
        .write_all(include_bytes!("static/error.html"))?;
    let fontawesome = output.sub_sink("fontawesome")?;
    fontawesome
        .create_with_hash("brands.js", FileTag::FaBrand, hash_store)?
        .write_all(include_bytes!("static/fontawesome/brands.js"))?;
    fontawesome
        .create_with_hash("solid.js", FileTag::FaSolid, hash_store)?
        .write_all(include_bytes!("static/fontawesome/solid.js"))?;
    fontawesome
        .create_with_hash("fontawesome.min.js", FileTag::Fa, hash_store)?
        .write_all(include_bytes!("static/fontawesome/fontawesome.min.js"))?;
    Ok(())
}

pub fn gen_part_color_css(hash_store: &mut HashStore, output: &impl Sink) -> Result<()> {
    let mut file = output.create_with_hash("part_color.css", FileTag::PartColor, hash_store)?;

    for (i, color) in PART_COLORS.iter().enumerate() {
        writeln!(file, ".mh-part-{i} {{background-color: {color}}}")?;
    }

    Ok(())
}

pub fn gen_website(
    hash_store: &mut HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
) -> Result<()> {
    let mut toc = Toc::new();
    gen_static(hash_store, output)?;
    gen_part_color_css(hash_store, output)?;
    gen_quests(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_quest_list(hash_store, &pedia_ex.quests, output)?;
    gen_npc_missions(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_npc_mission_list(hash_store, pedia_ex, output)?;
    gen_skills(hash_store, pedia_ex, config, output, &mut toc)?;
    gen_skill_list(hash_store, &pedia_ex.skills, output)?;
    gen_hyakuryu_skills(hash_store, pedia_ex, config, output, &mut toc)?;
    gen_hyakuryu_skill_list(hash_store, &pedia_ex.hyakuryu_skills, output)?;
    gen_armors(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_armor_list(hash_store, &pedia_ex.armors, output)?;
    gen_monsters(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_items(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_item_list(hash_store, pedia_ex, output)?;
    gen_weapons(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_maps(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    gen_map_list(hash_store, pedia, output)?;
    gen_otomo_equips(hash_store, pedia_ex, config, output, &mut toc)?;
    gen_otomo_equip_list(hash_store, pedia_ex, output)?;
    gen_about(hash_store, output)?;
    gen_search(hash_store, output)?;
    gen_misc(hash_store, pedia, pedia_ex, output, &mut toc)?;
    gen_dlc_list(hash_store, pedia_ex, output)?;
    gen_dlcs(hash_store, pedia, pedia_ex, config, output, &mut toc)?;
    toc.finalize(&output.sub_sink("tocv2")?)?;
    Ok(())
}
