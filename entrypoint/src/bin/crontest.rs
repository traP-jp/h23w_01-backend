use std::sync::Arc;

use tokio::time::{sleep, Duration};

use domain::bot_client::MockBotClient;
use domain::cron::Cron;
use domain::repository::{MockCardRepository, MockImageRepository};

use cron::CronImpl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MockBotClient::new();
    let card_repository = MockCardRepository::new();
    let image_repository = MockImageRepository::new();
    let cron = Arc::new(CronImpl::new(
        Arc::new(card_repository),
        Arc::new(image_repository),
        Arc::new(client),
    ));
    tokio::spawn(async move { cron.run().await });
    sleep(Duration::from_secs(100)).await;
    Ok(())
}
