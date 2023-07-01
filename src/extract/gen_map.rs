use super::gen_common::*;
use super::gen_item::*;
use super::gen_monster::*;
use super::gen_pedia::ITEM_ICON_SPECIAL_COLOR;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::prepare_map::*;
use super::sink::*;
use crate::msg::*;
use crate::rsz;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

fn map_page(id: i32) -> String {
    format!("{id:02}.html")
}

pub fn get_map_name(id: i32, pedia: &Pedia) -> Option<&MsgEntry> {
    // another brilliant idea from crapcom
    let name_id = match id {
        12 => 31,
        13 => 32,
        14 => 41,
        15 => 42,
        id => id,
    };

    let name_name = format!("Stage_Name_{name_id:02}");
    pedia
        .map_name
        .get_entry(&name_name)
        .or_else(|| pedia.map_name_mr.get_entry(&name_name))
}

pub fn gen_map_label(id: i32, pedia: &Pedia) -> Box<a<String>> {
    let link = format!("map/{}", map_page(id));
    let name = get_map_name(id, pedia);
    if let Some(name) = name {
        html!(<a href={link}>{ gen_multi_lang(name) }</a>)
    } else {
        html!(<a href={link}>{ text!("Map {:02}", id) }</a>)
    }
}

// This is unfortunately hardcoded in the game code
// So let's also hard code it here
//
// class snow.stage.StageDef {
//     snow.data.ContentsIdSystem.ItemId getFishItemId(snow.stage.StageDef.FishId);
// }
pub fn get_fish_item_id(fish_id: i32) -> Option<rsz::ItemId> {
    match fish_id {
        0 => Some(rsz::ItemId::Normal(0x1f3)),
        1 => Some(rsz::ItemId::Normal(0x30e)),
        2 => Some(rsz::ItemId::Normal(0x1d6)),
        3 => Some(rsz::ItemId::Normal(0x30f)),
        4 => Some(rsz::ItemId::Normal(0x310)),
        5 => Some(rsz::ItemId::Normal(0x311)),
        6 => Some(rsz::ItemId::Normal(0x312)),
        7 => Some(rsz::ItemId::Normal(0x313)),
        8 => Some(rsz::ItemId::Normal(0x314)),
        9 => Some(rsz::ItemId::Normal(0x204)),
        10 => Some(rsz::ItemId::Normal(0x205)),
        11 => Some(rsz::ItemId::Normal(0x315)),
        12 => Some(rsz::ItemId::Normal(0x316)),
        13 => Some(rsz::ItemId::Normal(0x317)),
        14 => Some(rsz::ItemId::Normal(0x318)),
        15 => Some(rsz::ItemId::Normal(0x319)),
        16 => Some(rsz::ItemId::Normal(0x31a)),
        17 => Some(rsz::ItemId::Normal(0x31b)),
        18 => Some(rsz::ItemId::Normal(0x367)),
        _ => None,
    }
}

// class snow.gui.SnowGuiCommonUtility.Icon {
//     snow.gui.SnowGuiCommonUtility.Icon.ItemIconPatternNo getEcIconState(snow.envCreature.EnvironmentCreatureType type);
// }
pub fn get_ec_icon_pattern(ec_type: i32) -> i32 {
    match ec_type {
        0 => 0x61,
        1 | 2 | 3 | 4 => 0x56,
        5 => 0x50,
        6 => 0x51,
        7 => 0x52,
        8 => 0x53,
        9 | 0x1b | 0x3e | 0x3f => 0x54,
        10 | 0xb | 0xc | 0xd => 0x5e,
        0xe => 0x5d,
        0xf | 0x10 | 0x11 | 0x12 => 0x55,
        0x13 => 0x6b,
        0x14 | 0x15 => 0x6d,
        0x16 => 0x6c,
        0x17 => 0x57,
        0x18 => 0x59,
        0x19 | 0x40 => 0x5b,
        0x1a => 0x58,
        0x1c => 0x5f,
        0x1d => 0x60,
        0x1e | 0x1f => 0x62,
        0x22 | 0x23 | 0x24 | 0x25 => 0x5a,
        0x26 => 0x69,
        0x27 => 0x6a,
        0x2d => 0x67,
        0x2e => 0x68,
        0x2f => 0x5c,
        0x30 => 0x65,
        0x31 => 0x64,
        0x32 => 0x63,
        0x33 => 0x66,
        0x39 => 0xfa,
        0x3a => 0xfb,
        0x3c => 0xfd,
        _ => 35, // arbitrarily chosen, "?"
    }
}

pub fn get_ec_icon_color(ec_type: i32) -> &'static str {
    match ec_type {
        0 | 8 | 9 | 0xc => "#26e196",
        1 | 0xd | 0x10 | 0x13 => "#ffff71",
        2 | 0x15 => "#a0cdff",
        3 | 0x12 => "#ff7316",
        4 => "#a540e1",
        5 | 10 => "#ff4b0d",
        6 => "#9b784f",
        7 | 0x11 => "#ffffff",
        0xb => "#ffb355",
        0xf => "#5aaaff",
        0x14 => "#476eff",
        0x16 => "#ff7316",
        0x17 => "#ffffff",
        0x18 => "#26e196",
        0x19 => "#ffff71",
        0x1a => "#9b784f",
        0x1b => "#a0cdff",
        0x1c => "#ff4b0d",
        0x1d | 0x1e | 0x1f => "#ffff71",
        0x22 => "#ff4b0d",
        0x23 => "#ffb355",
        0x24 => "#26e196",
        0x25 => "#ffff71",
        0x26 => "#476eff",
        0x27 => "#ff4b0d",
        0x2d => "#ffb355",
        0x2e => "#476eff",
        0x2f => "#26e196",
        0x30 => "#ef73b9",
        0x31 => "#ff7316",
        0x32 => "#ff4b0d",
        0x33 => "#ffffff",
        0x39 => "#ff4b0d",
        0x3a => "#ffb355",
        0x3c => "#476eff",
        0x3e => "#ff4b0d",
        0x3f => "#ffff71",
        0x40 => "#5aaaff",
        _ => "#ffffff", // arbitrarily chosen
    }
}

// Another unfortunately hard-coded mapping...
static EC_ID_MAP: &[&str] = &[
    "001_00", "002_00", "002_01", "002_02", "002_03", "004_00", "005_00", "006_00", "007_00",
    "008_00", "009_00", "009_01", "009_02", "009_03", "009_08", "010_00", "010_01", "010_02",
    "010_04", "011_00", "012_00", "012_01", "012_02", "014_00", "015_00", "017_00", "018_00",
    "019_00", "020_00", "021_00", "022_00", "022_01", "023_00", "023_01", "024_00", "024_01",
    "024_02", "024_03", "025_00", "026_00", "027_00", "028_00", "029_00", "030_00", "031_00",
    "032_00", "033_00", "034_00", "035_00", "036_00", "037_00", "038_00", "009_09", "009_10",
    "009_11", "050_00", "051_00", "052_00", "053_00", "054_00", "055_00", "056_00", "056_01",
    "056_02", "057_00",
];

fn gen_map(
    hash_store: &HashStore,
    id: i32,
    map: &GameMap,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    path: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let (mut output, mut toc_sink) = path.create_html_with_toc(&map_page(id), toc)?;

    let gen_fish_table = |tag: &str, fishes: &[rsz::FishSpawnGroupInfo]| -> Box<div<String>> {
        html!(<div class="mh-reward-box"><div class="mh-table"><table>
        <thead><tr>
        <th>""</th>
        <th>{text!("{}", tag)}</th>
        <th>"Probability"</th>
        </tr></thead>
        <tbody> {
            fishes.iter().enumerate().flat_map(|(i, fish)| {
                let mut is_first = true;
                let span = fish.fish_spawn_rate_list.len();
                fish.fish_spawn_rate_list.iter().map(move |f| {
                    let first = is_first.then(|| -> Box<td<String>> {
                        html!(<td rowspan={span}>{text!("{}", i)}</td>)
                    } );
                    is_first = false;

                    let item_id = get_fish_item_id(f.fish_id);
                    let item = if let Some(item) = item_id {
                        html!(<div class="il">{gen_item_label_from_id(item, pedia_ex)}</div>)
                    } else {
                        html!(<div class="il">{text!("Unknown fish {}", f.fish_id)}</div>)
                    };

                    html!(<tr>
                        {first}
                        <td>{ item }</td>
                        <td>{text!("{}%", f.spawn_rate)}</td>
                    </tr>)
                })
            })
        } </tbody></table></div></div>)
    };

    let ec_name = pedia.ec_name.get_name_map();
    let ec_name_mr = pedia.ec_name_mr.get_name_map();
    let get_ec_name = |type_: i32| {
        usize::try_from(type_)
            .ok()
            .and_then(|i| EC_ID_MAP.get(i))
            .and_then(|id| {
                let tag = format!("EC_NAME_{id}");
                if let Some(msg) = ec_name.get(&tag) {
                    return Some(*msg);
                }
                let tag_mr = format!("EC_NAME_{id}_MR");
                if let Some(msg) = ec_name_mr.get(&tag_mr) {
                    return Some(*msg);
                }
                None
            })
            .map_or_else(
                || html!(<span>{text!("Unknown {}", type_)}</span>),
                gen_multi_lang,
            )
    };
    let get_ec_icon = |type_: i32| {
        let pattern = get_ec_icon_pattern(type_);
        if ITEM_ICON_SPECIAL_COLOR.contains(&pattern) {
            let icon_path = format!("resources/item/{pattern:03}.png");
            html!(<div class="mh-icon-container">
                <img alt="map icon" src={icon_path}
                    class="mh-wire-long-jump-icon undraggable" /*style={rotate}*/ draggable=false/>
            </div>)
        } else {
            let icon_path = format!("resources/item/{pattern:03}");
            gen_colored_icon_inner(get_ec_icon_color(type_), &icon_path, [], false)
        }
    };

    let mut map_icons = vec![];
    let mut map_explains = vec![];
    for (i, pop) in map.pops.iter().enumerate() {
        let x = (pop.position.x + map.x_offset) / map.map_scale * 100.0;
        let y = (pop.position.y + map.y_offset) / map.map_scale * 100.0;

        let icon_inner: Box<dyn Fn() -> Box<div<String>>>;
        let explain_inner;
        let filter;
        match &pop.kind {
            MapPopKind::Item { behavior, relic } => {
                icon_inner = Box::new(|| {
                    let icon_path = format!("resources/item/{:03}", behavior.pop_icon);
                    gen_colored_icon(behavior.pop_icon_color, &icon_path, [], false)
                });

                let relic_explain = relic.as_ref().map(|relic| {
                    let name = get_map_name(relic.note_map_no, pedia);
                    let relic_map_name = if let Some(name) = name {
                        gen_multi_lang(name)
                    } else {
                        html!(<span> "Unknown map" </span>)
                    };

                    html!(<div class="mh-reward-box">{
                        text!("Unlock note {} for ", relic.relic_id + 1)
                    } { relic_map_name} </div>)
                });

                if relic_explain.is_some() {
                    filter = "relic";
                } else {
                    filter = "item";
                }

                if let Some(lot) = pedia_ex
                    .item_pop
                    .get(&(behavior.pop_id, id))
                    .or_else(|| pedia_ex.item_pop.get(&(behavior.pop_id, -1)))
                {
                    explain_inner = html!(
                        <div class="mh-reward-tables">
                        { relic_explain }
                        <div class="mh-reward-box"><div class="mh-table"><table>
                            <thead><tr>
                            <th>"Low rank material"</th>
                            <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &lot.lower_id,
                                    &lot.lower_num,
                                    &lot.lower_probability)
                            } </tbody>
                        </table></div></div>

                        <div class="mh-reward-box"><div class="mh-table"><table>
                            <thead><tr>
                            <th>"High rank material"</th>
                            <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &lot.upper_id,
                                    &lot.upper_num,
                                    &lot.upper_probability)
                            } </tbody>
                        </table></div></div>

                        <div class="mh-reward-box"><div class="mh-table"><table>
                            <thead><tr>
                            <th>"Master rank material"</th>
                            <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &lot.master_id,
                                    &lot.master_num,
                                    &lot.master_probability)
                            } </tbody>
                        </table></div></div>
                    </div>);
                } else {
                    explain_inner = html!(<div class="mh-reward-tables">
                        { relic_explain }
                        <div class="mh-reward-box">"No material data"</div>
                    </div>)
                }
            }
            MapPopKind::WireLongJump { behavior, angle: _ } => {
                //let angle = *angle;
                icon_inner = Box::new(move || {
                    //let rotate = format!("transform:rotate({}rad);", angle);
                    html!(<div class="mh-icon-container">
                        <img alt="Wirebug jump point" src="resources/item/115.png"
                        class="mh-wire-long-jump-icon undraggable" /*style={rotate}*/ draggable=false/></div>)
                });

                explain_inner = html!(<div class="mh-reward-tables">
                    { text!("ID: {}", behavior.wire_long_jump_id) }
                </div>);

                filter = "jump";
            }
            MapPopKind::Camp { behavior } => {
                icon_inner = Box::new(|| {
                    html!(<div class="mh-icon-container"> {
                        if behavior.camp_type == rsz::CampType::BaseCamp {
                            html!(<img alt="Main camp" src="resources/main_camp.png"
                                class="mh-main-camp undraggable" draggable=false/>)
                        } else {
                            html!(<img alt="Sub camp" src="resources/sub_camp.png"
                                class="mh-sub-camp undraggable" draggable=false/>)
                        }
                    } </div>)
                });

                explain_inner = html!(<div class="mh-reward-tables">
                    { text!("ID: {:?}", behavior.camp_type) }
                </div>);

                filter = "camp";
            }
            MapPopKind::FishingPoint { behavior } => {
                icon_inner = Box::new(|| gen_colored_icon(0, "resources/item/046", [], false));

                explain_inner = html!(<div class="mh-reward-tables">
                    { gen_fish_table("Low rank fish",
                        &behavior.fish_spawn_data.unwrap().spawn_group_list_info_low) }
                    { gen_fish_table("High rank fish",
                        &behavior.fish_spawn_data.unwrap().spawn_group_list_info_high) }
                    { gen_fish_table("Master rank fish",
                        &behavior.fish_spawn_data.unwrap().spawn_group_list_info_master) }
                </div>);

                filter = "fish";
            }
            MapPopKind::Recon { behavior } => {
                icon_inner = Box::new(|| {
                    html!(<div class="mh-icon-container">
                        <img alt="Recon point" src="resources/recon.png"
                            class="mh-recon undraggable" draggable=false/>
                    </div>)
                });

                explain_inner = html!(<div class="mh-reward-tables">
                    { text!("ID: {:?}", behavior.spot_index) }
                </div>);

                filter = "camp";
            }
            MapPopKind::Ec { behavior } => {
                icon_inner = Box::new(|| get_ec_icon(behavior.base.type_));
                let name = get_ec_name(behavior.base.type_);

                explain_inner = html!(<div>
                    <div class="mh-kvlist">
                    <p class="mh-kv"><span>"Name"</span>
                    <span>{name}</span>
                    </p>
                    <p class="mh-kv"><span>"Respawn time"</span>
                    <span>{text!("{}", behavior.base.repop_wait_time)}</span>
                    </p>
                    </div>
                </div>);

                filter = "ec";
            }
            MapPopKind::Fg { behavior } => 'assign_content: {
                let icon;
                let color;
                match (behavior.base.type_, &behavior.extra) {
                    (0, _) => {
                        explain_inner = html!(<div>"Gimmick 0"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 9;
                    }
                    (1, rsz::fg::Extra::Fg002(extra)) => {
                        explain_inner = html!(<div>
                        <div class="mh-kvlist">
                        <p class="mh-kv"><span>"Name"</span>
                        <span>"Lucky life waypoint"</span>
                        </p>
                        <p class="mh-kv"><span>"Select probability"</span>
                        <span>{text!("{}", extra.select_probability)}</span>
                        </p>
                        <p class="mh-kv"><span>"Searching distance"</span>
                        <span>{text!("{}", extra.search_dist)}</span>
                        </p>
                        <p class="mh-kv"><span>"Cooldown time"</span>
                        <span>{text!("{}", extra.cool_time)}</span>
                        </p>
                        </div>

                        <div class="mh-reward-tables">
                        <div class="mh-reward-box"><div class="mh-table"><table>
                            <thead><tr>
                            <th>"Lucky life"</th>
                            <th>"Probability"</th></tr></thead>
                            <tbody> {
                                extra.fg002_type_list.iter().zip(&extra.probability_list).map(|(&type_, &prob)| {
                                    html!(<tr>
                                        <td><div class="mh-icon-text">
                                            {get_ec_icon(type_)}
                                            {get_ec_name(type_)}
                                        </div></td>
                                        <td>{text!("{}%", prob)}</td>
                                    </tr>)
                                })
                            } </tbody>
                        </table></div></div>
                        </div>
                        </div>);
                        filter = "ec";
                        icon = 35;
                        color = 10;
                    }
                    (2, rsz::fg::Extra::Fg003(extra)) => {
                        let table = map.ec_data.as_ref().and_then(|ec_data| {
                            ec_data.fg003_table_data.get(extra.table_id as usize)
                        });

                        explain_inner = if let Some(table) = table {
                            html!(<div>
                            <div class="mh-kvlist">
                            <p class="mh-kv"><span>"Name"</span>
                            <span>"Random endemic life"</span>
                            </p>
                            <p class="mh-kv"><span>"Min number"</span>
                            <span>{text!("{}", table.min_num)}</span>
                            </p>
                            <p class="mh-kv"><span>"Max number"</span>
                            <span>{text!("{}", table.max_num)}</span>
                            </p>
                            </div>

                            <div class="mh-reward-tables">
                            <div class="mh-reward-box"><div class="mh-table"><table>
                                <thead><tr>
                                <th>"Endemic life"</th>
                                <th>"Probability"</th></tr></thead>
                                <tbody> {
                                    table.ec_data.unwrap().data_list.iter().map(|data| {
                                        html!(<tr>
                                            <td><div class="mh-icon-text">
                                                {get_ec_icon(data.type_)}
                                                {get_ec_name(data.type_)}
                                            </div></td>
                                            <td>{text!("{}%", data.rate)}</td>
                                        </tr>)
                                    })
                                } </tbody>
                            </table></div></div>
                            </div>
                            </div>)
                        } else {
                            html!(<div>{text!("Random endemic life with unknown table {}", extra.table_id)}</div>)
                        };
                        filter = "ec";
                        icon = 35;
                        color = 8;
                        // TODO
                    }
                    (3, _) => {
                        explain_inner = html!(
                            <div class="mh-kvlist">
                            <p class="mh-kv"><span>"Name"</span>
                            <span>"Golden/Gilded Spiribug"</span>
                            </p>
                            </div>
                        );
                        filter = "ec";
                        icon = 98;
                        color = 4;
                    }
                    (4, rsz::fg::Extra::Fg005(extra)) => {
                        explain_inner = html!(<div>
                            "Vent"
                        </div>);
                        filter = "fg";
                        icon_inner = Box::new(move || {
                            let color = extra.color_standby.to_le_bytes();
                            gen_colored_icon_inner(
                                &format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2]),
                                "resources/item/112",
                                [],
                                false,
                            )
                        });
                        break 'assign_content;
                    }
                    (5, _) => {
                        explain_inner = html!(<div>"Quicksand"</div>);
                        filter = "fg";
                        icon = 113;
                        color = 5;
                    }
                    (6, _) => {
                        explain_inner = html!(<div>"Ring"</div>);
                        filter = "fg";
                        icon = 116;
                        color = 4;
                    }
                    (9, _) => {
                        explain_inner = html!(<div>"Falling Boulder"</div>);
                        filter = "fg";
                        icon = 300;
                        color = 2;
                    }
                    (11, _) => {
                        explain_inner = html!(<div>"Swamp"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 13;
                    }
                    (13, _) => {
                        explain_inner = html!(<div>"Poison Cypress"</div>);
                        filter = "fg";
                        icon = 303;
                        color = 9;
                    }
                    (14, _) => {
                        explain_inner = html!(<div>"Thornytoad biting area"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 7;
                    }
                    (15, _) => {
                        explain_inner = html!(<div>"Bees"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 6;
                    }
                    (16, _) => {
                        explain_inner = html!(<div>"Gimmick 16"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 10;
                    }
                    (17, _) => {
                        explain_inner = html!(<div>"Gimmick 17"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 11;
                    }
                    (18, _) => {
                        explain_inner = html!(<div>"Gimmick 18"</div>);
                        filter = "fg";
                        icon = 35;
                        color = 12;
                    }
                    _ => {
                        explain_inner = html!(<div>
                            {text!("Field gimmick {}", behavior.base.type_)}
                        </div>);

                        filter = "fg";
                        icon = 35;
                        color = 0;
                    }
                }

                icon_inner = Box::new(move || {
                    let icon_path = format!("resources/item/{:03}", icon);
                    gen_colored_icon(color, &icon_path, [], false)
                });
            }
        }
        let map_icon_id = format!("mh-map-icon-{i}");
        let map_explain_id = format!("mh-map-explain-{i}");

        map_icons.push(
            html!(<div class="mh-map-filter-item" id={map_icon_id.as_str()} data-filter={filter}
                style={format!("left:{x}%;top:{y}%")}> {icon_inner()} </div>),
        );
        map_explains.push(html!(<div class="mh-hidden" id={map_explain_id.as_str()}>
            {icon_inner()}
            <p>{ text!("level: {}", pop.position.z) }</p>
            {explain_inner}
        </div>))
    }

    let name = get_map_name(id, pedia);

    let title = if let Some(name) = name {
        toc_sink.add(name);
        gen_multi_lang(name)
    } else {
        html!(<span>{ text!("Map {:02}", id) }</span>)
    };

    let mut sections = vec![];

    sections.push(Section {
        title: "Map".to_owned(),
        content: html!(
            <section id="s-map">
            <div class="mh-filters"><ul>
            <li id="mh-map-filter-button-all" class="mh-map-filter-button is-active"><a>"All icons"</a></li>
            <li id="mh-map-filter-button-item" class="mh-map-filter-button"><a>"Gathering"</a></li>
            <li id="mh-map-filter-button-relic" class="mh-map-filter-button"><a>"Relics"</a></li>
            <li id="mh-map-filter-button-camp" class="mh-map-filter-button"><a>"Camps"</a></li>
            <li id="mh-map-filter-button-jump" class="mh-map-filter-button"><a>"Jumping points"</a></li>
            <li id="mh-map-filter-button-fish" class="mh-map-filter-button"><a>"Fishing points"</a></li>
            <li id="mh-map-filter-button-ec" class="mh-map-filter-button"><a>"Endemic life"</a></li>
            <li id="mh-map-filter-button-fg" class="mh-map-filter-button"><a>"Other"</a></li>
            </ul></div>

            <div class="columns">

            <div class="column is-two-thirds">
            <div class="mh-map-outer">
                <div class="mh-map-container" id="mh-map-container">
                    <div class="mh-map" id="mh-map">
                    {(0..map.layer_count).map(|j| {
                        let c = if j == 0 {
                            "mh-map-layer undraggable"
                        } else {
                            "mh-map-layer undraggable mh-hidden"
                        };
                        let html_id = format!("mh-map-layer-{j}");
                        html!(
                            <img alt="Map" class={c} id={html_id.as_str()} draggable=false
                                src={format!("resources/map{id:02}_{j}.png")}/>
                        )
                    })}
                    { map_icons }
                    </div>
                </div>

                <div class="mh-map-buttons">
                    <button class="button" id="button-scale-down" disabled=true>
                        <span class="icon"><i class="fas fa-magnifying-glass-minus"></i></span>
                    </button>
                    <button class="button" id="button-scale-up">
                        <span class="icon"><i class="fas fa-magnifying-glass-plus"></i></span>
                    </button>
                    {
                        (map.layer_count > 1).then(||html!(
                            <button class="button" id="button-map-layer">
                            <span class="icon"><i class="fas fa-layer-group"></i></span>
                            <span>"Change Layer"</span>
                            </button>))
                    }
                </div>
            </div>
            </div>

            <div class="column">

            <div>
            { map_explains }
            <div id="mh-map-explain-default">"Click an icon on the map to learn the detail."</div>
            </div>

            </div> // right column

            </div> // columns
            </section>
        ),
    });

    let discovery_map_index = rsz::DISCOVER_MAP_LIST.iter().position(|&i| i == id);

    if let Some(discovery_map_index) = discovery_map_index {
        sections.push(Section {
            title: "Monsters in tour".to_owned(),
            content: html!(
                <section id="s-monster">
                <h2>"Monsters in tour"</h2>
                <ul class="mh-item-list">{
                    pedia_ex.monsters.iter().filter(|(_, monster)|
                        if let Some(discovery) = &monster.discovery {
                            discovery.map_flag[discovery_map_index]
                        } else {
                            false
                        }
                    ).map(|(&em, _)|
                        html!(<li>{gen_monster_tag(pedia_ex, em, false, false, None, None)}</li>)
                    )
                }</ul>
                </section>
            ),
        });
    };

    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Map {:02}", id)}</title>
                { head_common(hash_store, path) }
                { open_graph(name, "",
                    None, "", None, toc_sink.path(), config) }
                { name.iter().flat_map(|&name|title_multi_lang(name)) }
                <style id="mh-map-list-style">""</style>
            </head>
            <body>
            { navbar() }
            { gen_menu(&sections, toc_sink.path()) }
            <main>

            <header><h1>{title}</h1></header>

            { sections.into_iter().map(|s|s.content) }

            </main>
            { right_aside() }
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_maps(
    hash_store: &HashStore,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let map_path = output.sub_sink("map")?;
    for (&id, map) in &pedia.maps {
        gen_map(hash_store, id, map, pedia, pedia_ex, config, &map_path, toc)?
    }
    Ok(())
}

pub fn gen_map_list(hash_store: &HashStore, pedia: &Pedia, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Maps - MHRice")}</title>
                { head_common(hash_store, output) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Maps"</h1></header>
                <ul>
                {
                    pedia.maps.iter().map(|(&i, _)|{
                        html!(<li>
                            {gen_map_label(i, pedia)}
                        </li>)
                    })
                }
                </ul>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output
        .create_html("map.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}
