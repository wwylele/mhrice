use super::*;
use crate::part_color::PART_COLORS;
use anyhow::Context;
use nalgebra_glm::*;
use ordered_float::*;
use std::collections::HashSet;
use std::convert::TryFrom;

pub struct HitzoneDiagram {
    pub meat: RgbaImage,
    pub parts_group: RgbaImage,
}

pub struct ColoredVertex {
    pub position: Vec3,
    pub meat: HashSet<usize>,
    pub parts_group: HashSet<usize>,
}

fn crop_image(image: texture::RawImage2d<u8>) -> anyhow::Result<RgbaImage> {
    let mut min_x = image.width;
    let mut max_x = 0;
    let mut min_y = image.height;
    let mut max_y = 0;
    for x in 0..image.width {
        for y in 0..image.height {
            let index = usize::try_from(x + y * image.width)?;
            if image.data[index * 4 + 3] != 0 {
                min_x = std::cmp::min(min_x, x);
                min_y = std::cmp::min(min_y, y);
                max_x = std::cmp::max(max_x, x);
                max_y = std::cmp::max(max_y, y);
            }
        }
    }

    let new_width = max_x - min_x + 1;
    let new_height = max_y - min_y + 1;
    let mut new_data = vec![0; usize::try_from(new_width * new_height * 4)?];

    for x in 0..new_width {
        for y in 0..new_height {
            let new_index = usize::try_from(x + y * new_width)?;
            let index = usize::try_from(x + min_x + (y + min_y) * image.width)?;
            new_data[new_index * 4..][..4].copy_from_slice(&image.data[index * 4..][..4]);
        }
    }

    Ok(RgbaImage {
        data: new_data,
        width: new_width,
        height: new_height,
    })
}

pub fn gen_hitzone_diagram(
    vertexs: Vec<ColoredVertex>,
    indexs: Vec<u32>,
) -> anyhow::Result<HitzoneDiagram> {
    CONTEXT.run(move |gl| {
        let x_min = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.x).ok())
            .min()
            .context("null mesh")?
            .into_inner();

        let y_min = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.y).ok())
            .min()
            .context("null mesh")?
            .into_inner();

        let z_min = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.z).ok())
            .min()
            .context("null mesh")?
            .into_inner();

        let x_max = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.x).ok())
            .max()
            .context("null mesh")?
            .into_inner();

        let y_max = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.y).ok())
            .max()
            .context("null mesh")?
            .into_inner();

        let z_max = vertexs
            .iter()
            .filter_map(|v| NotNan::new(v.position.z).ok())
            .max()
            .context("null mesh")?
            .into_inner();

        let center = vec3(
            (x_min + x_max) * 0.5,
            (y_min + y_max) * 0.5,
            (z_min + z_max) * 0.5,
        );

        let move_to_center = translate(&identity(), &-center);
        let upside_down = rotate_z(&identity(), std::f32::consts::PI);
        let rotate_to_side = rotate_y(&identity(), std::f32::consts::PI * 0.7);
        let up_a_bit = rotate_x(&identity(), std::f32::consts::PI * 0.05);

        let transform_pre_scale = up_a_bit * rotate_to_side * upside_down * move_to_center;

        let mut max_xy = 0.0;
        let mut max_z = 0.0;
        for v in &vertexs {
            let transformed =
                transform_pre_scale * vec4(v.position.x, v.position.y, v.position.z, 1.0);
            if max_xy < transformed.x.abs() {
                max_xy = transformed.x.abs();
            }
            if max_xy < transformed.y.abs() {
                max_xy = transformed.y.abs();
            }
            if max_z < transformed.z.abs() {
                max_z = transformed.z.abs();
            }
        }

        let scale_to_fit = scale(&identity(), &vec3(1.0 / max_xy, 1.0 / max_xy, 1.0 / max_z));
        let transform = scale_to_fit * transform_pre_scale;

        let mut color_list_data: Vec<_> = PART_COLORS
            .iter()
            .map(|color_code| {
                [
                    u8::from_str_radix(&color_code[1..3], 16).unwrap() as f32 / 255.0,
                    u8::from_str_radix(&color_code[3..5], 16).unwrap() as f32 / 255.0,
                    u8::from_str_radix(&color_code[5..7], 16).unwrap() as f32 / 255.0,
                ]
            })
            .collect();
        color_list_data.push([0.0, 0.0, 0.0]);
        color_list_data.push([1.0, 1.0, 1.0]);

        let color_list = texture::buffer_texture::BufferTexture::new(
            &gl.display,
            &color_list_data,
            texture::buffer_texture::BufferTextureType::Float,
        )?;

        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            color_meat: u32,
            color_parts_group: u32,
        }

        implement_vertex!(Vertex, position, color_meat, color_parts_group);

        fn get_color_attr(numbers: HashSet<usize>) -> u32 {
            if numbers.is_empty() {
                return 1 << (PART_COLORS.len() + 1);
            }
            let mut code = 0;
            for number in numbers {
                if number >= PART_COLORS.len() {
                    code |= 1 << PART_COLORS.len()
                } else {
                    code |= 1 << number
                }
            }

            code
        }

        let vertex_buffer_raw: Vec<Vertex> = vertexs
            .into_iter()
            .map(|v| {
                Ok(Vertex {
                    position: [v.position.x, v.position.y, v.position.z],
                    color_meat: get_color_attr(v.meat),
                    color_parts_group: get_color_attr(v.parts_group),
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let vertex_buffer = VertexBuffer::new(&gl.display, &vertex_buffer_raw)?;
        let index_buffer =
            IndexBuffer::new(&gl.display, index::PrimitiveType::TrianglesList, &indexs)?;

        let width = 800;
        let height = 800;

        let program = Program::from_source(
            &gl.display,
            "#version 330 core

            uniform mat4 matrix;
            uniform bool parts_group;

            in vec3 position;
            in uint color_meat;
            in uint color_parts_group;

            out uint color_attr;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
                if (parts_group) {
                    color_attr = color_parts_group;
                } else {
                    color_attr = color_meat;
                }
            }
        ",
            "#version 330 core

            uniform samplerBuffer color_list;

            flat in uvec3 color_attr_composed;
            in vec3 triangle_coord;

            layout(location = 0) out vec4 out_color;

            void main() {
                uint color_attr;
                if (triangle_coord.x > triangle_coord.y && triangle_coord.x > triangle_coord.z) {
                    color_attr = color_attr_composed[0];
                } else if (triangle_coord.y > triangle_coord.z) {
                    color_attr = color_attr_composed[1];
                } else {
                    color_attr = color_attr_composed[2];
                }

                int color_count = 0;
                int color_indexs[32];
                for (int i = 0; i < 32; ++i) {
                    if (((color_attr >> i) & 1U) != 0U) {
                        color_indexs[color_count++] = i;
                    }
                }

                ivec2 coord = ivec2(gl_FragCoord.xy);

                out_color = vec4(texelFetch(color_list, color_indexs[
                    ((coord.x + coord.y) / 2) % color_count]
                ).xyz, 1.0);
            }
        ",
            Some(
                "#version 330 core
                layout(triangles) in;
                layout(triangle_strip, max_vertices=3) out;
                in uint color_attr[];
                flat out uvec3 color_attr_composed;
                out vec3 triangle_coord;

                void main() {
                    uvec3 composed = uvec3(color_attr[0],  color_attr[1], color_attr[2]);
                    gl_Position = gl_in[0].gl_Position;
                    color_attr_composed = composed;
                    triangle_coord = vec3(1.0, 0.0, 0.0);
                    EmitVertex();
                    gl_Position = gl_in[1].gl_Position;
                    color_attr_composed = composed;
                    triangle_coord = vec3(0.0, 1.0, 0.0);
                    EmitVertex();
                    gl_Position = gl_in[2].gl_Position;
                    color_attr_composed = composed;
                    triangle_coord = vec3(0.0, 0.0, 1.0);
                    EmitVertex();

                    EndPrimitive();
                }
            ",
            ),
        )?;

        let rec_program = Program::from_source(
            &gl.display,
            "#version 330 core

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
            "#version 330 core
            layout(origin_upper_left) in vec4 gl_FragCoord;
            layout(location = 0) out vec4 out_color;
            uniform sampler2D depth;
            uniform float width;
            uniform float height;

            float gd(float dx, float dy) {
                vec2 v = (gl_FragCoord.xy + vec2(dx, dy)) / vec2(width, height);
                return texture(depth, vec2(v.x, 1.0-v.y)).x;
            }

            bool dis(float dx, float dy) {
                float dd = gd(dx - 1.0, dy) + gd(dx + 1.0, dy)
                         + gd(dx, dy - 1.0) + gd(dx, dy + 1.0) - 4.0 * gd(dx, dy);
                return abs(dd) < 0.05;
            }

            void main() {
                if (dis(0.0, 0.0)) {
                    discard;
                }
                out_color = vec4(0.0,0.0,0.0,1.0);
            }
        ",
            None,
        )?;

        #[derive(Copy, Clone)]
        struct SimpleVertex {
            position: [f32; 2],
        }
        implement_vertex!(SimpleVertex, position);
        let rec_vertex_buffer = VertexBuffer::new(
            &gl.display,
            &[
                SimpleVertex {
                    position: [-1.0, -1.0],
                },
                SimpleVertex {
                    position: [-1.0, 1.0],
                },
                SimpleVertex {
                    position: [1.0, -1.0],
                },
                SimpleVertex {
                    position: [1.0, 1.0],
                },
            ],
        )?;

        let rec_index_buffer = IndexBuffer::new(
            &gl.display,
            index::PrimitiveType::TriangleFan,
            &[1u16, 0u16, 2u16, 3u16],
        )?;

        let render = |parts_group: bool| -> anyhow::Result<RgbaImage> {
            let color = texture::Texture2d::empty_with_format(
                &gl.display,
                texture::UncompressedFloatFormat::U8U8U8U8,
                texture::MipmapsOption::NoMipmap,
                width,
                height,
            )?;
            let depth = texture::DepthTexture2d::empty(&gl.display, width, height)?;
            let mut framebuffer =
                framebuffer::SimpleFrameBuffer::with_depth_buffer(&gl.display, &color, &depth)?;
            framebuffer.clear_all((0.0, 0.0, 0.0, 0.0), 1.0, 0);

            let uniforms = uniform! {
                matrix: *transform.as_ref(),
                parts_group: parts_group,
                color_list: &color_list,
            };

            let param = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            framebuffer.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &param)?;

            let mut rec_framebuffer = framebuffer::SimpleFrameBuffer::new(&gl.display, &color)?;

            let depth_sampler = uniforms::Sampler::new(&depth)
                .wrap_function(uniforms::SamplerWrapFunction::Clamp)
                .minify_filter(uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(uniforms::MagnifySamplerFilter::Nearest);

            let rec_uniforms = uniform! {
                depth: depth_sampler,
                width: width as f32,
                height: height as f32,
            };

            rec_framebuffer.draw(
                &rec_vertex_buffer,
                &rec_index_buffer,
                &rec_program,
                &rec_uniforms,
                &Default::default(),
            )?;

            let image: texture::RawImage2d<u8> = color.read();

            Ok(crop_image(image)?)
        };

        Ok(HitzoneDiagram {
            meat: render(false)?,
            parts_group: render(true)?,
        })
    })
}
