use time;

use tokio_core::io::Io;
use minihttp::ResponseWriter;

pub mod index;
pub mod not_found;
pub mod ip;
pub mod user_agent;


fn std_headers<S: Io>(res: &mut ResponseWriter<S>) {
    res.format_header("Date", time::now_utc().rfc822()).unwrap();
    res.add_header("Server", concat!("httpbin-rs/",
                             env!("CARGO_PKG_VERSION"))).unwrap();
}
