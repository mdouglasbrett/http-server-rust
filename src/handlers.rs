use std::io::prelude::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::errors::{AppError, ClientError, ServerError};
use crate::http::{Request, Response};
use crate::utils::{get_header_value, get_path_parts, read_file, write_file};

pub fn handle_empty(s: &mut TcpStream) -> Result<(), AppError> {
    s.write_all(&Response::Ok(None).to_vec())?;
    Ok(())
}

pub fn handle_echo(s: &mut TcpStream, r: &Request) -> Result<(), AppError> {
    let encoding = get_header_value("Accept-Encoding", &r.headers);
    let body = get_path_parts(r.path.as_str())[1];
    s.write_all(
        &Response::Ok(Some((body.to_owned(), "text/plain".to_owned(), encoding))).to_vec(),
    )?;
    Ok(())
}

pub fn handle_user_agent(s: &mut TcpStream, r: &Request) -> Result<(), AppError> {
    let body = get_header_value("User-Agent", &r.headers);
    let encoding = get_header_value("Accept-Encoding", &r.headers);
    if body.is_none() {
        return Err(ClientError::BadRequest.into());
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
) -> Result<(), AppError> {
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
) -> Result<(), AppError> {
    let filename = get_path_parts(&r.path)[1];
    if !r.body.is_empty() {
        write_file(fp, filename, r)?;
    } else {
        return Err(ClientError::BadRequest.into());
    };
    s.write_all(&Response::Created.to_vec())?;
    Ok(())
}

pub fn handle_unknown(s: &mut TcpStream) -> Result<(), AppError> {
    s.write_all(&Response::NotFound.to_vec())?;
    Ok(())
}

pub fn handle_error(s: &mut TcpStream, err: AppError) -> Result<(), AppError> {
    match err {
        AppError::Server(e) => s.write_all(&Response::ServerError(e).to_vec())?,
        AppError::Client(e) => s.write_all(&Response::ClientError(e).to_vec())?,
        AppError::IO(_) => s.write_all(&Response::ServerError(ServerError::Internal).to_vec())?,
        AppError::Parse(_) => {
            s.write_all(&Response::ClientError(ClientError::BadRequest).to_vec())?
        }
    }
    Ok(())
}
