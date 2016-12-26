use tokio_core::io::Io;

use pages::{Response};
use service::{Request};

const INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");


pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    req.html(INDEX_TEMPLATE)
}
