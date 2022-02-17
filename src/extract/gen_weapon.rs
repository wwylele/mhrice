use super::gen_common::*;
use super::gen_hyakuryu_skill::*;
use super::gen_item::*;
use super::gen_website::*;
use super::pedia::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::HashSet;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_weapon_icon(weapon: &WeaponBaseData) -> Box<div<String>> {
    let icon = format!("/resources/equip/{:03}", weapon.id.icon_index());
    gen_rared_icon(weapon.rare_type, &icon)
}

pub fn gen_weapon_label<Param>(weapon: &Weapon<Param>) -> Box<a<String>>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let main = weapon.param.to_base();
    let link = format!("/weapon/{}.html", main.id.to_tag());
    html!(
        <a href={link} class="mh-icon-text">
            {gen_weapon_icon(main)}
            <span>{gen_multi_lang(weapon.name)}</span>
        </a>
    )
}

#[allow(unused_variables)]
fn no<T, Base>(t: &T) -> Option<&Base> {
    None
}

#[allow(clippy::unnecessary_wraps)]
fn yes<T, Base>(t: &T) -> Option<&Base>
where
    T: ToBase<Base>,
{
    Some(t.to_base())
}

fn gen_craft_row(
    pedia_ex: &PediaEx,
    label: Box<td<String>>,
    data: &WeaponCraftingData,
    output: Option<(&[ItemId], &[u32])>,
) -> Box<tr<String>> {
    let category = gen_category(pedia_ex, data.material_category, data.material_category_num);

    let materials = gen_materials(pedia_ex, &data.item, &data.item_num, data.item_flag);

    let output = if let Some((output_item, output_item_num)) = output {
        gen_materials(pedia_ex, output_item, output_item_num, ItemId::None)
    } else {
        html!(<td>"-"</td>)
    };

    html!(<tr>
        {label}
        {category}
        {materials}
        {output}
    </tr>)
}

fn display_bullet_type(bullet: BulletType) -> &'static str {
    match bullet {
        BulletType::None => "<None>",
        BulletType::Normal1 => "Normal Ammo 1",
        BulletType::Normal2 => "Normal Ammo 2",
        BulletType::Normal3 => "Normal Ammo 3",
        BulletType::Kantsu1 => "Pierce Ammo 1",
        BulletType::Kantsu2 => "Pierce Ammo 2",
        BulletType::Kantsu3 => "Pierce Ammo 3",
        BulletType::SanW1 => "Spread Ammo 1",
        BulletType::SanW2 => "Spread Ammo 2",
        BulletType::SanW3 => "Spread Ammo 3",
        BulletType::SanO1 => "Shrapnel Ammo 1",
        BulletType::SanO2 => "Shrapnel Ammo 2",
        BulletType::SanO3 => "Shrapnel Ammo 3",
        BulletType::Tekko1 => "Sticky Ammo 1",
        BulletType::Tekko2 => "Sticky Ammo 2",
        BulletType::Tekko3 => "Sticky Ammo 3",
        BulletType::Kakusan1 => "Cluster Bomb 1",
        BulletType::Kakusan2 => "Cluster Bomb 2",
        BulletType::Kakusan3 => "Cluster Bomb 3",
        BulletType::Poison1 => "Poison Ammo 1",
        BulletType::Poison2 => "Poison Ammo 2",
        BulletType::Paralyze1 => "Paralysis Ammo 1",
        BulletType::Paralyze2 => "Paralysis Ammo 2",
        BulletType::Sleep1 => "Sleep Ammo 1",
        BulletType::Sleep2 => "Sleep Ammo 2",
        BulletType::Genki1 => "Exhaust Ammo 1",
        BulletType::Genki2 => "Exhaust Ammo 2",
        BulletType::Heal1 => "Recover Ammo 1",
        BulletType::Heal2 => "Recover Ammo 2",
        BulletType::Kijin => "Demon Ammo",
        BulletType::Kouka => "Amor Ammo",
        BulletType::Fire => "Flaming Ammo",
        BulletType::FireKantsu => "Piercing Fire Ammo",
        BulletType::Water => "Water Ammo",
        BulletType::WaterKantsu => "Piercing Water Ammo",
        BulletType::Ice => "Freeze Ammo",
        BulletType::IceKantsu => "Piercing Ice Ammo",
        BulletType::Thunder => "Thunder Ammo",
        BulletType::ThunderKantsu => "Piercing Thunder Ammo",
        BulletType::Dragon => "Dragon Ammo",
        BulletType::DragonKantsu => "Piercing Drago Ammo",
        BulletType::Zanretsu => "Slicing Ammo",
        BulletType::Ryugeki => "Wyvern Ammo",
        BulletType::Capture => "Tranq Ammo",
        BulletType::Setti => "<Setti>",
        BulletType::Gatling => "<Gatling>",
        BulletType::Snipe => "<Snipe>",
        BulletType::GatlingHeal => "<GatlingHeal>",
        BulletType::SnipeHeal => "<SnipeHeal>",
        BulletType::WireBullet => "<WireBullet>",
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn gen_weapon<Param>(
    weapon: &Weapon<Param>,
    weapon_tree: &WeaponTree<'_, Param>,
    pedia_ex: &PediaEx,
    path: &Path,
    has_element: fn(&Param) -> Option<&ElementWeaponBaseData>,
    has_second_element: fn(&Param) -> Option<&DualBladesBaseUserDataParam>,
    has_close_range: fn(&Param) -> Option<&CloseRangeWeaponBaseData>,
    has_horn: fn(&Param) -> Option<&HornBaseUserDataParam>,
    has_bullet: fn(&Param) -> Option<&BulletWeaponBaseUserDataParam>,
    has_lbg: fn(&Param) -> Option<&LightBowgunBaseUserDataParam>,
    has_bow: fn(&Param) -> Option<&BowBaseUserDataParam>,
    special: Option<fn(&Param) -> Vec<Box<p<String>>>>,
) -> Result<()>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let param = weapon.param;
    let main = param.to_base();
    let first_element = has_element(param);
    let second_element = has_second_element(param);
    let close_range = has_close_range(param);
    let horn = has_horn(param);
    let bullet = has_bullet(param);
    let lbg = has_lbg(param);
    let bow = has_bow(param);

    let bowgun_param = bullet.into_iter().flat_map(|bullet| {
        [
            html!(<p class="mh-kv"><span>"Fluctuation"</span>
            <span>{ text!("{:?}", bullet.fluctuation) }</span></p>),
            html!(<p class="mh-kv"><span>"Reload"</span>
            <span>{ text!("{}", bullet.reload) }</span></p>),
            html!(<p class="mh-kv"><span>"Recoil"</span>
            <span>{ text!("{}", bullet.recoil) }</span></p>),
            html!(<p class="mh-kv"><span>"Kakusan type"</span>
            <span>{ text!("{:?}", bullet.kakusan_type) }</span></p>),
        ]
    });

    let bow_param = bow.into_iter().flat_map(|bow| {
        let charge_type: Vec<String> = bow
            .bow_charge_type_list
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();
        [
            html!(<p class="mh-kv"><span>"Default charge lv"</span>
            <span>{ text!("{}", bow.bow_default_charge_lv_limit.0) }</span></p>),
            html!(<p class="mh-kv"><span>"Charge type"</span>
            <span>{ text!("{}", charge_type.join("-")) }</span></p>),
            html!(<p class="mh-kv"><span>"Curve type"</span>
            <span>{ text!("{:?}", bow.bow_curve_type) }</span></p>),
        ]
    });

    let sharpness = close_range.map(|close_range| {
        let highest = close_range
            .sharpness_val_list
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &s)| s != 0)
            .map_or(0, |(i, _)| i);
        html!(
        <p class="mh-kv"><span>"Sharpness"</span>
        <span>
        <span class="mh-sharpness-bar">
            {
                close_range.sharpness_val_list.iter().enumerate().map(|(i, &s)|{
                    let class = format!("mh-sharpness mh-sharpness-color-{}", i);
                    let style = format!("width:{}%;", s as f32 * 0.25);
                    html!(<span class={class.as_str()} style={style.as_str()} />)
                })
            }
            {
                close_range.takumi_val_list.iter().enumerate().map(|(i, &s)|{
                    let class = format!("mh-sharpness-half mh-sharpness-color-{}", i + highest);
                    let style = format!("width:{}%;", s as f32 * 0.25);
                    html!(<span class={class.as_str()} style={style.as_str()} />)
                })
            }
        </span>
        </span></p>)
    });

    let horn = horn.map(|horn| {
        html!(<section class="section">
        <h2 class="title">"Melody"</h2>
        <ul> {
            horn.horn_melody_type_list.iter().map(|id| {
                html!(<li> {
                    if let Some(name) = pedia_ex.horn_melody.get(id) {
                        gen_multi_lang(name)
                    } else {
                        html!(<span>{ text!("[{}]", id) }</span>)
                    }
                } </li>)
            })
        } </ul>
        </section>)
    });

    struct BowBottleMap {
        name: &'static str,
        power_up: Option<BottlePowerUpTypes>,
    }

    const BOW_BOTTLE_MAP: [BowBottleMap; 7] = [
        BowBottleMap {
            name: "Close range",
            power_up: Some(BottlePowerUpTypes::ShortRange),
        },
        BowBottleMap {
            name: "Power",
            power_up: None,
        },
        BowBottleMap {
            name: "Poison",
            power_up: Some(BottlePowerUpTypes::Poison),
        },
        BowBottleMap {
            name: "Paralyze",
            power_up: Some(BottlePowerUpTypes::Paralyze),
        },
        BowBottleMap {
            name: "Sleep",
            power_up: Some(BottlePowerUpTypes::Sleep),
        },
        BowBottleMap {
            name: "Blast",
            power_up: None,
        },
        BowBottleMap {
            name: "Exhaust",
            power_up: None,
        },
    ];

    let bow = bow.map(|bow| {
        html!(<section class="section">
        <h2 class="title">"Bottle"</h2>
        <ul> {
            BOW_BOTTLE_MAP.iter().enumerate().filter(|&(i,_)| {
                bow.bow_bottle_equip_flag_list[i]
            }).map(|(_, bottle)| {
                let power_up = if let Some(power_up) = bottle.power_up {
                    bow.bow_bottle_power_up_type_list.contains(&power_up)
                } else {
                    false
                }.then(|| html!(<span class="tag">"Power up"</span>));
                html!(<li>{ text!("{}", bottle.name) }
                {power_up}
                </li>)
            })
        } </ul>
        </section>)
    });

    let more_bullet: HashSet<BulletType> = main
        .hyakuryu_skill_id_list
        .iter()
        .flat_map(|skill| {
            pedia_ex
                .hyakuryu_skills
                .get(skill)
                .map(|skill| {
                    skill
                        .data
                        .add_bullet_type_list
                        .iter()
                        .cloned()
                        .filter(|&bullet| bullet != BulletType::None)
                })
                .into_iter()
                .flatten()
        })
        .collect();

    let rapid = lbg.map_or(&[][..], |lbg| &lbg.rapid_shot_list[..]);

    let bullet = bullet.map(|bullet| {
        html!(<section class="section">
        <h2 class="title">"Ammo list"</h2>
        <table>
        <thead><tr>
            <th>"Ammo Type"</th>
            <th>"Capacity"</th>
            <th>"Shot Type"</th>
        </tr></thead>
        <tbody> {
            bullet.bullet_equip_flag_list.iter()
                .zip(bullet.bullet_num_list.iter())
                .zip(bullet.bullet_type_list.iter())
                .enumerate()
                .map(|(bullet_type, ((flag, num), shoot_type))|
                    (BulletType::from_raw(bullet_type as u32).unwrap(), *flag, *num, *shoot_type)
                )
                .filter(|(bullet_type, flag, _, _)|*flag || more_bullet.contains(bullet_type))
                .map(|(bullet_type, flag, num, shoot_type)| {
                    let class = if flag {
                        ""
                    } else {
                        "mh-disabled"
                    };
                    let mut shoot_types = vec![];
                    let shoot_type = shoot_type.to_flags();
                    if shoot_type.moving_shot {
                        shoot_types.push("Moving shot")
                    }
                    if shoot_type.moving_reload {
                        shoot_types.push("Moving reload")
                    }
                    if shoot_type.single_auto {
                        shoot_types.push("Single shot auto reload")
                    }
                    if rapid.contains(&bullet_type) {
                        shoot_types.push("Rapid shot")
                    }
                    let shoot_types = shoot_types.join(", ");
                    html!(<tr class={class}>
                        <td>{ text!("{}", display_bullet_type(bullet_type)) }</td>
                        <td>{ text!("{}", num) }</td>
                        <td>{ text!("{}", shoot_types) }</td>
                    </tr>)
                })
        }
        { lbg.map(|lbg| {
            html!(<tr><td>{ text!("{}", display_bullet_type(lbg.unique_bullet)) }</td></tr>)
        }) }
        </tbody>
        </table>
        </section>)
    });

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Weapon - MHRice"</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    {gen_weapon_icon(main)}
                </div>
                <h1 class="title">
                    {gen_multi_lang(weapon.name)}
                </h1>

                <section class="section"><p>
                    {gen_multi_lang(weapon.explain)}
                </p></section>

                <section class="section">
                <h2 class="title">"Stat"</h2>
                <div class="mh-kvlist">
                <p class="mh-kv"><span>"Attack"</span>
                <span>{text!("{}", main.atk)}</span></p>
                <p class="mh-kv"><span>"Affinity"</span>
                <span>{text!("{}%", main.critical_rate)}</span></p>
                <p class="mh-kv"><span>"Defense"</span>
                <span>{text!("{}", main.def_bonus)}</span></p>
                <p class="mh-kv"><span>"Slot"</span>
                <span>{gen_slot(&main.slot_num_list)}</span></p>

                {first_element.map(|first_element| html!(
                    <p class="mh-kv"><span>"Element"</span>
                    <span>
                        <span>{text!("{:?} {}",
                            first_element.main_element_type,
                            first_element.main_element_val
                        )}</span>
                        {
                            second_element.map(|second_element| html!(
                                <span>{text!(" {:?} {}",
                                    second_element.sub_element_type,
                                    second_element.sub_element_val
                                )}</span>
                            ))
                        }
                    </span></p>
                ))}

                { sharpness }

                { bowgun_param }

                { bow_param }

                { special.map(|special|special(param)).into_iter().flatten() }

                </div>
                </section>

                <section class="section">
                <h2 class="title">"Ramp-up skills"</h2>
                <ul> {
                    main.hyakuryu_skill_id_list.iter()
                    .filter(|&&skill|skill != PlHyakuryuSkillId::None)
                    .map(|skill|{
                        if let Some(skill) = pedia_ex.hyakuryu_skills.get(skill) {
                            html!(<li> {
                                gen_hyakuryu_skill_label(skill)
                            } </li>)
                        } else {
                            html!(<li>{ text!("Unknown {:?}", skill) }</li>)
                        }
                    })
                } </ul>
                </section>

                { horn }

                { bullet }

                { bow }

                <section class="section">
                <h2 class="title">"Crafting"</h2>
                <table>
                    <thead><tr>
                        <th>""</th>
                        <th>"Categorized Material"</th>
                        <th>"Material"</th>
                        <th>"Output"</th>
                    </tr></thead>
                    <tbody>
                        {weapon.product.as_ref().map(|product| {
                            gen_craft_row(pedia_ex, html!(<td>"Forge"</td>),
                                &product.base, Some((&product.output_item, &product.output_item_num)))
                        })}
                        {weapon.process.as_ref().map(|process| {
                            let label = if let Some(parent) = weapon.parent {
                                let parent = weapon_tree.weapons.get(&parent).unwrap();
                                html!(<td>"Upgrade from " {gen_weapon_label(parent)}</td>)
                            } else {
                                html!(<td>"Upgrade from unknown"</td>)
                            };

                            gen_craft_row(pedia_ex, label,
                                &process.base, Some((&process.output_item, &process.output_item_num)))
                        })}
                        {weapon.change.as_ref().map(|change| {
                            gen_craft_row(pedia_ex, html!(<td>"As layered"</td>),
                                &change.base, None)
                        })}
                    </tbody>
                </table>
                </section>

                <section class="section">
                <h2 class="title">"Upgrade"</h2>
                <ul> {
                    weapon.children.iter().map(|child| {
                        let weapon = weapon_tree.weapons.get(child).unwrap();
                        html!(<li>{gen_weapon_label(weapon)}</li>)
                    })
                } </ul>
                </section>

                </div></div></main>
            </body>
        </html>
    );
    write(&path, doc.to_string())?;

    Ok(())
}

fn gen_tree_rec<Param>(weapon_tree: &WeaponTree<Param>, list: &[WeaponId]) -> Box<ul<String>>
where
    Param: ToBase<MainWeaponBaseData>,
{
    html!(<ul> {
        list.iter().map(|id| {
            let weapon = weapon_tree.weapons.get(id).unwrap();
            html!(<li>
                { gen_weapon_label(weapon) }
                { gen_tree_rec(weapon_tree, &weapon.children) }
            </li>)
        })
    } </ul>)
}

fn gen_tree<Param>(
    weapon_tree: &WeaponTree<Param>,
    weapon_path: &Path,
    tag: &str,
    name: &str,
) -> Result<()>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let list_path = weapon_path.join(format!("{}.html", tag));

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("{} - MHRice", name)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">
                    {text!("{}", name)}
                </h1>
                <div class="mh-weapon-tree">
                { gen_tree_rec(weapon_tree, &weapon_tree.roots) }
                </div>
                </div></main>
            </body>
        </html>
    );

    write(&list_path, doc.to_string())?;

    Ok(())
}

#[allow(clippy::vec_box)]
fn slash_axe(param: &SlashAxeBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Bottle"</span>
    <span>{text!("{:?} {}", param.slash_axe_bottle_type,
            param.slash_axe_bottle_element_val)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn gun_lance(param: &GunLanceBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Type"</span>
    <span>{text!("{:?} {}", param.gun_lance_fire_type,
            param.gun_lance_fire_lv.0)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn insect_glaive(param: &InsectGlaiveBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Insect level"</span>
    <span>{text!("{}", param.insect_glaive_insect_lv.0)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn charge_axe(param: &ChargeAxeBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Bottle"</span>
    <span>{text!("{:?}", param.charge_axe_bottle_type)}</span>
    </p>)]
}

#[allow(clippy::vec_box)]
fn heavy_bowgun(param: &HeavyBowgunBaseUserDataParam) -> Vec<Box<p<String>>> {
    vec![html!(<p class="mh-kv">
    <span>"Unique bullet"</span>
    <span>{text!("{:?}", param.heavy_bowgun_unique_bullet_type)}</span>
    </p>)]
}

pub fn gen_weapons(pedia_ex: &PediaEx, root: &Path) -> Result<()> {
    let path = root.join("weapon");
    create_dir(&path)?;

    macro_rules! weapon {
        ($label:ident, $name:expr,
            element:$element:ident,
            second_element:$second_element:ident,
            close_range:$close_range:ident,
            horn:$horn:ident,
            bullet:$bullet:ident,
            lbg:$lbg:ident,
            bow:$bow:ident,
            special:$special:expr
        ) => {{
            gen_tree(&pedia_ex.$label, &path, stringify!($label), $name)?;
            for (weapon_id, weapon) in &pedia_ex.$label.weapons {
                let file_path = path.join(format!("{}.html", weapon_id.to_tag()));
                gen_weapon(
                    weapon,
                    &pedia_ex.$label,
                    pedia_ex,
                    &file_path,
                    $element,
                    $second_element,
                    $close_range,
                    $horn,
                    $bullet,
                    $lbg,
                    $bow,
                    $special,
                )?;
            }
        }};
    }

    weapon!(
        great_sword,
        "Great sword",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        short_sword,
        "Sword & shield",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        hammer,
        "Hammer",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        lance,
        "Lance",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        long_sword,
        "Long sword",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        slash_axe,
        "Switch axe",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(slash_axe)
    );
    weapon!(
        gun_lance,
        "Gunlance",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(gun_lance)
    );
    weapon!(
        dual_blades,
        "Dual Blades",
        element: yes,
        second_element: yes,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        horn,
        "Hunting horn",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: yes,
        bullet: no,
        lbg: no,
        bow: no,
        special: None
    );
    weapon!(
        insect_glaive,
        "Insect glaive",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(insect_glaive)
    );
    weapon!(
        charge_axe,
        "Charge blade",
        element: yes,
        second_element: no,
        close_range: yes,
        horn: no,
        bullet: no,
        lbg: no,
        bow: no,
        special: Some(charge_axe)
    );
    weapon!(
        light_bowgun,
        "Light bowgun",
        element: no,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: yes,
        lbg: yes,
        bow: no,
        special: None
    );
    weapon!(
        heavy_bowgun,
        "Heavy bowgun",
        element: no,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: yes,
        lbg: no,
        bow: no,
        special: Some(heavy_bowgun)
    );
    weapon!(
        bow,
        "Bow",
        element: yes,
        second_element: no,
        close_range: no,
        horn: no,
        bullet: no,
        lbg: no,
        bow: yes,
        special: None
    );

    Ok(())
}
