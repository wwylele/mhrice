use std::{collections::HashMap, hash::Hash};

// a temporary store of some website files
// we attach them in html href to clear browser cache

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum FileTag {
    MainJs,
    MainCss,
    Fa,
    FaBrand,
    FaSolid,
    PartColor,
    Masonry,
}

pub struct HashStore {
    store: HashMap<FileTag, String>,
}

impl HashStore {
    pub fn new() -> HashStore {
        HashStore {
            store: HashMap::new(),
        }
    }

    pub fn add(&mut self, file_tag: FileTag, hash: String) {
        if self.store.insert(file_tag, hash).is_some() {
            panic!("Double hash for {file_tag:?}")
        }
    }

    pub fn get(&self, file_tag: FileTag) -> &str {
        &self.store[&file_tag]
    }
}
