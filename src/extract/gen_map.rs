use super::gen_common::*;
use super::gen_website::*;
use super::pedia::*;
use super::prepare_map::*;
use super::sink::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

fn map_page(id: i32) -> String {
    format!("{id:02}.html")
}

fn gen_map(id: i32, map: &GameMap, mut output: impl Write) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Item - MHRice"</title>
                { head_common() }
            </head>
            <body>
            <main><div class="container"> <div class="content">

            <h1 class="title">
                "map"
            </h1>

            <div class="mh-map">
            {(0..map.layer_count).map(|j| html!(
                <img class="mh-map-layer" src={format!("/resources/map{id:02}_{j}.png")}/>
            ))}

            {map.pops.iter().map(|pop| {
                let x = (pop.x + map.x_offset) / map.map_scale * 100.0;
                let y = (pop.y + map.y_offset) / map.map_scale * 100.0;
                html!(
                    <span class="mh-map-pop icon has-text-warning" style={format!("left:{x}%;top:{y}%")}>
                    <i class="fas fa-exclamation-triangle"/>
                    </span>)
            })}

            </div>

            </div></div></main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_maps(pedia: &Pedia, pedia_ex: &PediaEx, output: &impl Sink) -> Result<()> {
    let map_path = output.sub_sink("map")?;
    for (&id, map) in &pedia.maps {
        let path = map_path.create_html(&map_page(id))?;
        gen_map(id, map, path)?
    }
    Ok(())
}
