#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use http_server_rust::{Config, Result, Server};

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new()?;
    let server = Server::new(&config)?;
    server.start()
}
