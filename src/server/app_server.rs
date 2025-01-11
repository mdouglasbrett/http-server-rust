use super::ThreadPool;
use crate::router::Router;
use crate::{Config, Result};
use std::net::TcpListener;
use std::sync::{atomic::AtomicBool, Arc};

pub struct Server {
    listener: TcpListener,
    router: Arc<Router>,
    thread_pool: ThreadPool,
    running: Arc<AtomicBool>,
}

impl Server {
    pub fn new(config: &Config) -> Result<Self> {
        let listener = TcpListener::bind(&config.address)?;
        listener.set_nonblocking(true)?;
        let router = Arc::new(Router::new(config.directory.clone()));
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
        todo!()
    }
}
