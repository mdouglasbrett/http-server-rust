mod config;
mod errors;
mod handlers;
mod http;
mod router;
mod server;
mod file;

// Re-exports for main.rs
pub use {
    config::{Config, HTTP_VERSION},
    errors::Result,
    server::Server,
};
