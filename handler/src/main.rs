#[macro_use]
extern crate rocket;

use std::{env, process::exit};

use once_cell::sync::Lazy;
use rocket::{fairing::AdHoc, http::Method};
use traq_bot_http::RequestParser;

use bot_client::BotClient;

use handler::cors::{options, CorsConfig};

static CORS_CONFIG: Lazy<CorsConfig> = Lazy::new(|| {
    let Ok(origin) = env::var("ALLOWED_ORIGIN") else {
        eprintln!("env_var ALLOWED_ORIGIN is unset");
        exit(1);
    };
    CorsConfig::new(origin)
});

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
        .mount("/", routes![options])
        .manage(parser)
        .manage(client)
        .attach(AdHoc::on_response("CORS wrapper", |req, res| {
            Box::pin(async move {
                use rocket::http::hyper::header::ORIGIN;
                if req.method() != Method::Options {
                    println!("CORS wrapper: method is not OPTION");
                    return;
                }
                let Some(origin) = req.headers().get_one(ORIGIN.as_str()) else {
                    println!("CORS wrapper: header Origin not found");
                    return;
                };
                if origin != CORS_CONFIG.origin {
                    println!("CORS wrapper: header Origin mismatch `{}`", origin);
                    return;
                }
                res.set_header(CORS_CONFIG.render_origin());
                res.set_header(CORS_CONFIG.render_methods());
                res.set_header(CORS_CONFIG.render_headers());
            })
        }))
}
