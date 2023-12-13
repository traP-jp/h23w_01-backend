#[macro_use]
extern crate rocket;

use std::env;

use traq_bot_http::RequestParser;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
async fn rocket() -> _ {
    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(&verification_token);
    rocket::build()
        .mount("/api", routes![ping])
        .mount("/api/cards", handler::cards::routes())
        .mount("/api/images", handler::images::routes())
        .mount("/bot", routes![handler::bot::bot_event])
        .manage(parser)
}
