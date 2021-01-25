use anyhow::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::Cursor;
use structopt::*;

mod align;
mod extract;
mod file_ext;
mod pak;
mod pfb;
mod rsz;
mod scn;
mod suffix;
mod user;

use pak::*;
use pfb::*;
use scn::*;
use user::*;

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

fn main() -> Result<()> {
    match Mhrice::from_args() {
        Mhrice::Dump { pak, name, output } => dump(pak, name, output),
        Mhrice::DumpIndex { pak, index, output } => dump_index(pak, index, output),
        Mhrice::Scan { pak } => scan(pak),
        Mhrice::GenJson { pak } => gen_json(pak),
    }
}
