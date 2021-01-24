use crate::file_ext::FileExt;
use crate::rsz::Rsz;
use crate::user::UserChild;
use anyhow::*;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct Scn {
    pub resource_a_names: Vec<String>,
    pub resource_b_names: Vec<String>,
    pub children: Vec<UserChild>,
    pub rsz: Rsz,
}

impl Scn {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Scn> {
        let magic = file.read_magic()?;
        if &magic != b"SCN\0" {
            bail!("Wrong magic for SCN file");
        }
        let a_count = file.read_u32()?;
        let resource_a_count = file.read_u32()?;
        let c_count = file.read_u32()?;
        let resource_b_count = file.read_u32()?;
        let child_count = file.read_u32()?;

        let what_offset = file.read_u64()?;
        let resource_a_list_offset = file.read_u64()?;
        let resource_b_list_offset = file.read_u64()?;
        let child_list_offset = file.read_u64()?;
        let rsz_offset = file.read_u64()?;
        // TODO: more data here

        file.seek(SeekFrom::Start(resource_a_list_offset))?; // TODO: Remove this
        file.seek_align_up(resource_a_list_offset, 16)
            .context("Undisconvered data before resource A list")?;
        let resource_a_name_offsets = (0..resource_a_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_align_up(resource_b_list_offset, 16)
            .context("Undisconvered data before resource B list")?;
        let resource_b_name_offsets = (0..resource_b_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_align_up(child_list_offset, 16)
            .context("Undisconvered data before child list")?;
        let child_info = (0..child_count)
            .map(|_| {
                let hash = file.read_u32()?;
                let padding = file.read_u32()?;
                if padding != 0 {
                    bail!("ChildInfo non-zero padding {}", padding);
                }
                let name_offset = file.read_u64()?;
                Ok((hash, name_offset))
            })
            .collect::<Result<Vec<_>>>()?;

        let resource_a_names = resource_a_name_offsets
            .into_iter()
            .map(|resource_name_offset| {
                file.seek_noop(resource_name_offset)
                    .context("Undiscovered data in resource A names")?;
                let name = file.read_u16str()?;
                if name.ends_with(".user") {
                    bail!("USER resource");
                }
                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;

        let resource_b_names = resource_b_name_offsets
            .into_iter()
            .map(|resource_name_offset| {
                file.seek_noop(resource_name_offset)
                    .context("Undiscovered data in resource B names")?;
                let name = file.read_u16str()?;
                if name.ends_with(".user") {
                    bail!("USER resource");
                }
                Ok(name)
            })
            .collect::<Result<Vec<_>>>()?;

        let children = child_info
            .into_iter()
            .map(|(hash, name_offset)| {
                file.seek_noop(name_offset)
                    .context("Undiscovered data in child info")?;
                let name = file.read_u16str()?;
                if !name.ends_with(".user") {
                    bail!("Non-USER child");
                }
                Ok(UserChild { hash, name })
            })
            .collect::<Result<Vec<_>>>()?;

        let rsz = Rsz::new(file, rsz_offset)?;

        Ok(Scn {
            resource_a_names,
            resource_b_names,
            children,
            rsz,
        })
    }
}
