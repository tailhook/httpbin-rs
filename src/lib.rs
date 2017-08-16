//! A http bin library embeddable to any `tk-http` application
//!
//! Use `HttpBin` as a factory and `HttpBin::instantiate` to create
//! `tk_http::server::Dispatcher`.
//!
//! # Example
//!
//! ```rust,no_run
//! extern crate tokio_core;
//! extern crate futures;
//! extern crate tk_http;
//! extern crate tk_listen;
//! extern crate httpbin;
//!
//! use std::time::Duration;
//! use tokio_core::reactor::Core;
//! use tokio_core::net::{TcpListener};
//! use futures::{Stream, Future};
//! use httpbin::HttpBin;
//! use tk_http::server::{Config, Proto};
//! use tk_listen::ListenExt;
//!
//! # fn main() {
//! let mut lp = Core::new().unwrap();
//!
//! let addr = "0.0.0.0:8080".parse().unwrap();
//! let listener = TcpListener::bind(&addr, &lp.handle()).unwrap();
//! let cfg = Config::new().done();
//! let bin = HttpBin::new();
//! let h1 = lp.handle();
//!
//! let done = listener.incoming()
//!     .sleep_on_error(Duration::from_millis(100), &h1)
//!     .map(move |(socket, addr)| {
//!         Proto::new(socket, &cfg, bin.instantiate(addr), &h1)
//!         .map_err(|e| { println!("Connection error: {}", e); })
//!     })
//!     .listen(1000);
//!
//! lp.run(done).unwrap();
//! # }
//!
//! ```
//!
#![warn(missing_docs)]
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

pub use service::{HttpBin, HttpBinDispatcher};
