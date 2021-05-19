use super::gen_website::*;
use super::pedia::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

pub fn gen_skill_list(skills: &BTreeMap<u8, Skill>, root: &Path) -> Result<()> {
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
                    skills.iter().map(|(id, skill)|{
                        html!(<li class="mh-list-skill">
                            <a href={format!("/skill/{:03}.html", id)} class="mh-icon-text">
                            {gen_colored_icon(skill.icon_color, "/resources/skill")}
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

pub fn gen_skill(id: u8, skill: &Skill, path: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Skill {:03} - MHRice", id)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    {gen_colored_icon(skill.icon_color, "/resources/skill")}
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

pub fn gen_skills(skills: &BTreeMap<u8, Skill>, root: &Path) -> Result<()> {
    let skill_path = root.join("skill");
    create_dir(&skill_path)?;
    for (&id, skill) in skills {
        let path = skill_path.join(format!("{:03}.html", id));
        gen_skill(id, skill, &path)?
    }
    Ok(())
}
