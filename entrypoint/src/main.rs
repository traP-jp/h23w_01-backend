use std::{env, process::exit};

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use rocket::{fairing::AdHoc, http::Method, routes};
use traq_bot_http::RequestParser;

use bot_client::BotClient;
use repository::card::{
    CardRepository, CardRepositoryConfig, CardRepositoryImpl, MigrationStrategy,
};

use handler::cors::{options, CorsConfig};

static CORS_CONFIG: Lazy<CorsConfig> = Lazy::new(|| {
    let Ok(origins) = env::var("ALLOWED_ORIGINS") else {
        eprintln!("env_var ALLOWED_ORIGIN is unset");
        exit(1);
    };
    CorsConfig::new(origins.split(' '))
});

#[tokio::main]
async fn main() -> Result<()> {
    let verification_token =
        env::var("VERIFICATION_TOKEN").context("env var VERIFICATION_TOKEN is unset")?;
    let access_token = env::var("BOT_ACCESS_TOKEN").context("env var BOT_ACCESS_TOKEN is unset")?;
    let check_auth = env::var("CHECK_AUTH")
        .ok()
        .and_then(|c| c.parse::<bool>().ok())
        .unwrap_or(true);
    let parser = RequestParser::new(&verification_token);
    let client = BotClient::new(access_token);
    let card_repository = {
        let load = |s: &str| CardRepositoryConfig::load_env_with_prefix(s);
        let config = load("")
            .or_else(|_| load("MYSQL_"))
            .or_else(|_| load("NS_MARIADB_"))
            .context("env var config for database not found")?;
        CardRepositoryImpl::connect_with_config(config)
            .await
            .context("failed to connect database")?
    };
    let migration_strategy = env::var("MIGRATION")
        .ok()
        .and_then(|m| m.parse::<MigrationStrategy>().ok())
        .unwrap_or_default();
    card_repository
        .migrate(migration_strategy)
        .await
        .context("failed white migration")?;
    rocket::build()
        .mount("/api", routes![handler::ping])
        .mount("/api/cards", handler::cards::routes())
        .mount("/api/images", handler::images::routes())
        .mount("/bot", routes![handler::bot::bot_event])
        .mount("/api/stamps", handler::traq_api::stamps::routes())
        .mount("/api/users", handler::traq_api::users::routes())
        .mount("/api/channels", handler::traq_api::channels::routes())
        .mount("/", routes![options])
        .manage(parser)
        .manage(client)
        .manage(handler::auth::AuthUserConfig(check_auth))
        .manage(card_repository)
        .attach(AdHoc::on_response("CORS wrapper", |req, res| {
            Box::pin(async move {
                use rocket::http::hyper::header::ORIGIN;
                let Some(origin) = req.headers().get_one(ORIGIN.as_str()) else {
                    println!("CORS wrapper: Origin not found in request header");
                    return;
                };
                let Some(origin_header) = CORS_CONFIG.render_origins(origin) else {
                    println!("CORS wrapper: Origin `{}` not allowed", origin);
                    return;
                };
                res.set_header(origin_header);
                if req.method() != Method::Options {
                    println!("CORS wrapper: method is not OPTION");
                    return;
                }
                res.set_header(CORS_CONFIG.render_methods());
                res.set_header(CORS_CONFIG.render_headers());
            })
        }))
        .launch()
        .await?;
    Ok(())
}
