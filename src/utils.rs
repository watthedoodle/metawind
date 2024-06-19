use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let Ok(entries) = read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return recurse(entry.path());
            }
            if meta.is_file() {
                return vec![entry.path()];
            }
            vec![]
        })
        .collect()
}

pub fn get_file_content_hash(path: impl AsRef<Path>) -> Option<String> {
    if let Ok(contents) = fs::read_to_string(path) {
        let digest = md5::compute(contents);
        return Some(format!("{:x}", digest));
    }
    None
}
