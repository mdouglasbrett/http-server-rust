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
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next();
    let path_split = request_parts
        .next()
        .unwrap()
        .split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let route = if path_split.len() == 0 {
        "/"
    } else {
        path_split[0]
    };

    match (method, route) {
        (Some("GET"), "/") => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
            Ok(())
        }
        (Some("GET"), "echo") => {
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    path_split[1].len(),
                    path_split[1]
                )
                .as_bytes(),
            )?;
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
            Ok(stream) => handle_request(stream).unwrap(),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
