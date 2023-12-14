use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(id) = req.headers().get_one("X-Forwarded-User") else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        Outcome::Success(AuthUser { id: id.to_string() })
    }
}
