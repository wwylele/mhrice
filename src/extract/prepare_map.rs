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
    ec_file: Option<&'static str>,
}

static MAP_FILES: [Option<MapFiles>; 16] = [
    None, // 0
    Some(MapFiles {
        // 1
        tex_files: &["gui/80_Texture/map/map_001_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_001.user",
        scene_file: "scene/m01/normal/m01_normal.scn",
        ec_file: Some("environmentCreature/UserData/m01_ECData.user"),
    }),
    Some(MapFiles {
        // 2
        tex_files: &[
            "gui/80_Texture/map/map_002_IAM.tex",
            "gui/80_Texture/map/map_002_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_002.user",
        scene_file: "scene/m02/normal/m02_normal.scn",
        ec_file: Some("environmentCreature/UserData/m02_ECData.user"),
    }),
    Some(MapFiles {
        // 3
        tex_files: &[
            "gui/80_Texture/map/map_003_IAM.tex",
            "gui/80_Texture/map/map_003_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_003.user",
        scene_file: "scene/m03/normal/m03_normal.scn",
        ec_file: Some("environmentCreature/UserData/m03_ECData.user"),
    }),
    Some(MapFiles {
        // 4
        tex_files: &[
            "gui/80_Texture/map/map_004_IAM.tex",
            "gui/80_Texture/map/map_004_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_004.user",
        scene_file: "scene/m04/normal/m04_normal.scn",
        ec_file: Some("environmentCreature/UserData/m04_ECData.user"),
    }),
    Some(MapFiles {
        // 5
        tex_files: &[
            "gui/80_Texture/map/map_005_IAM.tex",
            "gui/80_Texture/map/map_005_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_005.user",
        scene_file: "scene/m05/normal/m05_normal.scn",
        ec_file: Some("environmentCreature/UserData/m05_ECData.user"),
    }),
    None, // 6
    Some(MapFiles {
        // 7
        tex_files: &["gui/80_Texture/map/map_007_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_007_sp.user", // special type
        scene_file: "scene/m01/hyakuryu/m01_hyakuryu_B.scn",
        ec_file: None,
    }),
    None, // 8
    Some(MapFiles {
        // 9
        tex_files: &["gui/80_Texture/map/map_009_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_009.user",
        scene_file: "scene/m20/normal/m20_normal.scn",
        ec_file: Some("environmentCreature/UserData/m20_ECData.user"),
    }),
    Some(MapFiles {
        // 10
        tex_files: &["gui/80_Texture/map/map_010_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_010.user",
        scene_file: "scene/m21/normal/m21_normal.scn",
        ec_file: Some("environmentCreature/UserData/m21_ECData.user"),
    }),
    Some(MapFiles {
        // 11
        tex_files: &[
            "gui/80_Texture/map/map_011_IAM.tex",
            "gui/80_Texture/map/map_011_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_011.user",
        scene_file: "scene/m22/normal/m22_normal.scn",
        ec_file: Some("environmentCreature/UserData/m22_ECData.user"),
    }),
    Some(MapFiles {
        // 12
        tex_files: &[
            "gui/80_Texture/map/map_031_IAM.tex",
            "gui/80_Texture/map/map_031_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_031.user",
        scene_file: "scene/m31/normal/m31_normal.scn",
        ec_file: Some("environmentCreature/UserData/m31_ECData.user"),
    }),
    Some(MapFiles {
        // 13
        tex_files: &[
            "gui/80_Texture/map/map_032_IAM.tex",
            "gui/80_Texture/map/map_032_2_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_032.user",
        scene_file: "scene/m32/normal/m32_normal.scn",
        ec_file: Some("environmentCreature/UserData/m32_ECData.user"),
    }),
    Some(MapFiles {
        // 14
        tex_files: &["gui/80_Texture/map/map_041_IAM.tex"],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_041.user",
        scene_file: "scene/m41/normal/m41_normal.scn",
        ec_file: Some("environmentCreature/UserData/m41_ECData.user"),
    }),
    Some(MapFiles {
        // 15
        tex_files: &[
            "gui/80_Texture/map/map_042_IAM.tex",
            "gui/80_Texture/map/map_042_2_IAM.tex",
            "gui/80_Texture/map/map_042_3_IAM.tex",
        ],
        scale_file: "gui/01_Common/Map/MapScaleUserdata/GuiMapScaleDefineData_042.user",
        scene_file: "scene/m42/normal/m42_normal.scn",
        ec_file: Some("environmentCreature/UserData/m42_ECData.user"),
    }),
];

#[allow(clippy::large_enum_variant)]
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
    Ec {
        behavior: rsz::EnvironmentCreatureWrapper,
    },
    Fg {
        behavior: rsz::FieldGimmickWrapper,
    },
    Bush {
        behavior: Vec<rsz::DropObjectBehavior>,
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
    pub ec_data: Option<rsz::EnvironmentCreatureData>,
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

    let mut bush_groups: Vec<Vec<(Vec3, rsz::DropObjectBehavior)>> = vec![];

    scene.for_each_object(&mut |object: &GameObject, transforms: &[&rsz::Transform]| {
        // TODO: this isn't an accurate way to get the position, as it doesn't consider rotation and scaling
        let position: Vec3 = transforms.iter().map(|t| t.position.xzy()).sum();

        if let Ok(behavior) = object.get_component::<rsz::ItemPopBehavior>() {
            let relic = object.get_component::<rsz::RelicNoteUnlock>().ok();
            let kind = MapPopKind::Item {
                behavior: behavior.clone(),
                relic: relic.cloned(),
            };

            pops.push(MapPop { position, kind });
        } else if let Ok(behavior) = object.get_component::<rsz::WireLongJumpUnlock>() {
            // TODO: accurate way to rotate
            let transform = transforms.last().context("Expect at least one transform")?;
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
            let mut behavior = behavior.clone();
            behavior.fish_spawn_data.load(pak, None)?;

            let kind = MapPopKind::FishingPoint { behavior };

            pops.push(MapPop { position, kind });
        } else if let Ok(behavior) = object.get_component::<rsz::OtomoReconSpot>() {
            pops.push(MapPop {
                position,
                kind: MapPopKind::Recon {
                    behavior: behavior.clone(),
                },
            });
        } else if let Ok(behavior) = object
            .filter_component(|rsz| rsz::EC_TYPE_MAP.get(&rsz.symbol()).map(|f| f(rsz).unwrap()))
        {
            pops.push(MapPop {
                position,
                kind: MapPopKind::Ec { behavior },
            });
        } else if let Ok(behavior) = object
            .filter_component(|rsz| rsz::FG_TYPE_MAP.get(&rsz.symbol()).map(|f| f(rsz).unwrap()))
        {
            pops.push(MapPop {
                position,
                kind: MapPopKind::Fg { behavior },
            });
        } else if let Ok(behavior) = object.get_component::<rsz::DropObjectBehavior>() {
            let mut behavior = behavior.clone();
            behavior.env_creature_lottery_data.load(pak, None)?;
            let mut new_group = vec![];
            let mut i = 0;
            let max_dist = 4.0;
            while i < bush_groups.len() {
                if bush_groups[i]
                    .iter()
                    .any(|(bush_pos, _)| distance2(bush_pos, &position) < max_dist * max_dist)
                {
                    new_group.append(&mut bush_groups.remove(i));
                } else {
                    i += 1
                }
            }
            new_group.push((position, behavior));
            bush_groups.push(new_group);
        } else if let Ok(behavior) = object.get_component::<rsz::TentBehavior>() {
            pops.push(MapPop {
                position,
                kind: MapPopKind::Camp {
                    behavior: behavior.clone(),
                },
            });
        }

        Ok(true)
    })?;

    for group in bush_groups {
        let pos_sum: Vec3 = group.iter().map(|(p, _)| p).sum();
        let position = pos_sum / group.len() as f32;
        let behavior = group.into_iter().map(|(_, b)| b).collect();
        pops.push(MapPop {
            position,
            kind: MapPopKind::Bush { behavior },
        })
    }

    let ec_data = files
        .ec_file
        .map(|ec_file| -> Result<rsz::EnvironmentCreatureData> {
            let f = pak.find_file(ec_file)?;
            let mut data: rsz::EnvironmentCreatureData = User::new(Cursor::new(pak.read_file(f)?))?
                .rsz
                .deserialize_single(None)?;
            for table in &mut data.fg003_table_data {
                table.ec_data.load(pak, None)?;
            }
            Ok(data)
        })
        .transpose()?;

    Ok(Some(GameMap {
        layer_count: files.tex_files.len(),
        x_offset: scale.map_wide_min_pos,
        y_offset: scale.map_height_min_pos,
        map_scale: scale.map_scale,
        pops,
        ec_data,
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
