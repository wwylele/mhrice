use super::gen_website::*;
use super::pedia::*;
use crate::msg::*;
use crate::rsz::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, elements::*, html, text};

pub fn prepare_size_map(size_data: &EnemySizeListData) -> Result<HashMap<u32, &SizeInfo>> {
    let mut result = HashMap::new();
    for size_info in &size_data.size_info_list {
        if result.insert(size_info.em_type, size_info).is_some() {
            bail!("Duplicate size info for {}", size_info.em_type);
        }
    }
    Ok(result)
}

pub fn prepare_size_dist_map(
    size_dist_data: &EnemyBossRandomScaleData,
) -> Result<HashMap<i32, &[ScaleAndRateData]>> {
    let mut result = HashMap::new();
    for size_info in &size_dist_data.random_scale_table_data_list {
        if result
            .insert(size_info.type_, &size_info.scale_and_rate_data[..])
            .is_some()
        {
            bail!("Duplicate size dist for {}", size_info.type_);
        }
    }
    if result.contains_key(&0) {
        bail!("Defined size dist for 0");
    }
    result.insert(
        0,
        &[ScaleAndRateData {
            scale: 1.0,
            rate: 100,
        }],
    );
    Ok(result)
}

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

pub fn prepare_discoveries(pedia: &Pedia) -> Result<HashMap<u32, &DiscoverEmSetDataParam>> {
    let mut result = HashMap::new();
    for discovery in &pedia.discover_em_set_data.param {
        ensure!(discovery.param.route_no.len() == 5);
        ensure!(discovery.param.init_set_name.len() == 5);
        ensure!(discovery.param.sub_type.len() == 3);
        ensure!(discovery.param.vital_tbl.len() == 3);
        ensure!(discovery.param.attack_tbl.len() == 3);
        ensure!(discovery.param.parts_tbl.len() == 3);
        ensure!(discovery.param.other_tbl.len() == 3);
        ensure!(discovery.param.stamina_tbl.len() == 3);
        ensure!(discovery.param.scale.len() == 3);
        ensure!(discovery.param.scale_tbl.len() == 3);
        ensure!(discovery.param.difficulty.len() == 3);
        ensure!(discovery.param.boss_multi.len() == 3);

        if result.insert(discovery.em_type, discovery).is_some() {
            bail!("Duplicated discovery data for {}", discovery.em_type)
        }
    }

    Ok(result)
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

pub fn gen_quest_monster_data(
    enemy_param: Option<&SharedEnemyParam>,
    em_type: u32,
    index: usize,
    sizes: &HashMap<u32, &SizeInfo>,
    size_dists: &HashMap<i32, &[ScaleAndRateData]>,
    pedia: &Pedia,
) -> impl IntoIterator<Item = Box<td<String>>> {
    let enemy_param = if let Some(enemy_param) = enemy_param.as_ref() {
        enemy_param
    } else {
        return vec![html!(<td colspan=11>"[NO DATA]"</td>)];
    };

    let size = if let (Some(scale_tbl_i), Some(base_scale)) = (
        enemy_param.scale_tbl.get(index),
        enemy_param.scale.get(index),
    ) {
        if let (Some(size), Some(size_dist)) = (sizes.get(&em_type), size_dists.get(scale_tbl_i)) {
            let mut small_chance = 0;
            let mut large_chance = 0;
            for sample in *size_dist {
                let scale = sample.scale * (*base_scale as f32) / 100.0;
                if scale <= size.small_boarder {
                    small_chance += sample.rate;
                }
                if scale >= size.king_boarder {
                    large_chance += sample.rate;
                }
            }

            let small = (small_chance != 0).then(|| {
                html!(<span class="tag">
                    <img src="/resources/small_crown.png" />
                    {text!("{}%", small_chance)}
                </span>)
            });

            let large = (large_chance != 0).then(|| {
                html!(<span class="tag">
                    <img src="/resources/king_crown.png" />
                    {text!("{}%", large_chance)}
                </span>)
            });

            html!(<span>{small}{large}</span>)
        } else {
            html!(<span>"-"</span>)
        }
    } else {
        html!(<span>"-"</span>)
    };

    let hp = enemy_param.vital_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .vital_rate_table_list
                .get(usize::from(*v))
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.vital_rate))
        },
    );
    let attack = enemy_param.attack_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .attack_rate_table_list
                .get(usize::from(*v))
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.attack_rate))
        },
    );
    let parts = enemy_param.parts_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .parts_rate_table_list
                .get(usize::from(*v))
                .map_or_else(
                    || format!("~ {}", v),
                    |r| format!("x{}", r.parts_vital_rate),
                )
        },
    );

    let defense;
    let element_a;
    let element_b;
    let stun;
    let exhaust;
    let ride;

    if let Some(v) = enemy_param.other_tbl.get(index) {
        if let Some(r) = pedia
            .difficulty_rate
            .other_rate_table_list
            .get(usize::from(*v))
        {
            defense = format!("x{}", r.defense_rate);
            element_a = format!("x{}", r.damage_element_rate_a);
            element_b = format!("x{}", r.damage_element_rate_b);
            stun = format!("x{}", r.stun_rate);
            exhaust = format!("x{}", r.tired_rate);
            ride = format!("x{}", r.marionette_rate);
        } else {
            let placeholder = format!("~ {}", v);
            defense = placeholder.clone();
            element_a = placeholder.clone();
            element_b = placeholder.clone();
            stun = placeholder.clone();
            exhaust = placeholder.clone();
            ride = placeholder;
        }
    } else {
        defense = "-".to_owned();
        element_a = "-".to_owned();
        element_b = "-".to_owned();
        stun = "-".to_owned();
        exhaust = "-".to_owned();
        ride = "-".to_owned();
    };

    let stamina = enemy_param
        .stamina_tbl
        .get(index)
        .map_or_else(|| "-".to_owned(), |v| format!("{}", v));

    vec![
        html!(<td>{size}</td>),
        html!(<td>{text!("{}", hp)}</td>),
        html!(<td>{text!("{}", attack)}</td>),
        html!(<td>{text!("{}", parts)}</td>),
        html!(<td>{text!("{}", defense)}</td>),
        html!(<td>{text!("{}", element_a)}</td>),
        html!(<td>{text!("{}", element_b)}</td>),
        html!(<td>{text!("{}", stun)}</td>),
        html!(<td>{text!("{}", exhaust)}</td>),
        html!(<td>{text!("{}", ride)}</td>),
        html!(<td>{text!("{}", stamina)}</td>),
    ]
}

fn gen_multi_factor(data: &MultiData) -> Box<div<String>> {
    html!(<div><ul class="mh-multi-factor">
        <li><span>"2: "</span><span>{text!("x{}", data.two)}</span></li>
        <li><span>"3: "</span><span>{text!("x{}", data.three)}</span></li>
        <li><span>"4: "</span><span>{text!("x{}", data.four)}</span></li>
    </ul></div>)
}

fn gen_quest_monster_multi_player_data(
    enemy_param: Option<&SharedEnemyParam>,
    index: usize,
    pedia: &Pedia,
) -> impl IntoIterator<Item = Box<td<String>>> {
    let no_data = || vec![html!(<td colspan=9>"[NO DATA]"</td>)];

    let enemy_param = if let Some(enemy_param) = enemy_param.as_ref() {
        enemy_param
    } else {
        return no_data();
    };

    let multi = if let Some(multi) = enemy_param.boss_multi.get(index) {
        *multi
    } else {
        return no_data();
    };

    let table = if let Some(table) = pedia
        .difficulty_rate
        .multi_rate_table_list
        .get(usize::from(multi))
    {
        &table.multi_data_list
    } else {
        return no_data();
    };

    table
        .iter()
        .map(|d| html!(<td>{gen_multi_factor(d)}</td>))
        .collect()
}

fn gen_monster_tag(quest: &Quest, pedia: &Pedia, id: u32) -> Box<td<String>> {
    let monster = pedia.monsters.iter().find(|m| (m.id | m.sub_id << 8) == id);
    let monster_name = (|| {
        let name_name = format!(
            "EnemyIndex{:03}",
            monster?.boss_init_set_data.as_ref()?.enemy_type
        );
        Some(gen_multi_lang(pedia.monster_names.get_entry(&name_name)?))
    })()
    .unwrap_or(html!(<span>{text!("Monster {:03}_{:02}",
                                id & 0xFF, id >> 8)}</span>));

    let icon_path = format!("/resources/em{0:03}_{1:02}_icon.png", id & 0xFF, id >> 8);

    let target_tag = if quest.param.tgt_em_type.contains(&id) {
        html!(<span class="tag is-primary">"Target"</span>)
    } else {
        html!(<span />)
    };
    html!(<td>
        <a href={format!("/monster/{:03}_{1:02}.html", id & 0xFF, id >> 8)}>
            <img class="mh-quest-list-monster-icon" src=icon_path />
            <span  class="mh-quest-list-monster-name">
                {monster_name}
            </span>
        </a>
        {target_tag}
    </td>)
}

fn gen_quest(
    quest: &Quest,
    sizes: &HashMap<u32, &SizeInfo>,
    size_dists: &HashMap<i32, &[ScaleAndRateData]>,
    pedia: &Pedia,
    path: &Path,
) -> Result<()> {
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
                <section class="section">
                <h2 class="subtitle">"Monster stats"</h2>
                <table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"Size (?)"</th>
                        <th>"HP"</th>
                        <th>"Attack"</th>
                        <th>"Parts"</th>
                        <th>"Defense"</th>
                        <th>"Element A"</th>
                        <th>"Element B"</th>
                        <th>"Stun"</th>
                        <th>"Exhaust"</th>
                        <th>"Ride"</th>
                        <th>"Stamina"</th>
                    </tr></thead>
                    <tbody> {
                        quest.param.boss_em_type.iter().copied().enumerate().filter(|&(i, em_type)|em_type != 0)
                        .map(|(i, em_type)|{
                            html!(<tr>
                                { gen_monster_tag(quest, pedia, em_type) }
                                { gen_quest_monster_data(quest.enemy_param.as_ref().map(|p|&p.param),
                                    em_type, i, sizes,size_dists, pedia) }
                            </tr>)
                        })
                    } </tbody>
                </table>
                </section>
                <section class="section">
                <h2 class="subtitle">"Multiplayer Factor (Column header might be wrong)"</h2>

                <table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"HP"</th>
                        <th>"Attack"</th>
                        <th>"Parts"</th>
                        <th>"Other parts"</th>
                        <th>"Multi parts"</th>
                        <th>"Defense"</th>
                        <th>"Element A"</th>
                        <th>"Element B"</th>
                        <th>"Stun"</th>
                        <th>"Exhaust"</th>
                        <th>"Ride"</th>
                        <th>"Monster to monster"</th>
                    </tr></thead>
                    <tbody> {
                        quest.param.boss_em_type.iter().copied().enumerate().filter(|&(i, id)|id != 0)
                        .map(|(i, id)|{
                            html!(<tr>
                                { gen_monster_tag(quest, pedia, id) }
                                { gen_quest_monster_multi_player_data(
                                    quest.enemy_param.as_ref().map(|p|&p.param), i, pedia) }
                            </tr>)
                        })
                    } </tbody>
                </table>

                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    write(&path, doc.to_string())?;
    Ok(())
}

pub fn gen_quests(
    quests: &[Quest],
    sizes: &HashMap<u32, &SizeInfo>,
    size_dists: &HashMap<i32, &[ScaleAndRateData]>,
    pedia: &Pedia,
    root: &Path,
) -> Result<()> {
    let quest_path = root.join("quest");
    create_dir(&quest_path)?;
    for quest in quests {
        let path = quest_path.join(format!("{:06}.html", quest.param.quest_no));
        gen_quest(quest, sizes, size_dists, pedia, &path)?
    }
    Ok(())
}
