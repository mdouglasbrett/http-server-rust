use std::net::TcpListener;
use std::sync::Arc;

use crate::config::Config;
use crate::router::request_router;
use crate::thread_pool::ThreadPool;
use crate::Result;

pub fn app_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(&config.address)?;
    println!("Server listening on: {}", config.address);
    let partial_file_path = Arc::new(config.directory);
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let path = Arc::clone(&partial_file_path);
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    // TODO: error handling here, should I just eprintln! and continue?
                    if let Err(_e) = request_router(stream, path) {
                        // Err(e)
                    } else {
                        // Ok(())
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
