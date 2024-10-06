#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();
    let start_line = lines.next().unwrap()?;
    let mut start_parts = start_line.split_whitespace();
    let method = start_parts.next();
    let path = start_parts
        .next()
        // TODO: handle this...
        .unwrap()
        .split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let route = if path.is_empty() { "/" } else { path[0] };

    let mut headers_map = HashMap::new();

    while let Some(Ok(header_line)) = lines.next() {
        let key_value = header_line.split_terminator(":").collect::<Vec<&str>>();
        if key_value.is_empty() {
            // I think we have reached the body at this point
            break;
        }
        let key = key_value[0];
        let value = key_value[1].trim();
        let _ = &headers_map.insert(key.to_owned(), value.to_owned());
    }

    match (method, route) {
        (Some("GET"), "/") => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
            Ok(())
        }
        // TODO: extract this...
        (Some("GET"), "echo") => {
            let body = path[1];
            let content_length = body.len();
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content_length,
                    body
                )
                .as_bytes(),
            )?;
            Ok(())
        }
        (Some("GET"), "user-agent") => {
            let body = format!(
                "{}",
                headers_map.get("User-Agent").unwrap_or(&String::from(""))
            );
            let content_length = body.len();
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content_length, body
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
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TODO: naive!!
                std::thread::spawn(move || handle_request(stream).unwrap());
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
