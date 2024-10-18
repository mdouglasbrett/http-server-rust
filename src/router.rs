use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::http::{Method, Request};
use crate::routes::Route;
use crate::{
    errors::HandlerError,
    handlers::{
        handle_echo, handle_empty, handle_get_file, handle_post_file, handle_server_error,
        handle_unknown, handle_user_agent,
    },
};

pub fn request_router(
    mut stream: TcpStream,
    file_path: Arc<Mutex<Option<String>>>,
) -> Result<(), HandlerError> {
    if let Ok(req) = Request::try_from(&stream) {
        // TODO - handle_get_file can also result in a 404 - I would prefer to
        // bubble that up I think
        match (&req.method, &req.route) {
            (Method::Get, Route::Empty) => handle_empty(&mut stream)?,
            (Method::Get, Route::Echo) => handle_echo(&mut stream, &req)?,
            (Method::Get, Route::UserAgent) => handle_user_agent(&mut stream, &req)?,
            (Method::Get, Route::Files) => handle_get_file(&mut stream, &req, file_path)?,
            (Method::Post, Route::Files) => handle_post_file(&mut stream, &req, file_path)?,
            // TODO: rename to handle_client_error?
            _ => handle_unknown(&mut stream)?,
        }
    } else {
        handle_server_error(&mut stream)?;
    };

    Ok(())
}
