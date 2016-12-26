extern crate time;
extern crate tokio_core;
extern crate futures;
extern crate tk_bufstream;
extern crate netbuf;
extern crate minihttp;
extern crate httpbin;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener};
use futures::{Stream, Future};

use httpbin::HttpBin;
use minihttp::server::{Config, Proto};


fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn");
    }
    env_logger::init().expect("init logging");

    let mut lp = Core::new().unwrap();

    let addr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr, &lp.handle()).unwrap();
    let cfg = Config::new().done();
    let bin = HttpBin::new();

    let done = listener.incoming()
        .map_err(|e| { println!("Accept error: {}", e); })
        .map(move |(socket, addr)| {
            Proto::new(socket, &cfg, bin.instantiate(addr))
            .map_err(|e| { println!("Connection error: {}", e); })
        })
        .buffer_unordered(200000)
          .for_each(|()| Ok(()));

    lp.run(done).unwrap();
}
