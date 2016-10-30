use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Status, Request};
use serde_json::builder::ObjectBuilder;
use serde_json::ser::to_string_pretty;

use super::std_headers;

pub fn serve<S: Io>(request: Request)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    let ua = request.headers.iter()
        .find(|h| &h.0 == "User-Agent")
        .map(|h| &h.1[..])
        .unwrap_or("");
    let data = to_string_pretty(
        &ObjectBuilder::new()
        .insert("user_agent", ua)
        .build()).unwrap();
    ResponseFn::new(move |mut res| {
        res.status(Status::Ok);
        res.add_header("Content-Type", "application/json").unwrap();
        res.add_length(data.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(data.as_bytes());
        }
        res.done()
    })
}
