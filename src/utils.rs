use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::constants::headers::ACCEPT_ENCODING;
use crate::errors::{ClientError, ServerError};
use crate::http::request::{HeaderField, Request};
use crate::Result;

const ALLOWED_ENCODING: &str = "gzip";

pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}

// TODO: this should change really - and could possibly be reduced just to the
// encoding filtering. Instead of matching on string slices, it should probably
// be done via enum
pub fn get_header_value(val: &str, headers: &HashMap<String, HeaderField>) -> Option<String> {
    let header_val = headers.get(val);
    match val {
        ACCEPT_ENCODING => match header_val {
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

// TODO: these functions are going to be interesting to test...
// TODO: pass readers and writers to these?
pub fn read_file(fp: Arc<Mutex<Option<String>>>, filename: &str) -> Result<Vec<u8>> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let partial_path = &fp.lock().unwrap().clone().unwrap();
    let path = Path::new(partial_path).join(filename);
    if let Ok(val) = Path::try_exists(&path) {
        if val {
            let file_contents = fs::read(path)?;
            Ok(file_contents)
        } else {
            Err(ClientError::NotFound.into())
        }
    } else {
        Err(ServerError::Internal.into())
    }
}

pub fn write_file(fp: Arc<Mutex<Option<String>>>, filename: &str, req: &Request) -> Result<()> {
    // TODO: I really don't like this unwrap/clone/unwrap dance
    let path_inner = fp.lock().unwrap().clone().unwrap();
    let path = Path::new(&path_inner);
    let file_path = path.join(filename);
    // TODO: get rid of this unwrap
    if Path::try_exists(&file_path).unwrap() {
        fs::write(&file_path, &req.body)?;
        Ok(())
    } else {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(&req.body)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    mod get_path_parts {
        use crate::utils::get_path_parts;

        #[test]
        fn returns_correct_path_parts() {
            let path = "/test/path/string";
            let expected = vec!["test", "path", "string"];
            assert_eq!(expected, get_path_parts(path));
        }

        #[test]
        fn returns_empty_vec() {
            let path = "/";
            let expected: Vec<&str> = Vec::new();
            assert_eq!(expected, get_path_parts(path));
        }
    }

    // TODO: these tests may come out/change...
    mod get_header_value {
        use std::collections::HashMap;

        use crate::{
            constants::headers as header_fields, http::request::HeaderField,
            utils::get_header_value,
        };

        #[test]
        fn returns_correct_values() {
            let mut headers = HashMap::new();
            headers.insert(
                header_fields::CONTENT_LENGTH.to_owned(),
                HeaderField::Single("32".to_owned()),
            );
            let expected = Some(String::from("32"));

            assert_eq!(
                expected,
                get_header_value(header_fields::CONTENT_LENGTH, &headers)
            );
            assert_eq!(
                None,
                get_header_value(header_fields::ACCEPT_ENCODING, &headers)
            );
        }

        #[test]
        fn handles_enocoding_filtering() {
            let mut headers = HashMap::new();
            headers.insert(
                header_fields::ACCEPT_ENCODING.to_owned(),
                HeaderField::Multiple(vec![
                    "gzip".to_owned(),
                    "brotli".to_owned(),
                    "quux".to_owned(),
                ]),
            );
            let expected = Some(String::from("gzip"));

            assert_eq!(
                expected,
                get_header_value(header_fields::ACCEPT_ENCODING, &headers)
            );

            let mut headers = HashMap::new();
            headers.insert(
                header_fields::ACCEPT_ENCODING.to_owned(),
                HeaderField::Multiple(vec![
                    "blah".to_owned(),
                    "brotli".to_owned(),
                    "quux".to_owned(),
                ]),
            );

            assert_eq!(
                None,
                get_header_value(header_fields::ACCEPT_ENCODING, &headers)
            );
        }
    }
}
