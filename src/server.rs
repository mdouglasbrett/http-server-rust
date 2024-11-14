use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::router::request_router;
use crate::Result;

pub fn app_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(config.address)?;
    // TODO: this Option is temporary, update the type...
    let partial_file_path = Arc::new(Mutex::new(Some(config.directory)));

    // TODO: naive!! Should I be doing this in a pool?
    for stream in listener.incoming() {
        let path = Arc::clone(&partial_file_path);
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    // TODO: this is always going to be a string
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
