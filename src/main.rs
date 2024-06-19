use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

mod memory;
mod utils;
use crate::memory::FileMeta;

fn main() {
    let files = utils::recurse("./");

    for f in files {
        // TODO: use arg options to define file extension filters
        if f.as_path().extension() == Some(OsStr::new("txt")) {
            let hash = utils::get_file_content_hash(f.as_path());

            memory::in_mem_vec().lock().unwrap().push(FileMeta {
                path: f.as_path().display().to_string(),
                hash: hash.as_ref().unwrap().to_string(),
            });
            // TODO actually run the file patch
            println!(
                "{:?} -> {:?} {:?}",
                f.as_path(),
                f.as_path().extension(),
                hash
            );
        }
    }

    // TODO: we'll need to use args to flag if we want to go into file watch mode e.g --dev
    if let Err(error) = watch("") {
        println!("ERROR: {}", error);
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                // TODO: we only care about file writes 
                // so need to filter for file writes and correct file extention
                // then check against the in memory content hash to make sure to
                // only act on the file if it's actually changed
                println!("File Chang Detected: {:?}", event);
            }
            Err(error) => {
                println!("Error:{}", error);
            }
        }
    }
    Ok(())
}
