use super::gen_item::*;
use super::gen_weapon::*;
use super::gen_website::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_hyakuryu_skill_label(skill: &HyakuryuSkill) -> Box<a<String>> {
    html!(<a href={format!("/hyakuryu_skill/{}", hyakuryu_skill_page(skill.data.id))} class="mh-icon-text">
        {gen_colored_icon(skill.data.item_color, "/resources/rskill", &[])}
        <span>{gen_multi_lang(skill.name)}</span>
    </a>)
}

pub fn hyakuryu_skill_page(id: PlHyakuryuSkillId) -> String {
    match id {
        PlHyakuryuSkillId::None => "none.html".to_string(),
        PlHyakuryuSkillId::Skill(id) => format!("{:03}.html", id),
    }
}

pub fn gen_hyakuryu_skill_list(
    skills: &BTreeMap<PlHyakuryuSkillId, HyakuryuSkill>,
    output: &impl Sink,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Ramp-up skills - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Ramp-up skill"</h1>
                <ul class="mh-list-skill">
                {
                    skills.iter().map(|(_, skill)|{
                        html!(<li class="mh-list-skill"> {
                            gen_hyakuryu_skill_label(skill)
                        } </li>)
                    })
                }
                </ul>
                </div></main>
            </body>
        </html>
    );
    output
        .create_html("hyakuryu_skill.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_hyakuryu_source_weapon(
    id: PlHyakuryuSkillId,
    pedia_ex: &PediaEx,
) -> Option<Box<section<String>>> {
    let mut htmls = vec![];
    macro_rules! check_weapon {
        ($weapon:ident) => {
            let weapons = &pedia_ex.$weapon;
            for (_, weapon) in &weapons.weapons {
                let main: &MainWeaponBaseData = weapon.param.to_base();
                if main.hyakuryu_skill_id_list.contains(&id) ||
                    weapon.hyakuryu_weapon_buildup.values()
                        .any(|h|h.buildup_id_list.contains(&id)) {
                    htmls.push(html!(<li class="mh-list-item-in-out">{
                        gen_weapon_label(weapon)
                    }</li>));
                }
            }
        };
    }

    check_weapon!(great_sword);
    check_weapon!(short_sword);
    check_weapon!(hammer);
    check_weapon!(lance);
    check_weapon!(long_sword);
    check_weapon!(slash_axe);
    check_weapon!(gun_lance);
    check_weapon!(dual_blades);
    check_weapon!(horn);
    check_weapon!(insect_glaive);
    check_weapon!(charge_axe);
    check_weapon!(light_bowgun);
    check_weapon!(heavy_bowgun);
    check_weapon!(bow);

    if !htmls.is_empty() {
        Some(
            html!(<section class="section"> <div> <h2 class="title">"Available on weapons"</h2>
            <ul class="mh-list-item-in-out">{
                htmls
            }</ul> </div> </section>),
        )
    } else {
        None
    }
}

pub fn gen_hyakuryu_skill(
    skill: &HyakuryuSkill,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Ramp-up skill - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    {gen_colored_icon(skill.data.item_color, "/resources/rskill", &[])}
                </div>
                <h1 class="title">
                    {gen_multi_lang(skill.name)}
                </h1>
                <div>{gen_multi_lang(skill.explain)}</div>

                {skill.recipe.map(|recipe| html!(
                    <section class="section">
                    <h2 class="title">"Crafting"</h2>
                    <table>
                        <thead><tr>
                            <th>"Cost"</th>
                            <th>"Material"</th>
                        </tr></thead>
                        <tbody>
                        <tr>
                            <td>{ text!("{}", recipe.cost) }</td>
                            { gen_materials(pedia_ex, &recipe.recipe_item_id_list,
                                &recipe.recipe_item_num_list, ItemId::None) }
                        </tr>
                        </tbody>
                    </table>
                    </section>
                ))}

                { gen_hyakuryu_source_weapon(skill.data.id, pedia_ex) }

                </div></div></main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_hyakuryu_skills(pedia_ex: &PediaEx, output: &impl Sink) -> Result<()> {
    let skill_path = output.sub_sink("hyakuryu_skill")?;
    for (&id, skill) in &pedia_ex.hyakuryu_skills {
        let output = skill_path.create_html(&hyakuryu_skill_page(id))?;
        gen_hyakuryu_skill(skill, pedia_ex, output)?
    }
    Ok(())
}
