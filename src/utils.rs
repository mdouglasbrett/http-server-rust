use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::http::Request;

pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

pub fn get_encoding(headers: &HashMap<String, String>) -> Option<String> {
    match headers.get("Accept-Encoding") {
        Some(e) => {
            if e.as_str() == "gzip" {
                Some(e.to_owned())
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn read_file(fp: Arc<Mutex<Option<String>>>, filename: &str) -> Option<String> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let path = Path::new(&fp.lock().unwrap().clone().unwrap()).join(filename);
    if let Ok(f) = fs::read_to_string(path) {
        Some(f)
    } else {
        None
    }
}

pub fn write_file(
    fp: Arc<Mutex<Option<String>>>,
    filename: &str,
    req: &Request,
) -> Result<(), anyhow::Error> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let path_inner = fp.lock().unwrap().clone().unwrap();
    let path = Path::new(&path_inner);
    let file_path = path.join(filename);
    if Path::try_exists(&file_path).unwrap() {
        fs::write(&file_path, &req.body)?;
        Ok(())
    } else {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(&req.body)?;
        Ok(())
    }
}
