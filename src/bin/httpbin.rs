extern crate time;
extern crate tokio_core;
extern crate tokio_service;
extern crate futures;
extern crate tk_bufstream;
extern crate netbuf;
extern crate minihttp;
extern crate httpbin;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;

use tokio_core::reactor::Core;


fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn");
    }
    env_logger::init().expect("init logging");

    let mut lp = Core::new().unwrap();

    let addr = "0.0.0.0:8080".parse().unwrap();

    minihttp::serve(&lp.handle(), addr, || Ok(httpbin::HttpBin::new()));

    lp.run(futures::empty::<(), ()>()).unwrap();
}
