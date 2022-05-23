#![allow(unused_braces)]

mod gen_armor;
mod gen_common;
mod gen_hyakuryu_skill;
mod gen_item;
mod gen_map;
mod gen_monster;
mod gen_pedia;
mod gen_quest;
mod gen_skill;
mod gen_weapon;
mod gen_website;
mod gen_otomo;
mod pedia;
mod prepare_map;
pub mod sink;

pub use gen_pedia::gen_resources;
pub use gen_pedia::{gen_pedia, gen_pedia_ex};
pub use gen_website::{gen_multi_lang, gen_website};
pub use pedia::*;
