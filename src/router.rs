use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::errors::{AppError, ClientError};
use crate::handlers::{
    handle_echo, handle_empty, handle_error, handle_get_file, handle_post_file, handle_user_agent,
};
use crate::http::{Method, Request};
use crate::routes::Route;

pub fn request_router(
    mut stream: TcpStream,
    file_path: Arc<Mutex<Option<String>>>,
) -> Result<(), AppError> {
    match Request::try_from(&stream) {
        Ok(req) => {
            if let Err(e) = match (&req.method, &req.route) {
                (Method::Get, Route::Empty) => handle_empty(&mut stream),
                (Method::Get, Route::Echo) => handle_echo(&mut stream, &req),
                (Method::Get, Route::UserAgent) => handle_user_agent(&mut stream, &req),
                (Method::Get, Route::Files) => handle_get_file(&mut stream, &req, file_path),
                (Method::Post, Route::Files) => handle_post_file(&mut stream, &req, file_path),
                _ => handle_error(&mut stream, ClientError::NotFound.into()),
            } {
                let _ = handle_error(&mut stream, e);
            };
        }
        Err(e) => {
            handle_error(&mut stream, e)?;
        }
    }

    Ok(())
}
