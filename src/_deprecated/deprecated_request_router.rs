use std::{
    io::{Read, Write},
    sync::Arc,
};

use super::Route;
use crate::{
    errors::{ClientError, ServerError},
    handlers::deprecated_handlers::{
        handle_echo, handle_empty, handle_error, handle_get_file, handle_post_file,
        handle_user_agent,
    },
    http::{Method, Request},
    Result,
};

pub fn request_router<T: Read + Write>(mut stream: T, file_path: Arc<String>) -> Result<()> {
    match Request::try_new(&mut stream) {
        Ok(req) => {
            if let Err(e) = match (&req.method, &req.route) {
                (Method::Get, Route::Empty) => handle_empty(&mut stream),
                (Method::Get, Route::Echo) => handle_echo(&mut stream, &req),
                (Method::Get, Route::UserAgent) => handle_user_agent(&mut stream, &req),
                (Method::Get, Route::Files) => handle_get_file(&mut stream, &req, file_path),
                (Method::Post, Route::Files) => handle_post_file(&mut stream, &req, file_path),
                (Method::Get, _) => handle_error(&mut stream, ClientError::NotFound.into()),
                _ => handle_error(&mut stream, ServerError::NotImplemented.into()),
            } {
                handle_error(&mut stream, e)?;
            };
        }
        Err(e) => {
            handle_error(&mut stream, e)?;
        }
    }

    Ok(())
}
