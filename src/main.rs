#[allow(unused_imports)]
use std::net::TcpListener;
use codecrafters_http_server::request_router;

pub mod handlers;
pub mod request;
pub mod utils;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: naive!!
                std::thread::spawn(move || request_router(stream).unwrap());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
