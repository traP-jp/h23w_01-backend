use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use domain::bot_client::User;

use crate::BC;

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
pub struct AuthUser(pub Option<User>);

#[async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = req
            .rocket()
            .state::<AuthUserConfig>()
            .map(|c| c.0)
            .unwrap_or(true);
        let Some(bot_client) = req.rocket().state::<BC>() else {
            eprintln!("BC unmanaged");
            return Outcome::Error((Status::InternalServerError, ()));
        };
        if !auth {
            return Outcome::Success(AuthUser(None));
        }
        let Some(name) = req.headers().get_one("X-Forwarded-User") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let Ok(mut users) = bot_client.0.get_users(Some(name)).await.map_err(|e| {
            eprintln!("error in bot_client.get_users: {}", e);
            e
        }) else {
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let Some(user) = users.pop() else {
            return Outcome::Error((Status::Unauthorized, ()));
        };
        Outcome::Success(AuthUser(Some(user)))
    }
}
