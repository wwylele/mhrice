use super::gen_armor::*;
use super::gen_common::*;
use super::gen_hyakuryu_skill::*;
use super::gen_item::*;
use super::gen_map::*;
use super::gen_monster::*;
use super::gen_otomo::*;
use super::gen_skill::*;
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

pub fn quest_level_tag(quest: &Quest) -> Box<span<String>> {
    if let Some(anomaly) = quest.param.anomaly_level() {
        return html!(<span class="tag is-danger">{
            text!("MR-A{}", anomaly)}</span>);
    }
    let (quest_level_tag, name) = match quest.param.enemy_level {
        EnemyLevel::Village => ("mh-quest-village", "Vi"),
        EnemyLevel::Low => ("mh-quest-low", "LR"),
        EnemyLevel::High => ("mh-quest-high", "HR"),
        EnemyLevel::Master => ("mh-quest-master", "MR"),
    };
    let level = match quest.param.quest_level {
        QuestLevel::QL1 => "1",
        QuestLevel::QL2 => "2",
        QuestLevel::QL3 => "3",
        QuestLevel::QL4 => "4",
        QuestLevel::QL5 => "5",
        QuestLevel::QL6 => "6",
        QuestLevel::QL7 => "7",
        QuestLevel::QL7Ex => "7Ex",
    };
    let level_tag_class = format!("tag {quest_level_tag}");
    html!(<span class={level_tag_class.as_str()}>{
        text!("{}{}", name, level)}</span>)
}

pub fn gen_quest_tag(
    quest: &Quest,
    tag_level: bool,
    is_target: bool,
    mystery_type: Option<EnemyIndividualType>,
    sub_type_tag: Option<Box<span<String>>>,
) -> Box<div<String>> {
    let img = format!(
        "resources/questtype_{}.png",
        quest.param.quest_type.icon_index()
    );
    html!(<div>
        <a href={format!("quest/{:06}.html", quest.param.quest_no)} class="mh-icon-text">
        <img alt="Quest icon" src={img} class="mh-quest-icon"/>
        {
            tag_level.then(
                ||quest_level_tag(quest)
            )
        }
        {
            quest.is_dl.then(
                ||html!(<span class="tag mh-quest-event">{text!("Event")}</span>)
            )
        }
        {
            quest.param.is_servant_request().then(
                ||html!(<span class="tag mh-quest-follower">{text!("Follower")}</span>)
            )
        }
        {
            quest.param.is_kingdom().then(
                ||html!(<span class="tag mh-quest-follower">{text!("Survey")}</span>)
            )
        }
        {
            quest.param.is_from_npc.then(
                ||html!(<span class="tag mh-quest-npc">{text!("NPC")}</span>)
            )
        }
        {quest.name.map_or(
            html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
            gen_multi_lang
        )}
        {is_target.then(||html!(<span class="tag is-primary">"Target"</span>))}
        {gen_mystery_tag(mystery_type)}
        {sub_type_tag}
        </a>
    </div>)
}

pub fn gen_quest_list(
    hash_store: &HashStore,
    quests: &BTreeMap<i32, Quest>,
    output: &impl Sink,
) -> Result<()> {
    let mut quests_ordered: BTreeMap<_, BTreeMap<_, Vec<&Quest>>> = BTreeMap::new();
    let mut anomaly_ordered: BTreeMap<i32, Vec<&Quest>> = BTreeMap::new();
    for quest in quests.values() {
        if let Some(anomaly) = quest.param.anomaly_level() {
            anomaly_ordered.entry(anomaly).or_default().push(quest);
        } else {
            quests_ordered
                .entry(quest.param.enemy_level)
                .or_default()
                .entry(quest.param.quest_level)
                .or_default()
                .push(quest);
        }
    }

    let mut sections = vec![];

    for (enemy_level, quests) in quests_ordered {
        let quest_level_name = match enemy_level {
            EnemyLevel::Village => "Village",
            EnemyLevel::Low => "Low rank",
            EnemyLevel::High => "High rank",
            EnemyLevel::Master => "Master rank",
        };

        for (quest_level, quests) in quests {
            let level_name = match quest_level {
                QuestLevel::QL1 => "1★",
                QuestLevel::QL2 => "2★",
                QuestLevel::QL3 => "3★",
                QuestLevel::QL4 => "4★",
                QuestLevel::QL5 => "5★",
                QuestLevel::QL6 => "6★",
                QuestLevel::QL7 => "7★",
                QuestLevel::QL7Ex => "7★Ex",
            };

            let title = format!("{quest_level_name} {level_name}");
            let id = format!("s-{}s{}", enemy_level.into_raw(), quest_level.into_raw());
            sections.push(Section {
                title: title.clone(),
                content: html!(
                    <section id={id.as_str()}>
                        <h2>{text!("{}", title)}</h2>
                        <ul class="mh-quest-list">{
                            quests.into_iter().map(|quest|{
                                html!{<li>
                                    { gen_quest_tag(quest, false, false, None, None) }
                                </li>}
                            })
                        }</ul>
                    </section>
                ),
            })
        }
    }

    for (quest_level, quests) in anomaly_ordered {
        let title = format!("A{quest_level}★");
        let id = format!("s-a{quest_level}");
        sections.push(Section {
            title: title.clone(),
            content: html!(
                <section id={id.as_str()}>
                <h2>{text!("{}", title)}</h2>
                <ul class="mh-quest-list">{
                    quests.into_iter().map(|quest|{
                        html!{<li>
                            { gen_quest_tag(quest, false, false, None, None) }
                        </li>}
                    })
                }</ul>
            </section>
            ),
        });
    }

    let file_name = "quest.html";

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Quests - MHRice")}</title>
                { head_common(hash_store, output) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections, &(output.toc_path() + file_name)) }
                <main>
                <header><h1>"Quests"</h1></header>
                <div>
                    <a href="villager_request.html"><span class="icon-text">
                    <span class="icon">
                    <i class="fas fa-arrow-right"></i>
                    </span>
                    <span>"go to villager requests"</span>
                    </span></a>
                </div>
                { sections.into_iter().map(|s|s.content) }
                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html(file_name)?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_quest_monster_data(
    enemy_param: Option<&impl EnemyParam>,
    em_type_for_scale: Option<EmTypes>,
    index: usize,
    difficulty_rate: &SystemDifficultyRateData,
    pedia_ex: &PediaEx<'_>,
) -> impl IntoIterator<Item = Box<td<String>>> {
    let enemy_param = if let Some(enemy_param) = enemy_param.as_ref() {
        enemy_param
    } else {
        return vec![html!(<td colspan=12>"[NO DATA]"</td>)];
    };

    let size = em_type_for_scale.map(|em_type| {
        if let (Some(scale_tbl_i), Some(base_scale)) =
            (enemy_param.scale_tbl(index), enemy_param.scale(index))
        {
            if let (Some(size), Some(size_dist)) = (
                pedia_ex.sizes.get(&em_type),
                pedia_ex.size_dists.get(&scale_tbl_i),
            ) {
                let mut small_chance = 0;
                let mut large_chance = 0;
                for sample in *size_dist {
                    let scale = sample.scale * (base_scale as f32) / 100.0;
                    if scale <= size.small_boarder {
                        small_chance += sample.rate;
                    }
                    if scale >= size.king_boarder {
                        large_chance += sample.rate;
                    }
                }

                let small = (small_chance != 0).then(|| {
                    html!(<span class="mh-crown">
                    <img class="mh-crown-icon" alt="Small crown" src="resources/small_crown.png" />
                    {text!("{}%", small_chance)}
                </span>)
                });

                let large = (large_chance != 0).then(|| {
                    html!(<span class="mh-crown">
                    <img class="mh-crown-icon" alt="Large crown" src="resources/king_crown.png" />
                    {text!("{}%", large_chance)}
                </span>)
                });

                html!(<span>{small}{large}</span>)
            } else {
                html!(<span>"-"</span>)
            }
        } else {
            html!(<span>"-"</span>)
        }
    });

    let hp = enemy_param.vital_tbl(index).map_or_else(
        || "-".to_owned(),
        |v| {
            let mut s = difficulty_rate
                .vital_rate_table_list
                .get(usize::from(v))
                .map_or_else(|| format!("~ {v}"), |r| format!("x{}", r.vital_rate));
            match enemy_param.difficulty(index) {
                Some(NandoYuragi::True1) => s += "(±)",
                Some(NandoYuragi::True2) => s += "(±±)",
                _ => (),
            }
            s
        },
    );
    let attack = enemy_param.attack_tbl(index).map_or_else(
        || "-".to_owned(),
        |v| {
            difficulty_rate
                .attack_rate_table_list
                .get(usize::from(v))
                .map_or_else(|| format!("~ {v}"), |r| format!("x{}", r.attack_rate))
        },
    );
    let parts = enemy_param.parts_tbl(index).map_or_else(
        || "-".to_owned(),
        |v| {
            difficulty_rate
                .parts_rate_table_list
                .get(usize::from(v))
                .map_or_else(|| format!("~ {v}"), |r| format!("x{}", r.parts_vital_rate))
        },
    );

    let defense;
    let element_ab;
    let stun;
    let exhaust;
    let paralyze;
    let sleep;
    let ride;

    if let Some(v) = enemy_param.other_tbl(index) {
        if let Some(r) = difficulty_rate.other_rate_table_list.get(usize::from(v)) {
            defense = html!(<span>{text!("x{}", r.defense_rate)}</span>);
            element_ab = html!(<span>{text!("Ax{}, Bx{}", r.damage_element_rate_a, r.damage_element_rate_b)}
                <br/>{text!("①Ax{}, Bx{}", r.damage_element_first_rate_a, r.damage_element_first_rate_b)}</span>);
            stun = html!(<span>{text!("x{}", r.stun_rate)}
                <br/>{text!("①x{}", r.stun_first_rate)}</span>);
            exhaust = html!(<span>{text!("x{}", r.tired_rate)}
                <br/>{text!("①x{}", r.tired_first_rate)}</span>);
            paralyze = html!(<span>{text!("x{}", r.paralyze_rate)}
                <br/>{text!("①x{}", r.paralyze_first_rate)}</span>);
            sleep = html!(<span>{text!("x{}", r.sleep_rate)}
                <br/>{text!("①x{}", r.sleep_first_rate)}</span>);
            ride = html!(<span>{text!("x{}", r.marionette_rate)}</span>);
        } else {
            let placeholder = || html!(<span>{text!("~ {}", v)}</span>);
            defense = placeholder();
            element_ab = placeholder();
            stun = placeholder();
            exhaust = placeholder();
            paralyze = placeholder();
            sleep = placeholder();
            ride = placeholder();
        }
    } else {
        let placeholder = || html!(<span>"-"</span>);
        defense = placeholder();
        element_ab = placeholder();
        stun = placeholder();
        exhaust = placeholder();
        paralyze = placeholder();
        sleep = placeholder();
        ride = placeholder();
    };

    let stamina = enemy_param
        .stamina_tbl(index)
        .map_or_else(|| "-".to_owned(), |v| format!("{v}"));

    let mut result = if let Some(size) = size {
        vec![html!(<td>{size}</td>)]
    } else {
        vec![]
    };

    result.extend([
        html!(<td>{text!("{}", hp)}</td>),
        html!(<td>{text!("{}", attack)}</td>),
        html!(<td>{text!("{}", parts)}</td>),
        html!(<td class="mh-quest-detail">{defense}</td>),
        html!(<td class="mh-quest-detail">{element_ab}</td>),
        html!(<td class="mh-quest-detail">{stun}</td>),
        html!(<td class="mh-quest-detail">{exhaust}</td>),
        html!(<td class="mh-quest-detail">{paralyze}</td>),
        html!(<td class="mh-quest-detail">{sleep}</td>),
        html!(<td class="mh-quest-detail">{ride}</td>),
        html!(<td class="mh-quest-detail">{text!("{}", stamina)}</td>),
    ]);
    result
}

fn gen_multi_factor(data: &MultiData) -> Box<div<String>> {
    html!(<div><ul class="mh-multi-factor">
        <li><span>"2: "</span><span>{text!("x{}", data.two)}</span></li>
        <li><span>"3: "</span><span>{text!("x{}", data.three)}</span></li>
        <li><span>"4: "</span><span>{text!("x{}", data.four)}</span></li>
    </ul></div>)
}

pub fn translate_rule(rule: LotRule) -> Box<span<String>> {
    let desc = match rule {
        LotRule::Random => "Get random amount",
        LotRule::RandomOut1 => "Get one",
        LotRule::RandomOut2 => "Get two",
        LotRule::RandomOut3 => "Get three",
        LotRule::FirstFix => "First one fixed",
    };
    html!(<span class="mh-lot-rule">{ text!("{}", desc) }</span>)
}

#[allow(clippy::vec_box)]
fn gen_quest_monster_multi_player_data(
    enemy_param: Option<&NormalQuestDataForEnemyParam>,
    index: usize,
    pedia: &Pedia,
) -> Vec<Box<td<String>>> {
    let no_data = || vec![html!(<td colspan=13>"[NO DATA]"</td>)];

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
        .enumerate()
        .map(|(i, d)| {
            let class = if i > 2 { "mh-quest-detail" } else { "" };
            html!(<td class={class}>{gen_multi_factor(d)}</td>)
        })
        .collect()
}

impl HyakuryuQuestData {
    fn display(&self) -> String {
        let mut list = vec![];
        if self.attr.contains(HyakuryuQuestAttr::FIX_WAVE_ORDER) {
            list.push("Fixed wave order")
        }
        if self.attr.contains(HyakuryuQuestAttr::LOT_HIGH_EX) {
            list.push("Red 7* reward")
        }
        if self.attr.contains(HyakuryuQuestAttr::LOT_TRUE_ED) {
            list.push("After true ending reward")
        }
        if self.attr.contains(HyakuryuQuestAttr::FINAL_BOSS_KILL) {
            list.push("Requires true ending")
        }
        if self.category == HyakuryuQuestCategory::Nushi {
            list.push("Has apex")
        }
        if self.is_village {
            list.push("Village")
        } else {
            list.push("Hub")
        }
        list.join(" | ")
    }
}

fn gen_quest(
    hash_store: &HashStore,
    quest: &Quest,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    quest_path: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let (mut output, mut toc_sink) =
        quest_path.create_html_with_toc(&format!("{:06}.html", quest.param.quest_no), toc)?;

    if let Some(title) = quest.name {
        toc_sink.add(title);
    }

    let has_normal_em = quest
        .param
        .boss_em_type
        .iter()
        .any(|&em_type| em_type != EmTypes::Em(0));
    let img = format!(
        "resources/questtype_{}.png",
        quest.param.quest_type.icon_index()
    );

    let target = quest
        .param
        .target_type
        .iter()
        .zip(&quest.param.tgt_em_type)
        .zip(&quest.param.tgt_item_id)
        .zip(&quest.param.tgt_num)
        .filter(|(((&ty, _), _), _)| ty != QuestTargetType::None)
        .enumerate()
        .flat_map(|(i, (((ty, em), item), num))| {
            let em = if let Some(MonsterEx {
                name: Some(name), ..
            }) = pedia_ex.monsters.get(em)
            {
                gen_multi_lang(name)
            } else {
                html!(<span>{text!("Unknown monster {:?}", em)}</span>)
            };
            let result = match ty {
                QuestTargetType::ItemGet => {
                    let item = if let Some(item_entry) = pedia_ex.items.get(item) {
                        gen_multi_lang(item_entry.name)
                    } else {
                        html!(<span>{text!("Unknown item {:?}", item)}</span>)
                    };
                    html!(<span>{text!("Gather {}x ", num)} {item}</span>)
                }

                QuestTargetType::Hunting => {
                    html!(<span>{text!("Hunt {}x ", num)} {em}</span>)
                }
                QuestTargetType::Kill => html!(<span>{text!("Slay {}x ", num)} {em}</span>),
                QuestTargetType::Capture => html!(<span>{text!("Capture {}x ", num)} {em}</span>),
                QuestTargetType::EmTotal => {
                    if i == 0 {
                        html!(<span>{text!("Slay {}x ", num)} {em}</span>)
                    } else {
                        html!(<span>{em}</span>)
                    }
                }
                QuestTargetType::AllMainEnemy => {
                    html!(<span>{text!("Hunt all {}", num)}</span>)
                }
                QuestTargetType::FinalBarrierDefense => html!(<span>"Defend final barrier"</span>),
                QuestTargetType::FortLevelUp => html!(<span>"Level up fort"</span>),
                QuestTargetType::PlayerDown => html!(<span>"PlayerDown"</span>),
                QuestTargetType::FinalBoss => html!(<span>"Final boss"</span>),

                x => html!(<span>{text!("{:?}", x)}</span>),
            };
            if i == 0 {
                vec![result]
            } else {
                vec![html!(<span>", "</span>), result]
            }
        });

    let requirement = quest
        .param
        .order_type
        .iter()
        .filter(|&&t| t != QuestOrderType::None)
        .map(|t| format!("{t}"))
        .collect::<Vec<String>>()
        .join(", ");

    let has_target_material = quest
        .param
        .tgt_item_id
        .iter()
        .any(|&item| item != ItemId::None);

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            <p><span>"Objective: "</span><span> {
                quest.target.map_or(
                    html!(<span>"-"</span>),
                    gen_multi_lang
                )
            }</span></p>
            <p><span>"From: "</span><span> {
                quest.requester.map_or(
                    html!(<span>"-"</span>),
                    gen_multi_lang
                )
            }</span></p>
            <p><span>"Detail: "</span></p>{
                quest.detail.map_or(
                    html!(<div>"-"</div>),
                    |m|html!(<div><pre>{gen_multi_lang(m)}</pre></div>)
                )
            }
            </section>
        ),
    });

    let swap = quest
        .param
        .swap_em_rate
        .iter()
        .zip(&quest.param.swap_set_condition)
        .zip(&quest.param.swap_set_param)
        .filter(|((_, &condition), _)| condition != SwapSetCondition::None)
        .map(|((&rate, &condition), &param)| match condition {
            SwapSetCondition::None => unreachable!(),
            SwapSetCondition::QuestTimer => format!("{rate}% {param}min"),
        })
        .collect::<Vec<_>>()
        .join(", ");

    sections.push(Section {
        title: "Basic data".to_owned(),
        content: html!(
            <section id="s-basic">
            <h2 >"Basic data"</h2>
            <div class="mh-kvlist">
            <p class="mh-kv"><span>"Map"</span>
                <span>{ gen_map_label(quest.param.map_no, pedia) }</span></p>
            <p class="mh-kv"><span>"Base time"</span>
                <span>{ text!("{}", quest.param.base_time) }</span></p>
            <p class="mh-kv"><span>"Time variation"</span>
                <span>{ text!("{}", quest.param.time_variation) }</span></p>
            <p class="mh-kv"><span>"Time limit"</span>
                <span>{ text!("{}", quest.param.time_limit) }</span></p>
            <p class="mh-kv"><span>"Carts"</span>
                <span>{ text!("{}", quest.param.quest_life) }</span></p>
            <p class="mh-kv"><span>"Requirement"</span>
                <span>{ text!("{}", requirement) }</span></p>
            <p class="mh-kv"><span>"Target"</span>
                <span>{ target }</span></p>
            <p class="mh-kv"><span>"Reward money"</span>
                <span>{ text!("{}z", quest.param.rem_money) }</span></p>
            <p class="mh-kv"><span>"Reward village point"</span>
                <span>{ text!("{}", quest.param.rem_village_point) }</span></p>
            <p class="mh-kv"><span>"Reward rank point"</span>
                <span>{ text!("{}", quest.param.rem_rank_point) }</span></p>
            <p class="mh-kv"><span>"Is tutorial"</span>
                <span>{ text!("{}", quest.param.is_tutorial) }</span></p>
            <p class="mh-kv"><span>"Auto match HR"</span>
                <span>{ text!("{}", quest.param.auto_match_hr) }</span></p>
            <p class="mh-kv"><span>"Monster swap"</span>
                <span>{ text!("{:?} {}", quest.param.swap_exec_type, swap) }</span></p>
            <p class="mh-kv"><span>"Monster swap prevention"</span>
                <span>{ text!("{:?} {}", quest.param.swap_stop_type, quest.param.swap_stop_param) }</span></p>
            </div>
            </section>
        ),
    });

    if !quest.unlock.is_empty() {
        sections.push(Section {
            title: "Unlock".to_owned(),
            content: html!(<section id="s-unlock"><h2>"Unlock"</h2>
            {
                quest.unlock.iter().map(|unlock|
                match unlock {
                    QuestUnlock::Group(relation) => {
                        html!(<div class="mh-unlock-section">
                            {(!relation.request_group_idx.is_empty()).then(||html!(<div>
                                {text!("Unlock quest group by completing {} of following quests", relation.request_count)}
                                <ul>
                                {relation.request_group_idx.iter().flat_map(|&group_idx| {
                                    if let Some(group) = pedia.quest_unlock.quest_group.get(group_idx as usize) {
                                        group.quest_no_array.iter().map(|q|
                                            if let Some(quest) = pedia_ex.quests.get(q) {
                                                html!(<li>{gen_quest_tag(quest, false, false, None, None)}</li>)
                                            } else {
                                                html!(<li>{text!("Unknown quest {}", q)}</li>)
                                            }
                                        ).collect::<Vec<_>>()
                                    } else {
                                        vec![html!(<li>{text!("Unknown group {}", group_idx)}</li>)]
                                    }
                                })}
                                </ul>
                            </div>))}
                            {text!("Unlock quest group by NPC dialog {:?}", relation.request_talk_flag)}
                        </div>)
                    }
                    QuestUnlock::Talk(talk) => {
                        html!(<div>{text!("Unlock by NPC dialog {}. Auto-clear: {}", talk.talk_flag, talk.is_clear)}</div>)
                    },
                    QuestUnlock::Clear(clear) => {
                        let quest = clear.unlock_quest_no_list.iter().find(|q|q.unlock_quest == quest.param.quest_no).unwrap();
                        html!(<div>{text!("Unlock by clearing following quests. Auto-clear: {}", quest.is_clear)}
                        <ul>{ clear.clear_quest_no_list.iter().map(|q| {
                            if let Some(quest) = pedia_ex.quests.get(q) {
                                html!(<li>{gen_quest_tag(quest, false, false, None, None)}</li>)
                            } else {
                                html!(<li>{text!("Unknown quest {}", q)}</li>)
                            }
                        }) }</ul>
                        </div>)
                    },
                    QuestUnlock::Enemy(enemy) => {
                        // TODO: faster monster lookup
                        let em = pedia.monsters.iter().find(|m|m.enemy_type == Some(enemy.hunt_em_type));
                        let em_tag = if let Some(em) = em {
                            gen_monster_tag(pedia_ex, em.em_type, false, false, None, None)
                        } else {
                            html!(<div>{text!("Unknown monster {}", enemy.hunt_em_type)}</div>)
                        };
                        let rank = match enemy.enemy_rank {
                            EnemyRank::None => "any rank",
                            EnemyRank::Village => "village",
                            EnemyRank::Low => "low rank",
                            EnemyRank::High => "high rank",
                            EnemyRank::Master => "master rank",
                        };
                        html!(<div>
                            {text!("Unlock after hunting in {}", rank)}
                            {em_tag}
                            {text!("Auto-clear: {}", enemy.is_clear)}
                        </div>)
                    }
                })
            }
            </section>),
        });
    }

    if let Some(random) = quest.random_group {
        sections.push(Section {
            title: "Random rotation".to_owned(),
            content: html!(<section id="s-random"><h2>"Random rotation"</h2>
            {
                let self_quest = random.random_group.iter().find(|q|q.random_quest == quest.param.quest_no).unwrap();
                html!(<div>{text!("This is a random quest in the following group. Auto-clear: {}", self_quest.is_clear)}
                <ul>{ random.random_group.iter().map(|q| {
                    let rate = text!("{}% ", q.rate);
                    let trigger = q.is_triger.then(||html!(<span class="tag is-primary">"Key"</span>));
                    if let Some(quest) = pedia_ex.quests.get(&q.random_quest) {
                        html!(<li class="mh-quest-inline">{rate}{gen_quest_tag(quest, false, false, None, None)}
                        {trigger}
                        </li>)
                    } else {
                        html!(<li>{rate}{text!("Unknown quest {}", q.random_quest)}
                        {trigger}
                        </li>)
                    }
                }) }</ul>
                </div>)
            }
            </section>)
        });
    }

    if let Some(servants) = quest.servant {
        sections.push(Section {
            title: "Fixed followers".to_owned(),
            content: html!(<section id="s-follower"><h2>"Fixed followers"</h2>
            <ul> {
                servants.servant_info_list.iter().map(|servant| {
                    let name = pedia_ex.servant.get(&servant.servant_id)
                        .map_or_else(||html!(<span>{text!("{}", servant.servant_id)}</span>),
                        |s|gen_multi_lang(s.name));
                    html!(<li> "NPC: " {name} ", " {
                        text!("Weapon: {}", servant.weapon_type.name())
                    } </li>)
                })
            } </ul>
        </section>),
        });
    }

    if has_target_material {
        sections.push(Section {
            title: "Target material".to_owned(),
            content: html!(<section id="s-target-material">
        <h2 >"Target material"</h2>
        <ul class="mh-item-list">{
        quest.param.tgt_item_id.iter().zip(quest.param.tgt_num.iter())
            .filter(|&(&item, _)| item != ItemId::None)
            .map(|(&item, num)|{
            html!(<li>
                {text!("{}x ", num)}
                <div class="il">{gen_item_label_from_id(item, pedia_ex)}</div>
            </li>)
        })
        }</ul>
        </section>),
        });
    }

    // TODO: fence
    // TODO is_use_pillar

    if has_normal_em {
        sections.push(Section{title: "Monster stats".to_owned(), content:html!(
            <section id="s-stats">
            <h2 >"Monster stats"</h2>
            <div>
                <input type="checkbox" id="mh-non-target-check"/>
                <label for="mh-non-target-check">"Display non-target"</label>
            </div>
            <div>
                <input type="checkbox" id="mh-quest-detail-check"/>
                <label for="mh-quest-detail-check">"More detailed stat"</label>
            </div>
            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Monster"</th>
                    <th>"Size"</th>
                    <th>"HP"</th>
                    <th>"Attack"</th>
                    <th>"Parts"</th>
                    <th class="mh-quest-detail">"Defense"</th>
                    <th class="mh-quest-detail">"Element"</th>
                    <th class="mh-quest-detail">"Stun"</th>
                    <th class="mh-quest-detail">"Exhaust"</th>
                    <th class="mh-quest-detail">"Ride"</th>
                    <th class="mh-quest-detail">"Paralyze"</th>
                    <th class="mh-quest-detail">"Sleep"</th>
                    <th class="mh-quest-detail">"Stamina"</th>
                </tr></thead>
                <tbody> {
                    quest.param.boss_em_type.iter().copied().enumerate()
                    .filter(|&(_, em_type)|em_type != EmTypes::Em(0))
                    .map(|(i, em_type)|{
                        let is_target = quest.param.has_target(em_type);
                        let mystery = quest.enemy_param.and_then(|p|p.individual_type.get(i).cloned());
                        let sub_type = quest.enemy_param.and_then(|p|p.sub_type(i));
                        let class = if !is_target {
                            "mh-non-target"
                        } else {
                            ""
                        };
                        html!(<tr class={class}>
                            <td>{
                                gen_monster_tag(pedia_ex, em_type, is_target, false, mystery, sub_type)
                            }</td>
                            { gen_quest_monster_data(quest.enemy_param, Some(em_type), i, &pedia.difficulty_rate, pedia_ex) }
                        </tr>)
                    })
                } </tbody>
            </table></div>
            </section>
        )});

        sections.push(Section{title: "Multiplayer factor".to_owned(), content:html!(
            <section id="s-multiplayer">
            <h2 >"Multiplayer factor"</h2>

            <div class="mh-table"><table>
                <thead><tr>
                    <th>"Monster"</th>
                    <th>"HP"</th>
                    <th>"Attack"</th>
                    <th>"Parts"</th>
                    <th class="mh-quest-detail">"Other parts"</th>
                    <th class="mh-quest-detail">"Multi parts"</th>
                    <th class="mh-quest-detail">"Defense"</th>
                    <th class="mh-quest-detail">"Element A"</th>
                    <th class="mh-quest-detail">"Element B"</th>
                    <th class="mh-quest-detail">"Stun"</th>
                    <th class="mh-quest-detail">"Exhaust"</th>
                    <th class="mh-quest-detail">"Ride"</th>
                    <th class="mh-quest-detail">"Monster to monster"</th>
                    <th class="mh-quest-detail">"Qurio"</th>
                </tr></thead>
                <tbody> {
                    quest.param.boss_em_type.iter().copied().enumerate()
                    .filter(|&(_, em_type)|em_type != EmTypes::Em(0))
                    .map(|(i, em_type)|{
                        let is_target = quest.param.has_target(em_type);
                        let mystery = quest.enemy_param.and_then(|p|p.individual_type.get(i).cloned());
                        let sub_type = quest.enemy_param.and_then(|p|p.sub_type(i));
                        let class = if !is_target {
                            "mh-non-target"
                        } else {
                            ""
                        };
                        html!(<tr class={class}>
                            <td>{ gen_monster_tag(pedia_ex, em_type, is_target, false, mystery, sub_type)}</td>
                            { gen_quest_monster_multi_player_data(
                                quest.enemy_param, i, pedia) }
                        </tr>)
                    })
                } </tbody>
            </table></div>

            </section>
        )});

        let mut span = 1;
        let init_sets: Vec<_> = quest
            .param
            .boss_em_type
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, em_type)| em_type != EmTypes::Em(0))
            .map(|(i, em_type)| {
                let is_target = quest.param.has_target(em_type);
                let mystery = quest
                    .enemy_param
                    .and_then(|p| p.individual_type.get(i).cloned());
                let sub_type = quest.enemy_param.and_then(|p|p.sub_type(i));
                let init_set_name = quest
                    .enemy_param
                    .as_ref()
                    .and_then(|p| p.init_set_name.get(i))
                    .map(|s| s.as_str());
                let init_set = pedia_ex
                    .monsters
                    .get(&em_type)
                    .and_then(|m| m.data.boss_init_set_data.as_ref())
                    .and_then(|i| {
                        i.stage_info_list
                            .iter()
                            .find(|s| s.map_type == quest.param.map_no)
                    })
                    .and_then(|s| {
                        s.set_info_list
                            .iter()
                            .find(|s| Some(s.set_name.as_str()) == init_set_name)
                    });
                if let Some(init_set) = init_set {
                    span = std::cmp::max(span, init_set.info.iter().filter(|i| i.lot != 0).count());
                }
                let class = if !is_target { "mh-non-target" } else { "" };
                let condition = if let (Some(condition), Some(param)) = (
                    quest.param.boss_set_condition.get(i),
                    quest.param.boss_set_param.get(i),
                ) {
                    match (condition, param) {
                        (BossSetCondition::Default, 0) => text!("Initial"),
                        (BossSetCondition::Free1, 0) => text!("After one hunted (type1)"),
                        (BossSetCondition::Free2, 0) => text!("After one hunted (type2)"),
                        (BossSetCondition::Free3, 0) => text!("After one hunted (type3)"),
                        (BossSetCondition::Timer1, param) => text!("After {} minutes", param),
                        (BossSetCondition::Em1Hp, param) => text!("1st monster {}% hp left", param),
                        (BossSetCondition::Em2Hp, param) => text!("2nd monster {}% hp left", param),
                        (BossSetCondition::Em3Hp, param) => text!("3rd monster {}% hp left", param),
                        (BossSetCondition::Em4Hp, param) => text!("4th monster {}% hp left", param),
                        (BossSetCondition::Em5Hp, param) => text!("5th monster {}% hp left", param),
                        (BossSetCondition::HpEmx1, param) => {
                            text!("One monster {}% hp left", param)
                        }
                        (BossSetCondition::HpEmx2, param) => {
                            text!("Two monster {}% hp left", param)
                        }
                        (BossSetCondition::InitRandom, param) => text!("{}% chance initial", param),
                        (BossSetCondition::SwapRandom, 0) => text!("Random swap"),
                        (condition, param) => text!("{:?}({})", condition, param),
                    }
                } else {
                    text!("-")
                };

                html!(<tr class={class}>
                    <td>{ gen_monster_tag(pedia_ex, em_type, is_target, false, mystery, sub_type)}</td>
                    <td>{ condition }</td>
                    {init_set.into_iter().flat_map(|init_set|
                        init_set.info.iter().filter(|i|i.lot != 0).map(|i|html!(<td> {
                        text!("Area {}, {}%", i.block, i.lot)
                    } </td>)))}
                </tr>)
            })
            .collect();

        sections.push(Section {
            title: "Spawning".to_owned(),
            content: html!(
                    <section id="s-spawning">
                    <h2 >"Spawning"</h2>
                    <div class="mh-table"><table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"Spawning condition"</th>
                        <th colspan={span}>"Initial area"</th>
                    </tr></thead>
                    <tbody>
                    {init_sets}
                    </tbody>
                    </table></div>
                    </section>
            ),
        });
    }

    if let Some(h) = quest.hyakuryu {
        sections.push(Section {
            title: "Rampage information".to_owned(),
            content: html!(
            <section id="s-rampage-info">
            <h2 >"Rampage information"</h2>
            <div class="mh-kvlist">
            <p class="mh-kv"><span>"Attribute"</span>
                <span>{ text!("{}", h.display()) }</span></p>
            <p class="mh-kv"><span>"Base time"</span>
                <span>{ text!("{}", h.base_time) }</span></p>
            <p class="mh-kv"><span>"Map block"</span>
                <span>{ text!("{} - {}", h.start_block_no, h.end_block_no) }</span></p>
            <p class="mh-kv"><span>"Magnamalo appears at wave"</span>
                <span>{ text!("{}", h.extra_em_wave_no) }</span></p>
            <p class="mh-kv"><span>"Magnamalo difficulty table"</span>
                <span>{ text!("{}", h.extra_em_nando_tbl_no) }</span></p>
            <p class="mh-kv"><span>"Apex order table"</span>
                <span>{ text!("{}", h.nushi_order_tbl_no)}</span></p>
            <p class="mh-kv"><span>"Siege weapon unlock table"</span>
                <span>{ text!("{}", h.hm_unlock_tbl_no) }</span></p>
            </div>
            </section>),
        });

        sections.push(Section {
            title: "Rampage tasks".to_owned(),
            content: html!(
            <section id="s-rampage-task">
            <h2>"Rampage tasks"</h2><ul>{
            h.sub_target.iter().enumerate()
            .filter(|(_, target)|**target != QuestTargetType::None)
            .map(|(i, target)| {
                let extra_target = (i == 5).then(
                    ||html!(<span>{
                        text!(" (appears on wave {})", h.sub_target5_wave_no)}
                    </span>));
                let s = match target {
                    QuestTargetType::HuntingMachine => "Install siege weapons",
                    QuestTargetType::DropItem => "Collect drops",
                    QuestTargetType::EmStun => "Stun monsters",
                    QuestTargetType::EmElement => "Apply element blight",
                    QuestTargetType::EmCondition => "Apply status",
                    QuestTargetType::EmCntWeapon => "Repel using weapon",
                    QuestTargetType::EmCntHmBallista  => "Repel using ballista",
                    QuestTargetType::EmCntHmCannon  => "Repel using cannon",
                    QuestTargetType::EmCntHmGatling => "Repel using gatling",
                    QuestTargetType::EmCntHmTrap => "Repel using bomb trap",
                    QuestTargetType::EmCntHmFlameThrower => "Repel using flamethrower",
                    QuestTargetType::EmCntHmNpc => "Repel by NPC",
                    QuestTargetType::EmCntHmDragnator => "Repel using dragonator",
                    QuestTargetType::ExtraEmRunaway => "Repel Magnamalo",
                    x => return html!(<li>{ text!("{}", *x as u8) }{extra_target}</li>)
                };
                html!(<li>{ text!("{}", s) }{extra_target}</li>)
            })
            }</ul></section>),
        });

        sections.push(Section {
            title: "Rampage waves".to_owned(),
            content: html!(
            <section id="s-rampage-wave">
            <h2>"Rampage waves"</h2><div class="mh-table"><table>
            <thead><tr>
                <th>"Boss monster"</th>
                <th>"Sub type"</th>
                <th>"Boss scale table"</th>
                <th>"Other monsters"</th>
                <th>"Other scale table"</th>
                <th>"Order table"</th>
            </tr></thead>
            <tbody> {
                h.wave_data.iter()
                .filter(|wave|wave.boss_em != EmTypes::Em(0))
                .map(|wave| {
                    let sub_type = wave.boss_sub_type as u8;
                    html!(<tr>
                        <td>{ gen_monster_tag(pedia_ex, wave.boss_em, false, false, None, Some(sub_type)) }</td>
                        <td>{text!("{}", wave.boss_sub_type)}</td>
                        <td>{text!("{}", wave.boss_em_nando_tbl_no)}</td>
                        <td><ul class="mh-rampage-em-list"> {
                            wave.em_table.iter().filter(|&&em|em != EmTypes::Em(0))
                            .map(|&em|html!(<li>
                                { gen_monster_tag(pedia_ex, em, false, true, None, None) }
                            </li>))
                        } </ul></td>
                        <td>{text!("{}", wave.wave_em_nando_tbl_no)}</td>
                        <td>{text!("{}", wave.order_table_no)}</td>
                    </tr>)
                })
            } </tbody>
            </table></div>

            </section>),
        });
    }

    if quest.param.supply_tbl != 0 {
        let content = if let Some(supply) = pedia_ex.supply.get(&(quest.param.supply_tbl as i32)) {
            html!(<div><ul class="mh-item-list"> {
                supply.item_id.iter().zip(&supply.num).filter(|(&item, _)| item != ItemId::Null && item != ItemId::None )
                .map(|(item, &num)| {
                    html!(<li>
                        {text!("{}x ", num)}
                        <div class="il">{gen_item_label_from_id(*item, pedia_ex)}</div>
                    </li>)
                })
            } </ul></div>)
        } else {
            html!(<div>{text!("Unknown table {}", quest.param.supply_tbl)}</div>)
        };
        sections.push(Section {
            title: "Supply items".to_owned(),
            content: html!(<section id="s-supply">
            <h2>"Supply items"</h2>
            {content}
            </section>),
        })
    }

    if let Some(arena) = quest.arena {
        sections.push(Section {
            title: "Arena".to_owned(),
            content: html!(
                <section id="s-arena">
                <h2 >"Arena"</h2>
                <div class="mh-kvlist">
                <p class="mh-kv"><span>"Rank time (S/A/B)"</span>
                    <span>{ text!("{}s / {}s / {}s", arena.rank_time_s, arena.rank_time_a, arena.rank_time_b) }</span></p>
                <p class="mh-kv"><span>"Point modifier for rank (S/A)"</span>
                    <span>{ text!("x{} / x{}", arena.rank_point_rate_s, arena.rank_point_rate_a) }</span></p>
                <p class="mh-kv"><span>"Enemy-to-enemy attack"</span>
                    <span>{ text!("{}", arena.em2em_adjust_data) }</span></p>
                //<p class="mh-kv"><span>"dodge_blocking_damage_rate (s/m)"</span>
                //    <span>{ text!("{}/{}", arena.dodge_blocking_damage_rate_s, arena.dodge_blocking_damage_rate_m) }</span></p>
                <p class="mh-kv"><span>"Wall slam damage"</span> // TODO: ?
                    <span>{ text!("{:?}", arena.shoot_wall_hit_damage_rate_list) }</span></p>
                // TODO: what's these?
                //<p class="mh-kv"><span>"fapabtedmhr"</span>
                //    <span>{ text!("{}", arena.final_attack_point_add_by_target_enemy_damage_max_hp_rate) }</span></p>
                //<p class="mh-kv"><span>"start_wait_loop_sub_time_max_hp_rate"</span>
                //    <span>{ text!("{}", arena.start_wait_loop_sub_time_max_hp_rate) }</span></p>
                <p class="mh-kv"><span>"Base gimmick damage"</span>
                    <span>{ text!("{}", arena.base_gimmik_damage) }</span></p>
                </div>

                { arena.arena_pl.iter().enumerate().map(|(i, pl)|{
                    let weapon_control_ref = |name: &str| {
                        pedia.weapon_control.get_entry(name).or_else(||pedia.weapon_control_mr.get_entry(name))
                    };

                    let weapon_action = |actions: &[i32]| {
                        html!(<ul class="mh-arena-switch-skill">{actions.iter().map(|action|{
                            if let Some(skill) = pedia_ex.switch_skills.get(action) {
                                html!(<li>{gen_multi_lang_with_ref(skill.name, weapon_control_ref)}</li>)
                            } else {
                                html!(<li>{text!("Unknown skill {:?}", action)}</li>)
                            }
                        })}</ul>)
                    };

                    let deco_label_li = |id: DecorationsId| {
                        // TODO: fast lookup table
                        for (&skill_id, skill) in &pedia_ex.skills {
                            for deco in &skill.decos {
                                if deco.data.id == id {
                                    return html!(<li>
                                        <a href={format!("skill/{}", skill_page(skill_id))}>
                                        { gen_deco_label(deco) }
                                        </a>
                                    </li>)
                                }
                            }
                        }
                        html!(<li>{text!("Unknown deco {:?}", id)}</li>)
                    };

                    let armor_label_td = |id: PlArmorId| {
                        // TODO: fast lookup table
                        for series in pedia_ex.armors.values() {
                            for piece in series.pieces.iter().flatten() {
                                if piece.data.pl_armor_id == id {
                                    return html!(<td><a href={format!("armor/{:03}.html", series.series.armor_series.0)}>
                                        {gen_armor_label(Some(piece))}
                                    </a></td>)
                                }
                            }
                        }
                        html!(<td>{text!("Unknown armor {:?}", id)}</td>)
                    };

                    let items_list = |items: &[ItemWork]| {
                        html!(<ul class="mh-item-list-arena-set"> {
                            items.iter().filter(|item_work| item_work.item != ItemId::Null && item_work.item != ItemId::None )
                            .map(|item_work| {
                                html!(<li>
                                    {text!("{}x ", item_work.num)}
                                    <div class="il">{gen_item_label_from_id(item_work.item, pedia_ex)}</div>
                                </li>)
                            })
                        } </ul>)
                    };

                    let buff_cage = if let Some(buff_cage) = pedia_ex.buff_cage.get(&pl.lv_buff_cage_id) {
                        html!(<td>{gen_buff_cage_label(buff_cage)}</td>)
                    } else {
                        html!(<td>{text!("Unknown {:?}", pl.lv_buff_cage_id)}</td>)
                    };

                    html!(<section class="mh-arena-set">
                    <h3>{text!("Set {}", i + 1)}</h3>
                    <div class="mh-table"><table>
                    <thead><tr>
                        <th/>
                        <th>"Equipment"</th>
                        <th>"Detail"</th>
                        <th>"Decortion"</th>
                    </tr></thead>
                    <tbody>

                    <tr>
                        <td>"Weapon"</td>
                        <td>{ gen_weapon_label_from_id(pedia_ex, pl.wep_id) }</td>
                        <td><ul class="mh-armor-skill-list">{ pl.hyakuryu_skill.iter()
                            .filter(|&&s|s != PlHyakuryuSkillId::None).map(|s|
                            if let Some(skill) = pedia_ex.hyakuryu_skills.get(s) {
                                html!(<li>{gen_hyakuryu_skill_label(skill)}</li>)
                            } else {
                                html!(<li>{text!("Unknown skill {:?}", s)}</li>)
                            }
                        ) }
                        { (pl.wep_action2.is_empty()).then(|| {
                            html!(<li>"Switch skills: "{weapon_action(&pl.wep_action)}</li>)
                        }) }
                        { (!pl.wep_action2.is_empty()).then(|| {
                            [html!(<li>"Switch skills A: "{weapon_action(&pl.wep_action)}</li>),
                            html!(<li>"Switch skills B: "{weapon_action(&pl.wep_action2)}</li>)]
                        }).into_iter().flatten() }
                        </ul></td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_wep.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Helm"</td>
                        { armor_label_td(pl.armor_helm) }
                        <td>{ text!("Lv{}", pl.armor_lv_helm) }</td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_helm.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Chest"</td>
                        { armor_label_td(pl.armor_body) }
                        <td>{ text!("Lv{}", pl.armor_lv_body) }</td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_body.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Arm"</td>
                        { armor_label_td(pl.armor_arm) }
                        <td>{ text!("Lv{}", pl.armor_lv_arm) }</td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_arm.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Waist"</td>
                        { armor_label_td(pl.armor_waist) }
                        <td>{ text!("Lv{}", pl.armor_lv_waist) }</td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_waist.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Leg"</td>
                        { armor_label_td(pl.armor_leg) }
                        <td>{ text!("Lv{}", pl.armor_lv_leg) }</td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_leg.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Petalace"</td>
                        {buff_cage}
                        <td/>
                        <td/>
                    </tr>
                    <tr>
                        <td>"Talisman"</td>
                        <td/>
                        <td><ul class="mh-armor-skill-list">{
                            pl.talisman_skill.iter().filter(|s|s.id != PlEquipSkillId::None).map(|s|
                            gen_skill_lv_label(pedia_ex, s.id, s.lv)
                        ) }</ul></td>
                        <td><ul class="mh-armor-skill-list">{
                            pl.deco_talisman.iter().filter(|&&d|d != DecorationsId::None).map(|&d|deco_label_li(d))
                        }</ul></td>
                    </tr>
                    <tr>
                        <td>"Pouch"</td>
                        <td colspan=3>{items_list(&pl.pouch)}</td>
                    </tr>
                    <tr>
                        <td>"Gunner pouch"</td>
                        <td colspan=3>{items_list(&pl.ganner_pouch)}</td>
                    </tr>
                    </tbody>
                    </table></div>

                    </section>)
                }) }

                </section>
            ),
        });
    }

    sections.push(Section {
        title: "Rewards".to_owned(),
        content: html!(
            <section id="s-reward">
            <h2 >"Rewards"</h2>
            { if let Some(reward) = &quest.reward {
                html!(<div>
                <p>{text!("Addtional target rewards: {}", reward.param.target_reward_add_num)}</p>
                <p>{text!("Addtional quest rewards: {}", reward.param.common_material_add_num)}</p>
                <p>"See monster's page for target rewards."</p>
                <div class="mh-reward-tables">

                { if let Some(common_material_reward) = &reward.common_material_reward {
                    html!(<div class="mh-reward-box">
                    <div class="mh-table"><table>
                        <thead><tr>
                            <th>"Quest rewards"<br/>{
                                translate_rule(common_material_reward.lot_rule)
                            }</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &common_material_reward.item_id_list,
                                &common_material_reward.num_list,
                                &common_material_reward.probability_list)
                        } </tbody>
                    </table></div>
                    </div>)
                } else {
                    html!(<div></div>)
                }}

                { if let Some(additional_target_reward) = reward.additional_target_reward {
                    html!(<div class="mh-reward-box">
                    <div class="mh-table"><table>
                        <thead><tr>
                            <th>"Addtional target rewards"<br/>{
                                translate_rule(additional_target_reward.lot_rule)
                            }</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &additional_target_reward.item_id_list,
                                &additional_target_reward.num_list,
                                &additional_target_reward.probability_list)
                        } </tbody>
                    </table></div>
                    </div>)
                } else {
                    html!(<div></div>)
                }}

                { reward.additional_quest_reward.iter().map(|additional_quest_reward| {
                    html!(<div class="mh-reward-box">
                    <div class="mh-table"><table>
                        <thead><tr>
                            <th>"Quest bonus rewards"<br/>{
                                translate_rule(additional_quest_reward.lot_rule)
                            }</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &additional_quest_reward.item_id_list,
                                &additional_quest_reward.num_list,
                                &additional_quest_reward.probability_list)
                        } </tbody>
                    </table></div>
                    </div>)
                })}

                { if let Some(cloth_ticket) = &reward.cloth_ticket {
                    html!(<div class="mh-reward-box">
                    <div class="mh-table"><table>
                        <thead><tr>
                            <th>"Outfit voucher"<br/>{translate_rule(cloth_ticket.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &cloth_ticket.item_id_list,
                                &cloth_ticket.num_list,
                                &cloth_ticket.probability_list)
                        } </tbody>
                    </table></div>
                    </div>)
                } else {
                    html!(<div></div>)
                }}

                {
                    quest.time_attack_reward.iter().map(|ta| {
                        let rank = match ta.rank.rank {
                            RewardRank::RankSS => "SS",
                            RewardRank::RankS => "S",
                            RewardRank::RankA => "A",
                            RewardRank::RankB => "B",
                        };
                        html!(<div class="mh-reward-box">
                        <div class="mh-table"><table>
                        <thead><tr>
                            <th>{text!("Reward for rank {} ({}s)", rank, ta.rank.clear_time)}
                                <br/>{translate_rule(ta.reward.lot_rule)}</th>
                            <th>"Probability"</th>
                        </tr></thead>
                        <tbody> {
                            gen_reward_table(pedia_ex,
                                &ta.reward.item_id_list,
                                &ta.reward.num_list,
                                &ta.reward.probability_list)
                        } </tbody>
                        </table></div>
                        </div>)
                    })
                }

                </div>
                </div>)
            } else {
                html!(<div>"No data"</div>)
            }}
            </section>
        ),
    });

    let plain_title = format!("Quest {:06}", quest.param.quest_no);
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{}", plain_title)}</title>
                { head_common(hash_store, quest_path) }
                { quest.name.iter().flat_map(|&name|title_multi_lang(name)) }
                { open_graph(quest.name, &plain_title,
                    quest.target, "", Some(&img), toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections, toc_sink.path()) }
                <main>
                <header class="mh-quest-header">
                    <div class="mh-title-icon">
                        <img alt="Quest icon" src={img} class="mh-quest-icon"/>
                    </div>
                    <h1>
                    {quest_level_tag(quest)}
                    {
                        quest.is_dl.then(
                            ||html!(<span class="tag mh-quest-event">{text!("Event")}</span>)
                        )
                    }
                    {
                        quest.param.is_servant_request().then(
                            ||html!(<span class="tag mh-quest-follower">{text!("Follower")}</span>)
                        )
                    }
                    {
                        quest.param.is_kingdom().then(
                            ||html!(<span class="tag mh-quest-follower">{text!("Survey")}</span>)
                        )
                    }
                    {
                        quest.param.is_from_npc.then(
                            ||html!(<span class="tag mh-quest-npc">{text!("NPC")}</span>)
                        )
                    }
                    {
                        quest.name.map_or(
                            html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                            gen_multi_lang
                        )
                    }</h1>
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

pub fn gen_random_mystery_difficulty(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    category: usize,
    kind: usize,
    table: &RandomMysteryDifficultyRateKindData,
    quest_path: &impl Sink,
) -> Result<()> {
    let mut output =
        quest_path.create_html(&format!("anomaly_difficulty_{category}_{kind}.html"))?;

    let difficulty_rate_anomaly =
        if let Some(difficulty_rate_anomaly) = &pedia.difficulty_rate_anomaly {
            difficulty_rate_anomaly
        } else {
            return Ok(());
        };
    let doc: DOMTree<String> = html!(
        <html lang="en">
        <head itemscope=true>
            <title>{text!("Anomaly investigation stat table")}</title>
            { head_common(hash_store, quest_path) }
        </head>
        <body>
            { navbar() }
            <main>
            <header><h1>{
                let category = match category {
                    0 => "afflicted monster",
                    1 => "non-afflicted monster",
                    _ => "?",
                };
                text!("Anomaly investigation stats table {} for {}", kind, category)
            }</h1></header>

            <div>
                <input type="checkbox" id="mh-quest-detail-check"/>
                <label for="mh-quest-detail-check">"More detailed stat"</label>
            </div>
            <div class="mh-table"><table>
            <thead><tr>
                <th>"Level"</th>
                <th>"HP"</th>
                <th>"Attack"</th>
                <th>"Parts"</th>
                <th class="mh-quest-detail">"Defense"</th>
                <th class="mh-quest-detail">"Element"</th>
                <th class="mh-quest-detail">"Stun"</th>
                <th class="mh-quest-detail">"Exhaust"</th>
                <th class="mh-quest-detail">"Ride"</th>
                <th class="mh-quest-detail">"Paralyze"</th>
                <th class="mh-quest-detail">"Sleep"</th>
                <th class="mh-quest-detail">"Stamina"</th>
            </tr></thead>
            <tbody>
            {
                table.ref_table.ref_rate_table.iter().enumerate().map(|(level, diff)| {
                    html!(<tr>
                        <td>{text!("{}", level + 1)}</td>
                        {
                            gen_quest_monster_data(Some(diff), None, 0, difficulty_rate_anomaly, pedia_ex)
                        }
                    </tr>)
                })
            }
            </tbody>
            </table></div>
            <section>
            <h2>"Monsters that use this table"</h2>
            <ul class="mh-item-list">
            {
                pedia_ex.monsters.iter().filter_map(|(&em, monster)| {
                    let random_quest = if let Some(random_quest) = &monster.random_quest {
                        random_quest
                    } else {
                        return None
                    };
                    if category == 0 &&
                        usize::try_from(random_quest.difficulty_table_type) == Ok(kind) {
                        return Some(em)
                    }
                    if category == 1 &&
                        usize::try_from(random_quest.difficulty_table_type_extra) == Ok(kind) {
                        return Some(em)
                    }
                    None
                }).map(|em_type| {
                    html!(<li>{gen_monster_tag(pedia_ex, em_type, false, false, None, None)}</li>)
                })
            }
            </ul>
            </section>
            </main>
            { right_aside() }
        </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

pub fn gen_quests(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let quest_path = output.sub_sink("quest")?;
    for quest in pedia_ex.quests.values() {
        gen_quest(hash_store, quest, pedia, pedia_ex, config, &quest_path, toc)?;
    }

    if let Some(table) = &pedia.random_mystery_difficulty {
        for (category, t) in table.nand_data.iter().enumerate() {
            for (kind, t) in t.nand_kinds_data.iter().enumerate() {
                gen_random_mystery_difficulty(
                    hash_store,
                    pedia,
                    pedia_ex,
                    category,
                    kind,
                    t.nando_ref_table.unwrap(),
                    &quest_path,
                )?
            }
        }
    }

    Ok(())
}

pub fn gen_npc_mission_tag(mission: &NpcMission) -> Box<div<String>> {
    html!(<div>
        <a href={format!("villager_request/{:03}.html", mission.param.id)}>
        {gen_multi_lang(mission.name)}
        </a>
    </div>)
}

pub fn gen_npc_mission_list(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    output: &impl Sink,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Villager requests - MHRice")}</title>
                { head_common(hash_store, output) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Villager requests"</h1></header>
                <div>
                    <a href="quest.html"><span class="icon-text">
                    <span class="icon">
                    <i class="fas fa-arrow-right"></i>
                    </span>
                    <span>"go to main quests"</span>
                    </span></a>
                </div>
                /*<div class="select"><select id="scombo-npcmission" class="mh-scombo">
                    <option value="0">"Sort by internal ID"</option>
                    <option value="1">"Sort by in-game order"</option>
                </select></div>*/ // two has the same order now
                <ul class="mh-quest-list" id="slist-npcmission"> {
                    pedia_ex.npc_missions.values().map(|mission|{
                        // is the index actually the in-game order?
                        let sort_tag = format!("{},{}", mission.param.id, mission.param.index);
                        html!{<li data-sort=sort_tag>
                            { gen_npc_mission_tag(mission) }
                        </li>}
                    })
                }</ul>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("villager_request.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_npc_mission(
    hash_store: &HashStore,
    mission: &NpcMission,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    mission_path: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let (mut output, mut toc_sink) =
        mission_path.create_html_with_toc(&format!("{:03}.html", mission.param.id), toc)?;

    toc_sink.add(mission.name);

    let mut sections = vec![];

    sections.push(Section {
        title: "Description".to_owned(),
        content: html!(
            <section id="s-description">
            <h2 >"Description"</h2>
            <p><span>"Objective: "</span><span> {
                if let Some(target) = mission.target {
                    gen_multi_lang(target)
                } else {
                    html!(<span>"-"</span>)
                }
            }</span></p>
            <p><span>"From: "</span><span> {
                gen_multi_lang(mission.requester)
            }</span></p>
            <p><span>"Reward: "</span><span>{
                if let Some(reward) = mission.reward {
                    gen_multi_lang(reward)
                } else {
                    html!(<span>"-"</span>)
                }
            }</span></p>
            <p><span>"Detail: "</span></p><div><pre>{
                gen_multi_lang(mission.detail)
            }</pre></div>
            // <div>{text!("{:?}", mission.param)}</div>
            </section>
        ),
    });

    sections.push(Section {
        title: "Target monster".to_owned(),
        content: html!(
            <section id="s-monster">
            <h2 >"Target monster"</h2>
            {
                (mission.param.map_no != -1).then(||html!(
                    <div>"Map: " {gen_map_label(mission.param.map_no, pedia) }</div>
                ))
            }
            {
                (mission.param.em_type != EmTypes::Em(0)).then(||html!(<div>
                    {text!("{}x", mission.param.tgt_num.first().copied().unwrap_or(0))}
                    <div class="il">{ gen_monster_tag(pedia_ex, mission.param.em_type, false, false, None, None) }</div>
                    {match mission.param.rank {
                            MissionRank::Any => None,
                            MissionRank::Low => Some(text!(" (low rank)")),
                            MissionRank::High => Some(text!(" (high rank)")),
                            MissionRank::Master => Some(text!(" (master rank)")),
                     }}
                </div>))
            }
            </section>
        ),
    });
    sections.push(Section {
        title: "Target material".to_owned(),
        content: html!(
            <section id="s-material">
            <h2 >"Target material"</h2>
            <ul class="mh-item-list">{
                mission.param.item_id.iter().zip(mission.param.tgt_num.iter())
                    .filter(|&(&item, _)| item != ItemId::None)
                    .map(|(&item, num)|{
                    html!(<li>
                        {text!("{}x ", num)}
                        <div class="il">{gen_item_label_from_id(item, pedia_ex)}</div>
                    </li>)
                })
                }</ul>
            </section>
        ),
    });

    macro_rules! check_weapon {
        ($weapon:ident) => {{
            let weapons = &pedia_ex.$weapon;
            weapons
                .weapons
                .values()
                .filter_map(|w| {
                    let mut tags = vec![];
                    if w.product
                        .and_then(|p| pedia_ex.progress.get(&p.base.progress_flag))
                        .map(|p| p.talk_flag)
                        == Some(mission.param.end_flag)
                    {
                        tags.push(html!(<span class="tag">"Forge"</span>));
                    }

                    if w.process
                        .and_then(|p| pedia_ex.progress.get(&p.base.progress_flag))
                        .map(|p| p.talk_flag)
                        == Some(mission.param.end_flag)
                    {
                        tags.push(html!(<span class="tag">"Upgrade"</span>));
                    }

                    if w.change
                        .and_then(|p| pedia_ex.progress.get(&p.base.progress_flag))
                        .map(|p| p.talk_flag)
                        == Some(mission.param.end_flag)
                    {
                        tags.push(html!(<span class="tag">"Rampage layered"</span>));
                    }

                    if w.overwear_product
                        .and_then(|p| pedia_ex.progress.get(&p.progress_flag))
                        .map(|p| p.talk_flag)
                        == Some(mission.param.end_flag)
                    {
                        tags.push(html!(<span class="tag">"Layered"</span>));
                    }

                    if tags.is_empty() {
                        None
                    } else {
                        Some((w, tags))
                    }
                })
                .map(|(w, tags)| html!(<li>{gen_weapon_label(w)}{tags}</li>))
        }};
    }

    sections.push(Section {
        title: "Unlock".to_owned(),
        content: html!(
            <section id="s-unlock">
            <h2 >"Unlock"</h2>
            <div>"(Not an exhaustive list)"</div>
            <ul class="mh-item-list">
            {check_weapon!(great_sword)}
            {check_weapon!(short_sword)}
            {check_weapon!(hammer)}
            {check_weapon!(lance)}
            {check_weapon!(long_sword)}
            {check_weapon!(slash_axe)}
            {check_weapon!(gun_lance)}
            {check_weapon!(dual_blades)}
            {check_weapon!(horn)}
            {check_weapon!(insect_glaive)}
            {check_weapon!(charge_axe)}
            {check_weapon!(light_bowgun)}
            {check_weapon!(heavy_bowgun)}
            {check_weapon!(bow)}

            {pedia_ex.armors.values().flat_map(|a|&a.pieces).flatten().filter_map(|p| {
                let mut tags = vec![];
                if p.product.and_then(|p|pedia_ex.progress.get(&p.progress_flag)).map(|p|p.talk_flag)
                    == Some(mission.param.end_flag) {
                    tags.push(html!(<span class="tag">"Crafting"</span>))
                }
                if p.overwear_product.and_then(|p|pedia_ex.progress.get(&p.progress_flag)).map(|p|p.talk_flag)
                    == Some(mission.param.end_flag) {
                    tags.push(html!(<span class="tag">"Layered"</span>))
                }
                if tags.is_empty() {
                    None
                } else {
                    Some((p, tags))
                }
            }).map(|(p, tags)|html!(<li>
                <a class="il" href={format!("armor/{:03}.html", p.data.series.0)}>
                {gen_armor_label(Some(p))}
                </a>
            {tags}</li>))}

            {pedia_ex.ot_equip.values().filter(|ot|pedia_ex.progress.get(&ot.series.unlock_progress).map(|p|p.talk_flag)
                == Some(mission.param.end_flag)).flat_map(|ot| {
                    let mut items = vec![];
                    let href = format!("otomo/{}.html", ot.series.id.to_tag());
                    if let Some(weapon) = &ot.weapon {
                        items.push(html!(<li><a href={&href}>{gen_atomo_weapon_label(weapon)}</a></li>))
                    }
                    if let Some(head) = &ot.head {
                        items.push(html!(<li><a href={&href}>{gen_atomo_armor_label(head)}</a></li>))
                    }
                    if let Some(chest) = &ot.chest {
                        items.push(html!(<li><a href={&href}>{gen_atomo_armor_label(chest)}</a></li>))
                    }
                    items
                })
            }
            </ul>
            </section>
        ),
    });

    let plain_title = format!("Villager request {:03}", mission.param.id);
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("{}", plain_title)}</title>
                { head_common(hash_store, mission_path) }
                { title_multi_lang(mission.name)}
                { open_graph(Some(mission.name), &plain_title,
                    Some(mission.detail), "", None, toc_sink.path(), config) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections, toc_sink.path()) }
                <main>
                <header class="mh-quest-header">
                    <h1> { gen_multi_lang(mission.name) } </h1>
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

pub fn gen_npc_missions(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let mission_path = output.sub_sink("villager_request")?;
    for mission in pedia_ex.npc_missions.values() {
        gen_npc_mission(
            hash_store,
            mission,
            pedia,
            pedia_ex,
            config,
            &mission_path,
            toc,
        )?;
    }

    Ok(())
}
