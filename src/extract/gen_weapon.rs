use super::gen_common::*;
use super::gen_item::*;
use super::gen_website::*;
use super::pedia::*;
use crate::rsz::*;
use anyhow::*;
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
            <span>{gen_multi_lang(&weapon.name)}</span>
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

fn gen_weapon<Param>(
    weapon: &Weapon<Param>,
    weapon_tree: &WeaponTree<'_, Param>,
    pedia_ex: &PediaEx,
    path: &Path,
    has_element: fn(&Param) -> Option<&ElementWeaponBaseData>,
    has_second_element: fn(&Param) -> Option<&DualBladesBaseUserDataParam>,
    has_close_range: fn(&Param) -> Option<&CloseRangeWeaponBaseData>,
    has_bullet: fn(&Param) -> Option<&BulletWeaponBaseUserDataParam>,
    special: Option<fn(&Param) -> Vec<Box<p<String>>>>,
) -> Result<()>
where
    Param: ToBase<MainWeaponBaseData>,
{
    let param = weapon.param;
    let main = param.to_base();
    let element = has_element(param);
    let second_element = has_second_element(param);
    let close_range = has_close_range(param);
    let bullet = has_bullet(param);

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
                    {gen_weapon_icon(&main)}
                </div>
                <h1 class="title">
                    {gen_multi_lang(&weapon.name)}
                </h1>

                <section class="section"><p>
                    {gen_multi_lang(&weapon.explain)}
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

                {element.map(|element| html!(
                    <p class="mh-kv"><span>"Element"</span>
                    <span>
                        <span>{text!("{:?} {}",
                            element.main_element_type,
                            element.main_element_val
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

                { special.map(|special|special(param)).into_iter().flatten() }

                </div>
                </section>

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
                        let weapon = weapon_tree.weapons.get(&child).unwrap();
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
                { gen_weapon_label(&weapon) }
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

pub fn gen_weapons(pedia_ex: &PediaEx, root: &Path) -> Result<()> {
    let path = root.join("weapon");
    create_dir(&path)?;

    macro_rules! weapon {
        ($label:ident, $name:expr,
            element:$element:ident,
            second_element:$second_element:ident,
            close_range:$close_range:ident,
            bullet:$bullet:ident,
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
                    $bullet,
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
        bullet: no,
        special: None
    );
    weapon!(
        short_sword,
        "Sword & shield",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        hammer,
        "Hammer",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        lance,
        "Lance",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        long_sword,
        "Long sword",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        slash_axe,
        "Switch axe",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: Some(slash_axe)
    );
    weapon!(
        gun_lance,
        "Gunlance",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: Some(gun_lance)
    );
    weapon!(
        dual_blades,
        "Dual Blades",
        element: yes,
        second_element: yes,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        horn,
        "Hunting horn",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: None
    );
    weapon!(
        insect_glaive,
        "Insect glaive",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: Some(insect_glaive)
    );
    weapon!(
        charge_axe,
        "Charge blade",
        element: yes,
        second_element: no,
        close_range: yes,
        bullet: no,
        special: Some(charge_axe)
    );
    weapon!(
        light_bowgun,
        "Light bowgun",
        element: no,
        second_element: no,
        close_range: no,
        bullet: yes,
        special: None
    );
    weapon!(
        heavy_bowgun,
        "Heavy bowgun",
        element: no,
        second_element: no,
        close_range: no,
        bullet: yes,
        special: None
    );
    weapon!(
        bow,
        "Bow",
        element: yes,
        second_element: no,
        close_range: no,
        bullet: no,
        special: None
    );

    Ok(())
}
