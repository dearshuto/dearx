use hyper::{header::CONTENT_TYPE, http::HeaderValue};
use warp::{reply::Response, Reply};

pub struct BinaryRequest {
    binary: Vec<u8>,
}

impl BinaryRequest {
    pub fn new(binary: Vec<u8>) -> BinaryRequest {
        Self { binary }
    }
}

impl Reply for BinaryRequest {
    fn into_response(self) -> warp::reply::Response {
        let mut res = Response::new(self.binary.into());
        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        res
    }
}
