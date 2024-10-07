use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

pub fn get_file_contents(fp: Arc<Mutex<Option<String>>>, filename: &str) -> Option<String> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let path = Path::new(&fp.lock().unwrap().clone().unwrap()).join(filename);
    if let Ok(f) = fs::read_to_string(path) {
        Some(f)
    } else {
        None
    }
}
