use super::sink::Sink;
use crate::pak::*;
use crate::rsz;
use crate::scn::*;
use crate::tex::*;
use crate::user::*;
use anyhow::{Context, Result};
use serde::*;
use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek};

struct MapFiles {
    tex_files: &'static [&'static str],
    scale_file: &'static str,
    scene_file: &'static str,
}

static MAP_FILES: [Option<MapFiles>; 15] = [
    None, // 0
    Some(MapFiles {
        // 1
        tex_files: &["gui/80_Texture/map/map_001_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_001.user",
        scene_file: "scene/m01/normal/m01_normal.scn",
    }),
    Some(MapFiles {
        // 2
        tex_files: &["gui/80_Texture/map/map_002_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_002.user",
        scene_file: "scene/m02/normal/m02_normal.scn",
    }),
    None, // 3
    None, // 4
    None, // 5
    None, // 6
    None, // 7
    None, // 8
    None, // 9
    None, // 10
    None, // 11
    None, // 12
    None, // 13
    None, // 14
];

#[derive(Debug, Serialize)]
pub enum MapPopKind {
    Relic { id: i32, map: i32 },
    Stuff,
}

#[derive(Debug, Serialize)]
pub struct MapPop {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pop_behavior: rsz::ItemPopBehavior,
    pub kind: MapPopKind,
}

#[derive(Debug, Serialize)]
pub struct GameMap {
    pub layer_count: usize,
    pub x_offset: f32,
    pub y_offset: f32,
    pub map_scale: f32,
    pub pops: Vec<MapPop>,
}

fn get_map<F: Read + Seek>(pak: &mut PakReader<F>, files: &MapFiles) -> Result<GameMap> {
    let scale = pak.find_file(files.scale_file)?;
    let scale: rsz::GuiMapScaleDefineData = User::new(Cursor::new(pak.read_file(scale)?))?
        .rsz
        .deserialize_single()?;

    let scene = Scene::new(pak, files.scene_file)?;

    let mut pops = vec![];

    scene.for_each_free_object(&mut |object: &GameObject| {
        if let Ok(pop_behavior) = object.get_component::<rsz::ItemPopBehavior>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;

            let x = transform.position.x;
            let y = transform.position.z; // swap the cursed y/z
            let z = transform.position.y;

            let kind = if let Ok(relic) = object.get_component::<rsz::RelicNoteUnlock>() {
                MapPopKind::Relic {
                    id: relic.relic_id,
                    map: relic.note_map_no,
                }
            } else {
                //return Ok(());
                MapPopKind::Stuff
            };

            pops.push(MapPop {
                x,
                y,
                z,
                kind,
                pop_behavior: pop_behavior.clone(),
            });
        }

        Ok(())
    })?;

    Ok(GameMap {
        layer_count: files.tex_files.len(),
        x_offset: scale.map_wide_min_pos,
        y_offset: scale.map_height_min_pos,
        map_scale: scale.map_scale,
        pops,
    })
}

pub fn prepare_maps(pak: &mut PakReader<impl Read + Seek>) -> Result<BTreeMap<i32, GameMap>> {
    MAP_FILES
        .iter()
        .enumerate()
        .filter_map(|(i, f)| f.as_ref().map(|f| (i as i32, f)))
        .map(|(i, f)| Ok((i, get_map(pak, f)?)))
        .collect()
}

pub fn gen_map_resource(pak: &mut PakReader<impl Read + Seek>, output: &impl Sink) -> Result<()> {
    for (i, f) in MAP_FILES.iter().enumerate() {
        if let Some(f) = f {
            for (j, &name) in f.tex_files.iter().enumerate() {
                let tex = pak.find_file(name)?;
                let output_file = output.create(&format!("map{i:02}_{j}.png"))?;
                Tex::new(Cursor::new(pak.read_file(tex)?))?.save_png(0, 0, output_file)?
            }
        }
    }
    Ok(())
}
