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
            }
            _ => None,
        },
        _ => {
            if let Some(HeaderField::Single(val)) = header_val {
                Some(val.to_owned())
            } else {
                None
            }
        }
    }
}

// TODO: custom errors
pub fn read_file(
    fp: Arc<Mutex<Option<String>>>,
    filename: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let partial_path = &fp.lock().unwrap().clone().unwrap();
    let path = Path::new(partial_path).join(filename);
    let file_contents = fs::read_to_string(path)?;
    Ok(file_contents)
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
