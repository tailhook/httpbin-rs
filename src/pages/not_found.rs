use tk_http::Status;

use pages::{Response};
use service::{Request};


const NOT_FOUND: &'static str = include_str!("../templates/not_found.html");


pub fn serve<S: 'static>(req: Request) -> Response<S> {
    req.html_error(Status::NotFound, NOT_FOUND)
}
