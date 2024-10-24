#![warn(clippy::style, clippy::complexity, clippy::perf, clippy::correctness)]

use std::env;

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

fn main() -> Result<(), AppError> {
    // TODO: cli front end
    let mut args = env::args();
    let _ = args.next();
    // --directory flag
    let _ = args.next();

    app_server(args.next())
}
