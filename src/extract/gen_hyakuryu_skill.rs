use super::gen_common::*;
use super::gen_item::*;
use super::gen_monster::gen_monster_tag;
use super::gen_weapon::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_hyakuryu_skill_label(skill: &HyakuryuSkill) -> Box<a<String>> {
    html!(<a href={format!("hyakuryu_skill/{}", hyakuryu_skill_page(skill.id()))} class="mh-icon-text">
        {gen_colored_icon(skill.color(), "resources/rskill", [], false)}
        <span>{gen_multi_lang(skill.name)}</span>
        {skill.recipe.is_some().then(||html!(<span class="tag">"HR"</span>))}
        {skill.deco.is_some().then(||html!(<span class="tag">"MR"</span>))}
    </a>)
}

pub fn hyakuryu_skill_page(id: PlHyakuryuSkillId) -> String {
    match id {
        PlHyakuryuSkillId::None => "none.html".to_string(),
        PlHyakuryuSkillId::Skill(id) => format!("{id:03}.html"),
    }
}

pub fn gen_hyakuryu_skill_list(
    hash_store: &HashStore,
    skills: &BTreeMap<PlHyakuryuSkillId, HyakuryuSkill>,
    output: &impl Sink,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Rampage skills - MHRice")}</title>
                { head_common(hash_store, output) }
                <style id="mh-skill-list-style">""</style>
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Rampage skills"</h1></header>

                <div>
                    <a href="skill.html"><span class="icon-text">
                    <span class="icon">
                    <i class="fas fa-arrow-right"></i>
                    </span>
                    <span>"go to armor skill"</span>
                    </span></a>
                </div>

                <div class="mh-filters"><ul>
                    <li id="mh-skill-filter-button-all" class="is-active mh-skill-filter-button">
                        <a>"All skills"</a></li>
                    <li id="mh-skill-filter-button-hr" class="mh-skill-filter-button">
                        <a>"HR"</a></li>
                    <li id="mh-skill-filter-button-deco1" class="mh-skill-filter-button">
                        <a>"Lv1 deco"</a></li>
                    <li id="mh-skill-filter-button-deco2" class="mh-skill-filter-button">
                        <a>"Lv2 deco"</a></li>
                    <li id="mh-skill-filter-button-deco3" class="mh-skill-filter-button">
                        <a>"Lv3 deco"</a></li>
                </ul></div>

                <ul class="mh-item-list">
                {
                    skills.iter().map(|(_, skill)|{
                        let mut filter_tags = vec![];
                        if skill.recipe.is_some() {
                            filter_tags.push("hr".to_owned())
                        }
                        if let Some(deco) = &skill.deco {
                            filter_tags.push(format!("deco{}", deco.data.decoration_lv))
                        }
                        let filter = filter_tags.join(" ");
                        html!(<li data-filter={filter} class="mh-skill-filter-item"> {
                            gen_hyakuryu_skill_label(skill)
                        } </li>)
                    })
                }
                </ul>
                </main>
                { right_aside() }
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
        Some(
            html!(<section id="s-source"> <div> <h2 >"Available on weapons"</h2>
            <ul class="mh-item-list">{
                htmls
            }</ul> </div> </section>),
        )
    } else {
        None
    }
}

pub fn gen_hyakuryu_deco_label(deco: &HyakuryuDeco) -> Box<div<String>> {
    let icon_id = if deco.data.decoration_lv == 4 {
        200
    } else {
        63 + deco.data.decoration_lv
    };
    let icon = format!("resources/item/{icon_id:03}");
    html!(<div class="mh-icon-text">
        { gen_colored_icon(deco.data.icon_color, &icon, ["mh-addon-hyakuryu"], false) }
        <span>{gen_multi_lang(deco.name)}</span>
    </div>)
}

pub fn gen_hyakuryu_skill(
    hash_store: &HashStore,
    skill: &HyakuryuSkill,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    skill_path: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let (mut output, mut toc_sink) =
        skill_path.create_html_with_toc(&hyakuryu_skill_page(skill.id()), toc)?;

    toc_sink.add(skill.name);

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            <pre>{gen_multi_lang(skill.explain)}</pre>
            </section>
        ),
    });

    if let Some(recipe) = skill.recipe {
        sections.push(Section {
            title: "Crafting on weapon".to_owned(),
            content: html!(<section id="s-crafting">
                <h2 >"Crafting on weapon"</h2>
                <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Cost"</th>
                        <th>"Material"</th>
                    </tr></thead>
                    <tbody>
                    <tr>
                        <td>{ text!("{}z", recipe.cost) }</td>
                        { gen_materials(pedia_ex, &recipe.recipe_item_id_list,
                            &recipe.recipe_item_num_list, &[]) }
                    </tr>
                    </tbody>
                </table></div>
                </section>),
        });
    }

    if let Some(source) = gen_hyakuryu_source_weapon(skill.id(), pedia_ex) {
        sections.push(Section {
            title: "Available on weapons".to_owned(),
            content: source,
        });
    }

    if let Some(deco) = &skill.deco {
        toc_sink.add(deco.name);
        sections.push(Section {
            title: "Decoration".to_owned(),
            content: html!(
                <section id="s-decoration">
                <h2 >"Decoration"</h2>
                <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Name"</th>
                        <th>"Unlock at"</th>
                        <th>"Key Monster"</th>
                        <th>"Cost"</th>
                        <th>"Categorized Material"</th>
                        <th>"Material"</th>
                    </tr></thead>
                    <tbody>
                        <tr>
                            <td>{gen_hyakuryu_deco_label(deco)}</td>
                            <td>{gen_progress(deco.product.progress_flag, pedia_ex)}</td>
                            <td>{(deco.product.enemy_flag != EmTypes::Em(0)).then(
                                ||gen_monster_tag(pedia_ex, deco.product.enemy_flag, false, false, None, None)
                            )}</td>
                            <td>{text!("{}z", deco.data.base_price * 2)}</td>
                            { gen_category(pedia_ex, deco.product.material_category,
                                deco.product.point) }
                            { gen_materials(pedia_ex, &deco.product.item_id_list,
                                &deco.product.item_num_list, &[deco.product.item_flag]) }
                        </tr>
                    </tbody>
                </table></div>
                </section>
            ),
        })
    }

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Rampage skill - MHRice")}</title>
                { head_common(hash_store, skill_path) }
                { title_multi_lang(skill.name) }
                { open_graph(Some(skill.name), "",
                    Some(skill.explain), "", None, toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections, toc_sink.path()) }
                <main>
                <header>
                    <div class="mh-title-icon">
                        {gen_colored_icon(skill.color(), "resources/rskill", [], false)}
                    </div>
                    <h1>{gen_multi_lang(skill.name)}</h1>
                </header>

                { sections.into_iter().map(|s|s.content) }

                </main>
                { right_aside() }
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_hyakuryu_skills(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let skill_path = output.sub_sink("hyakuryu_skill")?;
    for skill in pedia_ex.hyakuryu_skills.values() {
        gen_hyakuryu_skill(hash_store, skill, pedia_ex, config, &skill_path, toc)?
    }
    Ok(())
}
