#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub mod errors;
pub mod handlers;
pub mod http;
pub mod router;
pub mod routes;
pub mod utils;
pub mod constants;

use crate::errors::AppError;
use crate::router::request_router;

fn main() -> Result<(), AppError> {
    // TODO: cli front end
    let mut args = env::args();
    let _ = args.next();
    // --directory flag
    let _ = args.next();
    let partial_file_path = Arc::new(Mutex::new(args.next()));
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    // TODO: naive!! Should I be doing this in a pool?
    for stream in listener.incoming() {
        let path = Arc::clone(&partial_file_path);
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    if let Err(e) = request_router(stream, path) {
                        Err(e)
                    } else {
                        Ok(())
                    }
                });
            }
            Err(e) => {
                panic!("Panicked with: {:?}", e);
            }
        }
    }

    Ok(())
}
