#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

// @mdouglasbrett - This could error from both the lines iterator and
// the write. Using a flexible error type, but not sure if it's idiomatic.
// This will compile though.
fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();
    let request_line = lines.next().unwrap()?;
    match request_line.as_str() {
        "GET / HTTP/1.1" => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
            Ok(())
        }
        _ => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
            Ok(())
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream).unwrap()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
