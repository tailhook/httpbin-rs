use tokio_core::io::Io;
use serde_json::builder::ObjectBuilder;

use pages::{Response};
use service::{Request};


pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    req.json(ObjectBuilder::new()
        .insert("origin", format!("{}", req.ip()))
        .build())
}
