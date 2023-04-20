use super::sink::Sink;
use crate::pak::*;
use crate::rsz;
use crate::rsz::FromRsz;
use crate::scn::*;
use crate::tex::*;
use crate::user::*;
use anyhow::{anyhow, bail, Context, Result};
use nalgebra::*;
use nalgebra_glm::*;
use serde::*;
use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek};
use std::rc::*;

struct MapFiles {
    tex_files: &'static [&'static str],
    scale_file: &'static str,
    scene_file: &'static str,
}

static MAP_FILES: [Option<MapFiles>; 16] = [
    None, // 0
    Some(MapFiles {
        // 1
        tex_files: &["gui/80_Texture/map/map_001_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_001.user",
        scene_file: "scene/m01/normal/m01_normal.scn",
    }),
    Some(MapFiles {
        // 2
        tex_files: &[
            "gui/80_Texture/map/map_002_IAM.tex",
            "gui/80_Texture/map/map_002_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_002.user",
        scene_file: "scene/m02/normal/m02_normal.scn",
    }),
    Some(MapFiles {
        // 3
        tex_files: &[
            "gui/80_Texture/map/map_003_IAM.tex",
            "gui/80_Texture/map/map_003_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_003.user",
        scene_file: "scene/m03/normal/m03_normal.scn",
    }),
    Some(MapFiles {
        // 4
        tex_files: &[
            "gui/80_Texture/map/map_004_IAM.tex",
            "gui/80_Texture/map/map_004_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_004.user",
        scene_file: "scene/m04/normal/m04_normal.scn",
    }),
    Some(MapFiles {
        // 5
        tex_files: &[
            "gui/80_Texture/map/map_005_IAM.tex",
            "gui/80_Texture/map/map_005_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_005.user",
        scene_file: "scene/m05/normal/m05_normal.scn",
    }),
    None, // 6
    Some(MapFiles {
        // 7
        tex_files: &["gui/80_Texture/map/map_007_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_007_sp.user", // special type
        scene_file: "scene/m01/hyakuryu/m01_hyakuryu_A.scn",
    }),
    None, // 8
    Some(MapFiles {
        // 9
        tex_files: &["gui/80_Texture/map/map_009_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_009.user",
        scene_file: "scene/m20/normal/m20_normal.scn",
    }),
    Some(MapFiles {
        // 10
        tex_files: &["gui/80_Texture/map/map_010_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_010.user",
        scene_file: "scene/m21/normal/m21_normal.scn",
    }),
    Some(MapFiles {
        // 11
        tex_files: &[
            "gui/80_Texture/map/map_011_IAM.tex",
            "gui/80_Texture/map/map_011_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_011.user",
        scene_file: "scene/m22/normal/m22_normal.scn",
    }),
    Some(MapFiles {
        // 12
        tex_files: &[
            "gui/80_Texture/map/map_031_IAM.tex",
            "gui/80_Texture/map/map_031_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_031.user",
        scene_file: "scene/m31/normal/m31_normal.scn",
    }),
    Some(MapFiles {
        // 13
        tex_files: &[
            "gui/80_Texture/map/map_032_IAM.tex",
            "gui/80_Texture/map/map_032_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_032.user",
        scene_file: "scene/m32/normal/m32_normal.scn",
    }),
    Some(MapFiles {
        // 14
        tex_files: &["gui/80_Texture/map/map_041_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_041.user",
        scene_file: "scene/m41/normal/m41_normal.scn",
    }),
    Some(MapFiles {
        // 15
        tex_files: &[
            "gui/80_Texture/map/map_042_IAM.tex",
            "gui/80_Texture/map/map_042_2_IAM.tex",
            "gui/80_Texture/map/map_042_3_IAM.tex",
        ],
        // This scale doesn't look right
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_042.user",
        scene_file: "scene/m42/normal/m42_normal.scn",
    }),
];

#[derive(Debug, Serialize)]
pub enum MapPopKind {
    Item {
        behavior: rsz::ItemPopBehavior,
        relic: Option<rsz::RelicNoteUnlock>,
    },
    WireLongJump {
        behavior: rsz::WireLongJumpUnlock,
        angle: f32,
    },
    Camp {
        behavior: rsz::TentBehavior,
    },
    FishingPoint {
        behavior: rsz::FishingPoint,
    },
    Recon {
        behavior: rsz::OtomoReconSpot,
    },
}

#[derive(Debug, Serialize)]
pub struct MapPop {
    pub position: Vec3,
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

fn get_map<F: Read + Seek>(pak: &mut PakReader<F>, files: &MapFiles) -> Result<Option<GameMap>> {
    if pak.find_file(files.scene_file).is_err() {
        return Ok(None);
    }

    let scale = pak.find_file(files.scale_file)?;
    let scale = User::new(Cursor::new(pak.read_file(scale)?))?
        .rsz
        .deserialize_single_any(None)?;

    let scale: rsz::GuiMapScaleDefineData = if scale.symbol() == rsz::GuiMapScaleDefineData::SYMBOL
    {
        Rc::try_unwrap(scale.downcast().unwrap()).map_err(|_| anyhow!("Shared node"))?
    } else if scale.symbol() == rsz::GuiMap07DefineData::SYMBOL {
        let scale: rsz::GuiMap07DefineData =
            Rc::try_unwrap(scale.downcast().unwrap()).map_err(|_| anyhow!("Shared node"))?;
        scale.base.0
    } else {
        bail!("Unknown map scale type {}", scale.symbol())
    };

    let scene = Scene::new(pak, files.scene_file)?;

    let mut pops = vec![];

    scene.for_each_object(&mut |object: &GameObject| {
        if object
            .get_component::<rsz::M31IsletArrivalChecker>()
            .is_ok()
        {
            return Ok(true);
        } else if let Ok(behavior) = object.get_component::<rsz::ItemPopBehavior>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;

            let position = transform.position.xzy();
            let relic = object.get_component::<rsz::RelicNoteUnlock>().ok();
            let kind = MapPopKind::Item {
                behavior: behavior.clone(),
                relic: relic.cloned(),
            };

            pops.push(MapPop { position, kind });
        } else if let Ok(behavior) = object.get_component::<rsz::WireLongJumpUnlock>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;

            let position = transform.position.xzy();

            let mat = geometry::UnitQuaternion::new_normalize(Quat::from(transform.rotation))
                .to_rotation_matrix();
            let tester = make_vec3(&[1.0, 0.0, 0.0]);
            let rotated = mat * tester;
            let angle = f32::atan2(rotated.x, rotated.z);

            let kind = MapPopKind::WireLongJump {
                behavior: behavior.clone(),
                angle,
            };

            pops.push(MapPop { position, kind });
        } else if let Ok(behavior) = object.get_component::<rsz::FishingPoint>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;

            let position = transform.position.xzy();
            let mut behavior = behavior.clone();
            behavior.fish_spawn_data.load(pak, None)?;

            let kind = MapPopKind::FishingPoint { behavior };

            pops.push(MapPop { position, kind });
        } else if let Ok(behavior) = object.get_component::<rsz::OtomoReconSpot>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;
            let position = transform.position.xzy();
            pops.push(MapPop {
                position,
                kind: MapPopKind::Recon {
                    behavior: behavior.clone(),
                },
            });
        } else if let Ok(behavior) = object.get_component::<rsz::TentBehavior>() {
            let transform = object
                .get_component::<rsz::Transform>()
                .context("Lack of transform")?;
            let position = transform.position.xzy();
            pops.push(MapPop {
                position,
                kind: MapPopKind::Camp {
                    behavior: behavior.clone(),
                },
            });
        } else {
            for child in &object.children {
                if let Ok(behavior) = child.get_component::<rsz::TentBehavior>() {
                    let transform = object
                        .get_component::<rsz::Transform>()
                        .context("Lack of transform")?;
                    let sub_transform = child
                        .get_component::<rsz::Transform>()
                        .context("Lack of transform")?;
                    let position = transform.position.xzy() + sub_transform.position.xzy();
                    pops.push(MapPop {
                        position,
                        kind: MapPopKind::Camp {
                            behavior: behavior.clone(),
                        },
                    });
                }
            }
        }

        Ok(false)
    })?;

    Ok(Some(GameMap {
        layer_count: files.tex_files.len(),
        x_offset: scale.map_wide_min_pos,
        y_offset: scale.map_height_min_pos,
        map_scale: scale.map_scale,
        pops,
    }))
}

pub fn prepare_maps(pak: &mut PakReader<impl Read + Seek>) -> Result<BTreeMap<i32, GameMap>> {
    MAP_FILES
        .iter()
        .enumerate()
        .filter_map(|(i, f)| f.as_ref().map(|f| (i as i32, f)))
        .filter_map(|(i, f)| {
            let game_map = match get_map(pak, f) {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            game_map.map(|game_map| Ok((i, game_map)))
        })
        .collect()
}

pub fn gen_map_resource(pak: &mut PakReader<impl Read + Seek>, output: &impl Sink) -> Result<()> {
    for (i, f) in MAP_FILES.iter().enumerate() {
        if let Some(f) = f {
            for (j, &name) in f.tex_files.iter().enumerate() {
                let tex = if let Ok(tex) = pak.find_file(name) {
                    tex
                } else {
                    continue;
                };
                let output_file = output.create(&format!("map{i:02}_{j}.png"))?;
                Tex::new(Cursor::new(pak.read_file(tex)?))?.save_png(0, 0, output_file)?
            }
        }
    }
    Ok(())
}
