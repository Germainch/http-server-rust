use crate::http::request::{Method, Request};
use crate::http::response::Response;

pub struct Route {
    pub(crate) method: Method,
    pub(crate) uri: String,
    pub(crate) handler: fn(Request, Response),
}


impl Route {

    pub(crate) fn get(uri: &str, f: fn(Request, Response)) -> Self {
        Self {
            method: Method::GET,
            uri: uri.to_string(),
            handler: f,
        }
    }
}

