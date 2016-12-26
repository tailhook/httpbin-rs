use std::str::from_utf8;
use std::path::{PathBuf, Path};
use std::sync::Arc;
use std::net::{SocketAddr, IpAddr};
use std::os::unix::prelude::*;

use httparse::Header;
use tokio_core::io::Io;

use pages;
use minihttp::server::{Dispatcher, Error, Head};
use pages::{Response};

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
            "" => Ok(pages::index::serve(req)),
            "ip" => Ok(pages::ip::serve(req)),
            "user-agent" => Ok(pages::user_agent::serve(req)),
            "headers" => Ok(pages::headers::serve(req)),
            "encoding" => Ok(pages::utf8::serve(req)),
            "status" => Ok(pages::status::serve(req)),
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
    pub fn ip(&self) -> IpAddr {
        self.addr.ip()
    }
    pub fn headers(&self) -> ::std::slice::Iter<Header> {
        self.head.headers().iter()
    }
    pub fn suffix(&self) -> &Path {
        self.suffix
    }
}
