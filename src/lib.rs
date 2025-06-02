mod config;
mod errors;
mod file;
mod handlers;
mod http;
mod router;
mod server;

// Re-exports for main.rs
pub use {
    config::{Config, HTTP_VERSION},
    errors::Result,
    server::Server,
};
