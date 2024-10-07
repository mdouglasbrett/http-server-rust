use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::request::Request;
use crate::utils::{get_path_parts, get_file_contents};

// TODO: make more meaningful errors!!
type HandlerError = Box<dyn std::error::Error>;

pub fn handle_empty(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = get_path_parts(r.path.as_str())[1];
    let content_length = body.len();
    s.write_all(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length, body
        )
        .as_bytes(),
    )?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = r
        .headers
        .get("User-Agent")
        .unwrap_or(&String::from(""))
        .to_owned();
    let content_length = body.len();
    s.write_all(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length, body
        )
        .as_bytes(),
    )?;
    Ok(())
}

pub fn handle_files(
    s: &mut TcpStream,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    let filename = get_path_parts(&r.path)[1];
    let contents = get_file_contents(fp, filename);
    if let None = contents {
        handle_unknown(s)
    } else {
        let body = contents.unwrap();
        s.write_all(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                body.len(), body
                )
            .as_bytes(),
            )?;
        Ok(())
    }
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    s.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
    Ok(())
}
