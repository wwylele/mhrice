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
    match id {
        PlEquipSkillId::None => "none.html".to_string(),
        PlEquipSkillId::Skill(id) => format!("{:03}.html", id),
    }
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
                <main> <div class="container">
                <h1 class="title">"Skill"</h1>
                <ul class="mh-list-skill">
                {
                    skills.iter().map(|(&id, skill)|{
                        html!(<li class="mh-list-skill">
                            <a href={format!("/skill/{}", skill_page(id))} class="mh-icon-text">
                            {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                            <span>{gen_multi_lang(skill.name)}</span>
                            </a>
                        </li>)
                    })
                }
                </ul>
                </div></main>
            </body>
        </html>
    );

    output
        .create_html("skill.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_deco_label(deco: &Deco) -> Box<div<String>> {
    let icon = format!("/resources/item/{:03}", 63 + deco.data.decoration_lv);
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
                htmls.push(html!(<li class="mh-list-item-in-out">
                    <a href={format!("/armor/{:03}.html", series.series.armor_series.0)}>
                        { gen_armor_label(Some(piece)) }
                    </a>
                </li>))
            }
        }
    }

    if !htmls.is_empty() {
        Some(
            html!(<section class="section"> <div> <h2 class="title">"Available on armors"</h2>
            <ul class="mh-list-item-in-out">{
                htmls
            }</ul> </div> </section>),
        )
    } else {
        None
    }
}

pub fn gen_skill(
    id: PlEquipSkillId,
    skill: &Skill,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let deco = skill.deco.as_ref().map(|deco| {
        html!(<section class="section">
        <h2 class="title">"Decoration"</h2>
        <table>
            <thead><tr>
                <th>"Name"</th>
                <th>"Material"</th>
            </tr></thead>
            <tbody>
            <tr>
                <td>{gen_deco_label(deco)}</td>
                { gen_materials(pedia_ex, &deco.product.item_id_list,
                    &deco.product.item_num_list, deco.product.item_flag) }
            </tr>
            </tbody>
        </table>
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
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                </div>
                <h1 class="title">
                    {gen_multi_lang(skill.name)}
                </h1>
                <div>{gen_multi_lang(skill.explain)}</div>
                <ul>{
                    skill.levels.iter().enumerate().map(|(level, detail)| {
                        html!(<li>
                            <span>{text!("Level {}: ", level + 1)}</span>
                            <span>{gen_multi_lang(detail)}</span>
                        </li>)
                    })
                }</ul>

                { deco }

                { gen_skill_source_gear(id, pedia_ex) }

                </div></div></main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_skills(pedia_ex: &PediaEx, output: &impl Sink) -> Result<()> {
    let skill_path = output.sub_sink("skill")?;
    for (&id, skill) in &pedia_ex.skills {
        let output = skill_path.create_html(&skill_page(id))?;
        gen_skill(id, skill, pedia_ex, output)?
    }
    Ok(())
}
