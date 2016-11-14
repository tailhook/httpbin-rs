use std::str::from_utf8;
use std::marker::PhantomData;
use std::path::Path;
use std::os::unix::prelude::*;

use futures::{Finished, finished};
use tokio_core::io::Io;
use tokio_service::Service;
use tk_bufstream::IoBuf;

use pages;
use minihttp::{ResponseFn, Error};
use minihttp::request::Request;

pub struct HttpBin<S> {
    p: PhantomData<S>
}


impl<S: Io> Service for HttpBin<S> {
    type Request = Request;
    type Response = ResponseFn<Finished<IoBuf<S>, Error>, S>;
    type Error = Error;
    type Future = Finished<Self::Response, Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut path = Path::new(&req.path).iter()
            .map(|x| from_utf8(x.as_bytes()).unwrap());
        path.next();  // first empty component
        let serializer = match (path.next().unwrap_or(""), path.next()) {
            ("", _) => pages::index::serve(),
            ("ip", None) => pages::ip::serve(req.peer_addr),
            ("user-agent", None) => pages::user_agent::serve(&req),
            ("headers", None) => pages::headers::serve(&req),
            ("encoding", Some("utf-8")) => pages::utf8::serve(),
            ("status", Some(x)) => pages::status::serve(x),
            _ => pages::not_found::serve(),
        };
        return finished(serializer)
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
