use crate::{
    constants::headers::USER_AGENT,
    http::{Request, Response, StatusCode},
    Result,
};

pub struct UserAgentHandler;

impl UserAgentHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        Response::builder()
            .status_code(StatusCode::Ok)
            .body(
                request
                    .get_header(USER_AGENT)
                    .unwrap_or("")
                    .as_bytes(),
            )
            .build()
    }
}
