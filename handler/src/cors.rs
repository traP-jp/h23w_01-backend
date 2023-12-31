use itertools::Itertools;
use rocket::http::{hyper, Header, Method, Status};

use hyper::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN,
};

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub origins: Vec<String>,
    pub allow_credentials: bool,
    pub methods: Vec<Method>,
    pub headers: Vec<String>,
}

impl CorsConfig {
    pub fn new<'s>(origins: impl Iterator<Item = &'s str>) -> Self {
        Self {
            origins: origins.map(|s| s.to_string()).collect(),
            allow_credentials: true,
            methods: vec![],
            headers: vec![],
        }
    }

    pub fn load_env() -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        use std::env::var;
        let origins = var("ALLOWED_ORIGINS")?;
        let allow_credentials = match var("ALLOW_CREDENTIALS") {
            Ok(c) => c.parse::<bool>()?,
            Err(_) => true,
        };
        let methods = var("ALLOWED_METHODS").unwrap_or_default();
        let headers = var("ALLOWED_HEADERS").unwrap_or_default();
        Ok(Self {
            origins: origins.split(' ').map(|s| s.to_string()).collect(),
            allow_credentials,
            methods: methods
                .split(' ')
                .map(|s| {
                    s.parse::<Method>()
                        .map_err(|_| format!("unknown method name {}", s))
                })
                .collect::<Result<Vec<_>, _>>()?,
            headers: headers.split(' ').map(|s| s.to_string()).collect(),
        })
    }

    pub fn credentials(self, value: bool) -> Self {
        Self {
            allow_credentials: value,
            ..self
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

    pub fn render_origins<'r>(&self, origin: &'r str) -> Option<Header<'r>> {
        self.origins
            .iter()
            .any(|o| o == origin)
            .then(|| Header::new(ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), origin))
    }

    pub fn render_credentials(&self) -> Header<'_> {
        Header::new(
            ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
            self.allow_credentials.to_string(),
        )
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
