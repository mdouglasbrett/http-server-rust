//use crate::{Result, handlers::HandlerArg};
use crate::Result;
//use std::io::Write;

pub trait FileSystemAccess {
    fn try_read(&self, src: &str) -> Result<Vec<u8>>;
    fn try_write(&self, src: &str, d: &[u8]) -> Result<()>;
    fn check_dir_exists(&self) -> bool;
    fn try_create(&self) -> Result<()>;
}


// TODO: Do the filesystem access part first
//pub trait Handler {
//    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
//    where
//        T: Write,
//        U: FileSystemAccess + Default;
//}
