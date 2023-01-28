use crate::align::*;
use crate::file_ext::*;
use anyhow::{bail, Context, Result};
use half::f16;
use nalgebra_glm::*;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
pub struct Model {
    pub material_index: u32,
    pub index_count: u32,
    pub index_buffer_start: u32,
    pub vertex_buffer_start: u32,
}

#[derive(Clone)]
pub struct ModelGroup {
    pub group_id: u8,
    pub models: Vec<Model>,
}

#[derive(Clone)]
pub struct ModelLod {
    pub model_groups: Vec<ModelGroup>,
}

#[derive(Debug)]
pub struct VertexLayout {
    pub usage: u16, // position, normal, uv, uv2, weight, ?
    pub width: u16,
    pub offset: u32,
}

pub struct Point4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct G {
    pub a: Point4,
    pub b: Point4,
}

pub struct Bone {
    pub parent: Option<usize>,
    pub first_child: Option<usize>,
    pub sibling: Option<usize>,
    pub relative_transform: Mat4x4,
    pub absolute_transform: Mat4x4,
    pub absolute_reverse: Mat4x4,
    pub name: String,
}

pub struct Mesh {
    pub main_model_lods: Vec<ModelLod>,
    pub aux_model_lods: Vec<ModelLod>,
    pub material_names: Vec<String>,
    pub vertex_layouts: Vec<VertexLayout>,
    pub main_vertex_layout_count: usize,
    pub vertex_buffer: Vec<u8>,
    pub index_buffer: Vec<u8>,
    pub gs: Vec<G>,
    pub bones: Vec<Bone>,
    pub bone_names: HashMap<String, usize>,
    pub bone_remap: Vec<u16>,
}

impl Mesh {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Mesh> {
        const VERSION_A: u32 = 0x77a2d00d;
        const VERSION_B: u32 = 0x0141D2B8;
        const VERSION_C: u32 = 0x014160A8;

        if &file.read_magic()? != b"MESH" {
            bail!("Wrong magic for MESH");
        }
        let version = file.read_u32()?;
        if !matches!(version, VERSION_A | VERSION_B | VERSION_C) {
            bail!("Wrong version for MESH");
        }
        let total_len = file.read_u32()?.into();
        let _what = file.read_u32()?;

        let _a_count = file.read_u16()?;
        let string_count = file.read_u16()?;
        let x = file.read_u32()?;
        if x != 0 {
            bail!("Expected 0");
        }

        let main_models_offset = file.read_u64()?;
        let aux_model_offset = file.read_u64()?;
        let c_offset = file.read_u64()?;
        let skeleton_offset = file.read_u64()?;
        let e_offset = file.read_u64()?;
        let blend_shape_offset = file.read_u64()?;
        let g_offset = file.read_u64()?; // after string table
        let mesh_data_offset = file.read_u64()?; // after string table
        let i_offset = file.read_u64()?;
        let materials_offset = file.read_u64()?; // lists to string table entry
        let bone_names_offset = file.read_u64()?; // lists to string table entry
        let blend_shape_names_offset = file.read_u64()?; // lists to string table entry
        let string_table_offset = file.read_u64()?; // string table

        let mut model_lod_cache: HashMap<u64, ModelLod> = HashMap::new();

        let read_model_group = |file: &mut F, model_group_offset| {
            file.seek_noop(model_group_offset)?;

            let group_id = file.read_u8()?;
            let model_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let x = file.read_u32()?;
            if x != 0 {
                bail!("Expected 0: {x}");
            }

            // How much vertex buffer to transfer to render this group
            let _vertex_count = file.read_u32()?;

            let total_index_count_aligned = file.read_u32()?;

            let models = (0..model_count)
                .map(|_| {
                    let material_index = file.read_u32()?;
                    let index_count = file.read_u32()?;
                    let index_buffer_start = file.read_u32()?;
                    let vertex_buffer_start = file.read_u32()?;

                    let _ = file.read_u32()?;
                    let _ = file.read_u32()?;

                    Ok(Model {
                        material_index,
                        index_count,
                        index_buffer_start,
                        vertex_buffer_start,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            let total_index_count_expected: u32 =
                models.iter().map(|m| align_up(m.index_count, 2)).sum();
            if total_index_count_aligned != total_index_count_expected {
                bail!(
                    "unexpected aligned index count {total_index_count_aligned} \
                    for index count {total_index_count_expected}"
                );
            }

            Ok(ModelGroup { group_id, models })
        };

        let mut read_model_lod = |file: &mut F, model_lod_offset| -> Result<ModelLod> {
            if let Some(cache) = model_lod_cache.get(&model_lod_offset) {
                return Ok(cache.clone());
            }

            file.seek_noop(model_lod_offset)?;

            let model_group_count = file.read_u32()?;
            let _lod_range = file.read_f32()?; // controls when to choose this LOD?
            let model_group_list_offset = file.read_u64()?;
            file.seek_noop(model_group_list_offset)?;

            let model_group_offsets = (0..model_group_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;

            let model_groups = model_group_offsets
                .into_iter()
                .map(|model_group_offset| read_model_group(file, model_group_offset))
                .collect::<Result<Vec<_>>>()?;

            let ab = ModelLod { model_groups };
            model_lod_cache.insert(model_lod_offset, ab.clone());
            Ok(ab)
        };

        let material_count;
        let main_model_lods;
        if main_models_offset != 0 {
            file.seek_noop(main_models_offset)?;
            let model_lod_count = file.read_u8()?;
            material_count = file.read_u8()?;
            let _a3_count = file.read_u8()?;
            let _a4_count = file.read_u8()?;
            let _a5_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let _f0 = file.read_f32()?;
            let _f1 = file.read_f32()?;
            let _f2 = file.read_f32()?;
            let _f3 = file.read_f32()?;

            let _g0 = file.read_f32()?;
            let _g1 = file.read_f32()?;
            let _g2 = file.read_f32()?;
            let _g3 = file.read_f32()?;

            let _h0 = file.read_f32()?;
            let _h1 = file.read_f32()?;
            let _h2 = file.read_f32()?;
            let _h3 = file.read_f32()?;

            let model_lod_list_offset = file.read_u64()?;
            file.seek_noop(model_lod_list_offset)?;

            let model_lod_offsets = (0..model_lod_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;

            main_model_lods = model_lod_offsets
                .into_iter()
                .map(|offset| read_model_lod(&mut file, offset))
                .collect::<Result<Vec<_>>>()?;
        } else {
            material_count = 0;
            main_model_lods = vec![];
        }

        let aux_model_lods = if aux_model_offset != 0 {
            file.seek_noop(aux_model_offset)?;

            let lod_count = file.read_u8()?;
            let _b2_count = file.read_u8()?;
            let _b3_count = file.read_u8()?;
            let _b4_count = file.read_u8()?;
            let _b5_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let lod_list_offset = file.read_u64()?;

            let _f0 = file.read_f32()?;
            let _f1 = file.read_f32()?;
            let _f2 = file.read_f32()?;
            let _f3 = file.read_f32()?;

            let _g0 = file.read_f32()?;
            let _g1 = file.read_f32()?;
            let _g2 = file.read_f32()?;
            let _g3 = file.read_f32()?;

            let _h0 = file.read_f32()?;
            let _h1 = file.read_f32()?;
            let _h2 = file.read_f32()?;
            let _h3 = file.read_f32()?;

            file.seek_noop(lod_list_offset)?;

            let lod_offsets = (0..lod_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;

            lod_offsets
                .into_iter()
                .map(|offset| read_model_lod(&mut file, offset))
                .collect::<Result<Vec<_>>>()?
        } else {
            vec![]
        };

        // verify that model name indices are all in-bound
        for set in [&main_model_lods, &aux_model_lods] {
            for lod in set {
                for group in &lod.model_groups {
                    for model in &group.models {
                        if model.material_index >= u32::from(material_count) {
                            bail!("model name index {} out of bound", model.material_index);
                        }
                    }
                }
            }
        }

        if c_offset != 0 {
            file.seek_noop(c_offset)?;

            let c1_count = file.read_u8()?;
            let _c2_count = file.read_u8()?;
            let _c3_count = file.read_u8()?;
            let _c4_count = file.read_u8()?;
            let _f0 = file.read_f32()?;

            let ca_offset = file.read_u64()?;

            file.seek_noop(ca_offset)?;

            let cb_offsets = (0..c1_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;

            cb_offsets
                .into_iter()
                .map(|offset| read_model_group(&mut file, offset))
                .collect::<Result<Vec<_>>>()?;
        }

        let bone_count;
        let bone_remap;
        let mut bones;
        if skeleton_offset != 0 {
            file.seek_assert_align_up(skeleton_offset, 8)?;
            bone_count = file.read_u32()?;
            let bone_remap_count = file.read_u32()?;
            let x = file.read_u64()?;
            if x != 0 {
                bail!("Expected zero");
            }
            let bone_hierarchy_offset = file.read_u64()?;
            let bone_matrix_a_offset = file.read_u64()?;
            let bone_matrix_b_offset = file.read_u64()?;
            let bone_matrix_c_offset = file.read_u64()?;

            bone_remap = (0..bone_remap_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_hierarchy_offset, 16)?;

            struct BoneInfo {
                parent_index: u16,
                sibling_index: u16,
                first_child_index: u16,
            }
            let bone_infos = (0..bone_count)
                .map(|i| {
                    let index = file.read_u16()?;
                    if i != u32::from(index) {
                        bail!("Unexpected index")
                    }
                    let parent_index = file.read_u16()?;
                    let sibling_index = file.read_u16()?;
                    let first_child_index = file.read_u16()?;
                    let _ = file.read_u16()?;
                    file.seek_align_up(16)?;
                    Ok(BoneInfo {
                        parent_index,
                        sibling_index,
                        first_child_index,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_a_offset, 16)?;

            // join to join transform?
            let bone_rel_transform = (0..bone_count)
                .map(|_| file.read_f32m4x4())
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_b_offset, 16)?;

            // base to join transform?
            let bone_abs_transform = (0..bone_count)
                .map(|_| file.read_f32m4x4())
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_c_offset, 16)?;

            // join to base transform?
            let bone_abs_reverse = (0..bone_count)
                .map(|_| file.read_f32m4x4())
                .collect::<Result<Vec<_>>>()?;

            fn convert_index(index: u16) -> Result<Option<usize>> {
                if index == 0xFFFF {
                    Ok(None)
                } else {
                    Ok(Some(index.try_into()?))
                }
            }

            bones = (0..usize::try_from(bone_count)?)
                .map(|i| {
                    if !(bone_abs_transform[i] * bone_abs_reverse[i]).is_identity(0.001) {
                        eprintln!("bone_abs_transform * bone_abs_reverse verfication failed: expected identity")
                    }
                    let parent = convert_index(bone_infos[i].parent_index)?;
                    if let Some(parent) = parent {
                        if !(bone_abs_transform[parent]
                            * bone_rel_transform[i]
                            * bone_abs_reverse[i])
                            .is_identity(0.001)
                        {
                            eprintln!("bone_rel_transform verfication failed: expected identity")
                        }
                    }
                    Ok(Bone {
                        parent,
                        first_child: convert_index(bone_infos[i].first_child_index)?,
                        sibling: convert_index(bone_infos[i].sibling_index)?,
                        relative_transform: bone_rel_transform[i],
                        absolute_transform: bone_abs_transform[i],
                        absolute_reverse: bone_abs_reverse[i],
                        name: "".to_string(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
        } else {
            bone_count = 0;
            bone_remap = vec![];
            bones = vec![];
        }

        if e_offset != 0 {
            // this only worked for version A
            /*file.seek_assert_align_up(e_offset, 16)?;
            let p_offset = file.read_u64()?;
            let _q_offset = file.read_u64()?;
            file.seek_noop(p_offset)?;*/
            //..
            //file.seek_noop(q_offset)?;
            //..
        }

        let mut blend_shape_name_count = 0;
        if blend_shape_offset != 0 {
            file.seek(SeekFrom::Start(blend_shape_offset))?;
            //file.seek_assert_align_up(blend_shape_offset, 8)?;
            let blend_shape_count = file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.seek_align_up(8)?;
            let fa_offset = file.read_u64()?;
            if version == VERSION_B || version == VERSION_C {
                file.read_u64()?;
                file.read_u64()?;
            }
            file.seek_noop(fa_offset)?;
            let fb_offsets = (0..blend_shape_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;
            fb_offsets
                .into_iter()
                .map(|offset| {
                    //file.seek_noop(offset)?;
                    file.seek(SeekFrom::Start(offset))?;
                    file.read_u64()?;
                    file.read_u64()?;
                    let m_offset = file.read_u64()?;
                    let n_offset = file.read_u64()?;
                    if version == VERSION_B || version == VERSION_C {
                        let _mm_offset = file.read_u64()?;
                        let _nn_offset = file.read_u64()?;
                    }
                    file.seek_noop(m_offset)?;
                    file.read_u64()?;
                    file.read_u16()?;
                    let name_count = file.read_u16()?;
                    blend_shape_name_count += name_count;
                    file.read_u32()?;
                    file.seek_noop(n_offset)?;
                    file.read_u64()?;
                    file.read_u64()?;
                    file.read_u64()?;
                    file.read_u64()?;
                    // There are some more data at _mm_offset and _nn_offset
                    Ok(())
                })
                .collect::<Result<Vec<_>>>()?;
        }

        //***

        let next_offsets = [
            i_offset,
            materials_offset,
            bone_names_offset,
            blend_shape_names_offset,
            string_table_offset,
        ];
        let next_offset = *next_offsets.iter().find(|x| **x != 0).unwrap();
        file.seek(SeekFrom::Start(next_offset))?;

        //***

        if i_offset != 0 {
            file.seek_assert_align_up(i_offset, 8)?;
            file.seek(SeekFrom::Start(i_offset))?;
            let len = usize::try_from(file.read_u32()?)?;
            file.seek_align_up(8)?;
            let i_array_offset = file.read_u64()?;
            file.seek_noop(i_array_offset)?;
            let mut i_buffer = vec![0; len];
            file.read_exact(&mut i_buffer)?;
        }

        let material_names = if materials_offset != 0 {
            file.seek_assert_align_up(materials_offset, 16)?;
            (0..material_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?
        } else {
            vec![]
        };

        let bone_names = if bone_names_offset != 0 {
            file.seek_assert_align_up(bone_names_offset, 16)?;
            (0..bone_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?
        } else {
            vec![]
        };

        let blend_shape_names = if blend_shape_names_offset != 0 {
            file.seek_assert_align_up(blend_shape_names_offset, 16)?;
            (0..blend_shape_name_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?
        } else {
            vec![]
        };

        if material_count as u32 + bone_count + blend_shape_name_count as u32 != string_count as u32
        {
            bail!("Strange count")
        }

        file.seek_assert_align_up(string_table_offset, 16)?;
        let string_offsets = (0..string_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_align_up(16)?;
        let strings = string_offsets
            .into_iter()
            .map(|o| {
                file.seek_noop(o)?;
                let mut buf = vec![];
                loop {
                    let c = file.read_u8()?;
                    if c == 0 {
                        break;
                    }
                    buf.push(c);
                }
                Ok(String::from_utf8(buf)?)
            })
            .collect::<Result<Vec<_>>>()?;

        let material_names = material_names
            .into_iter()
            .map(|name_index| {
                Ok(strings
                    .get(usize::try_from(name_index)?)
                    .context("Material name out of bound")?
                    .clone())
            })
            .collect::<Result<Vec<_>>>()?;

        let _blend_shape_names = blend_shape_names
            .into_iter()
            .map(|name_index| {
                Ok(strings
                    .get(usize::try_from(name_index)?)
                    .context("Blend shape name out of bound")?
                    .clone())
            })
            .collect::<Result<Vec<_>>>()?;

        let bone_names = bone_names
            .into_iter()
            .enumerate()
            .map(|(bone_index, name_index)| {
                let name = strings
                    .get(usize::try_from(name_index)?)
                    .context("Bone name out of bound")?
                    .clone();
                bones[bone_index].name = name.clone();
                Ok((name, bone_index))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        if bone_names.len() != bones.len() {
            bail!("Bone name collision")
        }

        let gs = if g_offset != 0 {
            file.seek_assert_align_up(g_offset, 16)?;

            let g_count = file.read_u32()?;
            file.seek_align_up(8)?;
            let g_array_offset = file.read_u64()?;
            file.seek_noop(g_array_offset)?;

            let read_point4 = |file: &mut F| -> Result<Point4> {
                Ok(Point4 {
                    x: file.read_f32()?,
                    y: file.read_f32()?,
                    z: file.read_f32()?,
                    w: file.read_f32()?,
                })
            };

            (0..g_count)
                .map(|_| {
                    let a = read_point4(&mut file)?;
                    let b = read_point4(&mut file)?;
                    Ok(G { a, b })
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            vec![]
        };

        file.seek_assert_align_up(mesh_data_offset, 16)?;
        let vertex_layout_offset = file.read_u64()?;
        let vertex_buffer_offset = file.read_u64()?;
        let index_buffer_offset = file.read_u64()?;
        let _zinogre_offset = (version == VERSION_B || version == VERSION_C)
            .then(|| file.read_u64())
            .transpose()?;
        let vertex_buffer_len = usize::try_from(file.read_u32()?)?;
        let index_buffer_len = usize::try_from(file.read_u32()?)?;

        let main_vertex_layout_count = file.read_u16()?; // sometimes half of layout count
        let vertex_layout_count = file.read_u16()?; // sometimes it looks like it has two sets
        if main_vertex_layout_count > vertex_layout_count {
            bail!(
                "main_vertex_layout_count {main_vertex_layout_count} is larger \
            than vertex_layout_count {vertex_layout_count}"
            );
        }

        let _short_index_buffer_len = file.read_u32()?; // sometimes equal, sometimes smaller, sometimes 0
        let _ = file.read_u32()?; // ?

        // offset into vertex buffer
        // If doesn't exist, this is 0x100000000-vertex_buffer_offset
        let _what_offset = file.read_u32()?;

        let _zinogre = (version == VERSION_B || version == VERSION_C)
            .then(|| file.read_u64())
            .transpose()?;

        file.seek_noop(vertex_layout_offset)?;
        let vertex_layouts = (0..vertex_layout_count)
            .map(|_| {
                let usage = file.read_u16()?;
                let width = file.read_u16()?;
                let offset = file.read_u32()?;
                Ok(VertexLayout {
                    usage,
                    width,
                    offset,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_noop(vertex_buffer_offset)?;
        let mut vertex_buffer = vec![0; vertex_buffer_len];
        file.read_exact(&mut vertex_buffer)?;

        file.seek_noop(index_buffer_offset)?;
        let mut index_buffer = vec![0; index_buffer_len];
        file.read_exact(&mut index_buffer)?;

        file.seek_assert_align_up(total_len, 16)?;

        Ok(Mesh {
            aux_model_lods,
            main_model_lods,
            material_names,
            vertex_layouts,
            main_vertex_layout_count: usize::from(main_vertex_layout_count),
            vertex_buffer,
            index_buffer,
            gs,
            bones,
            bone_names,
            bone_remap,
        })
    }

    pub fn dump(&self, output: String) -> Result<()> {
        let mut output = std::fs::File::create(output)?;
        let position = self
            .vertex_layouts
            .iter()
            .find(|layout| layout.usage == 0)
            .context("No position data")?;

        let normal = self
            .vertex_layouts
            .iter()
            .find(|layout| layout.usage == 1)
            .context("No normal data")?;

        let texcoord = self
            .vertex_layouts
            .iter()
            .find(|layout| layout.usage == 2)
            .context("No texcoord data")?;

        let vertex_count = (self.vertex_layouts[1].offset - self.vertex_layouts[0].offset)
            / self.vertex_layouts[0].width as u32;

        if position.width != 12 {
            bail!("Unexpected width for position {}", position.width);
        }

        let mut buffer = &self.vertex_buffer[position.offset as usize..];
        for _ in 0..vertex_count {
            let x = buffer.read_f32()?;
            let y = buffer.read_f32()?;
            let z = buffer.read_f32()?;
            writeln!(output, "v {x} {y} {z}")?;
        }

        let mut buffer = &self.vertex_buffer[normal.offset as usize..];
        for _ in 0..vertex_count {
            if normal.width == 4 {
                let x = buffer.read_i8()? as f32 / 128.0;
                let y = buffer.read_i8()? as f32 / 128.0;
                let z = buffer.read_i8()? as f32 / 128.0; // always 0?
                let _ = buffer.read_u8()?;
                writeln!(output, "vn {x} {y} {z}")?;
            } else if normal.width == 8 {
                let x = buffer.read_i8()? as f32 / 128.0;
                let y = buffer.read_i8()? as f32 / 128.0;
                let z = buffer.read_i8()? as f32 / 128.0;
                let _ = buffer.read_i8()? as f32 / 128.0; // always 0?

                // These might be tangents
                let _tx = buffer.read_i8()? as f32 / 128.0;
                let _ty = buffer.read_i8()? as f32 / 128.0;
                let _tz = buffer.read_i8()? as f32 / 128.0;
                let _ = buffer.read_i8()? as f32 / 128.0; // always 1?
                writeln!(output, "vn {x} {y} {z} ",)?;
            } else {
                bail!("Unknown width for normal {}", normal.width)
            }
        }

        if texcoord.width != 4 {
            bail!("Unexpected width for texcoord {}", texcoord.width);
        }

        let mut buffer = &self.vertex_buffer[texcoord.offset as usize..];
        for _ in 0..vertex_count {
            let u = f16::from_bits(buffer.read_u16()?);
            let v = f16::from_bits(buffer.read_u16()?);
            writeln!(output, "vt {} {}", u, 1.0 - v.to_f32())?;
        }

        let lod = &self.main_model_lods[0];

        for group in &lod.model_groups {
            for model in &group.models {
                let mut index_buffer = &self.index_buffer[model.index_buffer_start as usize * 2..];
                for _ in 0..model.index_count / 3 {
                    let a = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    let b = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    let c = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    writeln!(
                        output,
                        "f {0}/{0}/{0} {1}/{1}/{1} {2}/{2}/{2}",
                        a + 1,
                        b + 1,
                        c + 1
                    )?;
                }
            }
        }

        Ok(())
    }

    pub fn dump_dae(&self, output: String) -> Result<()> {
        use crate::collada::*;
        use std::path::Path;

        let remapped_joints: Vec<String> = self
            .bone_remap
            .iter()
            .map(|i| self.bones[usize::from(*i)].name.clone())
            .collect();

        let mut inv_bind_matrices = vec![];
        for &remap_index in &self.bone_remap {
            let bone = &self.bones[usize::from(remap_index)];
            for row in bone.absolute_reverse.row_iter() {
                for e in &row {
                    inv_bind_matrices.push(*e);
                }
            }
        }

        enum MeshNodeContent {
            Leaf {
                geometry_name: String,
                controller_name: String,
            },
            Branch {
                children: Vec<MeshNode>,
            },
        }

        struct MeshNode {
            id: String,
            name: String,
            content: MeshNodeContent,
        }

        let mut geometries = vec![];
        let mut controllers = vec![];
        let mut all_meshes = vec![];
        for (lod_i, lod) in self.main_model_lods.iter().enumerate() {
            let mut lod_meshes = vec![];
            for (group_i, group) in lod.model_groups.iter().enumerate() {
                let mut group_meshes = vec![];
                for (model_i, model) in group.models.iter().enumerate() {
                    if model.index_count == 0 {
                        continue;
                    }
                    let material_name =
                        &self.material_names[usize::try_from(model.material_index)?];
                    let model_id = format!("mesh-lod{lod_i}-group{group_i}-model{model_i}");

                    let index_buffer_start = usize::try_from(model.index_buffer_start * 2)?;
                    let index_buffer_end =
                        index_buffer_start + usize::try_from(model.index_count * 2)?;
                    let index_buffer = self
                        .index_buffer
                        .get(index_buffer_start..index_buffer_end)
                        .context("Index buffer out-of-bound")?;
                    let indices: Vec<u16> = index_buffer
                        .chunks(2)
                        .map(|c| u16::from_le_bytes(c.try_into().unwrap()))
                        .collect();
                    let vcount_for_weight: Vec<u8> =
                        std::iter::repeat(8).take(indices.len()).collect();
                    let index_bound = indices.iter().max().unwrap() + 1;

                    let mut sources = vec![];
                    let mut vertices_inputs = vec![];
                    let mut primitive_inputs = vec![SharedInput {
                        semantic: "VERTEX".to_owned(),
                        source: format!("#{model_id}-vertices"),
                        offset: 0,
                        set: None,
                    }];

                    let mut v_for_weight: Vec<u32> = vec![];
                    let mut weight_array = vec![];
                    for (layout_i, layout) in self
                        .vertex_layouts
                        .iter()
                        .take(self.main_vertex_layout_count)
                        .enumerate()
                    {
                        let vertex_buffer_start = usize::try_from(
                            layout.offset + model.vertex_buffer_start * (u32::from(layout.width)),
                        )?;
                        let vertex_buffer_end = vertex_buffer_start
                            + usize::from(layout.width) * usize::from(index_bound);
                        let data = self
                            .vertex_buffer
                            .get(vertex_buffer_start..vertex_buffer_end)
                            .context("Vertex buffer out-of-bound")?;

                        match layout.usage {
                            0 => {
                                if layout.width != 12 {
                                    bail!("Unexpected width for position {}", layout.width);
                                }
                                let array: Vec<f32> = data
                                    .chunks(4)
                                    .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                                    .collect();

                                sources.push(Source {
                                    id: format!("{model_id}-layout{layout_i}"),
                                    array_element: ArrayElement::FloatArray {
                                        id: format!("{model_id}-layout{layout_i}-array"),
                                        array,
                                    },
                                    technique_common: TechniqueCommon {
                                        elements: vec![TechniqueCommonElement::Accessor {
                                            count: index_bound.into(),
                                            source: format!("#{model_id}-layout{layout_i}-array"),
                                            stride: 3,
                                            params: vec![
                                                Param {
                                                    name: "X".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "Y".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "Z".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                            ],
                                        }],
                                    },
                                });

                                vertices_inputs.push(Input {
                                    semantic: "POSITION".to_owned(),
                                    source: format!("#{model_id}-layout{layout_i}"),
                                });
                            }

                            1 => {
                                if layout.width != 4 && layout.width != 8 {
                                    bail!("Unexpected width for normal {}", layout.width);
                                }
                                fn u8_to_f32(b: &u8) -> f32 {
                                    *b as i8 as f32 / 128.0
                                }
                                let array: Vec<f32> = data
                                    .chunks(layout.width.into())
                                    .flat_map(|c| &c[0..3])
                                    .map(u8_to_f32)
                                    .collect();
                                sources.push(Source {
                                    id: format!("{model_id}-layout{layout_i}"),
                                    array_element: ArrayElement::FloatArray {
                                        id: format!("{model_id}-layout{layout_i}-array"),
                                        array,
                                    },
                                    technique_common: TechniqueCommon {
                                        elements: vec![TechniqueCommonElement::Accessor {
                                            count: index_bound.into(),
                                            source: format!("#{model_id}-layout{layout_i}-array"),
                                            stride: 3,
                                            params: vec![
                                                Param {
                                                    name: "X".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "Y".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "Z".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                            ],
                                        }],
                                    },
                                });

                                primitive_inputs.push(SharedInput {
                                    semantic: "NORMAL".to_owned(),
                                    source: format!("#{model_id}-layout{layout_i}"),
                                    offset: 0,
                                    set: None,
                                });

                                if layout.width == 8 {
                                    let array: Vec<f32> = data
                                        .chunks(layout.width.into())
                                        .flat_map(|c| &c[4..7])
                                        .map(u8_to_f32)
                                        .collect();
                                    sources.push(Source {
                                        id: format!("{model_id}-layout{layout_i}tangent"),
                                        array_element: ArrayElement::FloatArray {
                                            id: format!("{model_id}-layout{layout_i}tangent-array"),
                                            array,
                                        },
                                        technique_common: TechniqueCommon {
                                            elements: vec![TechniqueCommonElement::Accessor {
                                                count: index_bound.into(),
                                                source: format!(
                                                    "#{model_id}-layout{layout_i}tangent-array"
                                                ),
                                                stride: 3,
                                                params: vec![
                                                    Param {
                                                        name: "X".to_owned(),
                                                        type_: "float".to_owned(),
                                                    },
                                                    Param {
                                                        name: "Y".to_owned(),
                                                        type_: "float".to_owned(),
                                                    },
                                                    Param {
                                                        name: "Z".to_owned(),
                                                        type_: "float".to_owned(),
                                                    },
                                                ],
                                            }],
                                        },
                                    });

                                    primitive_inputs.push(SharedInput {
                                        semantic: "TANGENT".to_owned(),
                                        source: format!("#{model_id}-layout{layout_i}tangent"),
                                        offset: 0,
                                        set: None,
                                    });
                                }
                            }
                            2 | 3 => {
                                if layout.width != 4 {
                                    bail!("Unexpected width for texcoord {}", layout.width);
                                }

                                let array: Vec<f32> = data
                                    .chunks(2)
                                    .enumerate()
                                    .map(|(index, c)| {
                                        let v = f16::from_le_bytes(c.try_into().unwrap()).to_f32();
                                        if index % 2 == 0 {
                                            v
                                        } else {
                                            1.0 - v
                                        }
                                    })
                                    .collect();

                                sources.push(Source {
                                    id: format!("{model_id}-layout{layout_i}"),
                                    array_element: ArrayElement::FloatArray {
                                        id: format!("{model_id}-layout{layout_i}-array"),
                                        array,
                                    },
                                    technique_common: TechniqueCommon {
                                        elements: vec![TechniqueCommonElement::Accessor {
                                            count: index_bound.into(),
                                            source: format!("#{model_id}-layout{layout_i}-array"),
                                            stride: 2,
                                            params: vec![
                                                Param {
                                                    name: "U".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "V".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                            ],
                                        }],
                                    },
                                });

                                primitive_inputs.push(SharedInput {
                                    semantic: "TEXCOORD".to_owned(),
                                    source: format!("#{model_id}-layout{layout_i}"),
                                    offset: 0,
                                    set: Some(if layout.usage == 2 { 0 } else { 1 }),
                                });
                            }
                            4 => {
                                if layout.width != 16 {
                                    bail!("Unexpected width for bone weight {}", layout.width);
                                }

                                for (i, chunk) in data.chunks(16).enumerate() {
                                    let joints = &chunk[0..8];
                                    let weights = &chunk[8..16];
                                    weight_array.extend(weights.iter().map(|w| *w as f32 / 255.0));

                                    #[allow(clippy::needless_range_loop)]
                                    for j in 0..8 {
                                        v_for_weight.push(u32::from(joints[j]));
                                        v_for_weight.push(u32::try_from(i * 8 + j)?);
                                    }
                                }
                            }
                            5 => {
                                if layout.width != 4 {
                                    bail!("Unexpected width for color {}", layout.width);
                                }
                                let array: Vec<f32> =
                                    data.iter().map(|&b| b as f32 / 255.0).collect();
                                sources.push(Source {
                                    id: format!("{model_id}-layout{layout_i}"),
                                    array_element: ArrayElement::FloatArray {
                                        id: format!("{model_id}-layout{layout_i}-array"),
                                        array,
                                    },
                                    technique_common: TechniqueCommon {
                                        elements: vec![TechniqueCommonElement::Accessor {
                                            count: index_bound.into(),
                                            source: format!("#{model_id}-layout{layout_i}-array"),
                                            stride: 4,
                                            params: vec![
                                                Param {
                                                    name: "R".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "G".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "B".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                                Param {
                                                    name: "A".to_owned(),
                                                    type_: "float".to_owned(),
                                                },
                                            ],
                                        }],
                                    },
                                });

                                primitive_inputs.push(SharedInput {
                                    semantic: "COLOR".to_owned(),
                                    source: format!("#{model_id}-layout{layout_i}"),
                                    offset: 0,
                                    set: None,
                                });
                            }
                            _ => (),
                        }
                    }

                    let vertices = Vertices {
                        id: format!("{model_id}-vertices"),
                        inputs: vertices_inputs,
                    };
                    let primitive_elements = vec![PrimitiveElements::Triangles {
                        count: u32::try_from(indices.len() / 3)?,
                        inputs: primitive_inputs,
                        p: indices,
                    }];

                    let geometry = Geometry {
                        id: model_id.clone(),
                        geometric_element: GeometricElement::Mesh {
                            sources,
                            vertices,
                            primitive_elements,
                        },
                    };
                    geometries.push(geometry);

                    let weight_sources = vec![
                        Source {
                            id: format!("{model_id}-joint"),
                            array_element: ArrayElement::NameArray {
                                id: format!("{model_id}-joint-array"),
                                array: remapped_joints.clone(),
                            },
                            technique_common: TechniqueCommon {
                                elements: vec![TechniqueCommonElement::Accessor {
                                    count: remapped_joints.len().try_into()?,
                                    source: format!("#{model_id}-joint-array"),
                                    stride: 1,
                                    params: vec![Param {
                                        name: "JOINT".to_owned(),
                                        type_: "name".to_owned(),
                                    }],
                                }],
                            },
                        },
                        Source {
                            id: format!("{model_id}-weight"),
                            array_element: ArrayElement::FloatArray {
                                id: format!("{model_id}-weight-array"),
                                array: weight_array,
                            },
                            technique_common: TechniqueCommon {
                                elements: vec![TechniqueCommonElement::Accessor {
                                    count: u32::from(index_bound) * 8,
                                    source: format!("#{model_id}-weight-array"),
                                    stride: 1,
                                    params: vec![Param {
                                        name: "WEIGHT".to_owned(),
                                        type_: "float".to_owned(),
                                    }],
                                }],
                            },
                        },
                        Source {
                            id: format!("{model_id}-inv"),
                            array_element: ArrayElement::FloatArray {
                                id: format!("{model_id}-inv-array"),
                                array: inv_bind_matrices.clone(),
                            },
                            technique_common: TechniqueCommon {
                                elements: vec![TechniqueCommonElement::Accessor {
                                    count: remapped_joints.len().try_into()?,
                                    source: format!("#{model_id}-inv-array"),
                                    stride: 16,
                                    params: vec![Param {
                                        name: "TRANSFORM".to_owned(),
                                        type_: "float4x4".to_owned(),
                                    }],
                                }],
                            },
                        },
                    ];

                    let controller = Controller {
                        id: format!("{model_id}-controller"),
                        skin: Skin {
                            source: format!("#{model_id}"),
                            sources: weight_sources,
                            joints: Joints {
                                inputs: vec![
                                    Input {
                                        semantic: "JOINT".to_owned(),
                                        source: format!("#{model_id}-joint"),
                                    },
                                    Input {
                                        semantic: "INV_BIND_MATRIX".to_owned(),
                                        source: format!("#{model_id}-inv"),
                                    },
                                ],
                            },
                            vertex_weights: VertexWeights {
                                count: index_bound.try_into()?,
                                inputs: vec![
                                    SharedInput {
                                        semantic: "JOINT".to_owned(),
                                        source: format!("#{model_id}-joint"),
                                        offset: 0,
                                        set: None,
                                    },
                                    SharedInput {
                                        semantic: "WEIGHT".to_owned(),
                                        source: format!("#{model_id}-weight"),
                                        offset: 1,
                                        set: None,
                                    },
                                ],
                                vcount: vcount_for_weight,
                                v: v_for_weight,
                            },
                        },
                    };
                    controllers.push(controller);

                    group_meshes.push(MeshNode {
                        id: format!("meshnode-lod{lod_i}-group{group_i}-model{model_i}"),
                        name: format!("[{material_name}]Lod{lod_i}-Group{group_i}-Model{model_i}"),
                        content: MeshNodeContent::Leaf {
                            geometry_name: format!("#{model_id}"),
                            controller_name: format!("#{model_id}-controller"),
                        },
                    });
                }
                let group_id = group.group_id;
                lod_meshes.push(MeshNode {
                    id: format!("meshnode-lod{lod_i}-group{group_i}"),
                    name: format!("[{group_id}]Lod{lod_i}-Group{group_i}"),
                    content: MeshNodeContent::Branch {
                        children: group_meshes,
                    },
                })
            }
            all_meshes.push(MeshNode {
                id: format!("meshnode-lod{lod_i}"),
                name: format!("Lod{lod_i}"),
                content: MeshNodeContent::Branch {
                    children: lod_meshes,
                },
            })
        }

        let mesh_root = MeshNode {
            id: "meshnode-root".to_owned(),
            name: "Mesh".to_owned(),
            content: MeshNodeContent::Branch {
                children: all_meshes,
            },
        };

        fn make_mesh_node(
            node: MeshNode,
            leaf_mapper: fn(geometry_name: String, controller_name: String, node: &mut Node),
        ) -> Node {
            let mut result = Node {
                id: node.id,
                name: node.name,
                type_: NodeType::Node,
                matrix: None,
                instance_controllers: vec![],
                instance_geometries: vec![],
                nodes: vec![],
            };

            match node.content {
                MeshNodeContent::Leaf {
                    geometry_name,
                    controller_name,
                } => {
                    leaf_mapper(geometry_name, controller_name, &mut result);
                }
                MeshNodeContent::Branch { children } => {
                    result.nodes = children
                        .into_iter()
                        .map(|c| make_mesh_node(c, leaf_mapper))
                        .collect();
                }
            }

            result
        }

        let asset = Asset {
            created: "2022-06-19T15:05:15".to_owned(),
            modified: "2022-06-19T15:05:15".to_owned(),
        };

        let collada = if self.bones.is_empty() {
            fn mesh_to_geometry(geometry_name: String, _controller_name: String, node: &mut Node) {
                node.instance_geometries = vec![InstanceGeometry { url: geometry_name }]
            }

            let mesh_node_root = make_mesh_node(mesh_root, mesh_to_geometry);

            let visual_scene = VisualScene {
                id: "scene".to_owned(),
                nodes: vec![mesh_node_root],
            };

            let library_geometries = Library::Geometries { geometries };
            let library_visual_scenes = Library::VisualScenes {
                visual_scenes: vec![visual_scene],
            };

            Collada {
                asset,
                libraries: vec![library_geometries, library_visual_scenes],
                scene: Scene {
                    instance_visual_scene: "#scene".to_owned(),
                },
            }
        } else {
            fn make_joint(index: usize, bones: &[Bone]) -> Node {
                let mut nodes = vec![];
                let bone = &bones[index];
                for (i, bone) in bones.iter().enumerate() {
                    if Some(index) == bone.parent {
                        nodes.push(make_joint(i, bones));
                    }
                }

                Node {
                    id: bone.name.clone(),
                    name: bone.name.clone(),
                    type_: NodeType::Joint,
                    matrix: Some(bone.relative_transform),
                    instance_controllers: vec![],
                    instance_geometries: vec![],
                    nodes,
                }
            }
            let bone_root = self
                .bones
                .iter()
                .position(|b| b.parent.is_none())
                .context("No bone root")?;

            let joint_root = make_joint(bone_root, &self.bones);

            fn mesh_to_controller(
                _geometry_name: String,
                controller_name: String,
                node: &mut Node,
            ) {
                node.instance_controllers = vec![InstanceController {
                    url: controller_name,
                    skeletons: vec!["#__root__".to_owned()],
                }]
            }

            let mesh_node_root = make_mesh_node(mesh_root, mesh_to_controller);

            let top_nodes = vec![
                Node {
                    id: "__root__".to_owned(),
                    name: "Joints".to_owned(),
                    type_: NodeType::Node,
                    matrix: None,
                    instance_controllers: vec![],
                    instance_geometries: vec![],
                    nodes: vec![joint_root],
                },
                mesh_node_root,
            ];

            let visual_scene = VisualScene {
                id: "scene".to_owned(),
                nodes: top_nodes,
            };

            let library_geometries = Library::Geometries { geometries };
            let library_controllers = Library::Controllers { controllers };
            let library_visual_scenes = Library::VisualScenes {
                visual_scenes: vec![visual_scene],
            };

            Collada {
                asset,
                libraries: vec![
                    library_geometries,
                    library_controllers,
                    library_visual_scenes,
                ],
                scene: Scene {
                    instance_visual_scene: "#scene".to_owned(),
                },
            }
        };

        collada.save(Path::new(&output))
    }
}
