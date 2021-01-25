use crate::file_ext::*;
use crate::rsz::Rsz;
use crate::user::UserChild;
use anyhow::*;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Pfb {
    pub resource_names: Vec<String>,
    pub children: Vec<UserChild>,
    pub rsz: Rsz,
}

impl Pfb {
    pub fn new<F: Read + Seek>(mut file: F) -> Result<Pfb> {
        let magic = file.read_magic()?;
        if &magic != b"PFB\0" {
            bail!("Wrong magic for PFB file");
        }
        let zinogre_count = file.read_u32()?;
        let resource_count = file.read_u32()?;
        let rathalos_count = file.read_u32()?;
        let child_count = file.read_u32()?;
        let padding = file.read_u32()?;
        if padding != 0 {
            bail!("Unexpected non-zero padding A: {}", padding);
        }
        let rathalos_offset = file.read_u64()?;
        let resource_list_offset = file.read_u64()?;
        let child_list_offset = file.read_u64()?;
        let rsz_offset = file.read_u64()?;

        let _ = (0..zinogre_count)
            .map(|_| {
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;
        file.seek_noop(rathalos_offset)
            .context("Undisconvered data before rathalos list")?;

        let _ = (0..rathalos_count)
            .map(|_| {
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                file.read_u32()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(resource_list_offset, 16)
            .context("Undisconvered data before resource list")?;
        let resource_name_offsets = (0..resource_count)
            .map(|_| file.read_u64())
            .collect::<Result<Vec<_>>>()?;

        file.seek_assert_align_up(child_list_offset, 16)
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

        let resource_names = resource_name_offsets
            .into_iter()
            .map(|resource_name_offset| {
                file.seek_noop(resource_name_offset)
                    .context("Undiscovered data in resource names")?;
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

        Ok(Pfb {
            resource_names,
            children,
            rsz,
        })
    }
}
