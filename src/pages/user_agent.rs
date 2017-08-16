use std::str::from_utf8;
use std::ascii::AsciiExt;

use pages::{Response};
use service::{Request};


pub fn serve<S: 'static>(req: Request) -> Response<S> {
    let ua = req.headers()
        .find(|h| h.name.eq_ignore_ascii_case("User-Agent"))
        .map(|h| &h.value[..])
        .map(|v| from_utf8(v).unwrap_or("--<<Invalid Utf8>>--"))
        .unwrap_or("");
    req.json(json!({
        "user_agent": ua,
    }))
}
