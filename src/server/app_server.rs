use super::ThreadPool;
use crate::router::Router;
use std::net::TcpListener;
use std::sync::{atomic::AtomicBool, Arc};

pub struct Server {
    listener: TcpListener,
    router: Arc<Router>,
    thread_pool: ThreadPool,
    running: Arc<AtomicBool>,
}

impl Server {
    pub fn new() {}
    pub fn start() {}
}
