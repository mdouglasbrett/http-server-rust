mod config;
mod errors;
mod file;
mod handlers;
mod http;
mod router;
mod server;

pub(crate) mod constants {
    pub const TARGET_DIR: &str = "/tmp";
    pub const ADDRESS: &str = "127.0.0.1:4221";
    pub const HTTP_VERSION: &str = "HTTP/1.1";
}

// Re-exports for main.rs
pub use {config::Config, errors::Result, server::Server};
