use super::gen_common::*;
use super::gen_item::*;
use super::gen_map::*;
use super::gen_monster::*;
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
        QuestLevel::QL7Ex => "7+",
    };
    let level_tag_class = format!("tag {quest_level_tag}");
    html!(<span class={level_tag_class.as_str()}>{
        text!("{}{}", name, level)}</span>)
}

pub fn gen_quest_tag(
    quest: &Quest,
    tag_level: bool,
    is_target: bool,
    is_mystery: bool,
) -> Box<div<String>> {
    let img = format!(
        "/resources/questtype_{}.png",
        quest.param.quest_type.icon_index()
    );
    html!(<div>
        <a href={format!("/quest/{:06}.html", quest.param.quest_no)} class="mh-icon-text">
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
        {quest.name.map_or(
            html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
            gen_multi_lang
        )}
        {is_target.then(||html!(<span class="tag is-primary">"Target"</span>))}
        {is_mystery.then(||html!(<span class="tag is-danger">"Afflicted"</span>))}
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

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Quests - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Quests"</h1></header>
                {
                    quests_ordered.into_iter().map(|(enemy_level, quests)|{
                        html!(<section>
                        <h2>{text!("{:?}", enemy_level)}</h2>
                        { quests.into_iter().map(|(quest_level, quests)|{
                            html!(
                                <section class="mh-quest-list">
                                    <h3>{text!("{:?}", quest_level)}</h3>
                                    <ul class="mh-quest-list">{
                                        quests.into_iter().map(|quest|{
                                            html!{<li>
                                                { gen_quest_tag(quest, false, false, false) }
                                            </li>}
                                        })
                                    }</ul>
                                </section>
                            )
                        })}</section>)
                    })
                }
                <section>
                <h2>"Anomaly quests"</h2>
                {
                    anomaly_ordered.into_iter().map(|(quest_level, quests)|{
                        html!(
                            <section class="mh-quest-list">
                                <h3>{text!("A{}", quest_level)}</h3>
                                <ul class="mh-quest-list">{
                                    quests.into_iter().map(|quest|{
                                        html!{<li>
                                            { gen_quest_tag(quest, false, false, false) }
                                        </li>}
                                    })
                                }</ul>
                            </section>
                        )
                    })
                }
                </section>
                </main>
                { right_aside() }
            </body>
        </html>
    );

    output
        .create_html("quest.html")?
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
                    <img class="mh-crown-icon" alt="Small crown" src="/resources/small_crown.png" />
                    {text!("{}%", small_chance)}
                </span>)
                });

                let large = (large_chance != 0).then(|| {
                    html!(<span class="mh-crown">
                    <img class="mh-crown-icon" alt="Large crown" src="/resources/king_crown.png" />
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
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.vital_rate));
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
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.attack_rate))
        },
    );
    let parts = enemy_param.parts_tbl(index).map_or_else(
        || "-".to_owned(),
        |v| {
            difficulty_rate
                .parts_rate_table_list
                .get(usize::from(v))
                .map_or_else(
                    || format!("~ {}", v),
                    |r| format!("x{}", r.parts_vital_rate),
                )
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
        .map_or_else(|| "-".to_owned(), |v| format!("{}", v));

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
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    if let Some(title) = quest.name {
        toc_sink.add(title);
    }

    let has_normal_em = quest
        .param
        .boss_em_type
        .iter()
        .any(|&em_type| em_type != EmTypes::Em(0));
    let img = format!(
        "/resources/questtype_{}.png",
        quest.param.quest_type.icon_index()
    );

    let target = quest
        .param
        .target_type
        .iter()
        .filter(|&&t| t != QuestTargetType::None)
        .map(|t| match t {
            QuestTargetType::ItemGet => "Collect".to_owned(),
            QuestTargetType::Hunting => "Hunt".to_owned(),
            QuestTargetType::Kill => "Slay".to_owned(),
            QuestTargetType::Capture => "Capture".to_owned(),
            QuestTargetType::AllMainEnemy => "Hunt all".to_owned(),
            QuestTargetType::EmTotal => "Hunt small monsters".to_owned(),
            QuestTargetType::FinalBarrierDefense => "Defend final barrier".to_owned(),
            QuestTargetType::FortLevelUp => "Level up fort".to_owned(),
            QuestTargetType::PlayerDown => "PlayerDown".to_owned(),
            QuestTargetType::FinalBoss => "Final boss".to_owned(),
            x => format!("{:?}", x),
        })
        .collect::<Vec<String>>()
        .join(", ");

    let requirement = quest
        .param
        .order_type
        .iter()
        .filter(|&&t| t != QuestOrderType::None)
        .map(|t| format!("{:?}", t))
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
                <span>{ text!("{}", target) }</span></p>
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
            </div>
            </section>
        ),
    });

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
            let item = if let Some(item) = pedia_ex.items.get(&item) {
                html!(<div class="il">{gen_item_label(item)}</div>)
            } else {
                html!(<div class="il">{text!("{:?}", item)}</div>)
            };
            html!(<li>
                {text!("{}x ", num)}
                {item}
            </li>)
        })
        }</ul>
        </section>),
        });
    }

    // TODO: monster spawn/swap behavior
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
                        let is_mystery = quest.enemy_param
                            .and_then(|p|p.individual_type.get(i))
                            .map(|&t|t == EnemyIndividualType::Mystery)
                            .unwrap_or(false);
                        let class = if !is_target {
                            "mh-non-target"
                        } else {
                            ""
                        };
                        html!(<tr class={class}>
                            <td>{
                                gen_monster_tag(pedia_ex, em_type, is_target, false, is_mystery)
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
                        let is_mystery = quest.enemy_param
                            .and_then(|p|p.individual_type.get(i))
                            .map(|&t|t == EnemyIndividualType::Mystery)
                            .unwrap_or(false);
                        let class = if !is_target {
                            "mh-non-target"
                        } else {
                            ""
                        };
                        html!(<tr class={class}>
                            <td>{ gen_monster_tag(pedia_ex, em_type, is_target, false, is_mystery)}</td>
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
                let is_mystery = quest
                    .enemy_param
                    .and_then(|p| p.individual_type.get(i))
                    .map(|&t| t == EnemyIndividualType::Mystery)
                    .unwrap_or(false);
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
                    <td>{ gen_monster_tag(pedia_ex, em_type, is_target, false, is_mystery)}</td>
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
                    html!(<tr>
                        <td>{ gen_monster_tag(pedia_ex, wave.boss_em, false, false, false) }</td>
                        <td>{text!("{}", wave.boss_sub_type)}</td>
                        <td>{text!("{}", wave.boss_em_nando_tbl_no)}</td>
                        <td><ul class="mh-rampage-em-list"> {
                            wave.em_table.iter().filter(|&&em|em != EmTypes::Em(0))
                            .map(|&em|html!(<li>
                                { gen_monster_tag(pedia_ex, em, false, true, false) }
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
                    let item = if let Some(item) = pedia_ex.items.get(item) {
                        html!(<div class="il">{gen_item_label(item)}</div>)
                    } else {
                        html!(<div class="il">{text!("{:?}", item)}</div>)
                    };
                    html!(<li>
                        {text!("{}x ", num)}
                        {item}
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
                            <th>"Addtional rewards"<br/>{
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

                </div>
                </div>)
            } else {
                html!(<div>"No data"</div>)
            }}
            </section>
        ),
    });

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Quest {:06}", quest.param.quest_no)}</title>
                { head_common(hash_store) }
                { quest.name.iter().flat_map(|&name|title_multi_lang(name)) }
            </head>
            <body>
                { navbar() }
                { gen_menu(&sections) }
                <main>
                <header>
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
    mut output: impl Write,
) -> Result<()> {
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
            { head_common(hash_store) }
        </head>
        <body>
            { navbar() }
            <main>
            <header><h1>{
                let category = match category {
                    0 => "main target",
                    1 => "sub target",
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
                    html!(<li>{gen_monster_tag(pedia_ex, em_type, false, false, false)}</li>)
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
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let quest_path = output.sub_sink("quest")?;
    for quest in pedia_ex.quests.values() {
        let (path, toc_sink) =
            quest_path.create_html_with_toc(&format!("{:06}.html", quest.param.quest_no), toc)?;
        gen_quest(hash_store, quest, pedia, pedia_ex, path, toc_sink)?;
    }

    if let Some(table) = &pedia.random_mystery_difficulty {
        for (category, t) in table.nand_data.iter().enumerate() {
            for (kind, t) in t.nand_kinds_data.iter().enumerate() {
                let output = quest_path
                    .create_html(&format!("anomaly_difficulty_{}_{}.html", category, kind))?;
                gen_random_mystery_difficulty(
                    hash_store,
                    pedia,
                    pedia_ex,
                    category,
                    kind,
                    t.nando_ref_table.unwrap(),
                    output,
                )?
            }
        }
    }

    Ok(())
}
