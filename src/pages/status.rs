use futures::{Finished};
use std::str::FromStr;
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::{ResponseFn, Error, Status};

use super::std_headers;

const PAGE: &'static str = include_str!("../templates/custom_status.html");

pub fn serve<S: Io>(status: &str) -> ResponseFn<Finished<IoBuf<S>, Error>, S> {
    let parsed = u16::from_str(status).ok().and_then(|x| Status::from(x));
    let status = match parsed {
        Some(status) => status,
        None => Status::BadRequest,
    };
    ResponseFn::new(move |mut res| {
        res.status(status);
        let page = PAGE
            .replace("{prefix}", "")
            .replace("{code}", &format!("{:03}", status.code()))
            .replace("{reason}", status.reason());
        res.add_length(page.as_bytes().len() as u64).unwrap();
        std_headers(&mut res);
        if res.done_headers().unwrap() {
            res.write_body(page.as_bytes());
        }
        res.done()
    })
}
