use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{Shutdown, TcpStream},
};

use crate::utils::get_path_parts;

pub enum Route {
    Empty,
    Echo,
    UserAgent,
    Files,
    Unknown,
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "echo" => Self::Echo,
            "user-agent" => Self::UserAgent,
            "files" => Self::Files,
            "/" => Self::Empty,
            _ => Self::Unknown,
        }
    }
}

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

impl TryFrom<&TcpStream> for Request {
    type Error = String;
    fn try_from(value: &TcpStream) -> Result<Self, Self::Error> {
        let err = "Couldn't get next line";
        let mut buf = BufReader::new(value);
        let mut start_line = String::new();
        // TODO: there is an error here
        buf.read_line(&mut start_line);
        let mut start_parts = start_line.split_whitespace();
        let method = Method::from(start_parts.next());
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            _ => {
                return Err(err.to_owned());
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(path_parts[0])
        };

        let mut headers = HashMap::new();

        // Do this in a loop rather than use the iterator...

        loop {
            let mut header_line = String::new();
            buf.read_line(&mut header_line);
            let trimmed_header_line = header_line.trim();
            println!("header_line: {:?}", &header_line);
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
            let len = len.parse::<u64>().unwrap();

            buf.take(len).read_to_end(&mut body_buf);

            println!("body_buf after read: {:?}", &body_buf);
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

pub enum Status {
    Ok,
    NotFound,
    Created,
}
