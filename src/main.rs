#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use codecrafters_http_server::{app_server, check_directory, Config, Result};

fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new();
    if !check_directory(&config.directory) {
        std::fs::create_dir(&config.directory)?;
    }
    app_server(config)
}
