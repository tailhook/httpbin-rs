use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Status};

use super::std_headers;

const INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");

pub fn serve<S: Io>() -> ResponseFn<Finished<IoBuf<S>, Error>, S> {
    ResponseFn::new(move |mut res| {
        res.status(Status::Ok);
        let index = INDEX_TEMPLATE.replace("{prefix}", "");
        res.add_length(index.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(index.as_bytes());
        }
        res.done()
    })
}
