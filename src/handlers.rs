use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::Arc;

use crate::request::Request;
use crate::utils::get_path_parts;

// TODO: make this better!!
type HandlerError = Box<dyn std::error::Error>;

pub fn handle_empty(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = get_path_parts(r.path.as_str())[1];
    let content_length = body.len();
    s.write(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length, body
        )
        .as_bytes(),
    )?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = format!(
        "{}",
        r.headers.get("User-Agent").unwrap_or(&String::from(""))
    );
    let content_length = body.len();
    s.write(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length, body
        )
        .as_bytes(),
    )?;
    Ok(())
}

pub fn handle_files(s: &mut TcpStream, r: &Request, fp: Arc<Option<String>>) -> Result<(), HandlerError> {
    todo!();
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    s.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
    Ok(())
}
