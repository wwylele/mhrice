use super::gen_item::*;
use super::gen_website::*;
use super::pedia::*;
use super::prepare_map::*;
use super::sink::*;
use crate::rsz;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

fn map_page(id: i32) -> String {
    format!("{id:02}.html")
}

pub fn gen_map_label(id: i32, pedia: &Pedia) -> Box<a<String>> {
    let link = format!("/map/{}", map_page(id));
    let name_name = format!("Stage_Name_{id:02}");
    let name = pedia.map_name.get_entry(&name_name);
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

fn gen_map(
    id: i32,
    map: &GameMap,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    let gen_fish_table = |tag: &str, fishes: &[rsz::FishSpawnGroupInfo]| -> Box<div<String>> {
        html!( <div class="mh-reward-box"><table>
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
                        if let Some(item) = pedia_ex.items.get(&item) {
                            html!(<span>{gen_item_label(item)}</span>)
                        } else {
                            html!(<span>{text!("{:?}", item)}</span>)
                        }
                    } else {
                        html!(<span>{text!("Unknown fish {}", f.fish_id)}</span>)
                    };

                    html!(<tr>
                        {first}
                        <td>{ item }</td>
                        <td>{text!("{}%", f.spawn_rate)}</td>
                    </tr>)
                })
            })
        } </tbody></table></div>)
    };

    let mut map_icons = vec![];
    let mut map_explains = vec![];
    for (i, pop) in map.pops.iter().enumerate() {
        let x = (pop.position.x + map.x_offset) / map.map_scale * 100.0;
        let y = (pop.position.y + map.y_offset) / map.map_scale * 100.0;

        let icon_inner: Box<dyn Fn() -> Box<div<String>>>;
        let explain_inner;
        let tag_list;
        match &pop.kind {
            MapPopKind::Item { behavior, relic } => {
                icon_inner = Box::new(|| {
                    let icon_path = format!("/resources/item/{:03}", behavior.pop_icon);
                    gen_colored_icon(behavior.pop_icon_color, &icon_path, &[])
                });

                let relic_explain = relic.as_ref().map(|relic| {
                    let name_name = format!("Stage_Name_{:02}", relic.note_map_no);
                    let name = pedia.map_name.get_entry(&name_name);
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
                    tag_list = "mh-map-tag-relic";
                } else {
                    tag_list = "mh-map-tag-item";
                }

                if let Some(lot) = pedia_ex
                    .item_pop
                    .get(&(behavior.pop_id, id))
                    .or_else(|| pedia_ex.item_pop.get(&(behavior.pop_id, -1)))
                {
                    explain_inner = html!(
                        <div class="mh-reward-tables">
                        { relic_explain }
                        <div class="mh-reward-box"><table>
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
                        </table></div>

                        <div class="mh-reward-box"><table>
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
                        </table></div>
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
                    html!(<div class="mh-icon-container"><img src="/resources/item/115.png"
                    class="mh-wire-long-jump-icon" /*style={rotate}*/ /></div>)
                });

                explain_inner = html!(<div class="mh-reward-tables">
                    { text!("ID: {}", behavior.wire_long_jump_id) }
                </div>);

                tag_list = "mh-map-tag-jump";
            }
            MapPopKind::Camp { behavior } => {
                icon_inner = Box::new(|| {
                    html!(<div class="mh-icon-container"> {
                        if behavior.camp_type == rsz::CampType::BaseCamp {
                            html!(<img src="/resources/main_camp.png"
                                class="mh-main-camp"/>)
                        } else {
                            html!(<img src="/resources/sub_camp.png"
                                class="mh-sub-camp"/>)
                        }
                    } </div>)
                });

                explain_inner = html!(<div class="mh-reward-tables">
                    { text!("ID: {:?}", behavior.camp_type) }
                </div>);

                tag_list = "mh-map-tag-camp";
            }
            MapPopKind::FishingPoint { behavior } => {
                icon_inner = Box::new(|| gen_colored_icon(0, "/resources/item/046", &[]));

                explain_inner = html!(<div class="mh-reward-tables">
                    { gen_fish_table("Low rank fish",
                        &behavior.fish_spawn_data.unwrap().spawn_group_list_info_low) }
                    { gen_fish_table("High rank fish",
                        &behavior.fish_spawn_data.unwrap().spawn_group_list_info_high) }
                </div>);

                tag_list = "mh-map-tag-fish";
            }
        }
        let map_icon_id = format!("mh-map-icon-{i}");
        let map_explain_id = format!("mh-map-explain-{i}");
        let map_explain_event = format!("onShowMapExplain('{i}');");
        let map_icon_class_string = format!("mh-map-pop {}", tag_list);

        map_icons.push(html!(<div class={map_icon_class_string.as_str()} id={map_icon_id.as_str()}
            style={format!("left:{x}%;top:{y}%")} onclick={map_explain_event.as_str()}> {icon_inner()} </div>: String
        ));
        map_explains.push(html!(<div class="mh-hidden" id={map_explain_id.as_str()}>
            {icon_inner()}
            <p>{ text!("level: {}", pop.position.z) }</p>
            {explain_inner}
        </div>))
    }

    let name_name = format!("Stage_Name_{id:02}");
    let name = pedia.map_name.get_entry(&name_name);

    let title = if let Some(name) = name {
        toc_sink.add(name);
        gen_multi_lang(name)
    } else {
        html!(<span>{ text!("Map {:02}", id) }</span>)
    };

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Map {:02}", id)}</title>
                { head_common() }
            </head>
            <body>
            { navbar() }
            <main><div class="container"> <div class="content">

            <h1 class="title">
            {title}
            </h1>

            <div class="columns">

            <div class="column is-two-thirds">
            <div class="mh-map-container">
            <div class="mh-map" id="mh-map">
            {(0..map.layer_count).map(|j| {
                let c = if j == 0 {
                    "mh-map-layer"
                } else {
                    "mh-map-layer mh-hidden"
                };
                let html_id = format!("mh-map-layer-{}", j);
                html!(
                    <img class={c} id={html_id.as_str()} src={format!("/resources/map{id:02}_{j}.png")}/>
                )
            })}
            { map_icons }
            </div>
            </div>
            </div>

            <div class="column">

            <div>
            <button id="mh-map-filter-all" class="button is-primary" onclick="changeMapFilter('all');">"All icons"</button>
            <button id="mh-map-filter-item" class="button" onclick="changeMapFilter('item');">"Gathering"</button>
            <button id="mh-map-filter-relic" class="button" onclick="changeMapFilter('relic');">"Relics"</button>
            <button id="mh-map-filter-camp" class="button" onclick="changeMapFilter('camp');">"Camps"</button>
            <button id="mh-map-filter-jump" class="button" onclick="changeMapFilter('jump');">"Jumping points"</button>
            <button id="mh-map-filter-fish" class="button" onclick="changeMapFilter('fish');">"Fishing points"</button>
            </div>

            <div>
            <button class="button" id="button-scale-down" onclick="scaleDownMap();" disabled=true>
                <span class="icon"><i class="fas fa-search-minus"></i></span>
            </button>
            <button class="button" id="button-scale-up" onclick="scaleUpMap();">
                <span class="icon"><i class="fas fa-search-plus"></i></span>
            </button>
            {
                (map.layer_count > 1).then(||html!(
                    <button class="button" onclick="switchMapLayer();">
                      <span class="icon"><i class="fas fa-layer-group"></i></span>
                      <span>"Change Layer"</span>
                    </button>: String))
            }
            </div>

            <div>
            { map_explains }
            <div id="mh-map-explain-default">"Click an icon on the map to learn the detail."</div>
            </div>

            </div> // right column

            </div> // columns

            </div></div></main>
            </body>
        </html>: String
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_maps(
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let map_path = output.sub_sink("map")?;
    for (&id, map) in &pedia.maps {
        let (path, toc_sink) = map_path.create_html_with_toc(&map_page(id), toc)?;
        gen_map(id, map, pedia, pedia_ex, path, toc_sink)?
    }
    Ok(())
}

pub fn gen_map_list(pedia: &Pedia, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Maps - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Map"</h1>
                <ul>
                {
                    pedia.maps.iter().map(|(&i, _)|{
                        html!(<li>
                            {gen_map_label(i, pedia)}
                        </li>)
                    })
                }
                </ul>
                </div></main>
            </body>
        </html>: String
    );
    output
        .create_html("map.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}
