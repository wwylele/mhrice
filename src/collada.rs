use anyhow::Result;
use quick_xml::events::BytesText;
use quick_xml::Writer;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub enum ArrayElement {
    FloatArray { id: String, array: Vec<f32> },
}

impl ArrayElement {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        match self {
            ArrayElement::FloatArray { id, array } => {
                let string: Vec<String> = array.iter().map(|e| format!("{e}")).collect();
                let string = string.join(" ");
                writer
                    .create_element("float_array")
                    .with_attribute(("id", id.as_str()))
                    .with_attribute(("count", array.len().to_string().as_str()))
                    .write_text_content(BytesText::from_plain(string.as_bytes()))?;
            }
        }
        Ok(())
    }
}

pub struct Param {
    pub name: String,
    pub type_: String,
}

impl Param {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("param")
            .with_attribute(("name", self.name.as_str()))
            .with_attribute(("type", self.type_.as_str()))
            .write_empty()?;
        Ok(())
    }
}

pub enum TechniqueCommonElement {
    Accessor {
        count: u32,
        source: String,
        stride: u32,
        params: Vec<Param>,
    },
}

impl TechniqueCommonElement {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        match self {
            TechniqueCommonElement::Accessor {
                count,
                source,
                stride,
                params,
            } => {
                writer
                    .create_element("accessor")
                    .with_attribute(("count", count.to_string().as_str()))
                    .with_attribute(("source", source.as_str()))
                    .with_attribute(("stride", stride.to_string().as_str()))
                    .write_inner_content(write_seq(params, Param::write))?;
            }
        }
        Ok(())
    }
}

pub struct TechniqueCommon {
    pub elements: Vec<TechniqueCommonElement>,
}

impl TechniqueCommon {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("technique_common")
            .write_inner_content(write_seq(&self.elements, TechniqueCommonElement::write))?;

        Ok(())
    }
}

pub struct Source {
    pub id: String,
    pub array_element: ArrayElement,
    pub technique_common: TechniqueCommon,
}

impl Source {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("source")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(|w| {
                self.array_element.write(w)?;
                self.technique_common.write(w)?;
                Ok(())
            })?;
        Ok(())
    }
}

pub struct Input {
    pub semantic: String,
    pub source: String,
}

impl Input {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("input")
            .with_attribute(("semantic", self.semantic.as_str()))
            .with_attribute(("source", self.source.as_str()))
            .write_empty()?;
        Ok(())
    }
}

pub struct Vertices {
    pub id: String,
    pub inputs: Vec<Input>,
}

impl Vertices {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("vertices")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(write_seq(&self.inputs, Input::write))?;
        Ok(())
    }
}

pub struct SharedInput {
    pub semantic: String,
    pub source: String,
    pub offset: u32,
    pub set: Option<u32>,
}

impl SharedInput {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        let mut writer = writer.create_element("input");

        if let Some(set) = self.set {
            writer = writer.with_attribute(("set", set.to_string().as_str()));
        }

        writer
            .with_attribute(("semantic", self.semantic.as_str()))
            .with_attribute(("source", self.source.as_str()))
            .with_attribute(("offset", self.offset.to_string().as_str()))
            .write_empty()?;
        Ok(())
    }
}

pub enum PrimitiveElements {
    Triangles {
        count: u32,
        inputs: Vec<SharedInput>,
        p: Vec<u16>,
    },
}

impl PrimitiveElements {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        match self {
            PrimitiveElements::Triangles { count, inputs, p } => {
                writer
                    .create_element("triangles")
                    .with_attribute(("count", count.to_string().as_str()))
                    .write_inner_content(|w| {
                        write_seq(inputs, SharedInput::write)(w)?;

                        let p: Vec<String> = p.iter().map(|p| format!("{p}")).collect();
                        let p = p.join(" ");
                        w.create_element("p")
                            .write_text_content(BytesText::from_plain(p.as_bytes()))?;

                        Ok(())
                    })?;
            }
        }

        Ok(())
    }
}

pub enum GeometricElement {
    Mesh {
        sources: Vec<Source>,
        vertices: Vertices,
        primitive_elements: Vec<PrimitiveElements>,
    },
}

impl GeometricElement {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        match self {
            GeometricElement::Mesh {
                sources,
                vertices,
                primitive_elements,
            } => {
                writer.create_element("mesh").write_inner_content(|w| {
                    write_seq(sources, Source::write)(w)?;
                    vertices.write(w)?;
                    write_seq(primitive_elements, PrimitiveElements::write)(w)?;
                    Ok(())
                })?;
                Ok(())
            }
        }
    }
}

pub struct Geometry {
    pub id: String,
    pub geometric_element: GeometricElement,
}

impl Geometry {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("geometry")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(|w| {
                self.geometric_element.write(w)?;
                Ok(())
            })?;
        Ok(())
    }
}

pub struct InstanceGeometry {
    pub url: String,
}

impl InstanceGeometry {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("instance_geometry")
            .with_attribute(("url", self.url.as_str()))
            .write_empty()?;
        Ok(())
    }
}

pub struct Node {
    pub id: String,
    pub instance_geometries: Vec<InstanceGeometry>,
}

impl Node {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("node")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(write_seq(
                &self.instance_geometries,
                InstanceGeometry::write,
            ))?;
        Ok(())
    }
}

pub struct VisualScene {
    pub id: String,
    pub nodes: Vec<Node>,
}

impl VisualScene {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer
            .create_element("visual_scene")
            .with_attribute(("id", self.id.as_str()))
            .write_inner_content(write_seq(&self.nodes, Node::write))?;
        Ok(())
    }
}

pub enum Library {
    LibraryGeometries { geometries: Vec<Geometry> },
    LibraryVisualScenes { visual_scenes: Vec<VisualScene> },
}

impl Library {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        match self {
            Library::LibraryGeometries { geometries } => {
                writer
                    .create_element("library_geometries")
                    .write_inner_content(write_seq(geometries, Geometry::write))?;
            }
            Library::LibraryVisualScenes { visual_scenes } => {
                writer
                    .create_element("library_visual_scenes")
                    .write_inner_content(write_seq(visual_scenes, VisualScene::write))?;
            }
        }
        Ok(())
    }
}

pub struct Asset {
    pub created: String,
    pub modified: String,
}

impl Asset {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer.create_element("asset").write_inner_content(|w| {
            w.create_element("created")
                .write_text_content(BytesText::from_plain(self.created.as_bytes()))?;
            w.create_element("modified")
                .write_text_content(BytesText::from_plain(self.modified.as_bytes()))?;
            Ok(())
        })?;

        Ok(())
    }
}

pub struct Scene {
    pub instance_visual_scene: String,
}

impl Scene {
    fn write(&self, writer: &mut Writer<File>) -> quick_xml::Result<()> {
        writer.create_element("scene").write_inner_content(|w| {
            w.create_element("instance_visual_scene")
                .with_attribute(("url", self.instance_visual_scene.as_str()))
                .write_empty()?;

            Ok(())
        })?;
        Ok(())
    }
}

pub struct Collada {
    pub asset: Asset,
    pub libraries: Vec<Library>,
    pub scene: Scene,
}

impl Collada {
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(br#"<?xml version="1.0" encoding="utf-8"?>"#)?;
        file.write_all(b"\n")?;

        let mut writer = Writer::new_with_indent(file, b' ', 2);
        writer
            .create_element("COLLADA")
            .with_attribute(("xmlns", "http://www.collada.org/2008/03/COLLADASchema"))
            .with_attribute(("version", "1.5.0"))
            .write_inner_content(|w| {
                self.asset.write(w)?;
                write_seq(&self.libraries, Library::write)(w)?;
                self.scene.write(w)?;
                Ok(())
            })?;

        Ok(())
    }
}

fn write_seq<'a, E: 'a>(
    elements: &'a [E],
    write: impl Fn(&E, &mut Writer<File>) -> quick_xml::Result<()> + 'a,
) -> impl Fn(&mut Writer<File>) -> quick_xml::Result<()> + 'a {
    move |w| {
        for e in elements {
            write(e, w)?
        }
        Ok(())
    }
}
