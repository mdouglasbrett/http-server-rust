#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use codecrafters_http_server::{Config, Result, Server};
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new();
    // TODO: move this into config implementation
    if !check_directory(&config.directory) {
        // TODO: permissions?
        std::fs::create_dir(&config.directory)?;
    }
    let server = Server::new(&config)?;
    server.start()
}

fn check_directory(dir: &str) -> bool {
    let path = Path::new(dir);
    path.exists() && path.is_dir()
}
