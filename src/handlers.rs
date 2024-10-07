use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::request::Request;
use crate::response::Status;
use crate::utils::{get_file_contents, get_path_parts, get_response};

// TODO: make more meaningful errors!!
type HandlerError = Box<dyn std::error::Error>;

pub fn handle_empty(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all(&get_response(Status::NotFound, None))?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = get_path_parts(r.path.as_str())[1];
    s.write_all(&get_response(
        Status::Ok,
        Some((body.to_owned(), "text/plain".to_owned())),
    ))?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = r
        .headers
        .get("User-Agent")
        .unwrap_or(&String::from(""))
        .to_owned();
    s.write_all(&get_response(
        Status::Ok,
        Some((body, "text/plain".to_owned())),
    ))?;
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
        s.write_all(&get_response(
            Status::Ok,
            Some((body, "application/octet".to_owned())),
        ))?;
        Ok(())
    }
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    s.write_all(&get_response(Status::Ok, None))?;
    Ok(())
}
