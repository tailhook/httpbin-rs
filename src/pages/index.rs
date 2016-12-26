use tokio_core::io::Io;
use minihttp::server::{Codec};

use response::Html;
use pages::{Response};
use service::{Request};

const INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");

pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    Box::new(req.html::<S>(INDEX_TEMPLATE))
}
