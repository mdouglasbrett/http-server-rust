use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use anyhow::anyhow;

use crate::routes::Route;
use crate::utils::get_path_parts;

pub enum Method {
    Get,
    Post,
}

impl From<Option<&str>> for Method {
    fn from(o: Option<&str>) -> Self {
        match o {
            Some("GET") => Self::Get,
            Some("POST") => Self::Post,
            // TODO: do I want to panic here?
            _ => panic!("Requires an HTTP method"),
        }
    }
}

pub struct Request {
    // TODO: avoiding lifetimes, stop doing this!!
    pub method: Method,
    pub route: Route,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// TODO: error handling
impl TryFrom<&TcpStream> for Request {
    type Error = anyhow::Error;
    fn try_from(value: &TcpStream) -> Result<Self, Self::Error> {
        let mut buf = BufReader::new(value);
        let mut start_line = String::new();
        buf.read_line(&mut start_line);
        let mut start_parts = start_line.split_whitespace();
        let method = Method::from(start_parts.next());
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            _ => {
                return Err(anyhow!("TODO: real errors"));
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(path_parts[0])
        };

        let mut headers = HashMap::new();

        loop {
            let mut header_line = String::new();
            buf.read_line(&mut header_line);
            let trimmed_header_line = header_line.trim();
            if trimmed_header_line.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key_value = trimmed_header_line
                .split_terminator(":")
                .collect::<Vec<&str>>();
            let key = key_value[0];
            let value = key_value[1].trim();
            let _ = headers.insert(key.to_owned(), value.to_owned());
        }

        let mut body_buf: Vec<u8> = vec![];

        // If there's no content length, do not attempt to parse the body
        if let Some(len) = headers.get("Content-Length") {
            if let Ok(len) = len.parse::<u64>() {
                buf.take(len).read_to_end(&mut body_buf)?;
            }
        }

        Ok(Self {
            route,
            path,
            method,
            headers,
            body: body_buf,
        })
    }
}

pub enum Response {
    Ok(Option<(String, String, Option<String>)>),
    NotFound,
    Created,
}

impl Response {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Response::Ok(Some((body, mime, encoding))) => {
                format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n{content_encoding}\r\n{}",
                mime,
                body.len(),
                body,
                content_encoding = match encoding { 
                    Some(e) => format!("Content-Encoding: {}\r\n", e),
                    None => "".to_owned()
                },
            )
            .as_bytes()
            .to_vec()
            },
            Response::Ok(None) => "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec(),
            Response::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec(),
            Response::Created => "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec(),
        }
    }
}
