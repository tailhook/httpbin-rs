use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Request};
use serde_json::builder::ObjectBuilder;

use super::json_page;

pub fn serve<S: Io>(request: &Request)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    json_page(&ObjectBuilder::new()
        .insert_object("headers", |mut ob| {
            for &(ref k, ref v) in request.headers.iter() {
                ob = ob.insert(k.as_ref(), v);
            }
            ob
        })
        .build())
}
