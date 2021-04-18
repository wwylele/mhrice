use super::gen_website::*;
use super::pedia::*;
use crate::msg::*;
use crate::rsz::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

pub struct Quest {
    pub param: NormalQuestDataParam,
    pub enemy_param: Option<NormalQuestDataForEnemyParam>,
    pub name: Option<MsgEntry>,
    pub target: Option<MsgEntry>,
    pub condition: Option<MsgEntry>,
}

pub fn prepare_quests(pedia: &Pedia) -> Result<Vec<Quest>> {
    let mut all_msg: HashMap<String, MsgEntry> = pedia
        .quest_hall_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .chain(
            pedia
                .quest_village_msg
                .entries
                .iter()
                .map(|entry| (entry.name.clone(), entry.clone())),
        )
        .chain(
            pedia
                .quest_tutorial_msg
                .entries
                .iter()
                .map(|entry| (entry.name.clone(), entry.clone())),
        )
        .chain(
            pedia
                .quest_arena_msg
                .entries
                .iter()
                .map(|entry| (entry.name.clone(), entry.clone())),
        )
        .collect();

    let mut enemy_params: HashMap<i32, NormalQuestDataForEnemyParam> = pedia
        .normal_quest_data_for_enemy
        .param
        .iter()
        .map(|param| (param.quest_no, param.clone()))
        .collect();

    pedia
        .normal_quest_data
        .param
        .iter()
        .filter(|param| param.quest_no != 0)
        .map(|param| {
            let name_msg_name = format!("QN{:06}_01", param.quest_no);
            let target_msg_name = format!("QN{:06}_04", param.quest_no);
            let condition_msg_name = format!("QN{:06}_05", param.quest_no);
            Ok(Quest {
                param: param.clone(),
                enemy_param: enemy_params.remove(&param.quest_no),
                name: all_msg.remove(&name_msg_name),
                target: all_msg.remove(&target_msg_name),
                condition: all_msg.remove(&condition_msg_name),
            })
        })
        .collect::<Result<Vec<_>>>()
}

pub fn gen_quest_list(quests: &[Quest], root: &Path) -> Result<()> {
    let mut quests_ordered: BTreeMap<_, BTreeMap<_, Vec<&Quest>>> = BTreeMap::new();
    for quest in quests {
        quests_ordered
            .entry(quest.param.enemy_level)
            .or_default()
            .entry(quest.param.quest_level)
            .or_default()
            .push(quest);
    }

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Quests - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"Quests"</h1>
                {
                    quests_ordered.into_iter().map(|(enemy_level, quests)|{
                        html!(<section>
                         <h2 class="title">{text!("{:?}", enemy_level)}</h2>
                         <ul class="mh-list-quest">{
                            quests.into_iter().map(|(quest_level, quests)|{
                                html!(
                                    <li class="mh-list-quest">
                                        <h3 class="title">{text!("{:?}", quest_level)}</h3>
                                        <ul>{
                                            quests.into_iter().map(|quest|{
                                                html!{<li>
                                                    <a href={format!("/quest/{:06}.html", quest.param.quest_no)}>
                                                    {quest.name.as_ref().map_or(
                                                        html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                                                        gen_multi_lang
                                                    )}
                                                    </a>
                                                </li>}
                                            })
                                        }</ul>
                                    </li>
                                )
                            })
                        }</ul></section>)
                    })
                }
                </div> </div> </main>
            </body>
        </html>
    );

    let quests_path = root.join("quest.html");
    write(&quests_path, doc.to_string())?;

    Ok(())
}

fn gen_quest(quest: &Quest, pedia: &Pedia, path: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Quest {:06}", quest.param.quest_no)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">
                <span class="tag">{text!("{:?}-{:?}", quest.param.enemy_level, quest.param.quest_level)}</span>
                {
                    quest.name.as_ref().map_or(
                        html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                        gen_multi_lang
                    )
                }</h1>
                <p><span>"Objective: "</span><span> {
                    quest.target.as_ref().map_or(
                        html!(<span>"-"</span>),
                        gen_multi_lang
                    )
                }</span></p>
                <table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"HP"</th>
                        <th>"Attack"</th>
                        <th>"Parts"</th>
                        <th>"Other"</th>
                        <th>"Stamina"</th>
                    </tr></thead>
                    <tbody> {
                        quest.param.boss_em_type.iter().copied().enumerate().filter(|&(i, id)|id != 0)
                        .map(|(i, id)|{
                            let monster = pedia.monsters.iter().find(|m|(m.id | m.sub_id << 8) == id);
                            let monster_name = (||{
                                let name_name = format!("EnemyIndex{:03}",
                                    monster?.boss_init_set_data.as_ref()?.enemy_type);
                                Some(gen_multi_lang(pedia.monster_names.get_entry(&name_name)?))
                            })().unwrap_or(html!(<span>{text!("Monster {:03}_{:02}",
                                id & 0xFF, id >> 8)}</span>));

                            let icon_path = format!("/resources/em{0:03}_{1:02}_icon.png", id & 0xFF, id >> 8);

                            let hp = quest.enemy_param.as_ref().and_then(|ep|ep.vital_tbl.get(i))
                                .map_or_else(||"-".to_owned(), |v|format!("~ {}", v));
                            let attack = quest.enemy_param.as_ref().and_then(|ep|ep.attack_tbl.get(i))
                                .map_or_else(||"-".to_owned(), |v|format!("~ {}", v));
                            let parts = quest.enemy_param.as_ref().and_then(|ep|ep.parts_tbl.get(i))
                                .map_or_else(||"-".to_owned(), |v|format!("~ {}", v));
                            let other = quest.enemy_param.as_ref().and_then(|ep|ep.other_tbl.get(i))
                                .map_or_else(||"-".to_owned(), |v|format!("~ {}", v));
                            let stamina = quest.enemy_param.as_ref().and_then(|ep|ep.stamina_tbl.get(i))
                                .map_or_else(||"-".to_owned(), |v|format!("{}", v));

                            let target_tag = if quest.param.tgt_em_type.contains(&id) {
                                html!(<span class="tag is-primary">"Target"</span>)
                            } else {
                                html!(<span />)
                            };

                            html!(<tr>
                                <td>
                                    <a href={format!("/monster/{:03}_{1:02}.html", id & 0xFF, id >> 8)}>
                                        <img class="mh-quest-list-monster-icon" src=icon_path />
                                        <span  class="mh-quest-list-monster-name">
                                            {monster_name}
                                        </span>
                                    </a>
                                    {target_tag}
                                </td>
                                <td>{text!("{}", hp)}</td>
                                <td>{text!("{}", attack)}</td>
                                <td>{text!("{}", parts)}</td>
                                <td>{text!("{}", other)}</td>
                                <td>{text!("{}", stamina)}</td>
                            </tr>)
                        })
                    } </tbody>
                </table>
                </div> </div> </main>
            </body>
        </html>
    );

    write(&path, doc.to_string())?;
    Ok(())
}

pub fn gen_quests(quests: &[Quest], pedia: &Pedia, root: &Path) -> Result<()> {
    let quest_path = root.join("quest");
    create_dir(&quest_path)?;
    for quest in quests {
        let path = quest_path.join(format!("{:06}.html", quest.param.quest_no));
        gen_quest(quest, pedia, &path)?
    }
    Ok(())
}
