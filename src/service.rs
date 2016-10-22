use time;
use futures::{Finished, Async, finished};
use std::marker::PhantomData;
use tokio_core::io::Io;
use tokio_service::Service;
use tk_bufstream::IoBuf;

use minihttp::{ResponseFn, Error};
use minihttp::request::Request;

pub struct HttpBin<S> {
    p: PhantomData<S>
}

const INDEX_TEMPLATE: &'static str = include_str!("templates/index.html");

impl<S: Io> Service for HttpBin<S> {
    type Request = Request;
    type Response = ResponseFn<Finished<IoBuf<S>, Error>, S>;
    type Error = Error;
    type Future = Finished<Self::Response, Error>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        finished(ResponseFn::new(move |mut res| {
            res.status(200, "OK");
            let index = INDEX_TEMPLATE.replace("{prefix}", "");
            res.add_length(index.as_bytes().len() as u64).unwrap();
            res.format_header("Date", time::now_utc().rfc822()).unwrap();
            res.add_header("Server", concat!("httpbin-rs/",
                                     env!("CARGO_PKG_VERSION"))).unwrap();
            if res.done_headers().unwrap() {
                res.write_body(index.as_bytes());
            }
            res.done()
        }))
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

impl<S: Io> HttpBin<S> {
    pub fn new() -> HttpBin<S> {
        HttpBin {
            p: PhantomData,
        }
    }
}

impl<S: Io> Clone for HttpBin<S> {
    fn clone(&self) -> HttpBin<S> {
        HttpBin {
            p: self.p,
        }
    }
}
