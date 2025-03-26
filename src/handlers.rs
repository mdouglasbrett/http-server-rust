use crate::{http::Request, Result};
use std::net::TcpStream;

#[derive(Debug)]
pub struct HandlerArg<'a> {
    pub req: &'a Request,
    pub stream: &'a TcpStream,
    // TODO: might get away with a &'a str here
    pub target_dir: &'a String,
}

pub struct EchoHandler;
pub struct EmptyHandler;
pub struct FileHandler;
pub struct UserAgentHandler;

pub trait Handler {
    fn handle(r: HandlerArg) -> Result<()>;
}

impl Handler for EchoHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        Ok(())
    }
}
impl Handler for EmptyHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        Ok(())
    }
}
impl Handler for FileHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        Ok(())
    }
}
impl Handler for UserAgentHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        Ok(())
    }
}
