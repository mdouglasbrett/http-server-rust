#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use std::env;

pub mod constants;
pub mod errors;
pub mod handlers;
pub mod http;
pub mod router;
pub mod routes;
pub mod server;
pub mod utils;

use errors::AppError;
use server::app_server;

fn main() -> Result<(), AppError> {
    // TODO: cli front end
    let mut args = env::args();
    let _ = args.next();
    // --directory flag
    let _ = args.next();

    app_server(args.next())
}
