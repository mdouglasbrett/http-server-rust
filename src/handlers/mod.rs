use crate::{
    http::{Request, Response},
    Result,
};

pub trait Handler {
    fn handle(&self, request: &Request) -> Result<Response>;
}

pub(crate) mod deprecated_handlers;
mod echo;
mod files;
mod user_agent;

pub(crate) use echo::EchoHandler;
pub(crate) use files::FileHandler;
pub(crate) use user_agent::UserAgentHandler;
