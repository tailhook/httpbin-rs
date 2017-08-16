extern crate time;
extern crate futures;
extern crate tokio_core;
extern crate tk_bufstream;
extern crate tk_http;
extern crate httparse;
#[macro_use] extern crate serde_json;

mod service;
mod pages;
mod response;

pub use service::HttpBin;
