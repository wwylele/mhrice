#![recursion_limit = "4096"]

use anyhow::*;
use rayon::prelude::*;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::*;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::*;
use std::sync::Mutex;
use structopt::*;
use walkdir::WalkDir;

mod align;
mod bitfield;
mod extract;
mod file_ext;
mod gpu;
mod gui;
mod hash;
mod mesh;
mod msg;
mod pak;
mod part_color;
mod pfb;
mod rcol;
mod rsz;
mod scn;
mod suffix;
mod tdb;
mod tex;
mod user;
mod uvs;

use gui::*;
use mesh::*;
use msg::*;
use pak::*;
use pfb::*;
use rcol::*;
use scn::*;
use tdb::*;
use tex::*;
use user::*;
use uvs::*;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(StructOpt)]
enum Mhrice {
    Dump {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        output: String,
    },

    DumpIndex {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        version: usize,
        #[structopt(short, long)]
        index: u32,
        #[structopt(short, long)]
        output: String,
    },

    Scan {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    GenJson {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    GenWebsite {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        output: String,
        #[structopt(long)]
        s3: Option<String>,
    },

    ReadTdb {
        #[structopt(short, long)]
        tdb: String,
    },

    ReadMsg {
        #[structopt(short, long)]
        msg: String,
    },

    ScanMsg {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        output: String,
    },

    GrepMsg {
        #[structopt(short, long)]
        pak: Vec<String>,

        pattern: String,
    },

    Grep {
        #[structopt(short, long)]
        pak: Vec<String>,

        pattern: String,
    },

    SearchPath {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    DumpTree {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        list: String,
        #[structopt(short, long)]
        output: String,
    },

    ScanMesh {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    ScanRcol {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    ScanTex {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    ScanGui {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    ScanUvs {
        #[structopt(short, long)]
        pak: Vec<String>,
    },

    DumpMesh {
        #[structopt(short, long)]
        mesh: String,
        #[structopt(short, long)]
        output: String,
    },

    DumpRcol {
        #[structopt(short, long)]
        rcol: String,
    },

    DumpMeat {
        #[structopt(short, long)]
        mesh: String,
        #[structopt(short, long)]
        rcol: String,
        #[structopt(short, long)]
        output: String,
    },

    DumpTex {
        #[structopt(short, long)]
        tex: String,
        #[structopt(short, long)]
        output: String,
    },

    DumpGui {
        #[structopt(short, long)]
        gui: String,
    },

    GenMeat {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        index: u32,
        #[structopt(short, long)]
        output: String,
    },

    GenResources {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        output: String,
    },

    Hash {
        input: String,
        #[structopt(short, long)]
        utf16: bool,
    },
}

fn open_pak_files(mut pak: Vec<String>) -> Result<Vec<File>> {
    if pak.len() == 1 && Path::new(&pak[0]).is_dir() {
        eprintln!("Listing all PAK files in the folder...");
        let dir = pak.pop().unwrap();
        let dir = Path::new(&dir);
        for entry in std::fs::read_dir(dir)?.into_iter() {
            let entry = entry?;
            if entry
                .file_name()
                .to_str()
                .context("Bad path")?
                .ends_with(".pak")
            {
                let path = entry.path().to_str().context("Bad path")?.to_string();
                pak.push(path);
            }
        }
        pak.sort();
        for path in &pak {
            eprintln!("Found PAK file: {}", path);
        }
    }

    pak.into_iter().map(|path| Ok(File::open(path)?)).collect()
}

fn dump(pak: Vec<String>, name: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let index = pak.find_file(&name).context("Cannot find subfile")?;
    println!("Index {:?}", index);
    let content = pak.read_file(index)?;
    std::fs::write(output, content)?;
    Ok(())
}

fn dump_index(pak: Vec<String>, version: usize, index: u32, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let content = pak.read_file_at(version, index)?;
    std::fs::write(output, content)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct TreeNode {
    parsed: bool,
    children: Vec<usize>,
    name: Option<String>,
    has_parent: bool,
    visited: bool,
}

/*fn visit_tree(nodes: &mut [TreeNode], current: usize, depth: i32) {
    for _ in 0..depth {
        print!("    ")
    }
    print!("- ");
    if let Some(name) = &nodes[current].name {
        println!("{}", name);
    } else {
        println!("{}", current);
    }
    for child in nodes[current].children.clone() {
        visit_tree(nodes, child, depth + 1);
    }
    nodes[current].visited = true;
}*/

fn scan(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    for index in pak.all_file_indexs() {
        let content = pak
            .read_file(index)
            .context(format!("Failed to open file at {:?}", index))?;
        if content.len() < 3 {
            continue;
        }

        if &content[0..3] == b"USR" {
            let _ = User::new(Cursor::new(&content))
                .context(format!("Failed to open USER at {:?}", index))?;
        } else if &content[0..3] == b"PFB" {
            let _ = Pfb::new(Cursor::new(&content))
                .context(format!("Failed to open PFB at {:?}", index))?;
        } else if &content[0..3] == b"SCN" {
            let _ = Scn::new(Cursor::new(&content))
                .context(format!("Failed to open SCN at {:?}", index))?;
        }
    }
    Ok(())
}

fn gen_json(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let pedia = extract::gen_pedia(&mut pak)?;
    let json = serde_json::to_string_pretty(&pedia)?;
    println!("{}", json);
    Ok(())
}

async fn upload_s3(
    path: PathBuf,
    len: u64,
    mime: &str,
    bucket: String,
    key: String,
    client: &S3Client,
) -> Result<()> {
    use futures::StreamExt;
    use tokio_util::codec;
    let file = tokio::fs::File::open(path).await?;
    let stream =
        codec::FramedRead::new(file, codec::BytesCodec::new()).map(|r| r.map(|r| r.freeze()));
    let request = PutObjectRequest {
        bucket,
        key,
        body: Some(ByteStream::new(stream)),
        content_length: Some(i64::try_from(len)?),
        content_type: Some(mime.to_owned()),
        ..PutObjectRequest::default()
    };
    client.put_object(request).await?;
    Ok(())
}

async fn cleanup_s3_old_files(path: &Path, bucket: String, client: &S3Client) -> Result<()> {
    let mut objects = vec![];
    let mut continuation_token = None;
    loop {
        println!("Continue listing files..");
        let request = ListObjectsV2Request {
            bucket: bucket.clone(),
            continuation_token: continuation_token.take(),
            delimiter: None,
            encoding_type: None,
            expected_bucket_owner: None,
            fetch_owner: None,
            max_keys: None,
            prefix: None,
            request_payer: None,
            start_after: None,
        };
        let result = client.list_objects_v2(request).await?;

        for object in result.contents.into_iter().flatten() {
            let key = if let Some(key) = object.key {
                key
            } else {
                continue;
            };
            if !path.join(&key).is_file() {
                println!("Deleting {}...", key);
                objects.push(ObjectIdentifier {
                    key,
                    version_id: None,
                });
            }
        }

        if result.is_truncated.unwrap_or(false) {
            continuation_token = result.next_continuation_token;
        } else {
            break;
        }
    }

    if !objects.is_empty() {
        let request = DeleteObjectsRequest {
            bucket,
            bypass_governance_retention: None,
            delete: Delete {
                objects,
                quiet: Some(true),
            },
            expected_bucket_owner: None,
            mfa: None,
            request_payer: None,
        };

        client.delete_objects(request).await?;
    }

    Ok(())
}

fn upload_s3_folder(path: &Path, bucket: String, client: &S3Client) -> Result<()> {
    use futures::future::try_join_all;
    use tokio::runtime::Runtime;
    let mut futures = vec![];
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let len = entry.metadata()?.len();
        let mime = match entry
            .path()
            .extension()
            .context("Missing extension")?
            .to_str()
            .context("Extension is not UTF-8")?
        {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "png" => "image/png",
            _ => bail!("Unknown extension"),
        };

        let key = entry
            .path()
            .strip_prefix(path)?
            .to_str()
            .context("Path contain non UTF-8 character")?
            .to_owned();

        println!("Uploading {}...", key);

        futures.push(upload_s3(
            entry.into_path(),
            len,
            mime,
            bucket.clone(),
            key,
            client,
        ));
    }

    let rt = Runtime::new()?;
    rt.block_on(try_join_all(futures))?;
    println!("Finished uploading. Cleaning deleted files...");
    rt.block_on(cleanup_s3_old_files(path, bucket, client))?;

    Ok(())
}

fn gen_website(pak: Vec<String>, output: String, s3: Option<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let pedia = extract::gen_pedia(&mut pak)?;
    let pedia_ex = extract::gen_pedia_ex(&pedia)?;
    extract::gen_website(&pedia, &pedia_ex, &output)?;
    extract::gen_resources(&mut pak, &Path::new(&output).to_owned().join("resources"))?;
    if let Some(bucket) = s3 {
        println!("Uploading to S3...");
        let s3client = S3Client::new(Region::UsEast1);
        upload_s3_folder(Path::new(&output), bucket, &s3client)?;
    }

    Ok(())
}

fn read_tdb(tdb: String) -> Result<()> {
    let _ = Tdb::new(File::open(tdb)?)?;
    Ok(())
}

fn read_msg(msg: String) -> Result<()> {
    let msg = Msg::new(File::open(msg)?)?;
    println!("{}", serde_json::to_string_pretty(&msg)?);
    Ok(())
}

fn scan_msg(pak: Vec<String>, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    std::fs::create_dir_all(&output)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 8 || file[4..8] != b"GMSG"[..] {
            continue;
        }
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
        std::fs::write(
            PathBuf::from(&output).join(format!("{}.txt", i.short_string())),
            serde_json::to_string_pretty(&msg)?,
        )?;
    }
    Ok(())
}

fn grep_msg(pak: Vec<String>, pattern: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    use regex::*;
    let regex = RegexBuilder::new(&pattern).build()?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 8 || file[4..8] != b"GMSG"[..] {
            continue;
        }
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
        for entry in &msg.entries {
            for text in &entry.content {
                if regex.is_match(text) {
                    println!("Found @ {:?}", i);
                }
            }
        }
    }
    Ok(())
}

fn scan_mesh(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 4 || file[0..4] != b"MESH"[..] {
            continue;
        }
        let _ = Mesh::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
    }
    Ok(())
}

fn scan_rcol(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 4 || file[0..4] != b"RCOL"[..] {
            continue;
        }
        let _ = Rcol::new(Cursor::new(&file), false).context(format!("at {:?}", i))?;
    }

    Ok(())
}

fn scan_tex(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 4 || file[0..4] != b"TEX\0"[..] {
            continue;
        }
        let _ = Tex::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
    }

    Ok(())
}

fn scan_gui(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 8 || file[4..8] != b"GUIR"[..] {
            continue;
        }
        let _ = Gui::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
    }

    Ok(())
}

fn scan_uvs(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if file.len() < 4 || file[0..4] != b".SVU"[..] {
            continue;
        }
        let _ = Uvs::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
    }

    Ok(())
}

fn grep(pak: Vec<String>, pattern: String) -> Result<()> {
    use regex::bytes::*;
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    println!("Searching for patterns \"{}\"", &pattern);
    let re = RegexBuilder::new(&pattern).unicode(false).build()?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if re.is_match(&file) {
            println!("Matched @ {:?}", i);
        }
    }
    Ok(())
}

fn search_path(pak: Vec<String>) -> Result<()> {
    let pak = Mutex::new(PakReader::new(open_pak_files(pak)?)?);
    let indexs = pak.lock().unwrap().all_file_indexs();
    let counter = std::sync::atomic::AtomicU32::new(0);
    let paths: std::collections::BTreeMap<String, Vec<I18nPakFileIndex>> = indexs
        .into_par_iter()
        .map(|index| {
            let file = pak.lock().unwrap().read_file(index)?;
            let mut paths = vec![];
            for &suffix in suffix::SUFFIX_MAP.keys() {
                let mut full_suffix = vec![0; (suffix.len() + 2) * 2];
                full_suffix[0] = b'.';
                for (i, &c) in suffix.as_bytes().iter().enumerate() {
                    full_suffix[i * 2 + 2] = c;
                }
                for (suffix_pos, window) in file.windows(full_suffix.len()).enumerate() {
                    if window != full_suffix {
                        continue;
                    }
                    let end = suffix_pos + full_suffix.len() - 2;
                    let mut begin = suffix_pos;
                    loop {
                        if begin < 2 {
                            break;
                        }
                        let earlier = begin - 2;
                        if !file[earlier].is_ascii_graphic() {
                            break;
                        }
                        if file[earlier + 1] != 0 {
                            break;
                        }

                        begin = earlier;
                    }
                    let mut path = String::new();
                    for pos in (begin..end).step_by(2) {
                        path.push(char::from(file[pos]));
                    }
                    let index = pak.lock().unwrap().find_file_i18n(&path)?;
                    paths.push((path, index));
                }
            }

            let counter_prev = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if counter_prev % 100 == 0 {
                eprintln!("{}", counter_prev)
            }

            Ok(paths)
        })
        .flat_map_iter(|paths: Result<_>| paths.unwrap())
        .collect();

    for (path, index) in paths {
        println!("{} $ {:?}", path, index);
    }

    Ok(())
}

fn dump_tree(pak: Vec<String>, list: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let list = File::open(list)?;
    let mut unvisited: std::collections::HashSet<_> = pak.all_file_indexs().into_iter().collect();
    for line in BufReader::new(list).lines() {
        let line = line?;
        let path = line.split(' ').next().context("Empty line")?;

        for i18n_index in pak.find_file_i18n(path)? {
            let index = i18n_index.index;
            let path_i18n = if i18n_index.language.is_empty() {
                path.to_owned()
            } else {
                format!("{}.{}", path, i18n_index.language)
            };
            let path = PathBuf::from(&output).join(path_i18n);

            std::fs::create_dir_all(path.parent().context("no parent")?)?;
            std::fs::write(path, &pak.read_file(index)?)?;
            unvisited.remove(&index);
        }
    }

    for index in unvisited {
        let path = PathBuf::from(&output)
            .join("_unknown")
            .join(index.short_string());
        std::fs::create_dir_all(path.parent().context("no parent")?)?;
        std::fs::write(path, &pak.read_file(index)?)?;
    }

    Ok(())
}

fn dump_mesh(mesh: String, output: String) -> Result<()> {
    let mesh = Mesh::new(File::open(mesh)?)?;
    mesh.dump(output)?;
    Ok(())
}

fn dump_rcol(rcol: String) -> Result<()> {
    let rcol = if let Ok(rcol) = Rcol::new(File::open(&rcol)?, true) {
        rcol
    } else {
        Rcol::new(File::open(&rcol)?, false)?
    };
    rcol.dump()?;
    Ok(())
}

fn dump_tex(tex: String, output: String) -> Result<()> {
    let tex = Tex::new(File::open(tex)?)?;
    tex.save_png(0, 0, Path::new(&output))?;
    Ok(())
}

fn dump_gui(gui: String) -> Result<()> {
    let gui = Gui::new(File::open(gui)?)?;
    println!("{}", serde_json::to_string_pretty(&gui)?);
    Ok(())
}

fn dump_meat(_mesh: String, _rcol: String, _output: String) -> Result<()> {
    /*use std::io::*;
    let mesh = Mesh::new(File::open(mesh)?)?;
    let mut rcol = Rcol::new(File::open(rcol)?, true)?;

    rcol.apply_skeleton(&mesh)?;
    let (vertexs, indexs) = rcol.color_monster_model(&mesh)?;

    let mut output = File::create(output)?;

    writeln!(output, "ply")?;
    writeln!(output, "format ascii 1.0")?;
    writeln!(output, "element vertex {}", vertexs.len())?;
    writeln!(output, "property float x")?;
    writeln!(output, "property float y")?;
    writeln!(output, "property float z")?;
    writeln!(output, "property float red")?;
    writeln!(output, "property float green")?;
    writeln!(output, "property float blue")?;
    writeln!(output, "element face {}", indexs.len() / 3)?;
    writeln!(output, "property list uchar int vertex_index")?;
    writeln!(output, "end_header")?;

    let colors = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0],
        [1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 0.5, 0.0],
        [0.5, 1.0, 0.0],
        [1.0, 0.0, 0.5],
        [0.5, 0.0, 1.0],
        [0.0, 1.0, 0.5],
        [0.0, 0.5, 1.0],
    ];

    for vertex in vertexs {
        let color = if let Some(meat) = vertex.meat {
            colors[meat]
        } else {
            [0.5, 0.5, 0.5]
        };
        writeln!(
            output,
            "{} {} {} {} {} {}",
            vertex.position.x, -vertex.position.z, vertex.position.y, color[0], color[1], color[2]
        )?;
    }

    for index in indexs.chunks(3) {
        writeln!(output, "3 {} {} {}", index[0], index[1], index[2])?;
    }

    Ok(())*/
    unimplemented!()
}

fn gen_meat(pak: Vec<String>, index: u32, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let mesh_path = format!("enemy/em{0:03}/00/mod/em{0:03}_00.mesh", index);
    let rcol_path = format!(
        "enemy/em{0:03}/00/collision/em{0:03}_00_colliders.rcol",
        index
    );
    let mesh = pak.find_file(&mesh_path)?;
    let rcol = pak.find_file(&rcol_path)?;
    let mesh = Mesh::new(Cursor::new(pak.read_file(mesh)?))?;
    let mut rcol = Rcol::new(Cursor::new(pak.read_file(rcol)?), true)?;
    rcol.apply_skeleton(&mesh)?;
    let (vertexs, indexs) = rcol.color_monster_model(&mesh)?;

    let gpu::HitzoneDiagram { meat, .. } = gpu::gen_hitzone_diagram(vertexs, indexs)?;

    meat.save_png(Path::new(&output))?;

    Ok(())
}

fn gen_resources(pak: Vec<String>, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    extract::gen_resources(&mut pak, Path::new(&output))?;

    Ok(())
}

fn hash(input: String, utf16: bool) {
    if utf16 {
        println!("{:08X}", hash::hash_as_utf16(&input));
    } else {
        println!("{:08X}", hash::hash_as_utf8(&input));
    }
}

fn main() -> Result<()> {
    match Mhrice::from_args() {
        Mhrice::Dump { pak, name, output } => dump(pak, name, output),
        Mhrice::DumpIndex {
            pak,
            version,
            index,
            output,
        } => dump_index(pak, version, index, output),
        Mhrice::Scan { pak } => scan(pak),
        Mhrice::GenJson { pak } => gen_json(pak),
        Mhrice::GenWebsite { pak, output, s3 } => gen_website(pak, output, s3),
        Mhrice::ReadTdb { tdb } => read_tdb(tdb),
        Mhrice::ReadMsg { msg } => read_msg(msg),
        Mhrice::ScanMsg { pak, output } => scan_msg(pak, output),
        Mhrice::GrepMsg { pak, pattern } => grep_msg(pak, pattern),
        Mhrice::Grep { pak, pattern } => grep(pak, pattern),
        Mhrice::SearchPath { pak } => search_path(pak),
        Mhrice::DumpTree { pak, list, output } => dump_tree(pak, list, output),
        Mhrice::ScanMesh { pak } => scan_mesh(pak),
        Mhrice::ScanRcol { pak } => scan_rcol(pak),
        Mhrice::ScanTex { pak } => scan_tex(pak),
        Mhrice::ScanGui { pak } => scan_gui(pak),
        Mhrice::ScanUvs { pak } => scan_uvs(pak),
        Mhrice::DumpMesh { mesh, output } => dump_mesh(mesh, output),
        Mhrice::DumpRcol { rcol } => dump_rcol(rcol),
        Mhrice::DumpMeat { mesh, rcol, output } => dump_meat(mesh, rcol, output),
        Mhrice::DumpTex { tex, output } => dump_tex(tex, output),
        Mhrice::DumpGui { gui } => dump_gui(gui),
        Mhrice::GenMeat { pak, index, output } => gen_meat(pak, index, output),
        Mhrice::GenResources { pak, output } => gen_resources(pak, output),
        Mhrice::Hash { input, utf16 } => {
            hash(input, utf16);
            Ok(())
        }
    }
}
