#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

mod constants;
mod errors;
mod handlers;
mod http;
mod router;
mod routes;
mod server;
mod utils;

use constants::defaults;
use errors::AppError;
use server::app_server;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
struct Config {
    target_dir: String,
    address: String,
}

impl Config {
    fn new() -> Self {
        // TODO: implement options
        use lexopt::prelude::*;
        // TODO: we will fall back to default if lexopt errors out
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: defaults::TARGET_DIR.to_owned(),
            address: defaults::ADDRESS.to_owned(),
        }
    }
}

fn main() -> Result<()> {
    let config = Config::new();
    app_server(config)
}
