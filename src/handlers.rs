use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::errors::{HandlerError, RequestError};
use crate::http::{Request, Response};
use crate::utils::{get_header_value, get_path_parts, read_file, write_file};

pub fn handle_empty(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all(&Response::Ok(None).to_vec())?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let encoding = get_header_value("Accept-Encoding", &r.headers);
    let body = get_path_parts(r.path.as_str())[1];
    s.write_all(
        &Response::Ok(Some((body.to_owned(), "text/plain".to_owned(), encoding))).to_vec(),
    )?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), HandlerError> {
    let body = get_header_value("User-Agent", &r.headers);
    let encoding = get_header_value("Accept-Encoding", &r.headers);
    if body.is_none() {
        // TODO: do I just want to do this in the get_header_value func and use ?
        return Err(RequestError::HeaderError("No User-Agent in headers".to_owned()).into());
    } else {
        s.write_all(
            &Response::Ok(Some((body.unwrap(), "text/plain".to_owned(), encoding))).to_vec(),
        )?;
    }
    Ok(())
}

pub fn handle_get_file(
    s: &mut TcpStream,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    let filename = get_path_parts(&r.path)[1];
    // TODO: this _might_ break the api
    // TODO: should we be doing the 404 off the back of this error?
    // TODO: can we do this in the router? and get rid of the empty check?
    let contents = read_file(fp, filename)?;
    let encoding = get_header_value("Accept-Encoding", &r.headers);
    // TODO: is this legit now?
    if contents.is_empty() {
        handle_unknown(s)
    } else {
        s.write_all(
            &Response::Ok(Some((
                contents,
                "application/octet-stream".to_owned(),
                encoding,
            )))
            .to_vec(),
        )?;
        Ok(())
    }
}

pub fn handle_post_file(
    s: &mut TcpStream,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    let filename = get_path_parts(&r.path)[1];
    if !r.body.is_empty() {
        write_file(fp, filename, r)?;
    } else {
        // TODO: should this be handled in the Request parsing?
        // Maybe return a 500 there?
        return Err(RequestError::BodyError("Empty body".to_owned()).into());
    };
    s.write_all(&Response::Created.to_vec())?;
    Ok(())
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all(&Response::NotFound.to_vec())?;
    Ok(())
}

pub fn handle_server_error(s: &mut TcpStream) -> Result<(), HandlerError> {
    s.write_all(&Response::ServerError.to_vec())?;
    Ok(())
}
