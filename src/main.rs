#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use codecrafters_http_server::{app_server, Config, Result};
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new();
    if !check_directory(&config.directory) {
        std::fs::create_dir(&config.directory)?;
    }
    app_server(config)
}

fn check_directory(dir: &str) -> bool {
    let path = Path::new(dir);
    path.exists() && path.is_dir()
}
