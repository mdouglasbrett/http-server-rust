use super::StatusCode;

#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) status_code: StatusCode,
    // TODO: these are temp, are they the most appropriate?
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: Option<String>,
}

impl Response {}
