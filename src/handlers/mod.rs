use crate::{
    http::{Request, Response},
    Result,
};

pub trait Handler {
    fn handle(&self, request: &Request) -> Result<Response>;
}

pub mod deprecated_handlers;
pub mod echo;
pub mod files;
pub mod user_agent;

pub use echo::EchoHandler;
pub use files::FileHandler;
pub use user_agent::UserAgentHandler;
