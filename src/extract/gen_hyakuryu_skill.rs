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
                <main>
                <header><h1>"Ramp-up skill"</h1></header>
                <ul class="mh-item-list">
                {
                    skills.iter().map(|(_, skill)|{
                        html!(<li> {
                            gen_hyakuryu_skill_label(skill)
                        } </li>)
                    })
                }
                </ul>
                </main>
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
                    htmls.push(html!(<li>{
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
        Some(html!(<section> <div> <h2 >"Available on weapons"</h2>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div> </section>))
    } else {
        None
    }
}

pub fn gen_hyakuryu_skill(
    skill: &HyakuryuSkill,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    toc_sink.add(skill.name);
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Ramp-up skill - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_colored_icon(skill.data.item_color, "/resources/rskill", &[])}
                    </div>
                    <h1>{gen_multi_lang(skill.name)}</h1>
                </header>

                <section>
                    <pre>{gen_multi_lang(skill.explain)}</pre>
                </section>

                {skill.recipe.map(|recipe| html!(
                    <section>
                    <h2 >"Crafting"</h2>
                    <div class="mh-table"><table>
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
                    </table></div>
                    </section>
                ))}

                { gen_hyakuryu_source_weapon(skill.data.id, pedia_ex) }

                </main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_hyakuryu_skills(pedia_ex: &PediaEx, output: &impl Sink, toc: &mut Toc) -> Result<()> {
    let skill_path = output.sub_sink("hyakuryu_skill")?;
    for (&id, skill) in &pedia_ex.hyakuryu_skills {
        let (output, toc_sink) = skill_path.create_html_with_toc(&hyakuryu_skill_page(id), toc)?;
        gen_hyakuryu_skill(skill, pedia_ex, output, toc_sink)?
    }
    Ok(())
}
