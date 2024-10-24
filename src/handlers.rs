use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::constants::{headers, mime_types};
use crate::errors::{
    AppError,
    ClientError::{BadRequest, NotFound},
    ServerError::Internal,
};
use crate::http::{request::Request, response::Response};
use crate::utils::{get_header_value, get_path_parts, read_file, write_file};

pub fn handle_empty<T: Write>(s: &mut T) -> Result<(), AppError> {
    s.write_all(&Response::Ok(None).to_vec())?;
    Ok(())
}

pub fn handle_echo<T: Write>(s: &mut T, r: &Request) -> Result<(), AppError> {
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    s.write_all(
        &Response::Ok(Some((&r.body, mime_types::PLAIN_TEXT.to_owned(), encoding))).to_vec(),
    )?;
    Ok(())
}

pub fn handle_user_agent<T: Write>(s: &mut T, r: &Request) -> Result<(), AppError> {
    let body = get_header_value(headers::USER_AGENT, &r.headers);
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    if let Some(b) = body {
        let response = Response::Ok(Some((
            b.as_bytes(),
            mime_types::PLAIN_TEXT.to_owned(),
            encoding,
        )));
        s.write_all(&response.to_vec())?;
    } else {
        return Err(BadRequest.into());
    }
    Ok(())
}

pub fn handle_get_file<T: Write>(
    s: &mut T,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), AppError> {
    let filename = get_path_parts(&r.path)[1];
    // TODO: this _might_ break the api
    // TODO: should we be doing the 404 off the back of this error?
    // TODO: can we do this in the router? and get rid of the empty check?
    let contents = read_file(fp, filename)?;
    let encoding = get_header_value(headers::ACCEPT_ENCODING, &r.headers);
    // TODO: is this legit now?
    if contents.is_empty() {
        handle_error(s, NotFound.into())
    } else {
        let response = Response::Ok(Some((
            &contents,
            mime_types::OCTET_STREAM.to_owned(),
            encoding,
        )));
        s.write_all(&response.to_vec())?;
        Ok(())
    }
}

pub fn handle_post_file<T: Write>(
    s: &mut T,
    r: &Request,
    fp: Arc<Mutex<Option<String>>>,
) -> Result<(), AppError> {
    let filename = get_path_parts(&r.path)[1];
    if !r.body.is_empty() {
        write_file(fp, filename, r)?;
    } else {
        return Err(BadRequest.into());
    };
    let response = Response::Created;
    s.write_all(&response.to_vec())?;
    Ok(())
}

pub fn handle_error<T: Write>(s: &mut T, err: AppError) -> Result<(), AppError> {
    match err {
        AppError::Server(e) => s.write_all(&Response::ServerError(e).to_vec())?,
        AppError::Client(e) => s.write_all(&Response::ClientError(e).to_vec())?,
        AppError::IO(_) => s.write_all(&Response::ServerError(Internal).to_vec())?,
        AppError::Parse(_) => s.write_all(&Response::ClientError(BadRequest).to_vec())?,
    }
    Ok(())
}
