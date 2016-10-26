use futures::{Finished, Async, finished};
use std::marker::PhantomData;
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
        let serializer = match &req.path[..] {
            "/" => pages::index::serve(),
            _ => pages::not_found::serve(),
        };
        return finished(serializer)
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
