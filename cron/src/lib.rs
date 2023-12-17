use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use domain::{bot_client::BotClient, cron::Cron, repository::CardRepository};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type Result<T> = std::result::Result<T, JobSchedulerError>;

pub struct CronImpl<CR: CardRepository, BC: BotClient> {
    card_repository: Arc<CR>,
    bot_client: Arc<BC>,
}

#[async_trait]
impl<CR: CardRepository<Error = impl Debug>, BC: BotClient> Cron for CronImpl<CR, BC> {
    async fn run(self: Arc<Self>) -> () {
        let sched = JobScheduler::new().await.unwrap();
        sched
            .add(
                Job::new_async("* * * *", move |_uuid, _l| {
                    let card_repository = self.clone().card_repository.clone();
                    let bot_client = self.clone().bot_client.clone();
                    Box::pin(async move { task(card_repository, bot_client).await })
                })
                .unwrap(),
            )
            .await
            .unwrap();
        sched.start().await.unwrap();
    }
}

async fn task<CR: CardRepository<Error = impl Debug>, BC: BotClient>(
    card_repository: Arc<CR>,
    bot_client: Arc<BC>,
) {
    let cards = card_repository.get_all_cards().await.unwrap();
    // send messages
}
