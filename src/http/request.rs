use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::{
    errors::{AppError, ClientError},
    router::Route,
    Result,
};

use super::{Headers, Method};

fn get_path_parts(s: &str) -> Vec<String> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    // https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
    pub path: String,
    pub headers: HashMap<Headers, String>,
    pub body: Vec<u8>,
    pub path_parts: Vec<String>,
}

impl Request {
    pub fn get_header(&self, header: Headers) -> Option<&String> {
        let header_val = self.headers.get(&header);
        header_val
    }
}

impl<R: Read> TryFrom<&mut BufReader<R>> for Request {
    type Error = AppError;
    fn try_from(buf: &mut BufReader<R>) -> Result<Self>
    where
        R: Read,
    {
        let mut start_line = String::new();
        let _ = buf.read_line(&mut start_line)?;
        let mut start_parts = start_line.split_whitespace();
        let method = match Method::from(start_parts.next()) {
            Method::Unsupported(err) => {
                return Err(err.into());
            }
            Method::Unknown(err) => {
                return Err(err.into());
            }
            Method::Get => Method::Get,
            Method::Post => Method::Post,
        };
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            None => {
                return Err(ClientError::BadRequest.into());
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(&path_parts[0])
        };

        let mut headers = HashMap::new();

        loop {
            let mut header_line = String::new();
            let _ = buf.read_line(&mut header_line)?;
            let trimmed_header_line = header_line.trim();
            if trimmed_header_line.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key_value = trimmed_header_line
                .split_terminator(":")
                .collect::<Vec<&str>>();
            let key = Headers::from(key_value[0]);
            let raw_value = key_value[1].trim();
            let concat_parts = raw_value.replace(", ", ",");
            headers
                .entry(key)
                // in-place mutation
                .and_modify(|val| {
                    *val = format!("{},{}", val, concat_parts);
                })
                .or_insert(concat_parts.to_owned());
        }

        let mut body_buf: Vec<u8> = vec![];

        if route == Route::Echo && path_parts.len() > 1 {
            body_buf.extend(path_parts[1].as_bytes());
        } else {
            // If there's no content length, do not attempt to parse the body
            if let Some(len) = headers.get(&Headers::ContentLength) {
                let len = len.parse::<u64>()?;
                buf.take(len).read_to_end(&mut body_buf)?;
            }
        }

        Ok(Self {
            route,
            path,
            method,
            headers,
            body: body_buf,
            path_parts,
        })
    }
}

#[cfg(test)]
mod tests {

    mod request {
        use crate::errors::{AppError, ClientError};
        use crate::http::request::{Method::Get, Request};
        use crate::router::Route::Echo;
        use std::{collections::HashMap, io::BufReader};

        #[test]
        fn handles_http_request() {
            let req = b"GET /echo/abc HTTP/1.1\r\n\r\n";
            let mut req_slice = req.as_slice();
            let mut req_buf = BufReader::new(&mut req_slice);
            let expected = Request {
                method: Get,
                route: Echo,
                path: "/echo/abc".to_owned(),
                path_parts: vec!["echo".to_owned(), "abc".to_owned()],
                body: b"abc".to_vec(),
                headers: HashMap::new(),
            };
            assert_eq!(expected, Request::try_from(&mut req_buf).unwrap());
        }

        #[test]
        fn handles_bad_request() {
            let req = b"/echo/abc\r\n\r\n";
            let mut req_slice = req.as_slice();
            let mut req_buf = BufReader::new(&mut req_slice);
            assert_eq!(
                AppError::Client(ClientError::BadRequest),
                Request::try_from(&mut req_buf).unwrap_err()
            );
        }
    }
}
