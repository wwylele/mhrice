use super::*;
use crate::part_color::PART_COLORS;
use anyhow::Context;
use nalgebra_glm::*;
use ordered_float::*;
use std::convert::TryFrom;

pub struct HitzoneDiagram {
    pub meat: RgbaImage,
    pub parts_group: RgbaImage,
}

pub struct ColoredVertex {
    pub position: Vec3,
    pub meat: Option<usize>,
    pub parts_group: Option<usize>,
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

        let rr = 1.0
            / vertexs
                .iter()
                .filter_map(|v| NotNan::new(distance(&v.position, &center)).ok())
                .max()
                .context("null mesh")?
                .into_inner();

        let move_to_center = translate(&identity(), &-center);
        let scale_to_fit = scale(&identity(), &vec3(rr, rr, rr));
        let upside_down = rotate_z(&identity(), std::f32::consts::PI);
        let rotate_to_side = rotate_y(&identity(), std::f32::consts::PI * 0.7);
        let up_a_bit = rotate_x(&identity(), std::f32::consts::PI * 0.05);

        let transform = up_a_bit * rotate_to_side * upside_down * scale_to_fit * move_to_center;

        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            color_meat: [f32; 3],
            color_parts_group: [f32; 3],
        }

        implement_vertex!(Vertex, position, color_meat, color_parts_group);

        fn get_color(number: Option<usize>) -> [f32; 3] {
            if let Some(number) = number {
                if let Some(color_code) = PART_COLORS.get(number) {
                    [
                        u8::from_str_radix(&color_code[1..3], 16).unwrap() as f32 / 255.0,
                        u8::from_str_radix(&color_code[3..5], 16).unwrap() as f32 / 255.0,
                        u8::from_str_radix(&color_code[5..7], 16).unwrap() as f32 / 255.0,
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                }
            } else {
                [0.5, 0.5, 0.5]
            }
        }

        let vertex_buffer_raw: Vec<Vertex> = vertexs
            .into_iter()
            .map(|v| {
                Ok(Vertex {
                    position: [v.position.x, v.position.y, v.position.z],
                    color_meat: get_color(v.meat),
                    color_parts_group: get_color(v.parts_group),
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
            in vec3 color_meat;
            in vec3 color_parts_group;

            out vec3 color;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
                if (parts_group) {
                    color = color_parts_group;
                } else {
                    color = color_meat;
                }
            }
        ",
            "#version 330 core
            in vec3 color;
            layout(location = 0) out vec4 out_color;
            void main() {
                out_color = vec4(color, 1.0);
            }
        ",
            None,
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
