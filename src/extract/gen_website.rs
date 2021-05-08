use super::gen_armor::*;
use super::gen_monster::*;
use super::gen_quest::*;
use super::gen_skill::*;
use super::pedia::*;
use crate::msg::*;
use crate::part_color::*;
use crate::rsz::*;
use anyhow::*;
use chrono::prelude::*;
use std::collections::HashMap;
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
    None,
];

pub fn head_common() -> Vec<Box<dyn MetadataContent<String>>> {
    vec![
        html!(<meta charset="UTF-8" />),
        html!(<link rel="icon" type="image/png" href="/favicon.png" />),
        html!(<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css" />),
        html!(<link rel="stylesheet" href="/mhrice.css" />),
        html!(<link rel="stylesheet" href="/part_color.css" />),
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

pub fn gen_multi_lang(msg: &MsgEntry) -> Box<span<String>> {
    html!(<span> {
        (0..32).map(|i|{
            let class_string = format!("mh-lang-{}", i);
            let c: SpacedSet<Class> = class_string.as_str().try_into().unwrap();
            html! (<span class=c>
                {text!("{}", msg.content[i])}
            </span>)
        })
    } </span>)
}

pub fn gen_monsters(
    pedia: &Pedia,
    quests: &[Quest],
    sizes: &HashMap<u32, &SizeInfo>,
    size_dists: &HashMap<i32, &[ScaleAndRateData]>,
    root: &Path,
) -> Result<()> {
    let meat_names = prepare_meat_names(pedia)?;

    let monsters_path = root.join("monster.html");

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monsters - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"Monsters"</h1>
                <section class="section">
                <h2 class="subtitle">"Large monsters"</h2>
                <ul class="mh-list-monster">{
                    pedia.monsters.iter().filter_map(|monster| {
                        let icon_path = format!("/resources/em{0:03}_{1:02}_icon.png", monster.id, monster.sub_id);
                        let name_name = format!("EnemyIndex{:03}",
                            monster.boss_init_set_data.as_ref()?.enemy_type);
                        let name_entry = pedia.monster_names.get_entry(&name_name)?;
                        Some(html!{<li class="mh-list-monster">
                            <a href={format!("/monster/{:03}_{:02}.html", monster.id, monster.sub_id)}>
                                <img class="mh-list-monster-icon" src=icon_path />
                                <div>{gen_multi_lang(name_entry)}</div>
                            </a>
                        </li>})
                    }).collect::<Vec<_>>()
                }</ul>
                </section>
                <section class="section">
                <h2 class="subtitle">"Small monsters"</h2>
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
                </div> </div> </main>
            </body>
        </html>
    );

    write(&monsters_path, doc.to_string())?;

    let monster_path = root.join("monster");
    create_dir(&monster_path)?;
    for monster in &pedia.monsters {
        gen_monster(
            true,
            monster,
            &pedia.monster_aliases,
            &pedia.condition_preset,
            sizes,
            size_dists,
            quests,
            pedia,
            &meat_names,
            &monster_path,
        )?;
    }

    let monster_path = root.join("small-monster");
    create_dir(&monster_path)?;
    for monster in &pedia.small_monsters {
        gen_monster(
            false,
            monster,
            &pedia.monster_aliases,
            &pedia.condition_preset,
            sizes,
            size_dists,
            quests,
            pedia,
            &meat_names,
            &monster_path,
        )?;
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
                <h2 class="subtitle">"Build information"</h2>
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

pub fn gen_website(pedia: Pedia, output: &str) -> Result<()> {
    let root = PathBuf::from(output);
    if root.exists() {
        remove_dir_all(&root)?;
    }
    create_dir(&root)?;

    let quests = prepare_quests(&pedia)?;
    let skills = prepare_skills(&pedia)?;
    let armors = prepare_armors(&pedia)?;
    let sizes = prepare_size_map(&pedia.size_list)?;
    let size_dists = prepare_size_dist_map(&pedia.random_scale)?;

    gen_quests(&quests, &sizes, &size_dists, &pedia, &root)?;
    gen_skills(&skills, &root)?;
    gen_skill_list(&skills, &root)?;
    gen_armors(&armors, &skills, &root)?;
    gen_armor_list(&armors, &root)?;
    gen_monsters(&pedia, &quests, &sizes, &size_dists, &root)?;
    gen_quest_list(&quests, &root)?;
    gen_about(&root)?;
    gen_static(&root)?;
    gen_part_color_css(&root)?;
    Ok(())
}
