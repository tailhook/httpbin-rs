use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Request};
use serde_json::builder::ObjectBuilder;

use super::json_page;

pub fn serve<S: Io>(request: Request)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    let ua = request.headers.iter()
        .find(|h| &h.0 == "User-Agent")
        .map(|h| &h.1[..])
        .unwrap_or("");
    json_page(&ObjectBuilder::new()
        .insert("user_agent", ua)
        .build())
}
