pub(crate) mod deprecated_handlers;
mod echo;
mod files;
mod user_agent;
mod empty;

pub(crate) use echo::EchoHandler;
pub(crate) use files::FileHandler;
pub(crate) use user_agent::UserAgentHandler;
pub(crate) use empty::EmptyHandler;

