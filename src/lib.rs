use std::{
    io::BufReader,
    net::TcpStream,
};

mod handlers;
mod request;
mod utils;

use handlers::{handle_echo, handle_empty, handle_unknown, handle_user_agent};
use request::{Route, Method, Request};


pub fn request_router(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(&stream);
    let req = Request::try_from(reader)?;

    match (req.method, req.route) {
        (Method::Get, Route::Empty) => handle_empty(&mut stream),
        (Method::Get, Route::Echo) => handle_echo(&mut stream, &req),
        (Method::Get, Route::UserAgent) => handle_user_agent(&mut stream, &req),
        _ => handle_unknown(&mut stream),
    }
}
