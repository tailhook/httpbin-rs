use time;

use futures::{Finished};
use serde_json::Value;
use serde_json::ser::to_string_pretty;
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseWriter, ResponseFn, Error, Status};

pub mod index;
pub mod not_found;
pub mod ip;
pub mod user_agent;


fn std_headers<S: Io>(res: &mut ResponseWriter<S>) {
    res.format_header("Date", time::now_utc().rfc822()).unwrap();
    res.add_header("Server", concat!("httpbin-rs/",
                             env!("CARGO_PKG_VERSION"))).unwrap();
}

fn json_page<S: Io>(json: &Value)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    let data = to_string_pretty(json).unwrap();
    ResponseFn::new(move |mut res| {
        res.status(Status::Ok);
        res.add_header("Content-Type", "application/json").unwrap();
        res.add_length(data.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(data.as_bytes());
        }
        res.done()
    })
}
