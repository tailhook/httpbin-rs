use std::net::SocketAddr;
use futures::{Finished};
use tk_bufstream::IoBuf;
use tokio_core::io::Io;
use minihttp::server::{ResponseFn, Error};
use serde_json::builder::ObjectBuilder;

use super::json_page;

pub fn serve<S: Io>(origin: SocketAddr)
    -> ResponseFn<Finished<IoBuf<S>, Error>, S>
{
    json_page(&ObjectBuilder::new()
            .insert("origin", format!("{}", origin.ip()))
            .build())
}
