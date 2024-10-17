use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::http::{HeaderField, Request};

const ALLOWED_ENCODING: &str = "gzip";

pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

// TODO: I am going to end up having a headers enum, right?
pub fn get_header_value(val: &str, headers: &HashMap<String, HeaderField>) -> Option<String> {
    let header_val = headers.get(val);
    match val {
        "Content-Length" | "User-Agent" => {
            if let Some(HeaderField::Single(val)) = header_val {
                Some(val.to_owned())
            } else {
                None
            }
        }
        "Accept-Encoding" => match header_val {
            Some(HeaderField::Multiple(v)) => {
                    let filtered_encodings = v
                        .iter()
                        .filter(|e| e.as_str() == ALLOWED_ENCODING)
                        .collect::<Vec<&String>>();
                    if filtered_encodings.is_empty() {
                        None
                    } else {
                        Some(filtered_encodings[0].to_owned())
                    }
            },
            _ => None,
        },
        _ => None,
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
