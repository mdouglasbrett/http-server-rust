use crate::constants::defaults;
use lexopt::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub target_dir: String,
    pub address: String,
}

impl Config {
    pub fn new() -> Self {
        // TODO: implement option
        // TODO: we will fall back to default if lexopt errors out
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: defaults::TARGET_DIR.to_owned(),
            address: defaults::ADDRESS.to_owned(),
        }
    }
}
