use super::gen_website::*;
use super::pedia::*;
use crate::rsz::*;
use anyhow::*;
use std::fs::{create_dir, write};
use std::path::*;
use typed_html::{dom::*, elements::*, html, text};

fn gen_tree_rec<Param>(weapon_tree: &WeaponTree<Param>, list: &[WeaponId]) -> Box<ul<String>> {
    html!(<ul> {
        list.iter().map(|id| {
            let weapon = weapon_tree.weapons.get(id).unwrap();
            html!(<li>
                { gen_multi_lang(weapon.name) }
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
) -> Result<()> {
    let list_path = weapon_path.join(format!("{}.html", tag));

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("{} - MHRice", name)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <h1 class="title">
                    {text!("{}", name)}
                </h1>
                <div class="mh-weapon-tree">
                { gen_tree_rec(weapon_tree, &weapon_tree.roots) }
                </div>
                </div></div></main>
            </body>
        </html>
    );

    write(&list_path, doc.to_string())?;

    Ok(())
}

pub fn gen_weapons(pedia_ex: &PediaEx, root: &Path) -> Result<()> {
    let path = root.join("weapon");
    create_dir(&path)?;
    gen_tree(&pedia_ex.great_sword, &path, "great_sword", "Great sword")?;
    gen_tree(
        &pedia_ex.short_sword,
        &path,
        "short_sword",
        "Sword & shield",
    )?;
    gen_tree(&pedia_ex.hammer, &path, "hammer", "Hammer")?;
    gen_tree(&pedia_ex.lance, &path, "lance", "Lance")?;
    gen_tree(&pedia_ex.long_sword, &path, "long_sword", "Long sword")?;
    gen_tree(&pedia_ex.slash_axe, &path, "slash_axe", "Switch axe")?;
    gen_tree(&pedia_ex.gun_lance, &path, "gun_lance", "Gunlance")?;
    gen_tree(&pedia_ex.dual_blades, &path, "dual_blades", "Dual Blades")?;
    gen_tree(&pedia_ex.horn, &path, "horn", "Hunting horn")?;
    gen_tree(
        &pedia_ex.insect_glaive,
        &path,
        "insect_glaive",
        "Insect glaive",
    )?;
    gen_tree(&pedia_ex.charge_axe, &path, "charge_axe", "Charge blade")?;
    gen_tree(
        &pedia_ex.light_bowgun,
        &path,
        "light_bowgun",
        "Light bowgun",
    )?;
    gen_tree(
        &pedia_ex.heavy_bowgun,
        &path,
        "heavy_bowgun",
        "Heavy bowgun",
    )?;
    gen_tree(&pedia_ex.bow, &path, "bow", "Bow")?;

    Ok(())
}
