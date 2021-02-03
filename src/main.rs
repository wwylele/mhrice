#![recursion_limit = "2048"]

use anyhow::*;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::*;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs::File;
use std::io::Cursor;
use std::path::*;
use structopt::*;
use walkdir::WalkDir;

mod align;
mod extract;
mod file_ext;
mod msg;
mod pak;
mod pfb;
mod rsz;
mod scn;
mod suffix;
mod tdb;
mod user;

use msg::*;
use pak::*;
use pfb::*;
use scn::*;
use tdb::*;
use user::*;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(StructOpt)]
enum Mhrice {
    Dump {
        #[structopt(short, long)]
        pak: String,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        output: String,
    },

    DumpIndex {
        #[structopt(short, long)]
        pak: String,
        #[structopt(short, long)]
        index: u32,
        #[structopt(short, long)]
        output: String,
    },

    Scan {
        #[structopt(short, long)]
        pak: String,
    },

    GenJson {
        #[structopt(short, long)]
        pak: String,
    },

    GenWebsite {
        #[structopt(short, long)]
        pak: String,
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
        pak: String,
        #[structopt(short, long)]
        output: String,
    },
}

fn dump(pak: String, name: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(File::open(pak)?)?;
    let (index, full_path) = pak.find_file(&name).context("Cannot find subfile")?;
    println!("Full path: {}", full_path);
    println!("Index {}", index.raw());
    let content = pak.read_file(index)?;
    std::fs::write(output, content)?;
    Ok(())
}

fn dump_index(pak: String, index: u32, output: String) -> Result<()> {
    let mut pak = PakReader::new(File::open(pak)?)?;
    let content = pak.read_file_at(index)?;
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

fn visit_tree(nodes: &mut [TreeNode], current: usize, depth: i32) {
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
}

fn scan(pak: String) -> Result<()> {
    let mut pak = PakReader::new(File::open(pak)?)?;

    let mut nodes = vec![
        TreeNode {
            parsed: false,
            children: vec![],
            name: None,
            has_parent: false,
            visited: false,
        };
        pak.get_file_count().try_into().unwrap()
    ];

    for index in 0..pak.get_file_count() {
        let content = pak
            .read_file_at(index)
            .context(format!("Failed to open file at {}", index))?;
        if content.len() < 3 {
            continue;
        }

        if &content[0..3] == b"USR" {
            let user = User::new(Cursor::new(&content))
                .context(format!("Failed to open USER at {}", index))?;
            let index: usize = index.try_into()?;
            nodes[index].parsed = true;

            let children = user
                .children
                .into_iter()
                .map(|c| c.name)
                .chain(user.resource_names);

            for child in children {
                let (cindex, _) = if let Ok(found) = pak.find_file(&child) {
                    found
                } else {
                    println!("missing {}", child);
                    continue;
                };
                let cindex: usize = cindex.raw().try_into()?;
                nodes[cindex].name = Some(child);
                nodes[cindex].has_parent = true;
                nodes[index].children.push(cindex);
            }
        } else if &content[0..3] == b"PFB" {
            let pfb = Pfb::new(Cursor::new(&content))
                .context(format!("Failed to open PFB at {}", index))?;
            let index: usize = index.try_into()?;
            nodes[index].parsed = true;

            let children = pfb
                .children
                .into_iter()
                .map(|c| c.name)
                .chain(pfb.resource_names);

            for child in children {
                let (cindex, _) = if let Ok(found) = pak.find_file(&child) {
                    found
                } else {
                    println!("missing {}", child);
                    continue;
                };
                let cindex: usize = cindex.raw().try_into()?;
                nodes[cindex].name = Some(child);
                nodes[cindex].has_parent = true;
                nodes[index].children.push(cindex);
            }
        } else if &content[0..3] == b"SCN" {
            let scn = Scn::new(Cursor::new(&content))
                .context(format!("Failed to open SCN at {}", index))?;
            let index: usize = index.try_into()?;
            nodes[index].parsed = true;

            let children = scn
                .children
                .into_iter()
                .map(|c| c.name)
                .chain(scn.resource_a_names)
                .chain(scn.resource_b_names);

            for child in children {
                let (cindex, _) = if let Ok(found) = pak.find_file(&child) {
                    found
                } else {
                    println!("missing {}", child);
                    continue;
                };
                let cindex: usize = cindex.raw().try_into()?;
                nodes[cindex].name = Some(child);
                nodes[cindex].has_parent = true;
                nodes[index].children.push(cindex);
            }
        }
    }

    for index in 0..nodes.len() {
        if !nodes[index].parsed || nodes[index].has_parent {
            continue;
        }

        visit_tree(&mut nodes, index, 0);
    }

    let named = nodes.iter().filter(|p| p.name.is_some()).count();
    println!("Named file ratio = {}", named as f64 / nodes.len() as f64);

    for user in nodes {
        if user.parsed && !user.visited {
            bail!("Cycle detected")
        }
    }

    println!("Done");
    Ok(())
}

fn gen_json(pak: String) -> Result<()> {
    let pedia = extract::gen_pedia(pak)?;
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
            _ => "application/octet-stream",
        };

        let key = entry
            .path()
            .strip_prefix(path)?
            .to_str()
            .context("Path contain non UTF-8 character")?
            .to_owned();

        futures.push(upload_s3(
            entry.into_path(),
            len,
            mime,
            bucket.clone(),
            key,
            client,
        ));
    }

    Runtime::new()?.block_on(try_join_all(futures))?;

    Ok(())
}

fn gen_website(pak: String, output: String, s3: Option<String>) -> Result<()> {
    let pedia = extract::gen_pedia(pak)?;
    extract::gen_website(pedia, &output)?;
    if let Some(bucket) = s3 {
        println!("Uploading to S3...");
        let s3client = S3Client::new(Region::UsEast1);
        upload_s3_folder(Path::new(&output), bucket, &s3client)?;
    }

    Ok(())
}

fn read_tdb(tdb: String) -> Result<()> {
    let tdb = Tdb::new(File::open(tdb)?)?;
    Ok(())
}

fn read_msg(msg: String) -> Result<()> {
    let msg = Msg::new(File::open(msg)?)?;
    Ok(())
}

fn scan_msg(pak: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(File::open(pak)?)?;
    let count = pak.get_file_count();
    for i in 0..count {
        let file = pak.read_file_at(i)?;
        if file.len() < 8 || file[4..8] != b"GMSG"[..] {
            continue;
        }
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {}", i))?;
        std::fs::write(
            PathBuf::from(&output).join(format!("{}.txt", i)),
            serde_json::to_string_pretty(&msg)?,
        )?;
    }
    Ok(())
}

fn main() -> Result<()> {
    match Mhrice::from_args() {
        Mhrice::Dump { pak, name, output } => dump(pak, name, output),
        Mhrice::DumpIndex { pak, index, output } => dump_index(pak, index, output),
        Mhrice::Scan { pak } => scan(pak),
        Mhrice::GenJson { pak } => gen_json(pak),
        Mhrice::GenWebsite { pak, output, s3 } => gen_website(pak, output, s3),
        Mhrice::ReadTdb { tdb } => read_tdb(tdb),
        Mhrice::ReadMsg { msg } => read_msg(msg),
        Mhrice::ScanMsg { pak, output } => scan_msg(pak, output),
    }
}
