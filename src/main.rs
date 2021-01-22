use anyhow::*;
use std::fs::File;

mod pak;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let pak_path = args.get(1).context("Need PAK file path")?;
    let sub_path = args.get(2).context("Need subfile path")?;
    let mut pak = pak::PakReader::new(File::open(pak_path)?)?;
    let result = pak.find_file(sub_path);
    println!("Result: {:?}", result);

    Ok(())
}
