#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use std::env;
use std::net::TcpListener;

mod constants;
mod errors;
mod handlers;
mod http;
mod router;
mod routes;
mod server;
mod utils;

use errors::AppError;
use server::app_server;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Default)]
struct Config {
    dir: String,
    port: String,
}

impl Config {
    fn parse() -> Result<Self> {
        use lexopt::prelude::*;
        let mut config = Config::default();
        Ok(config)
    }
}

fn main() -> Result<()> {
    let mut args = env::args();
    let _ = args.next();
    // --directory flag
    let _ = args.next();
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    // TODO: pass a Config object
    app_server(args.next(), listener)
}
