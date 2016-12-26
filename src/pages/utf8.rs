use std::path::Path;

use tokio_core::io::Io;

use pages::{Response, not_found};
use service::{Request};

const PAGE_TEMPLATE: &'static str = include_str!("../templates/utf8.html");

pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    if req.suffix() == Path::new("utf-8") {
        req.html(PAGE_TEMPLATE)
    } else {
        not_found::serve(req)
    }
}
