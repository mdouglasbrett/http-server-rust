#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use http_server_rust::{Config, Result, Server};

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::try_new()?;
    let server = Server::try_new(&config)?;
    server.start()
}
