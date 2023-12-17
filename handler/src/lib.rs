use std::str::FromStr;
use std::sync::Arc;

use rocket::get;
use rocket::request::FromParam;
use uuid::Uuid;

use domain::bot_client::BotClient;
use domain::repository::{CardRepository, ImageRepository};

pub mod auth;
pub mod bot;
pub mod cards;
pub mod cors;
pub mod images;
pub mod traq_api;

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

pub struct UuidParam(pub Uuid);

impl<'r> FromParam<'r> for UuidParam {
    type Error = <Uuid as FromStr>::Err;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let id = param.parse()?;
        Ok(Self(id))
    }
}

pub struct CR(pub Arc<dyn CardRepository<Error = anyhow::Error>>);

impl<T> From<T> for CR
where
    T: CardRepository<Error = anyhow::Error>,
{
    fn from(value: T) -> Self {
        CR(Arc::new(value))
    }
}

pub struct IR(pub Arc<dyn ImageRepository<Error = anyhow::Error>>);

impl<T> From<T> for IR
where
    T: ImageRepository<Error = anyhow::Error>,
{
    fn from(value: T) -> Self {
        IR(Arc::new(value))
    }
}

pub struct BC(pub Arc<dyn BotClient<Error = anyhow::Error>>);

impl<T> From<T> for BC
where
    T: BotClient<Error = anyhow::Error>,
{
    fn from(value: T) -> Self {
        BC(Arc::new(value))
    }
}
