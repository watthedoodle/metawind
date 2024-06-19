use std::sync::{Mutex, OnceLock};

pub struct FileMeta {
    pub path: String,
    pub hash: String,
}

// TODO: maybe switch to using a HashMap, because we want the filename to be the unique key?
pub fn in_mem_vec() -> &'static Mutex<Vec<FileMeta>> {
    static ARR: OnceLock<Mutex<Vec<FileMeta>>> = OnceLock::new();
    ARR.get_or_init(|| Mutex::new(vec![]))
}
