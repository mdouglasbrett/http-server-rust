use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
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
    pub body: String,
}

impl TryFrom<BufReader<&TcpStream>> for Request {
    type Error = String;
    fn try_from(value: BufReader<&TcpStream>) -> Result<Self, Self::Error> {
        let err = "Couldn't get next line";
        let mut lines = value.lines();
        let start_line = match lines.next() {
            Some(Ok(s)) => s,
            _ => {
                return Err(err.to_owned());
            }
        };
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

        while let Some(Ok(header_line)) = lines.next() {
            let key_value = header_line.split_terminator(":").collect::<Vec<&str>>();
            if key_value.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key = key_value[0];
            let value = key_value[1].trim();
            let _ = headers.insert(key.to_owned(), value.to_owned());
        }

        let mut body_buf = String::new();

        if let Some(body_line) = lines.next() {
            if body_line.is_ok() {
                let len = headers.get("Content-Length").unwrap().parse::<u64>().unwrap();
                body_line.unwrap().as_bytes().take(len).read_to_string(&mut body_buf);
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
