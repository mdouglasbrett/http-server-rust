#[allow(unused_imports)]
use std::net::TcpListener;
use codecrafters_http_server::handle_request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: naive!!
                std::thread::spawn(move || handle_request(stream).unwrap());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
