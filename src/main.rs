use anyhow::*;
use std::fs::File;

mod pak;
mod read_ext;
mod suffix;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let pak_path = args.get(1).context("Need PAK file path")?;
    let sub_path = args.get(2).context("Need subfile path")?;
    let output_path = args.get(3).context("Need output path")?;
    let mut pak = pak::PakReader::new(File::open(pak_path)?)?;
    let (index, full_path) = pak.find_file(sub_path).context("Cannot find subfile")?;
    println!("Full path: {}", full_path);
    let content = pak.read_file(index)?;
    std::fs::write(output_path, content)?;

    Ok(())
}
