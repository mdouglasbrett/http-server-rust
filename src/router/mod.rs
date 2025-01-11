mod deprecated_request_router;
mod routes;

pub(crate) use deprecated_request_router::request_router;
pub(crate) use routes::Route;

// TODO: placeholder
pub(crate) struct Router;

impl Router {
    pub(crate) fn new(dir: String) -> Self {
        Router
    }
}
