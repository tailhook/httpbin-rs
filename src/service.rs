use std::str::from_utf8;
use std::marker::PhantomData;
use std::path::{PathBuf, Path};
use std::sync::Arc;
use std::net::SocketAddr;
use std::os::unix::prelude::*;

use futures::{Finished, finished};
use tokio_core::io::Io;
use tk_bufstream::IoBuf;

use pages;
use futures::Future;
use minihttp::server::{Dispatcher, Codec, Error, EncoderDone, Head};
use pages::{Response, ResponseFuture};

pub struct HttpBin {
    prefix: Arc<PathBuf>,
}

pub struct HttpBinDispatcher {
    addr: SocketAddr,
    prefix: Arc<PathBuf>,
}

pub struct Request<'a> {
    addr: SocketAddr,
    head: &'a Head<'a>,
    prefix: &'a Arc<PathBuf>,
    suffix: &'a Path,
}

impl<S: Io + 'static> Dispatcher<S> for HttpBinDispatcher {
    type Codec = Response<S>;

    fn headers_received(&mut self, headers: &Head)
        -> Result<Self::Codec, Error>
    {
        let path = headers.path()
            .and_then(|p| Path::new(p).strip_prefix(&*self.prefix).ok());
        let path = match path {
            Some(p) => p,
            None => {
                return Ok(pages::not_found::serve(Request {
                    addr: self.addr,
                    head: headers,
                    prefix: &self.prefix,
                    suffix: Path::new("--not_found--"),
                }));
            }
        };
        let mut path_iter = Path::new(&path).iter();
        let first_part = path_iter.next()
            .and_then(|x| from_utf8(x.as_bytes()).ok())
            .unwrap_or("");
        let req = Request {
            addr: self.addr,
            head: headers,
            prefix: &self.prefix,
            suffix: path_iter.as_path(),
        };
        match first_part {
            ///("", _) => pages::index::serve(req),
            /// ("", _) => pages::index::serve(),
            /// ("ip", None) => pages::ip::serve(self.addr),
            /// ("user-agent", None) => pages::user_agent::serve(headers),
            /// ("headers", None) => pages::headers::serve(headers),
            /// ("encoding", Some("utf-8")) => pages::utf8::serve(),
            /// ("status", Some(x)) => pages::status::serve(x),
            _ => Ok(pages::not_found::serve(req)),
        }
    }
}

impl HttpBin {
    /// Create a HttpBin instance at root of the domain
    pub fn new() -> HttpBin {
        HttpBin {
            prefix: Arc::new(PathBuf::from("/")),
        }
    }

    /// Create a HttpBin instance at specified path
    ///
    /// The path influences both: how path is matched (everything that does
    /// not start with prefix returns 404). And all paths rendered in the
    /// page.
    pub fn new_at(path: &Path) -> HttpBin {
        assert!(path.is_absolute());
        HttpBin {
            prefix: Arc::new(path.to_path_buf()),
        }
    }

    /// Create an instance of HttpBinDispatcher
    pub fn instantiate(&self, addr: SocketAddr) -> HttpBinDispatcher {
        HttpBinDispatcher {
            addr: addr,
            prefix: self.prefix.clone()
        }
    }
}

impl<'a> Request<'a> {
    pub fn prefix(&self) -> &Arc<PathBuf> {
        self.prefix
    }
}
