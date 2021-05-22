use super::gen_website::*;
use super::pedia::*;
use crate::rsz::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

pub fn skill_page(id: PlEquipSkillId) -> String {
    match id {
        PlEquipSkillId::None => "none.html".to_string(),
        PlEquipSkillId::Skill(id) => format!("{:03}.html", id),
    }
}

pub fn gen_skill_list(skills: &BTreeMap<PlEquipSkillId, Skill>, root: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Skills - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"Skill"</h1>
                <ul class="mh-list-skill">
                {
                    skills.iter().map(|(&id, skill)|{
                        html!(<li class="mh-list-skill">
                            <a href={format!("/skill/{}", skill_page(id))} class="mh-icon-text">
                            {gen_colored_icon(skill.icon_color, "/resources/skill", &[])}
                            <span>{gen_multi_lang(&skill.name)}</span>
                            </a>
                        </li>)
                    })
                }
                </ul>
                </div></div></main>
            </body>
        </html>
    );
    let quests_path = root.join("skill.html");
    write(&quests_path, doc.to_string())?;

    Ok(())
}

pub fn gen_skill(skill: &Skill, path: &Path) -> Result<()> {
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
                    {gen_multi_lang(&skill.name)}
                </h1>
                <div>{gen_multi_lang(&skill.explain)}</div>
                <ul>{
                    skill.levels.iter().enumerate().map(|(level, detail)| {
                        html!(<li>
                            <span>{text!("Level {}: ", level + 1)}</span>
                            <span>{gen_multi_lang(detail)}</span>
                        </li>)
                    })
                }</ul>
                </div></div></main>
            </body>
        </html>
    );

    write(&path, doc.to_string())?;

    Ok(())
}

pub fn gen_skills(skills: &BTreeMap<PlEquipSkillId, Skill>, root: &Path) -> Result<()> {
    let skill_path = root.join("skill");
    create_dir(&skill_path)?;
    for (&id, skill) in skills {
        let path = skill_path.join(skill_page(id));
        gen_skill(skill, &path)?
    }
    Ok(())
}
