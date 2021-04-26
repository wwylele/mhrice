use super::gen_skill::*;
use super::gen_website::*;
use super::pedia::*;
use crate::msg::*;
use crate::rsz::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

pub struct Armor {
    name: MsgEntry,
    data: ArmorBaseUserDataParam,
}

pub struct ArmorSeries {
    name: Option<MsgEntry>,
    series: ArmorSeriesUserDataParam,
    pieces: [Option<Armor>; 5],
}

pub fn prepare_armors(pedia: &Pedia) -> Result<Vec<ArmorSeries>> {
    /*let mut armor_head_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_head_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut armor_chest_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_chest_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut armor_arm_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_arm_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut armor_waist_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_waist_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();

    let mut armor_leg_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_leg_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();


    let mut armor_series_name_msg: HashMap<String, MsgEntry> = pedia
        .armor_series_name_msg
        .entries
        .iter()
        .map(|entry| (entry.name.clone(), entry.clone()))
        .collect();
    */

    let mut series_map: BTreeMap<i32, ArmorSeries> = BTreeMap::new();

    for armor_series in &pedia.armor_series.param {
        if series_map.contains_key(&armor_series.armor_series) {
            bail!(
                "Duplicate armor series for ID {}",
                armor_series.armor_series
            );
        }
        let name = /*
        armor_series_name_msg.remove(&format!(
            "ArmorSeries_Hunter_{:03}",
            armor_series.armor_series
        ));
        */
            pedia
            .armor_series_name_msg
            .entries.get(armor_series.armor_series as usize).cloned(); // ?!
        let series = ArmorSeries {
            name,
            series: armor_series.clone(),
            pieces: [None, None, None, None, None],
        };
        series_map.insert(armor_series.armor_series, series);
    }

    for armor in &pedia.armor.param {
        if !armor.is_valid {
            continue;
        }

        /*
        let (slot, type_name, msg) = match (armor.pl_armor_id >> 20) & 7 {
            1 => (0, "Head", &mut armor_head_name_msg),
            2 => (1, "Chest", &mut armor_chest_name_msg),
            3 => (2, "Arm", &mut armor_arm_name_msg),
            4 => (3, "Waist", &mut armor_waist_name_msg),
            5 => (4, "Leg", &mut armor_leg_name_msg),
            _ => bail!("Unknown armor type for ID {}", armor.pl_armor_id),
        };

        let name = msg
            .remove(&format!(
                "A_{}_{:03}_Name",
                type_name,
                armor.pl_armor_id & 0xFF
            ))
            .with_context(|| format!("Duplicate armor {}", armor.pl_armor_id))?;
        */

        let (slot, msg) = match (armor.pl_armor_id >> 20) & 7 {
            1 => (0, &pedia.armor_head_name_msg),
            2 => (1, &pedia.armor_chest_name_msg),
            3 => (2, &pedia.armor_arm_name_msg),
            4 => (3, &pedia.armor_waist_name_msg),
            5 => (4, &pedia.armor_leg_name_msg),
            _ => bail!("Unknown armor type for ID {}", armor.pl_armor_id),
        };

        let name = msg
            .entries
            .get((armor.pl_armor_id & 0xFF) as usize)
            .with_context(|| format!("Cannot find name for armor {}", armor.pl_armor_id))?
            .clone(); // ?!

        let series = series_map.get_mut(&armor.series).with_context(|| {
            format!(
                "Cannot find series {} for armor {}",
                armor.series, armor.pl_armor_id
            )
        })?;

        if series.pieces[slot].is_some() {
            bail!(
                "Duplicated pieces for series {} slot {}",
                armor.series,
                slot
            );
        }

        series.pieces[slot] = Some(Armor {
            name,
            data: armor.clone(),
        });
    }

    Ok(series_map.into_iter().map(|(_, v)| v).collect())
}

pub fn gen_armor_list(serieses: &[ArmorSeries], root: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Armors - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">"Armors"</h1>
                <article class="message is-warning">
                    <div class="message-body">
                        "Armor names are probably incorrect."
                    </div>
                </article>
                <ul class="mh-armor-series-list">{
                    serieses.into_iter().map(|series|{
                        let series_name = if let Some(name) = series.name.as_ref() {
                            gen_multi_lang(name)
                        } else {
                            html!(<span>"<Unknown>"</span>)
                        };
                        html!(
                            <li class="mh-armor-series-list">
                            <a href={format!("/armor/{:03}.html", series.series.armor_series)}>
                            <h2 class="title">{
                                series_name
                            }</h2>
                            <ul class="mh-armor-list"> {
                                series.pieces.iter().map(|piece| {
                                    let piece_name = if let Some(piece) = piece {
                                        gen_multi_lang(&piece.name)
                                    } else {
                                        html!(<span>"-"</span>)
                                    };
                                    html!(<li class="mh-armor-list">{ piece_name }</li>)
                                })
                            } </ul>
                            </a></li>
                        )
                    })
                }</ul>
                </div> </div> </main>
            </body>
        </html>
    );

    let armor_path = root.join("armor.html");
    write(&armor_path, doc.to_string())?;

    Ok(())
}

fn gen_armor(series: &ArmorSeries, skills: &BTreeMap<u8, Skill>, path: &Path) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Armor {:03}", series.series.armor_series)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title"> {
                    if let Some(name) = series.name.as_ref() {
                        gen_multi_lang(name)
                    } else {
                        html!(<span>"<Unknown>"</span>)
                    }
                } </h1>
                <table>
                    <thead><tr>
                        <th>"Name"</th>
                        <th>"Value (Sell / Buy)"</th>
                        <th>"Defense"</th>
                        <th>"Fire"</th>
                        <th>"Water"</th>
                        <th>"Ice"</th>
                        <th>"Thunder"</th>
                        <th>"Dragon"</th>
                        <th>"Slots"</th>
                        <th>"Skills"</th>
                    </tr></thead>
                    <tbody> {
                        series.pieces.iter().map(|piece| {
                            let piece = if let Some(piece) = piece {
                                piece
                            } else {
                                return html!(<tr><td colspan="10">"-"</td></tr>)
                            };

                            let mut slot_list = vec![];

                            for (i, num) in piece.data.decorations_num_list.iter().enumerate().rev() {
                                for _ in 0..*num {
                                    slot_list.push(i + 1);
                                }
                            }

                            let slots = slot_list.iter()
                                .map(|s|format!("{}", s)).collect::<Vec<String>>().join(" / ");

                            let skills = html!(<ul class="mh-armor-skill-list"> {
                                piece.data.skill_list.iter().zip(piece.data.skill_lv_list.iter())
                                    .filter(|(skill, lv)| **skill != 0)
                                    .map(|(skill, lv)| {
                                    let name = if let Some(skillz_data) = skills.get(&(skill - 1)) {
                                        html!(<span><a href={format!("/skill/{:03}.html", skill - 1)}>{
                                            gen_multi_lang(&skillz_data.name)
                                        }</a></span>)
                                    } else {
                                        html!(<span>"<UNKNOWN>"</span>)
                                    };
                                    html!(<li>
                                        {name}
                                        {text!(" + {}", lv)}
                                    </li>)
                                })
                            } </ul>);

                            html!(<tr>
                                <td>{gen_multi_lang(&piece.name)}</td>
                                <td>{text!("{} / {}", piece.data.value, piece.data.buy_value)}</td>
                                <td>{text!("{}", piece.data.def_val)}</td>
                                <td>{text!("{}", piece.data.fire_reg_val)}</td>
                                <td>{text!("{}", piece.data.water_reg_val)}</td>
                                <td>{text!("{}", piece.data.ice_reg_val)}</td>
                                <td>{text!("{}", piece.data.thunder_reg_val)}</td>
                                <td>{text!("{}", piece.data.dragon_reg_val)}</td>
                                <td>{text!("{}", slots)}</td>
                                <td>{skills}</td>
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

pub fn gen_armors(
    serieses: &[ArmorSeries],
    skills: &BTreeMap<u8, Skill>,
    root: &Path,
) -> Result<()> {
    let armor_path = root.join("armor");
    create_dir(&armor_path)?;
    for series in serieses {
        let path = armor_path.join(format!("{:03}.html", series.series.armor_series));
        gen_armor(series, skills, &path)?
    }
    Ok(())
}
