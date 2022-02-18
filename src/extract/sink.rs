use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub trait Sink: Sync {
    type File: Write;
    fn create(&self, name: &str) -> Result<Self::File>;
    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized;

    fn create_html(&self, name: &str) -> Result<Self::File> {
        let mut file = self.create(name)?;
        file.write_all(b"<!DOCTYPE html>\n")?;
        Ok(file)
    }
}

pub struct DiskSink {
    root: PathBuf,
}

impl DiskSink {
    pub fn init(root: &Path) -> Result<Self> {
        let root = PathBuf::from(root);
        if root.exists() {
            fs::remove_dir_all(&root)?;
        }
        fs::create_dir(&root)?;
        Ok(DiskSink { root })
    }
}

impl Sink for DiskSink {
    type File = fs::File;

    fn create(&self, name: &str) -> Result<Self::File> {
        let path = self.root.join(name);
        let file = fs::File::create(path)?;
        Ok(file)
    }

    fn sub_sink(&self, name: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let path = self.root.join(name);
        fs::create_dir(&path)?;
        Ok(DiskSink { root: path })
    }
}
