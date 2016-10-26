extern crate time;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tk_bufstream;
extern crate minihttp;

mod service;
mod pages;

pub use service::HttpBin;
