pub mod index;
pub mod not_found;
pub mod ip;
pub mod user_agent;
pub mod headers;
pub mod utf8;
pub mod status;


use futures::Future;
use minihttp::server::{EncoderDone, Codec, Error};

pub type ResponseFuture<S> = Box<Future<Item=EncoderDone<S>, Error=Error>>;
pub type Response<S> = Box<Codec<S, ResponseFuture=ResponseFuture<S>>>;
