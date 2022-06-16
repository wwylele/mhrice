#![recursion_limit = "4096"]

use anyhow::{anyhow, bail, Context, Result};
use minidump::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom, Write};
use std::path::*;
use std::sync::Mutex;
use structopt::*;

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

use extract::sink::*;
use file_ext::*;
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

const TDB_ANCHOR: &[u8] = b"TDB\0\x46\0\0\0";

pub static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut config = HashMap::new();
    if let Ok(file) = File::open("mhrice.config") {
        eprintln!("Reading global config file...");
        for line in BufReader::new(file).lines() {
            if let Ok(line) = line {
                let split = if let Some(split) = line.find('=') {
                    split
                } else {
                    continue;
                };
                let key = line[0..split].trim().to_string();
                let value = line[split + 1..].trim().to_string();
                eprintln!("Got {}", key);
                config.insert(key, value);
            } else {
                break;
            }
        }
        eprintln!("Finished reading global config file");
    }
    config
});

pub fn get_config(key: &str) -> Option<String> {
    if let Ok(value) = std::env::var(key) {
        return Some(value);
    }
    CONFIG.get(key).map(|s| s.to_string())
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
        index: usize,
        #[structopt(short, long)]
        output: String,
    },

    ScanRsz {
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

        #[structopt(short, long)]
        utf16: bool,

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

    ReadUser {
        #[structopt(short, long)]
        user: String,
    },

    ReadDmpTdb {
        #[structopt(short, long)]
        dmp: String,
        #[structopt(short, long)]
        map: Option<String>,
        #[structopt(short, long)]
        address: Option<String>,
    },

    DumpScn {
        #[structopt(short, long)]
        scn: String,
    },

    Scene {
        #[structopt(short, long)]
        pak: Vec<String>,
        #[structopt(short, long)]
        name: String,
    },

    TypeInfo {
        #[structopt(short, long)]
        dmp: String,

        #[structopt(short, long)]
        hash: String,

        #[structopt(short, long)]
        crc: String,
    },

    Map {
        #[structopt(short, long)]
        pak: Vec<String>,

        #[structopt(short, long)]
        name: String,

        #[structopt(short, long)]
        scale: String,

        #[structopt(short, long)]
        tex: String,

        #[structopt(short, long)]
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

fn scan_rsz(pak: Vec<String>) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let mut crc_mismatches = BTreeMap::new();

    for index in pak.all_file_indexs() {
        let content = pak
            .read_file(index)
            .context(format!("Failed to open file at {:?}", index))?;
        if content.len() < 4 {
            continue;
        }

        if &content[0..3] == b"USR" {
            User::new(Cursor::new(&content))
                .context(format!("Failed to open USER at {:?}", index))?
                .rsz
                .verify_crc(&mut crc_mismatches);
        } else if &content[0..3] == b"PFB" {
            Pfb::new(Cursor::new(&content))
                .context(format!("Failed to open PFB at {:?}", index))?
                .rsz
                .verify_crc(&mut crc_mismatches);
        } else if &content[0..3] == b"SCN" {
            Scn::new(Cursor::new(&content))
                .context(format!("Failed to open SCN at {:?}", index))?
                .rsz
                .verify_crc(&mut crc_mismatches);
        } else if &content[0..4] == b"RCOL" {
            Rcol::new(Cursor::new(&content), false)
                .context(format!("Failed to open RCOL at {:?}", index))?
                .rsz
                .verify_crc(&mut crc_mismatches);
        }
    }

    for (symbol, crc) in crc_mismatches {
        println!("Mismatch CRC {crc:08X} for {symbol}")
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

fn gen_website_to_sink(pak: Vec<String>, sink: impl Sink) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;
    let pedia = extract::gen_pedia(&mut pak)?;
    let pedia_ex = extract::gen_pedia_ex(&pedia)?;
    sink.create("mhrice.json")?
        .write_all(serde_json::to_string_pretty(&pedia)?.as_bytes())?;
    extract::gen_website(&pedia, &pedia_ex, &sink)?;
    extract::gen_resources(&mut pak, &sink.sub_sink("resources")?)?;
    sink.finalize()?;
    Ok(())
}

fn gen_website(pak: Vec<String>, output: String) -> Result<()> {
    if let Some(bucket) = output.strip_prefix("S3://") {
        let sink = S3Sink::init(bucket.to_string())?;
        gen_website_to_sink(pak, sink)?;
    } else if output == "null://" {
        let sink = NullSink;
        gen_website_to_sink(pak, sink)?;
    } else {
        let sink = DiskSink::init(Path::new(&output))?;
        gen_website_to_sink(pak, sink)?;
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
        of.seek(SeekFrom::Start(0))?;
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

fn read_tdb(tdb: String) -> Result<()> {
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

    let _ = Tdb::new(OffsetFile::new(file, offset)?, 0, None)?;
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

fn read_dmp_tdb(dmp: String, map: Option<String>, address: Option<String>) -> Result<()> {
    let dmp = Minidump::read_path(dmp).map_err(|e| anyhow!(e))?;
    let memory = dmp
        .get_stream::<MinidumpMemory64List>()
        .map_err(|e| anyhow!(e))?;

    if let Some(address) = address {
        let base = if let Some(hex) = address.strip_prefix("0x") {
            u64::from_str_radix(hex, 16)?
        } else {
            address.parse()?
        };
        let file = OffsetFile::new(MinidumpReader::new(&memory), base)?;

        let _ = Tdb::new(file, base, map)?;

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
            let file = OffsetFile::new(MinidumpReader::new(&memory), base)?;

            let _ = Tdb::new(file, base, map)?;

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
        let msg = Msg::new(Cursor::new(&file)).context(format!("at {:?}", i))?;
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
                        if !file[earlier].is_ascii_graphic() && file[earlier] != b' ' {
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
        let path = line.split(" $ ").next().context("Empty line")?;

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
    tex.save_png(0, 0, std::fs::File::create(&output)?)?;
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

    meat.save_png(output)?;

    Ok(())
}

fn gen_resources(pak: Vec<String>, output: String) -> Result<()> {
    let mut pak = PakReader::new(open_pak_files(pak)?)?;

    let sink = DiskSink::init(Path::new(&output))?;
    extract::gen_resources(&mut pak, &sink)?;

    Ok(())
}

fn hash(input: String, utf16: bool) {
    if utf16 {
        println!("{:08X}", hash::hash_as_utf16(&input));
    } else {
        println!("{:08X}", hash::hash_as_utf8(&input));
    }
}

fn read_user(user: String) -> Result<()> {
    let nodes = User::new(File::open(user)?)?.rsz.deserialize()?;
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
    let scale: rsz::GuiMapScaleDefineData =
        User::new(File::open(scale)?)?.rsz.deserialize_single()?;
    let tex = Tex::new(File::open(tex)?)?;
    let mut rgba = tex.to_rgba(0, 0)?;

    scene.for_each_free_object(&mut |object: &GameObject| {
        if let Ok(_pop) = object.get_component::<rsz::ItemPopBehavior>() {
            let transform = object.get_component::<rsz::Transform>()?;
            let x = (transform.position.x + scale.map_wide_min_pos) / scale.map_scale;
            let y = (transform.position.z + scale.map_height_min_pos) / scale.map_scale;
            let x = (x * rgba.width() as f32) as i32;
            let y = (y * rgba.height() as f32) as i32;
            if x < 0 || y < 0 || x >= rgba.width() as i32 || y >= rgba.height() as i32 {
                return Ok(());
            }
            let pixel = rgba.pixel(x as u32, y as u32);
            pixel.copy_from_slice(&[255, 0, 0, 255]);
        }

        Ok(())
    })?;

    rgba.save_png(File::create(output)?)?;

    Ok(())
}

fn main() -> Result<()> {
    gpu::gpu_init();
    match Mhrice::from_args() {
        Mhrice::Dump { pak, name, output } => dump(pak, name, output),
        Mhrice::DumpIndex {
            pak,
            version,
            index,
            output,
        } => dump_index(pak, version, index, output),
        Mhrice::ScanRsz { pak } => scan_rsz(pak),
        Mhrice::GenJson { pak } => gen_json(pak),
        Mhrice::GenWebsite { pak, output } => gen_website(pak, output),
        Mhrice::ReadTdb { tdb } => read_tdb(tdb),
        Mhrice::ReadMsg { msg } => read_msg(msg),
        Mhrice::ScanMsg { pak, output } => scan_msg(pak, output),
        Mhrice::GrepMsg { pak, pattern } => grep_msg(pak, pattern),
        Mhrice::Grep {
            pak,
            utf16,
            pattern,
        } => grep(pak, utf16, pattern),
        Mhrice::SearchPath { pak } => search_path(pak),
        Mhrice::DumpTree { pak, list, output } => dump_tree(pak, list, output),
        Mhrice::ScanMesh { pak } => scan_mesh(pak),
        Mhrice::ScanTex { pak } => scan_tex(pak),
        Mhrice::ScanGui { pak } => scan_gui(pak),
        Mhrice::ScanUvs { pak } => scan_uvs(pak),
        Mhrice::DumpMesh { mesh, output } => dump_mesh(mesh, output),
        Mhrice::DumpRcol { rcol } => dump_rcol(rcol),
        Mhrice::DumpMeat { mesh, rcol, output } => dump_meat(mesh, rcol, output),
        Mhrice::DumpTex { tex, output } => dump_tex(tex, output),
        Mhrice::DumpGui { gui } => dump_gui(gui),
        Mhrice::GenMeat { pak, index, output } => {
            gen_meat(pak, index, std::fs::File::create(output)?)
        }
        Mhrice::GenResources { pak, output } => gen_resources(pak, output),
        Mhrice::Hash { input, utf16 } => {
            hash(input, utf16);
            Ok(())
        }
        Mhrice::ReadUser { user } => read_user(user),
        Mhrice::ReadDmpTdb { dmp, map, address } => read_dmp_tdb(dmp, map, address),
        Mhrice::DumpScn { scn } => dump_scn(scn),
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
