use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub mod handlers;
pub mod frame;
pub mod routes;
pub mod router;
pub mod utils;

use crate::router::request_router;

fn main() {
    let mut args = env::args();
    // TODO: I will most likely just use clap here when I'm cleaning up...
    // program name
    let _ = args.next();
    // --directory flag
    let _ = args.next();
    let partial_file_path = Arc::new(Mutex::new(args.next()));
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {

        let path = Arc::clone(&partial_file_path);
        match stream {
            Ok(stream) => {
                // TODO: naive!!
                std::thread::spawn(move || {
                    request_router(stream, path).unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
