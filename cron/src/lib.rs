use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use domain::{
    bot_client::{BotClient, PostMessageParams, UploadFileParams},
    cron::Cron,
    repository::{CardRepository, ImageRepository},
};
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct CronImpl<CR: CardRepository, IR: ImageRepository, BC: BotClient> {
    card_repository: Arc<CR>,
    image_repository: Arc<IR>,
    bot_client: Arc<BC>,
}

#[async_trait]
impl<
        CR: CardRepository<Error = impl Debug>,
        IR: ImageRepository<Error = impl Debug>,
        BC: BotClient<Error = impl Debug>,
    > Cron for CronImpl<CR, IR, BC>
{
    async fn run(self: Arc<Self>) -> () {
        let sched = JobScheduler::new().await.unwrap();
        sched
            .add(
                Job::new_async("* * * *", move |_uuid, _l| {
                    let card_repository = self.clone().card_repository.clone();
                    let image_repository = self.clone().image_repository.clone();
                    let bot_client = self.clone().bot_client.clone();
                    Box::pin(
                        async move { task(card_repository, image_repository, bot_client).await },
                    )
                })
                .unwrap(),
            )
            .await
            .unwrap();
        sched.start().await.unwrap();
    }
}

async fn task<
    CR: CardRepository<Error = impl Debug>,
    IR: ImageRepository<Error = impl Debug>,
    BC: BotClient<Error = impl Debug>,
>(
    card_repository: Arc<CR>,
    image_repository: Arc<IR>,
    bot_client: Arc<BC>,
) {
    let cards = card_repository.get_all_cards().await.unwrap();
    let _ = cards.iter().map(|card| async {
        bot_client
            .uplodad_file(&UploadFileParams {
                id: card.id,
                channel_id: card.id,
                content: image_repository.get_png(card.id).await.unwrap().unwrap(),
                mime_type: "image/png".to_string(),
            })
            .await
            .unwrap();
        bot_client
            .post_message(&PostMessageParams {
                content: format!("hgoe"),
                channel_id: card.id,
                embed: false,
            })
            .await
    });
}
