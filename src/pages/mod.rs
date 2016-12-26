pub mod index;
pub mod not_found;
//pub mod ip;
//pub mod user_agent;
//pub mod headers;
//pub mod utf8;
//pub mod status;


use futures::Future;
use minihttp::server::{Head, EncoderDone, Codec, Error};

pub type ResponseFuture<S> = Box<Future<Item=EncoderDone<S>, Error=Error>>;
pub type Response<S> = Box<Codec<S, ResponseFuture=ResponseFuture<S>>>;

/*
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
*/
