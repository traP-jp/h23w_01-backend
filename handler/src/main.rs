#[macro_use]
extern crate rocket;

use std::env;

use traq_bot_http::RequestParser;

use bot_client::BotClient;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
async fn rocket() -> _ {
    let verification_token =
        env::var("VERIFICATION_TOKEN").expect("env var VERIFICATION_TOKEN is unset");
    let access_token = env::var("BOT_ACCESS_TOKEN").expect("env var BOT_ACCESS_TOKEN is unset");
    let parser = RequestParser::new(&verification_token);
    let client = BotClient::new(access_token);
    rocket::build()
        .mount("/api", routes![ping])
        .mount("/api/cards", handler::cards::routes())
        .mount("/api/images", handler::images::routes())
        .mount("/bot", routes![handler::bot::bot_event])
        .mount("/api/stamps", handler::traq_api::stamps::routes())
        .mount("/api/users", handler::traq_api::users::routes())
        .mount("/api/channels", handler::traq_api::channels::routes())
        .manage(parser)
        .manage(client)
}
