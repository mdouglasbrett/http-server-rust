pub(crate) mod deprecated_handlers;
mod echo;
mod empty;
mod files;
mod user_agent;

pub(crate) use echo::EchoHandler;
pub(crate) use empty::EmptyHandler;
pub(crate) use files::FileHandler;
pub(crate) use user_agent::UserAgentHandler;
