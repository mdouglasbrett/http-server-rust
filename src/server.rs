use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use log::{error, info};

use crate::config::Config;
use crate::router::request_router;
use crate::thread_pool::ThreadPool;
use crate::Result;

pub fn app_server(config: Config) -> Result<()> {
    let listener = TcpListener::bind(&config.address)?;
    listener.set_nonblocking(true)?;
    info!("Server listening on: {}", config.address);
    let partial_file_path = Arc::new(config.directory);
    let pool = ThreadPool::new(8);

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        info!("Starting shutdown...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("TODO: handle ctrlc error");

    while running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, addr)) => {
                info!("Connection from: {}", addr);
                let path = Arc::clone(&partial_file_path);
                pool.execute(move || {
                    // TODO: error handling here, should I just eprintln! and continue?
                    if let Err(e) = request_router(stream, path) {
                        error!("Error handling request, {}", e);
                    } else {
                        info!("Request handled OK");
                    }
                });
            }
            // The listener is non-blocking, so if there are no connections waiting we can just sleep for a bit and try again
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            // If there is an error accepting a connection, we'll just print it and continue
            Err(e) => {
                error!("Connection error: {:?}", e);
                continue;
            }
        }
    }

    info!("Shutting down server...");
    drop(pool);
    info!("Server shutdown complete.");
    Ok(())
}
