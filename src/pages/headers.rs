use std::str::from_utf8;
use std::ascii::AsciiExt;
use std::collections::HashMap;

use pages::{Response};
use service::{Request};

pub fn serve<S: 'static>(req: Request) -> Response<S> {
    // emulate case-insensitive dict by using lowercase keys
    let mut headers = HashMap::new();
    // and store first original name of each
    let mut real_names = HashMap::new();
    for h in req.headers() {
        let lower = h.name.to_ascii_lowercase();
        if let Some(mut value) = headers.remove(&lower) {
            value += ", ";
            value += from_utf8(h.value)
                .unwrap_or("--<<Invalid Utf8>>--");
            headers.insert(lower, value);
        } else {
            real_names.insert(lower.clone(), h.name);
            headers.insert(lower,
                from_utf8(h.value)
                .unwrap_or("--<<Invalid Utf8>>--")
                .to_string());
        }
    }
    let mut ob = HashMap::new();
    for (key, header) in real_names.into_iter() {
        ob.insert(header.to_string(), headers.remove(&key).unwrap());
    }
    req.json(json!({
        "headers": ob,
    }))
}

pub fn serve_stripped<S: 'static>(req: Request) -> Response<S> {
    // emulate case-insensitive dict by using lowercase keys
    let mut headers = HashMap::new();
    // and store first original name of each
    let mut real_names = HashMap::new();
    for (name, raw_value) in req.stripped_headers() {
        let lower = name.to_ascii_lowercase();
        if let Some(mut value) = headers.remove(&lower) {
            value += ", ";
            value += from_utf8(raw_value)
                .unwrap_or("--<<Invalid Utf8>>--");
            headers.insert(lower, value);
        } else {
            real_names.insert(lower.clone(), name);
            headers.insert(lower,
                from_utf8(raw_value)
                .unwrap_or("--<<Invalid Utf8>>--")
                .to_string());
        }
    }
    let mut ob = HashMap::new();
    for (key, header) in real_names.into_iter() {
        ob.insert(header.to_string(), headers.remove(&key).unwrap());
    }
    req.json(json!({
        "headers": ob,
    }))
}
