
use pages::{Response};
use service::{Request};

const INDEX_TEMPLATE: &'static str = include_str!("../templates/index.html");


pub fn serve<S: 'static>(req: Request) -> Response<S> {
    req.html(INDEX_TEMPLATE)
}
