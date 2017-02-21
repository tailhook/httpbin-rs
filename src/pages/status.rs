use std::str::{FromStr, from_utf8};
use std::path::PathBuf;
use std::sync::Arc;
use std::os::unix::ffi::OsStrExt;

use time;
use tokio_core::io::Io;
use minihttp::Status;
use minihttp::server::{Codec, Error, Encoder, RecvMode};
use futures::{Async};
use futures::future::{ok};

use pages::{ResponseFuture, Response};
use service::{Request};

const PAGE: &'static str = include_str!("../templates/custom_status.html");

pub struct CustomStatus {
    status: Status,
    prefix: Arc<PathBuf>,
}

impl<S: Io + 'static> Codec<S> for CustomStatus {
    type ResponseFuture = ResponseFuture<S>;

    fn recv_mode(&mut self) -> RecvMode {
        RecvMode::buffered_upfront(0)
    }
    fn data_received(&mut self, data: &[u8], end: bool)
        -> Result<Async<usize>, Error>
    {
        assert!(end);
        assert!(data.len() == 0);
        Ok(Async::Ready(0))
    }
    fn start_response(&mut self, mut e: Encoder<S>) -> ResponseFuture<S> {

        let strprefix = from_utf8(self.prefix.as_os_str().as_bytes())
            .expect("prefix is valid utf8");

        let page = PAGE
            .replace("{prefix}", strprefix)
            .replace("{code}", &format!("{:03}", self.status.code()))
            .replace("{reason}", self.status.reason());

        e.status(self.status);
        e.add_length(page.as_bytes().len() as u64).unwrap();

        e.format_header("Date", time::now_utc().rfc822()).unwrap();
        e.add_header("Content-Type", "text/html; charset=utf-8").unwrap();
        e.add_header("Server",
            concat!("httpbin-rs/", env!("CARGO_PKG_VERSION"))
        ).unwrap();

        if e.done_headers().unwrap() {
            e.write_body(page.as_bytes());
        }
        Box::new(ok(e.done()))
    }
}

pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    let parsed = from_utf8(req.suffix().as_os_str().as_bytes()).ok()
        .and_then(|s| u16::from_str(s).ok())
        .and_then(|x| Status::from(x));
    let status = match parsed {
        Some(status) => status,
        None => Status::BadRequest,
    };
    Box::new(CustomStatus { status: status, prefix: req.prefix().clone() })
}
