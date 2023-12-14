use itertools::Itertools;
use rocket::http::{hyper, Header, Method, Status};

use hyper::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};

pub struct CorsConfig {
    pub origin: String,
    pub methods: Vec<Method>,
    pub headers: Vec<String>,
}

impl CorsConfig {
    pub fn new(origin: String) -> Self {
        Self {
            origin,
            methods: vec![],
            headers: vec![],
        }
    }

    pub fn add_method(mut self, method: Method) -> Self {
        self.methods.push(method);
        self
    }

    pub fn add_header(mut self, header: String) -> Self {
        self.headers.push(header);
        self
    }

    pub fn render_origin(&self) -> Header<'_> {
        Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), &self.origin)
    }

    pub fn render_methods(&self) -> Header<'_> {
        if !self.methods.is_empty() {
            Header::new(
                ACCESS_CONTROL_ALLOW_METHODS.as_str(),
                self.methods.iter().map(|m| m.as_str()).join(", "),
            )
        } else {
            Header::new(ACCESS_CONTROL_ALLOW_METHODS.as_str(), "*")
        }
    }

    pub fn render_headers(&self) -> Header<'_> {
        if !self.headers.is_empty() {
            Header::new(
                ACCESS_CONTROL_ALLOW_HEADERS.as_str(),
                self.headers.iter().join(", "),
            )
        } else {
            Header::new(ACCESS_CONTROL_ALLOW_HEADERS.as_str(), "*")
        }
    }
}

#[rocket::options("/<_..>")]
pub async fn options() -> Status {
    Status::NoContent
}
