use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error};

use super::std_headers;

const NOT_FOUND: &'static str = include_str!("../templates/not_found.html");

pub fn serve<S: Io>() -> ResponseFn<Finished<IoBuf<S>, Error>, S> {
    ResponseFn::new(move |mut res| {
        res.status(404, "Not Found");
        let index = NOT_FOUND.replace("{prefix}", "");
        res.add_length(index.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(index.as_bytes());
        }
        res.done()
    })
}
