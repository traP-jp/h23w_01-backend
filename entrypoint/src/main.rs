use std::sync::Arc;

use crate::wrappers::CardRepositoryWrapper;
use anyhow::{Context, Result};
use bot_client::BotClientImpl;
use cron::CronImpl;
use domain::repository::{CardRepository, MigrationStrategy};
use once_cell::sync::Lazy;
use repository::card::{CardRepositoryConfig, CardRepositoryImpl};
use repository::image::{ImageRepositoryConfig, ImageRepositoryImpl};
use rocket::{fairing::AdHoc, http::Method, routes};
use traq_bot_http::RequestParser;

use domain::cron::Cron;
use handler::cors::{options, CorsConfig};

mod wrappers;

static CORS_CONFIG: Lazy<CorsConfig> =
    Lazy::new(|| CorsConfig::load_env().expect("failed to load CORS config"));

#[tokio::main]
async fn main() -> Result<()> {
    use std::env::var;

    use handler::{BC, CR};

    let verification_token =
        var("VERIFICATION_TOKEN").context("env var VERIFICATION_TOKEN is unset")?;
    let access_token = var("BOT_ACCESS_TOKEN").context("env var BOT_ACCESS_TOKEN is unset")?;
    let check_auth = var("CHECK_AUTH")
        .ok()
        .and_then(|c| c.parse::<bool>().ok())
        .unwrap_or(true);
    let parser = RequestParser::new(&verification_token);
    let client = BotClientImpl::new(access_token);
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
    let image_repository = {
        let load = |s: &str| ImageRepositoryConfig::load_env_with_prefix(s);
        let config = load("")
            .or_else(|_| load("MINIO_"))
            .or_else(|_| load("R2_"))
            .expect("env var config for object storage not found");
        ImageRepositoryImpl::new_with_config(config).expect("failed to connect object storage")
    };
    let card_repository = CardRepositoryWrapper(card_repository);
    let card_repository = Arc::new(card_repository);
    let image_repository = Arc::new(image_repository);
    let cron = CronImpl::new(
        card_repository.clone(),
        image_repository.clone(),
        Arc::new(client.clone()),
    );
    let cron = Arc::new(cron);
    let client: BC = wrappers::BotClientWrapper(client).into();
    tokio::spawn(async move { cron.run().await });
    let migration_strategy = var("MIGRATION")
        .ok()
        .and_then(|m| m.parse::<MigrationStrategy>().ok())
        .unwrap_or_default();
    card_repository
        .migrate(migration_strategy)
        .await
        .context("failed white migration")?;
    let card_repository: CR = CR(card_repository);
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
        .manage(image_repository)
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
                res.set_header(CORS_CONFIG.render_credentials());
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
