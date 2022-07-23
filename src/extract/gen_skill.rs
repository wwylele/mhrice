use super::gen_armor::*;
use super::gen_item::*;
use super::gen_website::*;
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

pub fn gen_skill_list(skills: &BTreeMap<PlEquipSkillId, Skill>, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Skills - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Skill"</h1></header>
                <ul class="mh-item-list">
                {
                    skills.iter().map(|(&id, skill)|{
                        html!(<li>
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

pub fn gen_deco_label(deco: &Deco) -> Box<div<String>> {
    let icon_id = if deco.data.decoration_lv == 4 {
        200
    } else {
        63 + deco.data.decoration_lv
    };
    let icon = format!("/resources/item/{:03}", icon_id);
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
        Some(html!(<section> <div> <h2 >"Available on armors"</h2>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div> </section>))
    } else {
        None
    }
}

pub fn gen_skill(
    id: PlEquipSkillId,
    skill: &Skill,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    toc_sink.add(skill.name);
    let deco = (!skill.decos.is_empty()).then(|| {
        html!(<section>
        <h2 >"Decoration"</h2>
        <div class="mh-table"><table>
            <thead><tr>
                <th>"Name"</th>
                <th>"Skill level"</th>
                <th>"Cost"</th>
                <th>"Material"</th>
            </tr></thead>
            <tbody>
            {
                skill.decos.iter().map(|deco|{html!(
                    <tr>
                        <td>{gen_deco_label(deco)}</td>
                        <td>{text!("{}", deco.data.skill_lv_list[0])}</td>
                        <td>{text!("{}", deco.data.base_price)}</td>
                        { gen_materials(pedia_ex, &deco.product.item_id_list,
                            &deco.product.item_num_list, deco.product.item_flag) }
                    </tr>
                )})
            }
            </tbody>
        </table></div>
        </section>)
    });

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Skill - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                    </div>
                    <h1> {gen_multi_lang(skill.name)} </h1>
                </header>
                <section>
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

                { deco }

                { gen_skill_source_gear(id, pedia_ex) }

                </main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_skills(pedia_ex: &PediaEx, output: &impl Sink, toc: &mut Toc) -> Result<()> {
    let skill_path = output.sub_sink("skill")?;
    for (&id, skill) in &pedia_ex.skills {
        let (output, toc_sink) = skill_path.create_html_with_toc(&skill_page(id), toc)?;
        gen_skill(id, skill, pedia_ex, output, toc_sink)?
    }
    Ok(())
}
