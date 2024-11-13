pub mod defaults {
    pub const TARGET_DIR: &str = "/tmp/";
    pub const ADDRESS: &str = "127.0.0.1:4221";
}
pub mod headers {
    pub const USER_AGENT: &str = "User-Agent";
    pub const CONTENT_LENGTH: &str = "Content-Length";
    pub const CONTENT_ENCONDING: &str = "Content-Encoding";
    pub const ACCEPT_ENCODING: &str = "Accept-Encoding";
    pub const CONTENT_TYPE: &str = "Content-Type";
}

pub mod mime_types {
    pub const PLAIN_TEXT: &str = "text/plain";
    pub const OCTET_STREAM: &str = "application/octect-stream";
}
