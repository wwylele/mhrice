use super::gen_website::*;
use super::pedia::*;
use anyhow::*;
use std::collections::BTreeMap;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, html, text};

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
                                    let name = if let Some(skill_data) = skills.get(&(skill - 1)) {
                                        html!(<span><a href={format!("/skill/{:03}.html", skill - 1)}
                                            class="mh-icon-text">
                                            {gen_colored_icon(skill_data.icon_color, "/resources/skill", &[])}
                                            {gen_multi_lang(&skill_data.name)}
                                        </a></span>)
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

pub fn gen_armors(pedia_ex: &PediaEx<'_>, root: &Path) -> Result<()> {
    let armor_path = root.join("armor");
    create_dir(&armor_path)?;
    for series in &pedia_ex.armors {
        let path = armor_path.join(format!("{:03}.html", series.series.armor_series));
        gen_armor(series, &pedia_ex.skills, &path)?
    }
    Ok(())
}
