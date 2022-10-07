use super::gen_armor::*;
use super::gen_common::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn skill_page(id: PlEquipSkillId) -> String {
    format!("{}.html", id.to_msg_tag())
}

pub fn gen_skill_list(
    hash_store: &HashStore,
    skills: &BTreeMap<PlEquipSkillId, Skill>,
    output: &impl Sink,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Armor skills - MHRice")}</title>
                { head_common(hash_store) }
                <style id="mh-skill-list-style">""</style>
            </head>
            <body>
                { navbar() }
                { right_aside() }
                <main>
                <header><h1>"Armor skills"</h1></header>

                <div>
                    <a href="/hyakuryu_skill.html"><span class="icon-text">
                    <span class="icon">
                    <i class="fas fa-arrow-right"></i>
                    </span>
                    <span>"go to rampage skill"</span>
                    </span></a>
                </div>

                <div class="mh-filters"><ul>
                    <li id="mh-skill-filter-button-all" class="is-active mh-skill-filter-button">
                        <a>"All skills"</a></li>
                    <li id="mh-skill-filter-button-deco1" class="mh-skill-filter-button">
                        <a>"Lv1 deco"</a></li>
                    <li id="mh-skill-filter-button-deco2" class="mh-skill-filter-button">
                        <a>"Lv2 deco"</a></li>
                    <li id="mh-skill-filter-button-deco3" class="mh-skill-filter-button">
                        <a>"Lv3 deco"</a></li>
                    <li id="mh-skill-filter-button-deco4" class="mh-skill-filter-button">
                        <a>"Lv4 deco"</a></li>
                    <li id="mh-skill-filter-button-cb3" class="mh-skill-filter-button">
                        <a><span class="tag mh-cb-lv3">"Pt3"</span>"qurious"</a></li>
                    <li id="mh-skill-filter-button-cb6" class="mh-skill-filter-button">
                        <a><span class="tag mh-cb-lv6">"Pt6"</span>"qurious"</a></li>
                    <li id="mh-skill-filter-button-cb9" class="mh-skill-filter-button">
                        <a><span class="tag mh-cb-lv9">"Pt9"</span>"qurious"</a></li>
                    <li id="mh-skill-filter-button-cb12" class="mh-skill-filter-button">
                        <a><span class="tag mh-cb-lv12">"Pt12"</span>"qurious"</a></li>
                    <li id="mh-skill-filter-button-cb15" class="mh-skill-filter-button">
                        <a><span class="tag mh-cb-lv15">"Pt15"</span>"qurious"</a></li>
                </ul></div>

                <ul class="mh-item-list">
                {
                    skills.iter().map(|(&id, skill)|{
                        let mut filter_tags = vec![];
                        for deco in &skill.decos {
                            filter_tags.push(format!("deco{}", deco.data.decoration_lv));
                        }
                        if let Some(cost) = skill.custom_buildup_cost {
                            filter_tags.push(format!("cb{cost}"));
                        }
                        let filter = filter_tags.join(" ");
                        html!(<li data-filter={filter} class="mh-skill-filter-item">
                            <a href={format!("/skill/{}", skill_page(id))} class="mh-icon-text">
                            {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                            <span>{gen_multi_lang(skill.name)}</span>
                            </a>
                        </li>)
                    })
                }
                </ul>
                </main>
            </body>
        </html>
    );

    output
        .create_html("skill.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn deco_icon_path(lv: i32) -> String {
    let icon_id = if lv == 4 { 200 } else { 63 + lv };
    format!("/resources/item/{:03}", icon_id)
}

pub fn gen_deco_label(deco: &Deco) -> Box<div<String>> {
    let icon = deco_icon_path(deco.data.decoration_lv);
    html!(<div class="mh-icon-text">
        { gen_colored_icon(deco.data.icon_color, &icon, &[]) }
        <span>{gen_multi_lang(deco.name)}</span>
    </div>)
}

fn gen_skill_source_gear(id: PlEquipSkillId, pedia_ex: &PediaEx) -> Option<Box<section<String>>> {
    let mut htmls = vec![];

    for series in &pedia_ex.armors {
        for piece in series.pieces.iter().flatten() {
            if piece.data.skill_list.contains(&id) {
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
            html!(<section id="s-source"> <div> <h2 >"Available on armors"</h2>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div> </section>),
        )
    } else {
        None
    }
}

pub fn gen_skill(
    hash_store: &HashStore,
    id: PlEquipSkillId,
    skill: &Skill,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    toc_sink.add(skill.name);

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
                <pre>{gen_multi_lang(skill.explain)}</pre>
                <ul>{
                    skill.levels.iter().enumerate().map(|(level, detail)| {
                        html!(<li>
                            <span>{text!("Level {}: ", level + 1)}</span>
                            <span>{gen_multi_lang(detail)}</span>
                        </li>)
                    })
                }</ul>
            </section>
        ),
    });

    if !skill.decos.is_empty() {
        sections.push(Section {
            title: "Decoration".to_owned(),
            content: html!(
                <section id="s-decoration">
                <h2 >"Decoration"</h2>
                <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Name"</th>
                        <th>"Skill level"</th>
                        <th>"Unlock at"</th>
                        <th>"Key Monster"</th>
                        <th>"Cost"</th>
                        <th>"Material"</th>
                    </tr></thead>
                    <tbody>
                    {
                        skill.decos.iter().map(|deco|{html!(
                            <tr>
                                <td>{gen_deco_label(deco)}</td>
                                <td>{text!("{}", deco.data.skill_lv_list[0])}</td>
                                <td>{gen_progress(deco.product.progress_flag, pedia_ex)}</td>
                                <td>{(deco.product.enemy_flag != EmTypes::Em(0)).then(
                                    ||gen_monster_tag(pedia_ex, deco.product.enemy_flag, false, false, false)
                                )}</td>
                                <td>{text!("{}z", deco.data.base_price)}</td>
                                { gen_materials(pedia_ex, &deco.product.item_id_list,
                                    &deco.product.item_num_list, deco.product.item_flag) }
                            </tr>
                        )})
                    }
                    </tbody>
                </table></div>
                </section>
            ),
        });
    }

    if let Some(cost) = skill.custom_buildup_cost {
        let class = format!("tag mh-cb-lv{}", cost);
        sections.push(Section {
            title: "Qurious crafting".to_owned(),
            content: html!(
                <section id="s-qurious">
                <h2>"Qurious crafting"</h2>
                <p><span class={class.as_str()}>
                    {text!("Pt{} skill", cost)}
                </span></p>
                </section>
            ),
        });
    }

    if let Some(source) = gen_skill_source_gear(id, pedia_ex) {
        sections.push(Section {
            title: "Available on armors".to_owned(),
            content: source,
        });
    }

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Skill - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                { right_aside() }
                { gen_menu(&sections) }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                    </div>
                    <h1> {gen_multi_lang(skill.name)} </h1>
                </header>

                { sections.into_iter().map(|s|s.content) }

                </main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_skills(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let skill_path = output.sub_sink("skill")?;
    for (&id, skill) in &pedia_ex.skills {
        let (output, toc_sink) = skill_path.create_html_with_toc(&skill_page(id), toc)?;
        gen_skill(hash_store, id, skill, pedia_ex, output, toc_sink)?
    }
    Ok(())
}
