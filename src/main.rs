#![recursion_limit = "4096"]

use anyhow::{anyhow, bail, Context, Result};
use clap::*;
use minidump::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::*;
use std::sync::Mutex;

mod align;
mod bitfield;
mod collada;
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

use extract::hash_store::*;
use extract::logger::*;
use extract::sink::*;
use file_ext::*;
use gui::*;
use mesh::*;
use msg::*;
use pak::*;
use pfb::*;
use rcol::*;
use scn::*;
use tex::*;
use user::*;
use uvs::*;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

const TDB_ANCHOR: &[u8] = b"TDB\0\x47\0\0\0";

#[derive(clap::Parser)]
pub struct TdbOptions {
    /// Optional output to a file in JSON
    #[clap(short, long)]
    pub json: Option<String>,

    /// Optional output to a folder of separate JSON files.
    /// Each file is a chunk of 1000 types.
    /// If this is dumped from DMP, you can also used the output
    /// with misc/ghidra_importTdb.py to import symbols into Ghidra.
    #[clap(long)]
    pub json_split: Option<String>,

    /// Optional output to a file in C#.
    #[clap(short, long)]
    pub cs: Option<String>,

    /// C#: Remove runtime addresses.
    #[clap(long)]
    pub no_runtime: bool,

    /// C#: Remove classes in System namespace.
    #[clap(long)]
    pub no_system: bool,

    /// C#: Remove template instantiation and array.
    #[clap(long)]
    pub no_compound: bool,

    /// C#: Remove type flags.
    #[clap(long)]
    pub no_type_flag: bool,
}

#[derive(clap::Parser)]
enum Mhrice {
    /// Dump a sub-file with specific name from the PAK file
    Dump {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Name of the sub-file to dump
        #[clap(short, long)]
        name: String,
        /// Output path
        #[clap(short, long)]
        output: String,
    },

    /// Dump a sub-file with specific index from the PAK file
    DumpIndex {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        #[clap(short, long, default_value_t = 0)]
        version: usize,
        /// Index of the sub-file to dump
        #[clap(short, long)]
        index: usize,
        /// Output path
        #[clap(short, long)]
        output: String,
    },

    /// Scan the PAK file to verify known file formats that contains RSZ
    ///
    /// This will verify the files conform the format,
    /// and list the CRC mismatch among RSZ types found in them.
    ScanRsz {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Print all gathered CRC instead of mismatched ones
        #[clap(short, long)]
        crc: bool,
    },

    /// Generate JSON file of game information from the PAK file
    GenJson {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Record SHA-256 of the PAK file
        #[clap(short, long)]
        sha: bool,
    },

    /// Generate the mhrice website the PAK file
    GenWebsite {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Output directory
        #[clap(short, long)]
        output: String,
        /// Target website origin. e.g. "https://mhrice.info"
        #[clap(long)]
        origin: Option<String>,
        /// Record SHA-256 of the PAK file
        #[clap(short, long)]
        sha: bool,
    },

    /// Find TDB in the given binary and print the converted TDB file
    ReadTdb {
        /// Path to a TDB file, or a binary that contains one
        #[clap(short, long)]
        tdb: String,

        #[clap(flatten)]
        options: TdbOptions,
    },

    /// Print messages from a MSG file
    ReadMsg {
        /// Path to the MSG file
        #[clap(short, long)]
        msg: String,
    },

    /// Scan the PAK file and output messages from all MSG files
    ScanMsg {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Output directory
        #[clap(short, long)]
        output: String,
    },

    /// Scan the PAK file and find a regex pattern in MSG files
    GrepMsg {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// The regex pattern
        pattern: String,
    },

    /// Scan the PAK file and find a regex pattern in all files
    Grep {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Search for UTF-16 string
        #[clap(short, long)]
        utf16: bool,
        /// The regex pattern
        pattern: String,
    },

    /// Scan the PAK file as well as optionally full minidump samples
    /// and print all potential sub-file names
    SearchPath {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,

        /// Path to the full minidump files
        #[clap(short, long)]
        dmp: Vec<String>,
    },

    /// Dump all sub-files from the PAK file
    DumpTree {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// File name list, can be the output from search-path command
        #[clap(short, long)]
        list: String,
        /// Output directory
        #[clap(short, long)]
        output: String,
    },

    /// Scan the PAK file and verify the format of all MESH files
    ScanMesh {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
    },

    /// Scan the PAK file and verify the format of all TEX files
    ScanTex {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
    },

    /// Scan the PAK file and verify the format of all GUI files
    ScanGui {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
    },

    /// Scan the PAK file and verify the format of all UVS files
    ScanUvs {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
    },

    /// Convert a MESH file to a OBJ model file
    DumpMesh {
        /// Path to the MESH file
        #[clap(short, long)]
        mesh: String,
        /// Output file
        #[clap(short, long)]
        output: String,
    },

    /// Convert a MESH file to a DAE (Collada) model file
    DumpMeshDae {
        /// Path to the MESH file
        #[clap(short, long)]
        mesh: String,
        /// Output file
        #[clap(short, long)]
        output: String,
    },

    /// Print information of a RCOL file
    DumpRcol {
        /// Path to the RCOL file
        #[clap(short, long)]
        rcol: String,
    },

    DumpMeat {
        #[clap(short, long)]
        mesh: String,
        #[clap(short, long)]
        rcol: String,
        #[clap(short, long)]
        output: String,
    },

    /// Convert a TEX file to a PNG file
    DumpTex {
        /// Path to the TEX file
        #[clap(short, long)]
        tex: String,
        /// Output PNG file
        #[clap(short, long)]
        output: String,
        /// Optional 4-character swizzle code. The default is "rgba"
        #[clap(short, long, default_value = "rgba")]
        swizzle: String,
    },

    /// Print information of a GUI file
    DumpGui {
        /// Path to the GUI file
        #[clap(short, long)]
        gui: String,
    },

    /// Generate meat diagram PNG file for a monster
    GenMeat {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Monster EmTypes ID
        #[clap(short, long)]
        index: u32,
        /// Output PNG file
        #[clap(short, long)]
        output: String,
    },

    /// Generate resource files (images etc.) for the website
    GenResources {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// Output directory
        #[clap(short, long)]
        output: String,
    },

    /// Calculate MurmurHash of a string
    Hash {
        /// The string to be hashed
        input: String,
        /// Hash as UTF-16. Otherwise, hash as UTF-8
        #[clap(short, long)]
        utf16: bool,
    },

    /// Print the content of a USER file
    ReadUser {
        /// Path to the USER file
        #[clap(short, long)]
        user: String,
        /// Version of the game, optional
        #[clap(short, long)]
        version: Option<u32>,
    },

    /// Find TDB in the a full minidump (DMP file) and print the converted TDB
    ReadDmpTdb {
        /// Path to the full minidump (DMP file)
        #[clap(short, long)]
        dmp: String,
        /// Optional memory address where TDB is allocated
        ///
        /// Specify this to skip search the entire minidump
        #[clap(short, long)]
        address: Option<String>,

        #[clap(flatten)]
        options: TdbOptions,
    },

    /// Print information of a SCN file
    DumpScn {
        /// Path to the SCN file
        #[clap(short, long)]
        scn: String,
    },

    /// Print information of a PFB file
    DumpPfb {
        /// Path to the SCN file
        #[clap(short, long)]
        pfb: String,
    },

    /// Print information of a SCN tree
    Scene {
        /// Paths to the PAK files, folder containing PAK files, or a .txt file listing all PAK files
        #[clap(short, long)]
        pak: Vec<String>,
        /// The name of the root SCN file
        #[clap(short, long)]
        name: String,
    },

    /// Print runtime information of a type
    TypeInfo {
        /// Path to the full minidump (DMP file)
        #[clap(short, long)]
        dmp: String,
        /// Hash of the type in hex
        #[clap(long)]
        hash: String,
        /// CRC of the type in hex
        #[clap(short, long)]
        crc: String,
    },

    Map {
        #[clap(short, long)]
        pak: Vec<String>,

        #[clap(short, long)]
        name: String,

        #[clap(short, long)]
        scale: String,

        #[clap(short, long)]
        tex: String,

        #[clap(short, long)]
        output: String,
    },
}

fn open_pak_files(mut pak: Vec<String>) -> Result<Vec<File>> {
    if pak.len() == 1 && Path::new(&pak[0]).is_dir() {
        eprintln!("Listing all PAK files in the folder...");
        let dir = pak.pop().unwrap();
        let dir = Path::new(&dir);
        for entry in std::fs::read_dir(dir)? {
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
            eprintln!("Found PAK file: {path}");
        }
    } else if pak.len() == 1 && pak[0].to_lowercase().ends_with(".txt") {
        eprintln!("Listing all PAK files from the txt file...");
        let mut txt = BufReader::new(File::open(&pak[0])?);
        pak.clear();
        loop {
            let mut line = String::new();
            if txt.read_line(&mut line)? == 0 {
                break;
            }
            let path = line.trim().to_owned();
            eprintln!("Found PAK file: {path}");
            pak.push(path);
        }
    }

    pak.into_iter().map(|path| Ok(File::open(path)?)).collect()
}

fn dump(pak: Vec<String>, name: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let index = pak.find_file(&name).context("Cannot find subfile")?;
    println!("Index {index:?}");
    let content = pak.read_file(index)?;
    std::fs::write(output, content)?;
    Ok(())
}

fn dump_index(pak: Vec<String>, version: usize, index: usize, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let content = pak.read_file_at(version, index)?;
    std::fs::write(output, content)?;
    Ok(())
}

/*

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
}*/

fn scan_rsz(pak: Vec<String>, print_all: bool) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let mut crc_mismatches = BTreeMap::new();

    for index in pak.all_file_indexs() {
        let content = pak
            .read_file(index)
            .context(format!("Failed to open file at {index:?}"))?;
        if content.len() < 4 {
            continue;
        }

        if &content[0..3] == b"USR" {
            User::new(Cursor::new(&content))
                .context(format!("Failed to open USER at {index:?}"))?
                .rsz
                .verify_crc(&mut crc_mismatches, print_all);
        } else if &content[0..3] == b"PFB" {
            Pfb::new(Cursor::new(&content))
                .context(format!("Failed to open PFB at {index:?}"))?
                .rsz
                .verify_crc(&mut crc_mismatches, print_all);
        } else if &content[0..3] == b"SCN" {
            Scn::new(Cursor::new(&content))
                .context(format!("Failed to open SCN at {index:?}"))?
                .rsz
                .verify_crc(&mut crc_mismatches, print_all);
        } else if &content[0..4] == b"RCOL" {
            Rcol::new(Cursor::new(&content), false)
                .context(format!("Failed to open RCOL at {index:?}"))?
                .rsz
                .verify_crc(&mut crc_mismatches, print_all);
        }
    }

    for (symbol, crc) in crc_mismatches {
        println!("Mismatch CRC {crc:08X} for {symbol}")
    }

    Ok(())
}

fn gen_json(pak: Vec<String>, sha: bool) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let mut logger_root = LoggerRoot::new();
    let logger = &mut logger_root.logger();
    let pedia = extract::gen_pedia(&mut pak, sha, logger)?;
    let json = serde_json::to_string_pretty(&pedia)?;
    println!("{json}");
    Ok(())
}

fn gen_website_to_sink(
    pak: Vec<String>,
    sink: impl Sink,
    config: extract::WebsiteConfig,
    sha: bool,
) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let mut logger_root = LoggerRoot::new();
    {
        let logger = &mut logger_root.logger();
        let pedia = extract::gen_pedia(&mut pak, sha, logger)?;
        let pedia_ex = extract::gen_pedia_ex(&pedia, logger)?;
        sink.create("mhrice.json")?
            .write_all(serde_json::to_string_pretty(&pedia)?.as_bytes())?;
        let mut hash_store = HashStore::new();
        extract::gen_website(&mut hash_store, &pedia, &pedia_ex, &config, &sink)?;
        extract::gen_resources(&mut pak, &sink.sub_sink("resources")?, logger)?;
    }

    let mut log = sink.create("log.html")?;
    write!(log, "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>MHRice log</title></head><body><pre>\n{}\n</pre></body></html>", logger_root.finalize())?;
    drop(log);

    sink.finalize()?;
    Ok(())
}

fn gen_website(pak: Vec<String>, output: String, origin: Option<String>, sha: bool) -> Result<()> {
    let config = extract::WebsiteConfig { origin };
    if let Some(bucket_and_prefix) = output.strip_prefix("S3://") {
        let (bucket, prefix) = if let Some((bucket, prefix)) = bucket_and_prefix.split_once('/') {
            (bucket, prefix)
        } else {
            (bucket_and_prefix, "")
        };
        let sink = S3Sink::init(bucket.to_string(), prefix.to_string())?;
        gen_website_to_sink(pak, sink, config, sha)?;
    } else if output == "null://" {
        let sink = NullSink;
        gen_website_to_sink(pak, sink, config, sha)?;
    } else {
        let sink = DiskSink::init(Path::new(&output))?;
        gen_website_to_sink(pak, sink, config, sha)?;
    }

    Ok(())
}

struct OffsetFile<F> {
    file: F,
    offset: u64,
}

impl<F: Seek> OffsetFile<F> {
    fn new(file: F, offset: u64) -> Result<Self> {
        let mut of = OffsetFile { file, offset };
        of.rewind()?;
        Ok(of)
    }
}

impl<F: Read> Read for OffsetFile<F> {
    fn read(&mut self, b: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.file.read(b)
    }
}

impl<F: Seek> Seek for OffsetFile<F> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(_) => self.file.seek(pos),
            SeekFrom::Start(x) => self.file.seek(SeekFrom::Start(x + self.offset)),
            _ => unimplemented!(),
        }
        .map(|x| x - self.offset)
    }
}

fn read_tdb(tdb: String, options: TdbOptions) -> Result<()> {
    let mut file = BufReader::new(File::open(tdb)?);
    let offset = loop {
        let mut magic = vec![0; TDB_ANCHOR.len()];
        file.read_exact(&mut magic)?;
        if magic == TDB_ANCHOR {
            break file.seek(SeekFrom::Current(-(TDB_ANCHOR.len() as i64)))?;
        } else {
            file.seek(SeekFrom::Current(-(TDB_ANCHOR.len() as i64) + 1))?;
        }
    };

    tdb::print(OffsetFile::new(file, offset)?, 0, options)?;
    Ok(())
}

struct MinidumpReader<'a> {
    memory_list: &'a MinidumpMemory64List<'a>,
    pos: u64,
}

impl<'a> MinidumpReader<'a> {
    fn new(memory_list: &'a MinidumpMemory64List<'a>) -> Self {
        MinidumpReader {
            memory_list,
            pos: 0,
        }
    }
}

impl<'a> Read for MinidumpReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut offset = 0;
        while offset != buf.len() as u64 {
            if let Some(block) = self.memory_list.memory_at_address(self.pos) {
                let available = std::cmp::min(
                    buf.len() as u64 - offset,
                    block.base_address + block.size - self.pos,
                );
                buf[offset as usize..][..available as usize].copy_from_slice(
                    &block.bytes[(self.pos - block.base_address) as usize..][..available as usize],
                );
                self.pos += available;
                offset += available;
            } else {
                buf[offset as usize] = 0xCC;
                self.pos += 1;
                offset += 1;
            }
        }
        Ok(buf.len())
    }
}

impl<'a> Seek for MinidumpReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(s) => self.pos = s,
            SeekFrom::End(e) => self.pos = e as u64,
            SeekFrom::Current(c) => self.pos = (self.pos as i64 + c) as u64,
        }
        Ok(self.pos)
    }
}

fn read_dmp_tdb(dmp: String, address: Option<String>, options: TdbOptions) -> Result<()> {
    let dmp = Minidump::read_path(dmp).map_err(|e| anyhow!(e))?;
    let memory = dmp
        .get_stream::<MinidumpMemory64List>()
        .map_err(|e| anyhow!(e))
        .context("No full dump memory found")?;

    if let Some(address) = address {
        let base = if let Some(hex) = address.strip_prefix("0x") {
            u64::from_str_radix(hex, 16)?
        } else {
            address.parse()?
        };
        let file = MinidumpReader::new(&memory);

        tdb::print(file, base, options)?;

        return Ok(());
    }

    for block in memory.iter() {
        if let Some(pos) = block
            .bytes
            .windows(TDB_ANCHOR.len())
            .position(|w| w == TDB_ANCHOR)
        {
            let base = block.base_address + u64::try_from(pos)?;
            eprintln!("Found at address 0x{base:016X}");
            let file = MinidumpReader::new(&memory);

            tdb::print(file, base, options)?;

            break;
        }
    }

    Ok(())
}

fn type_info(dmp: String, hash: String, crc: String) -> Result<()> {
    let hash = u32::from_str_radix(&hash, 16)?;
    let crc = u32::from_str_radix(&crc, 16)?;
    let dmp = Minidump::read_path(dmp).map_err(|e| anyhow!(e))?;
    let memory = dmp
        .get_stream::<MinidumpMemory64List>()
        .map_err(|e| anyhow!(e))?;

    let mut address = 0;
    'outer: for block in memory.iter() {
        let mut offset = 0;
        loop {
            if offset + 0x2C > block.bytes.len() {
                break;
            }
            let read_hash = u32::from_le_bytes(block.bytes[offset..][..4].try_into().unwrap());
            if read_hash == hash {
                let read_crc =
                    u32::from_le_bytes(block.bytes[offset + 0x28..][..4].try_into().unwrap());
                if read_crc == crc {
                    address = block.base_address + u64::try_from(offset - 8)?;
                    break 'outer;
                }
            }
            offset += 4;
        }
    }

    if address == 0 {
        bail!("Not found")
    }

    println!("Found at 0x{address:016X}");

    while address != 0 {
        println!("----------------------------------------");
        let mut memory = MinidumpReader::new(&memory);
        memory.seek(SeekFrom::Start(address + 0x20))?;
        let name_address = memory.read_u64()?;
        memory.seek(SeekFrom::Start(name_address))?;
        let name = memory.read_u8str()?;
        println!("name: {name}");
        memory.seek(SeekFrom::Start(address + 0x50))?;
        let fields_offset = memory.read_u64()?;
        memory.seek(SeekFrom::Start(fields_offset + 0x28))?;
        let deserializer = memory.read_u64()?;
        println!("Deserializer: 0x{deserializer:016X}");

        memory.seek(SeekFrom::Start(address + 0x38))?;
        address = memory.read_u64()?;
    }

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
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
        for e in &msg.entries {
            extract::gen_multi_lang(e);
        }
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
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
        for entry in &msg.entries {
            for text in &entry.content {
                if regex.is_match(text) {
                    println!("Found @ {i:?}");
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
        let _ = Mesh::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
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
        let _ = Tex::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
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
        let _ = Gui::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
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
        let _ = Uvs::new(Cursor::new(&file)).context(format!("at {i:?}"))?;
    }

    Ok(())
}

fn grep(pak: Vec<String>, utf16: bool, mut pattern: String) -> Result<()> {
    use regex::bytes::*;
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    if utf16 {
        pattern = pattern
            .encode_utf16()
            .map(|u| {
                let b = u.to_le_bytes();
                format!("\\x{:02X}\\x{:02X}", b[0], b[1])
            })
            .fold("".to_string(), |a, b| a + &b);
    }
    println!("Searching for patterns \"{}\"", &pattern);
    let re = RegexBuilder::new(&pattern).unicode(false).build()?;
    for i in pak.all_file_indexs() {
        let file = pak.read_file(i)?;
        if re.is_match(&file) {
            println!("Matched @ {i:?}");
        }
    }
    Ok(())
}

fn search_path(pak: Vec<String>, dmp: Vec<String>) -> Result<()> {
    let pak = Mutex::new(PakReader::new(open_pak_files(pak)?)?);
    let indexs = pak.lock().unwrap().all_file_indexs();
    let counter = std::sync::atomic::AtomicU32::new(0);

    let mut paths: Vec<(String, Vec<I18nPakFileIndex>)> = vec![];

    fn accept_char(c: u8) -> bool {
        if c == b' ' {
            return true;
        }
        if !c.is_ascii_graphic() {
            return false;
        }
        #[allow(clippy::needless_raw_string_hashes)]
        if br###""*\:<>?*|"###.contains(&c) {
            return false;
        }
        true
    }

    let search_memory = |memory: &[u8]| {
        let mut paths = vec![];
        for &suffix in suffix::SUFFIX_MAP.keys() {
            let mut full_suffix = vec![0; (suffix.len() + 2) * 2];
            full_suffix[0] = b'.';
            for (i, &c) in suffix.as_bytes().iter().enumerate() {
                full_suffix[i * 2 + 2] = c;
            }
            for (suffix_pos, window) in memory.windows(full_suffix.len()).enumerate() {
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
                    if !accept_char(memory[earlier]) {
                        break;
                    }
                    if memory[earlier + 1] != 0 {
                        break;
                    }

                    begin = earlier;
                }
                let mut path = String::new();
                for pos in (begin..end).step_by(2) {
                    path.push(char::from(memory[pos]));
                }
                let index = pak.lock().unwrap().find_file_i18n(&path)?;
                paths.push((path, index));
            }
        }

        let counter_prev = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if counter_prev % 100 == 0 {
            eprintln!("Found {counter_prev} paths so far")
        }

        Ok(paths)
    };

    for dmp in dmp {
        eprintln!("Scanning {dmp}..");

        let dmp = Minidump::read_path(dmp).map_err(|e| anyhow!(e))?;
        let memory = dmp
            .get_stream::<MinidumpMemory64List>()
            .map_err(|e| anyhow!(e))
            .context("No full dump memory found")?;

        let mut memory: Vec<_> = memory.iter().collect();
        // merge memory blocks
        memory.sort_by_key(|memory| memory.base_address);
        use std::borrow::*;
        struct Block<'a> {
            base: u64,
            len: u64,
            data: Cow<'a, [u8]>,
        }

        let mut memory_blocks: Vec<Block> = vec![];
        for piece in memory {
            if let Some(prev) = memory_blocks.last_mut() {
                if prev.base + prev.len == piece.base_address {
                    prev.data.to_mut().extend(piece.bytes);
                    prev.len += piece.size;
                    continue;
                }
            }
            memory_blocks.push(Block {
                base: piece.base_address,
                len: piece.size,
                data: Cow::Borrowed(piece.bytes),
            })
        }

        paths.par_extend(
            memory_blocks
                .par_iter()
                .map(|memory| search_memory(&memory.data))
                .flat_map_iter(|paths: Result<_>| paths.unwrap()),
        );
    }

    eprintln!("Scanning all PAK files..");
    paths.par_extend(
        indexs
            .into_par_iter()
            .map(|index| {
                let file = pak.lock().unwrap().read_file(index)?;
                search_memory(&file)
            })
            .flat_map_iter(|paths: Result<_>| paths.unwrap()),
    );

    paths.sort_by(|(p, _), (q, _)| p.cmp(q));
    paths.dedup_by(|(p, _), (q, _)| p == q);

    for (path, index) in paths {
        println!("{path} $ {index:?}");
    }

    Ok(())
}

fn dump_tree(pak: Vec<String>, list: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let list = File::open(list)?;
    let mut unvisited: std::collections::HashSet<_> = pak.all_file_indexs().into_iter().collect();
    for line in BufReader::new(list).lines() {
        let line = line?;
        let mut origin_path = line.split(" $ ").next().context("Empty line")?;
        if let Some(new_path) = origin_path.strip_prefix('@') {
            origin_path = new_path;
        }

        let streaming_path = "streaming/".to_owned() + origin_path;
        let paths = [origin_path, &streaming_path];

        for path in paths {
            for i18n_index in pak.find_file_i18n(path)? {
                let index = i18n_index.index;
                let path_i18n = if i18n_index.language.is_empty() {
                    path.to_owned()
                } else {
                    format!("{}.{}", path, i18n_index.language)
                };

                let mut path = PathBuf::from(&output);
                for component in path_i18n.split('/') {
                    path.push(component);
                }

                std::fs::create_dir_all(path.parent().context("no parent")?)?;
                std::fs::write(path, pak.read_file(index)?)?;
                unvisited.remove(&index);
            }
        }
    }

    for index in unvisited {
        let data = pak.read_file(index)?;
        let format = if let Some(magic) = data.get(0..4) {
            let mut format = String::new();
            for c in magic {
                if c.is_ascii_alphanumeric() {
                    format.push(*c as char);
                } else {
                    use std::fmt::Write as _;
                    write!(format, "_{c:02x}")?;
                }
            }
            format
        } else {
            "short".to_owned()
        };

        let mut path = PathBuf::from(&output);
        path.push("_unknown");
        path.push(&format);
        std::fs::create_dir_all(&path)?;
        path.push(index.short_string());
        std::fs::write(path, &data)?;
    }

    Ok(())
}

fn dump_mesh(mesh: String, output: String) -> Result<()> {
    let mesh = Mesh::new(File::open(mesh)?)?;
    mesh.dump(output)?;
    Ok(())
}

fn dump_mesh_dae(mesh: String, output: String) -> Result<()> {
    let mesh = Mesh::new(File::open(mesh)?)?;
    mesh.dump_dae(output)?;
    Ok(())
}

fn dump_rcol(rcol: String) -> Result<()> {
    let rcol = match Rcol::new(File::open(&rcol)?, true) {
        Ok(rcol) => rcol,
        Err(e) => {
            eprintln!("Deserialize RSZ failed because:\n {e}");
            Rcol::new(File::open(&rcol)?, false)?
        }
    };

    rcol.dump()?;
    Ok(())
}

fn dump_tex(tex: String, output: String, swizzle: String) -> Result<()> {
    let tex = Tex::new(File::open(tex)?)?;
    tex.save_png_swizzle(0, 0, std::fs::File::create(output)?, &swizzle)?;
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

fn gen_meat(pak: Vec<String>, index: u32, output: impl Write) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let mesh_path = format!("enemy/em{index:03}/00/mod/em{index:03}_00.mesh");
    let rcol_path = format!("enemy/em{index:03}/00/collision/em{index:03}_00_colliders.rcol");
    let mesh = pak.find_file(&mesh_path)?;
    let rcol = pak.find_file(&rcol_path)?;
    let mesh = Mesh::new(Cursor::new(pak.read_file(mesh)?))?;
    let mut rcol = Rcol::new(Cursor::new(pak.read_file(rcol)?), true)?;
    rcol.apply_skeleton(&mesh)?;
    let (vertexs, indexs) = rcol.color_monster_model(&mesh)?;

    let gpu::HitzoneDiagram { meat, .. } = gpu::gen_hitzone_diagram(vertexs, indexs)?;

    meat.save_png(output)?;

    Ok(())
}

fn gen_resources(pak: Vec<String>, output: String) -> Result<()> {
    let mut logger_root = LoggerRoot::new();
    let logger = &mut logger_root.logger();
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let sink = DiskSink::init(Path::new(&output))?;
    extract::gen_resources(&mut pak, &sink, logger)?;

    Ok(())
}

fn hash(input: String, utf16: bool) {
    if utf16 {
        println!("{:08X}", hash::hash_as_utf16(&input));
    } else {
        println!("{:08X}", hash::hash_as_utf8(&input));
    }
}

fn read_user(user: String, version_hint: Option<u32>) -> Result<()> {
    let nodes = User::new(File::open(user)?)?
        .rsz
        .deserialize(version_hint)?;
    for node in nodes {
        println!("{}", node.to_json()?);
    }
    Ok(())
}

fn dump_scn(scn: String) -> Result<()> {
    let scn = Scn::new(File::open(scn)?)?;
    scn.dump();

    Ok(())
}

fn dump_pfb(pfb: String) -> Result<()> {
    let pfb = Pfb::new(File::open(pfb)?)?;
    pfb.dump();

    Ok(())
}

fn scene_print_object(object: &GameObject, level: usize) {
    let ident_unit = 2;
    let ident = level * ident_unit;
    let padding = "";
    println!("{padding:ident$}Object {{");

    let next_level = level + 1;
    let next_ident = next_level * ident_unit;

    if let Some(prefab) = &object.prefab {
        println!("{padding:next_ident$}prefab = {prefab}");
    }

    let data = &object.object;
    println!("{padding:next_ident$}data = {data:?}");

    for component in &object.components {
        println!("{padding:next_ident$}+ {component:?}");
    }

    println!("{padding:next_ident$}Children = {{");
    for child in &object.children {
        scene_print_object(child, next_level + 1);
    }
    println!("{padding:next_ident$}}}");

    println!("{padding:ident$}}}");
}

fn scene_print_folder(folder: &Folder, level: usize) {
    let ident_unit = 2;
    let ident = level * ident_unit;
    let padding = "";
    println!("{padding:ident$}Folder {{");

    let next_level = level + 1;
    let next_ident = next_level * ident_unit;

    let data = &folder.folder;
    println!("{padding:next_ident$}data = {data:?}");

    println!("{padding:next_ident$}Subfolders = {{");
    for subfolder in &folder.subfolders {
        scene_print_folder(subfolder, next_level + 1)
    }
    println!("{padding:next_ident$}}}");

    println!("{padding:next_ident$}Children = {{");
    for child in &folder.children {
        scene_print_object(child, next_level + 1);
    }
    println!("{padding:next_ident$}}}");

    if let Some(subscene) = &folder.subscene {
        match subscene {
            Ok(subscene) => scene_print_scene(subscene, next_level),
            Err(e) => println!("{padding:next_ident$}Scene = ! {e}"),
        }
    }

    println!("{padding:ident$}}}");
}

fn scene_print_scene(scene: &Scene, level: usize) {
    let ident_unit = 2;
    let ident = level * ident_unit;
    let padding = "";
    println!("{padding:ident$}Scene {{");
    let next_level = level + 1;
    for object in &scene.objects {
        scene_print_object(object, next_level)
    }

    for folder in &scene.folders {
        scene_print_folder(folder, next_level)
    }

    println!("{padding:ident$}}}");
}

fn scene(pak: Vec<String>, name: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let scene = Scene::new(&mut pak, &name)?;
    scene_print_scene(&scene, 0);
    Ok(())
}

fn map(pak: Vec<String>, name: String, scale: String, tex: String, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let scene = Scene::new(&mut pak, &name)?;
    let scale: rsz::GuiMapScaleDefineData = User::new(File::open(scale)?)?
        .rsz
        .deserialize_single(None)?;
    let tex = Tex::new(File::open(tex)?)?;
    let mut rgba = tex.to_rgba(0, 0)?;

    scene.for_each_object(&mut |object: &GameObject, transforms| {
        if let Ok(_pop) = object.get_component::<rsz::ItemPopBehavior>() {
            let transform = transforms.last().context("no transform")?;
            let x = (transform.position.x + scale.map_wide_min_pos) / scale.map_scale;
            let y = (transform.position.z + scale.map_height_min_pos) / scale.map_scale;
            let x = (x * rgba.width() as f32) as i32;
            let y = (y * rgba.height() as f32) as i32;
            if x < 0 || y < 0 || x >= rgba.width() as i32 || y >= rgba.height() as i32 {
                return Ok(false);
            }
            let pixel = rgba.pixel(x as u32, y as u32);
            pixel.copy_from_slice(&[255, 0, 0, 255]);
        }

        Ok(false)
    })?;

    rgba.save_png(File::create(output)?)?;

    Ok(())
}

fn main() -> Result<()> {
    gpu::gpu_init();
    match Mhrice::parse() {
        Mhrice::Dump { pak, name, output } => dump(pak, name, output),
        Mhrice::DumpIndex {
            pak,
            version,
            index,
            output,
        } => dump_index(pak, version, index, output),
        Mhrice::ScanRsz { pak, crc } => scan_rsz(pak, crc),
        Mhrice::GenJson { pak, sha } => gen_json(pak, sha),
        Mhrice::GenWebsite {
            pak,
            output,
            origin,
            sha,
        } => gen_website(pak, output, origin, sha),
        Mhrice::ReadTdb { tdb, options } => read_tdb(tdb, options),
        Mhrice::ReadMsg { msg } => read_msg(msg),
        Mhrice::ScanMsg { pak, output } => scan_msg(pak, output),
        Mhrice::GrepMsg { pak, pattern } => grep_msg(pak, pattern),
        Mhrice::Grep {
            pak,
            utf16,
            pattern,
        } => grep(pak, utf16, pattern),
        Mhrice::SearchPath { pak, dmp } => search_path(pak, dmp),
        Mhrice::DumpTree { pak, list, output } => dump_tree(pak, list, output),
        Mhrice::ScanMesh { pak } => scan_mesh(pak),
        Mhrice::ScanTex { pak } => scan_tex(pak),
        Mhrice::ScanGui { pak } => scan_gui(pak),
        Mhrice::ScanUvs { pak } => scan_uvs(pak),
        Mhrice::DumpMesh { mesh, output } => dump_mesh(mesh, output),
        Mhrice::DumpMeshDae { mesh, output } => dump_mesh_dae(mesh, output),
        Mhrice::DumpRcol { rcol } => dump_rcol(rcol),
        Mhrice::DumpMeat { mesh, rcol, output } => dump_meat(mesh, rcol, output),
        Mhrice::DumpTex {
            tex,
            output,
            swizzle,
        } => dump_tex(tex, output, swizzle),
        Mhrice::DumpGui { gui } => dump_gui(gui),
        Mhrice::GenMeat { pak, index, output } => {
            gen_meat(pak, index, std::fs::File::create(output)?)
        }
        Mhrice::GenResources { pak, output } => gen_resources(pak, output),
        Mhrice::Hash { input, utf16 } => {
            hash(input, utf16);
            Ok(())
        }
        Mhrice::ReadUser { user, version } => read_user(user, version),
        Mhrice::ReadDmpTdb {
            dmp,
            address,
            options,
        } => read_dmp_tdb(dmp, address, options),
        Mhrice::DumpScn { scn } => dump_scn(scn),
        Mhrice::DumpPfb { pfb } => dump_pfb(pfb),
        Mhrice::Scene { pak, name } => scene(pak, name),
        Mhrice::TypeInfo { dmp, hash, crc } => type_info(dmp, hash, crc),
        Mhrice::Map {
            pak,
            name,
            scale,
            tex,
            output,
        } => map(pak, name, scale, tex, output),
    }
}
