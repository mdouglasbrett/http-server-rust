use super::ThreadPool;
use crate::dir::Dir;
use crate::router::Router;
use crate::{Config, Result};
use std::net::TcpListener;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use log::{error, info};

pub struct Server {
    listener: TcpListener,
    router: Arc<Router<Dir>>,
    thread_pool: ThreadPool,
    running: Arc<AtomicBool>,
}

impl Server {
    pub fn try_new(config: &Config) -> Result<Server> {
        let listener = TcpListener::bind(&config.address)?;
        listener.set_nonblocking(true)?;
        // I feel like trying to get rid of this clone would be overkill...
        // Clippy isn't annoyed with me about this
        let router: Arc<Router<Dir>> = Arc::new(Router::new(config.directory.clone()));
        // TODO: put this in config?
        let thread_pool = ThreadPool::new(8);
        let running = Arc::new(AtomicBool::new(true));
        Ok(Self {
            listener,
            router,
            thread_pool,
            running,
        })
    }
    pub fn start(&self) -> Result<()> {
        let r = Arc::clone(&self.running);

        ctrlc::set_handler(move || {
            info!("Starting shutdown...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Graceful shutdown failed!");

        while self.running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("Connection from: {}", addr);
                    let router: Arc<Router<Dir>> = Arc::clone(&self.router);
                    self.thread_pool.execute(move || {
                        if let Err(e) = router.route(&stream) {
                            error!("Error handling request, {}", e);
                        } else {
                            info!("Request handled OK");
                        }
                    })?;
                }
                // The listener is non-blocking, so if there are no connections
                // waiting we can just sleep for a bit and try again
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(100));
                    continue;
                }
                // If there is an error accepting a connection, we'll just
                // print it and continue
                Err(e) => {
                    error!("Connection error: {:?}", e);
                    continue;
                }
            }
        }

        info!("Shutting down server");
        Ok(())
    }
}
