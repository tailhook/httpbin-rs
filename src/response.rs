use std::path::PathBuf;
use std::sync::Arc;
use std::os::unix::ffi::OsStrExt;

use time;
use futures::future::{FutureResult, ok};
use futures::{Async};
use tokio_core::io::Io;
use minihttp::Status;
use minihttp::server::{Codec, Error, Encoder, EncoderDone, Head, RecvMode};

use pages::{ResponseFuture};
use service::{Request};


pub struct Html {
    status: Status,
    prefix: Arc<PathBuf>,
    data: &'static str,
}


impl<S: Io + 'static> Codec<S> for Html {
    type ResponseFuture = ResponseFuture<S>;

    fn recv_mode(&mut self) -> RecvMode {
        RecvMode::BufferedUpfront(0)
    }
    fn data_received(&mut self, data: &[u8], end: bool)
        -> Result<Async<usize>, Error>
    {
        assert!(end);
        assert!(data.len() == 0);
        Ok(Async::Ready(0))
    }
    fn start_response(&mut self, mut e: Encoder<S>) -> ResponseFuture<S> {

        let strprefix = self.prefix.as_os_str().as_bytes();
        let nprefixes = self.data.split("{prefix}").count() - 1;
        let blen = self.data.len() +
            nprefixes * strprefix.len() - nprefixes * "{prefix}".len();

        e.status(self.status);
        e.add_length(blen as u64).unwrap();
        e.format_header("Date", time::now_utc().rfc822()).unwrap();
        e.add_header("Server",
            concat!("httpbin-rs/", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        if e.done_headers().unwrap() {
            let mut iter = self.data.split("{prefix}");
            e.write_body(iter.next().unwrap().as_bytes());
            for chunk in iter {
                e.write_body(strprefix);
                e.write_body(chunk.as_bytes());
            }
        }
        Box::new(ok(e.done()))
    }
}

impl<'a> Request<'a> {
    pub fn html<S>(&self, data: &'static str) -> Html {
        Html {
            status: Status::Ok,
            prefix: self.prefix().clone(),
            data: data,
        }
    }
    pub fn html_error<S>(&self, status: Status, data: &'static str) -> Html {
        Html {
            status: status,
            prefix: self.prefix().clone(),
            data: data,
        }
    }
}
