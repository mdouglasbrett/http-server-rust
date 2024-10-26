use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::router::request_router;
use crate::Result;

pub fn app_server(filepath: Option<String>, listener: TcpListener) -> Result<()> {
    let partial_file_path = Arc::new(Mutex::new(filepath));

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
