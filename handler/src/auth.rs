use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthUserConfig(pub bool);

impl From<bool> for AuthUserConfig {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<AuthUserConfig> for bool {
    fn from(value: AuthUserConfig) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct AuthUser<'r> {
    pub id: Option<&'r str>,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthUser<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = req
            .rocket()
            .state::<AuthUserConfig>()
            .map(|c| c.0)
            .unwrap_or(true);
        if !auth {
            return Outcome::Success(AuthUser { id: None });
        }
        let Some(id) = req.headers().get_one("X-Forwarded-User") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        Outcome::Success(AuthUser { id: Some(id) })
    }
}
