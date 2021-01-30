use super::pedia::*;
use anyhow::*;
use chrono::prelude::*;
use std::fs::{create_dir, remove_dir_all, write};
use std::path::*;
use typed_html::{dom::*, elements::*, html, text};

fn head_common() -> Vec<Box<dyn MetadataContent<String>>> {
    vec![
        html!(<meta charset="UTF-8" />),
        html!(<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css" />),
        html!(<script src="https://kit.fontawesome.com/ceb13a2ba1.js" crossorigin="anonymous" />),
    ]
}

fn navbar() -> Box<dyn FlowContent<String>> {
    html!(<div>
        <nav class="navbar is-primary" role="navigation"> <div class="container">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    "MHRice 🍚"
                </a>

                <a class="navbar-item" href="/monster.html">
                    "Monsters"
                </a>

                <a class="navbar-item" href="/about.html">
                    "About"
                </a>

                <a class="navbar-burger" data-target="navbarBasicExample">
                    <span></span>
                    <span></span>
                    <span></span>
                </a>
            </div>
        </div> </nav>
    </div>)
}

fn gen_monster(monster: Monster, folder: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Monster {:03} - MHRice", monster.id)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">{text!("Monster {:03}", monster.id)}</h1>
                <section class="section">
                <h2 class="subtitle">"Hitzone data"</h2>
                <table>
                    <thead>
                    <tr>
                        <th>"Part"</th>
                        <th>"Phase"</th>
                        <th>"Slash"</th>
                        <th>"Impact"</th>
                        <th>"Shot"</th>
                        <th>"Fire"</th>
                        <th>"Water"</th>
                        <th>"Thunder"</th>
                        <th>"Ice"</th>
                        <th>"Dragon"</th>
                        <th>"Dizzy"</th>
                    </tr>
                    </thead>
                    <tbody>{
                        monster.meat_data.meat_containers.into_iter().enumerate().flat_map(|(part, container)| {
                            std::iter::repeat(part).zip(container.meat_group_infos.into_iter().enumerate())
                                .map(|(part, (phase, group_info))| {
                                    html!(<tr>
                                        <td>{text!("{}", part)}</td>
                                        <td>{text!("{}", phase)}</td>
                                        <td>{text!("{}", group_info.slash)}</td>
                                        <td>{text!("{}", group_info.impact)}</td>
                                        <td>{text!("{}", group_info.shot)}</td>
                                        <td>{text!("{}", group_info.fire)}</td>
                                        <td>{text!("{}", group_info.water)}</td>
                                        <td>{text!("{}", group_info.thunder)}</td>
                                        <td>{text!("{}", group_info.ice)}</td>
                                        <td>{text!("{}", group_info.dragon)}</td>
                                        <td>{text!("{}", group_info.dizzy)}</td>
                                    </tr>)
                                })
                        })
                    }</tbody>
                </table>
                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    let file = PathBuf::from(folder).join(format!("{:03}.html", monster.id));
    write(file, doc.to_string())?;
    Ok(())
}

pub fn gen_monsters(monsters: Vec<Monster>, root: &Path) -> Result<()> {
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
                <ul>{
                    monsters.iter().map(|monster| {
                        html!{<li>
                            <a href={format!("/monster/{:03}.html", monster.id)}>{
                                text!("Monster {:03}", monster.id)
                            }</a>
                        </li>}
                    })
                }</ul>
                </div> </div> </main>
            </body>
        </html>
    );

    write(&monsters_path, doc.to_string())?;

    let monster_path = root.join("monster");
    create_dir(&monster_path)?;
    for monster in monsters {
        gen_monster(monster, &monster_path)?;
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

pub fn gen_website(pedia: Pedia, output: &str) -> Result<()> {
    let root = PathBuf::from(output);
    if root.exists() {
        remove_dir_all(&root)?;
    }
    create_dir(&root)?;

    gen_monsters(pedia.monsters, &root)?;
    gen_about(&root)?;
    Ok(())
}
