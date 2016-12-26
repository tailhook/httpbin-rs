extern crate time;
extern crate futures;
extern crate tokio_core;
extern crate tk_bufstream;
extern crate minihttp;
extern crate serde_json;

mod service;
mod pages;
mod response;

pub use service::HttpBin;

use std::sync::Arc;
use std::path::{PathBuf, Path};
use std::net::SocketAddr;

use futures::Future;
use minihttp::server::{Head, EncoderDone, Codec, Error};
