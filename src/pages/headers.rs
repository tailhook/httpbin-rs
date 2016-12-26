use std::str::from_utf8;
use std::ascii::AsciiExt;
use std::collections::HashMap;

use tokio_core::io::Io;
use serde_json::builder::ObjectBuilder;

use pages::{Response};
use service::{Request};

pub fn serve<S: Io + 'static>(req: Request) -> Response<S> {
    req.json(ObjectBuilder::new()
        .insert_object("headers", |mut ob| {
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
            for (key, header) in real_names.into_iter() {
                ob = ob.insert(header.to_string(),
                               headers.remove(&key).unwrap());
            }
            ob
        })
        .build())
}
