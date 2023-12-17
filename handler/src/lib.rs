use rocket::get;

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
