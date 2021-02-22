use crate::file_ext::*;
use anyhow::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Clone)]
struct Model {
    vertex_count: u32,
    index_buffer_start: u32,
    vertex_buffer_start: u32,
}

#[derive(Clone)]
struct ModelGroup {
    models: Vec<Model>,
}

#[derive(Clone)]
struct ModelLod {
    model_groups: Vec<ModelGroup>,
}

struct VertexLayout {
    usage: u16, // position, normal, uv, uv2, weight
    width: u16,
    offset: u32,
}

struct Point4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

struct G {
    a: Point4,
    b: Point4,
}

pub struct Mesh {
    main_model_lods: Vec<ModelLod>,
    aux_model_lods: Vec<ModelLod>,
    vertex_layouts: Vec<VertexLayout>,
    vertex_buffer: Vec<u8>,
    index_buffer: Vec<u8>,
    gs: Vec<G>,
}

impl Mesh {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Mesh> {
        if &file.read_magic()? != b"MESH" {
            bail!("Wrong magic for MESH");
        }
        if file.read_u32()? != 0x77a2d00d {
            bail!("Wrong version for MESH");
        }
        let total_len = file.read_u64()?;

        let a_count = file.read_u16()?;
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
        let f_offset = file.read_u64()?;
        let g_offset = file.read_u64()?; // after string table
        let mesh_data_offset = file.read_u64()?; // after string table
        let i_offset = file.read_u64()?;
        let model_names_offset = file.read_u64()?; // lists to string table entry
        let bone_names_offset = file.read_u64()?; // lists to string table entry
        let f_names_offset = file.read_u64()?; // lists to string table entry
        let string_table_offset = file.read_u64()?; // string table

        let mut model_lod_cache: HashMap<u64, ModelLod> = HashMap::new();

        let read_model_group = |file: &mut F, model_group_offset| {
            file.seek_noop(model_group_offset)?;

            let _ = file.read_u8()?;
            let model_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let _ = file.read_u32()?;
            let _ = file.read_u32()?;
            let _ = file.read_u32()?;

            let models = (0..model_count)
                .map(|_| {
                    let j = file.read_u32()?;
                    let face_count = file.read_u32()?;
                    let index_buffer_start = file.read_u32()?;
                    let vertex_buffer_start = file.read_u32()?;

                    let _ = file.read_u32()?;
                    let _ = file.read_u32()?;

                    Ok(Model {
                        vertex_count: face_count,
                        index_buffer_start,
                        vertex_buffer_start,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(ModelGroup { models })
        };

        let mut read_model_lod = |file: &mut F, model_lod_offset| -> Result<ModelLod> {
            if let Some(cache) = model_lod_cache.get(&model_lod_offset) {
                return Ok(cache.clone());
            }

            file.seek_noop(model_lod_offset)?;

            let model_group_count = file.read_u32()?;
            let _ = file.read_f32()?;
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

        let model_name_count;
        let main_model_lods;
        if main_models_offset != 0 {
            file.seek_noop(main_models_offset)?;
            let model_lod_count = file.read_u8()?;
            model_name_count = file.read_u8()?;
            let a3_count = file.read_u8()?;
            let a4_count = file.read_u8()?;
            let a5_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let f0 = file.read_f32()?;
            let f1 = file.read_f32()?;
            let f2 = file.read_f32()?;
            let f3 = file.read_f32()?;

            let g0 = file.read_f32()?;
            let g1 = file.read_f32()?;
            let g2 = file.read_f32()?;
            let g3 = file.read_f32()?;

            let h0 = file.read_f32()?;
            let h1 = file.read_f32()?;
            let h2 = file.read_f32()?;
            let h3 = file.read_f32()?;

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
            model_name_count = 0;
            main_model_lods = vec![];
        }

        let aux_model_lods = if aux_model_offset != 0 {
            file.seek_noop(aux_model_offset)?;

            let lod_count = file.read_u8()?;
            let b2_count = file.read_u8()?;
            let b3_count = file.read_u8()?;
            let b4_count = file.read_u8()?;
            let b5_count = file.read_u8()?;
            file.seek_align_up(4)?;

            let lod_list_offset = file.read_u64()?;

            let f0 = file.read_f32()?;
            let f1 = file.read_f32()?;
            let f2 = file.read_f32()?;
            let f3 = file.read_f32()?;

            let g0 = file.read_f32()?;
            let g1 = file.read_f32()?;
            let g2 = file.read_f32()?;
            let g3 = file.read_f32()?;

            let h0 = file.read_f32()?;
            let h1 = file.read_f32()?;
            let h2 = file.read_f32()?;
            let h3 = file.read_f32()?;

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

        if c_offset != 0 {
            file.seek_noop(c_offset)?;

            let c1_count = file.read_u8()?;
            let c2_count = file.read_u8()?;
            let c3_count = file.read_u8()?;
            let c4_count = file.read_u8()?;
            let f0 = file.read_f32()?;

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

        let read_matrix4x4 = |file: &mut F| -> Result<()> {
            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;

            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;

            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;

            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;
            file.read_f32()?;
            Ok(())
        };

        let bone_count;
        if skeleton_offset != 0 {
            file.seek_assert_align_up(skeleton_offset, 8)?;
            bone_count = file.read_u32()?;
            let bone_remap_count = file.read_u32()?;
            let c = file.read_u32()?;
            let d = file.read_u32()?;
            let bone_hierarchy_offset = file.read_u64()?;
            let bone_matrix_a_offset = file.read_u64()?;
            let bone_matrix_b_offset = file.read_u64()?;
            let bone_matrix_c_offset = file.read_u64()?;

            let bone_remap = (0..bone_remap_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_hierarchy_offset, 16)?;

            (0..bone_count)
                .map(|_| {
                    let _ = file.read_u16()?;
                    let _ = file.read_u16()?;
                    let _ = file.read_u16()?;
                    let _ = file.read_u16()?;
                    let _ = file.read_u16()?;
                    file.seek_align_up(16)?;
                    Ok(())
                })
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_a_offset, 16)?;

            (0..bone_count)
                .map(|_| read_matrix4x4(&mut file))
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_b_offset, 16)?;

            (0..bone_count)
                .map(|_| read_matrix4x4(&mut file))
                .collect::<Result<Vec<_>>>()?;

            file.seek_assert_align_up(bone_matrix_c_offset, 16)?;

            (0..bone_count)
                .map(|_| read_matrix4x4(&mut file))
                .collect::<Result<Vec<_>>>()?;
        } else {
            bone_count = 0;
        }

        if e_offset != 0 {
            file.seek_assert_align_up(e_offset, 16)?;
            let p_offset = file.read_u64()?;
            let q_offset = file.read_u64()?;
            file.seek_noop(p_offset)?;
            //..
            //file.seek_noop(q_offset)?;
            //..
        }

        //***

        let next_offsets = [
            f_offset,
            i_offset,
            model_names_offset,
            bone_names_offset,
            f_names_offset,
            string_table_offset,
        ];
        let next_offset = *next_offsets.iter().find(|x| **x != 0).unwrap();
        if e_offset == 0 {
            let align = if f_offset != 0 || i_offset != 0 {
                8
            } else {
                16
            };
            file.seek_assert_align_up(next_offset, align)?;
        } else {
            file.seek(SeekFrom::Start(next_offset))?;
        }

        //***

        let mut f_name_count = 0;
        if f_offset != 0 {
            file.seek_assert_align_up(f_offset, 8)?;
            let f_count = file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.read_u8()?;
            file.seek_align_up(8)?;
            let fa_offset = file.read_u64()?;
            file.seek_noop(fa_offset)?;
            let fb_offsets = (0..f_count)
                .map(|_| file.read_u64())
                .collect::<Result<Vec<_>>>()?;

            file.seek_align_up(16)?;
            fb_offsets
                .into_iter()
                .map(|offset| {
                    file.seek_noop(offset)?;
                    file.read_u64()?;
                    file.read_u64()?;
                    let m_offset = file.read_u64()?;
                    let n_offset = file.read_u64()?;
                    file.seek_noop(m_offset)?;
                    file.read_u64()?;
                    file.read_u16()?;
                    let name_count = file.read_u16()?;
                    f_name_count += name_count;
                    file.read_u32()?;
                    file.seek_noop(n_offset)?;
                    file.read_u64()?;
                    file.read_u64()?;
                    file.read_u64()?;
                    file.read_u64()?;

                    Ok(())
                })
                .collect::<Result<Vec<_>>>()?;
        }

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

        if model_names_offset != 0 {
            file.seek_assert_align_up(model_names_offset, 16)?;
            (0..model_name_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?;
        }

        if bone_names_offset != 0 {
            file.seek_assert_align_up(bone_names_offset, 16)?;
            (0..bone_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?;
        }

        if f_names_offset != 0 {
            file.seek_assert_align_up(f_names_offset, 16)?;
            (0..f_name_count)
                .map(|_| file.read_u16())
                .collect::<Result<Vec<_>>>()?;
        }

        if model_name_count as u32 + bone_count + f_name_count as u32 != string_count as u32 {
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
        let vertex_buffer_len = usize::try_from(file.read_u32()?)?;
        let index_buffer_len = usize::try_from(file.read_u32()?)?;

        let _ = file.read_u16()?;
        let vertex_layout_count = file.read_u16()?;
        let _ = file.read_u32()?;
        let _ = file.read_u32()?;
        let _ = file.read_u32()?;

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
            vertex_layouts,
            vertex_buffer,
            index_buffer,
            gs,
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
            .context("No position data")?;

        let vertex_count = (self.vertex_layouts[1].offset - self.vertex_layouts[0].offset)
            / self.vertex_layouts[0].width as u32;

        if position.width != 12 {
            bail!("Unexpected width");
        }

        let mut buffer = &self.vertex_buffer[position.offset as usize..];
        for _ in 0..vertex_count {
            let x = buffer.read_f32()?;
            let y = buffer.read_f32()?;
            let z = buffer.read_f32()?;
            writeln!(output, "v {} {} {}", x, y, z)?;
        }

        let mut buffer = &self.vertex_buffer[normal.offset as usize..];
        for _ in 0..vertex_count {
            let x = buffer.read_u8()? as f32 / 255.0;
            let y = buffer.read_u8()? as f32 / 255.0;
            let z = buffer.read_u8()? as f32 / 255.0;
            let _ = buffer.read_u8()?;
            writeln!(output, "vn {} {} {}", x, y, z)?;
        }

        let lod = &self.main_model_lods[0];

        for group in &lod.model_groups {
            for model in &group.models {
                let mut index_buffer = &self.index_buffer[model.index_buffer_start as usize * 2..];
                for _ in 0..model.vertex_count / 3 {
                    let a = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    let b = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    let c = index_buffer.read_u16()? as u32 + model.vertex_buffer_start;
                    writeln!(output, "f {} {} {}", a + 1, b + 1, c + 1)?;
                }
            }
        }

        /*for g in &self.gs {
            writeln!(output, "v {} {} {}", g.a.x, g.a.y, g.a.z)?;
            writeln!(output, "v {} {} {}", g.b.x, g.b.y, g.b.z)?;
        }

        for i in 0..self.gs.len() {
            writeln!(output, "l {} {}", i * 2 + 1, i * 2 + 2)?;
        }*/

        Ok(())
    }
}
