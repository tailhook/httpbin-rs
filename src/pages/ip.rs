use std::net::SocketAddr;
use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Status};
use serde_json::builder::ObjectBuilder;

use super::std_headers;

pub fn serve<S: Io>(origin: SocketAddr)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    ResponseFn::new(move |mut res| {
        res.status(Status::Ok);
        res.add_header("Content-Type", "application/json").unwrap();
        let data = format!("{}",
            ObjectBuilder::new()
            .insert("origin", format!("{}", origin.ip()))
            .build());
        res.add_length(data.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(data.as_bytes());
        }
        res.done()
    })
}
