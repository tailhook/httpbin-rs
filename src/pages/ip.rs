use pages::{Response};
use service::{Request};


pub fn serve<S: 'static>(req: Request) -> Response<S> {
    req.json(json!({
        "origin": format!("{}", req.ip()),
    }))
}
