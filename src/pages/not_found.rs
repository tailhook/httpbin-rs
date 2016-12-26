use minihttp::Status;
use tokio_core::io::Io;

use pages::{Response};
use service::{Request};


const NOT_FOUND: &'static str = include_str!("../templates/not_found.html");

pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    Box::new(req.html_error::<S>(Status::NotFound, NOT_FOUND))
}
