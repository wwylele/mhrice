use super::gen_website::*;
use super::pedia::*;
use crate::msg::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

pub struct Skill {
    pub name: MsgEntry,
    pub explain: MsgEntry,
    pub levels: Vec<MsgEntry>,
}

pub fn prepare_skills(pedia: &Pedia) -> Result<BTreeMap<u8, Skill>> {
    let mut result = BTreeMap::new();

    let mut name_msg: HashMap<String, MsgEntry> = pedia
        .player_skill_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut explain_msg: HashMap<String, MsgEntry> = pedia
        .player_skill_explain_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut detail_msg: HashMap<String, MsgEntry> = pedia
        .player_skill_detail_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    for skill in &pedia.equip_skill.param {
        if skill.id == 0 {
            continue;
        }
        let id = skill.id - 1;
        if result.contains_key(&id) {
            bail!("Multiple definition for skill {}", id);
        }

        let name = name_msg
            .remove(&format!("PlayerSkill_{:03}_Name", id))
            .with_context(|| format!("Name for skill {}", id))?;

        let explain = explain_msg
            .remove(&format!("PlayerSkill_{:03}_Explain", id))
            .with_context(|| format!("Explain for skill {}", id))?;

        let levels = (0..(skill.max_level + 1))
            .map(|level| {
                detail_msg
                    .remove(&format!("PlayerSkill_{:03}_{:02}_Detail", id, level))
                    .with_context(|| format!("Detail for skill {} level {}", id, level))
            })
            .collect::<Result<Vec<_>>>()?;

        result.insert(
            id,
            Skill {
                name,
                explain,
                levels,
            },
        );
    }

    Ok(result)
}

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
                            <a href={format!("/skill/{:03}.html", id)}>
                            {gen_multi_lang(&skill.name)}
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
                <h1 class="title">{gen_multi_lang(&skill.name)}</h1>
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
