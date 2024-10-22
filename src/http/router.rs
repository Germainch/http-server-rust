use crate::http::request::Method;
use crate::http::route::Route;

pub struct Router{
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn route(&self, method: Method, uri: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.method == method && r.uri == uri)
    }
}