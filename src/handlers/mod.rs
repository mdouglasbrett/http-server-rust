pub mod deprecated_handlers;
mod echo;
mod empty;
mod files;
mod user_agent;

pub use echo::EchoHandler;
pub use empty::EmptyHandler;
pub use files::FileHandler;
pub use user_agent::UserAgentHandler;
