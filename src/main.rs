#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

mod config;
mod constants;
mod errors;
mod handlers;
mod http;
mod router;
mod routes;
mod server;
mod utils;

use config::Config;
use errors::AppError;
use server::app_server;

pub type Result<T> = std::result::Result<T, AppError>;

fn main() -> Result<()> {
    app_server(Config::new())
}
