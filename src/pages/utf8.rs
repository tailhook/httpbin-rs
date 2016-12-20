use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::server::{ResponseFn, Error};
use minihttp::{Status};

use super::std_headers;

const PAGE_TEMPLATE: &'static str = include_str!("../templates/utf8.html");

pub fn serve<S: Io>() -> ResponseFn<Finished<IoBuf<S>, Error>, S> {
    ResponseFn::new(move |mut res| {
        res.status(Status::Ok);
        let page = PAGE_TEMPLATE.replace("{prefix}", "");
        res.add_length(page.as_bytes().len() as u64).unwrap();
        res.add_header("Content-Type", "text/html; charset=utf-8").unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(page.as_bytes());
        }
        res.done()
    })
}
