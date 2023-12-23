use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::{
    bot_client::{BotClient, PostMessageParams, UploadFileParams},
    cron::Cron,
    repository::{CardRepository, ImageRepository},
};
use futures::future::join_all;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct CronImpl<CR: CardRepository, IR: ImageRepository, BC: BotClient> {
    card_repository: Arc<CR>,
    image_repository: Arc<IR>,
    bot_client: Arc<BC>,
}

impl<
        CR: CardRepository<Error = impl Debug + Send>,
        IR: ImageRepository<Error = impl Debug + Send>,
        BC: BotClient<Error = impl Debug + Send>,
    > CronImpl<CR, IR, BC>
{
    pub fn new(card_repository: Arc<CR>, image_repository: Arc<IR>, bot_client: Arc<BC>) -> Self {
        Self {
            card_repository,
            image_repository,
            bot_client,
        }
    }
}

#[async_trait]
impl<
        CR: CardRepository<Error = impl Debug + Send>,
        IR: ImageRepository<Error = impl Debug + Send>,
        BC: BotClient<Error = impl Debug + Send>,
    > Cron for CronImpl<CR, IR, BC>
{
    async fn run(self: Arc<Self>) -> () {
        let sched = JobScheduler::new().await.unwrap();
        sched
            .add(
                Job::new_async("0 * * * * * *", move |_uuid, _l| {
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
    CR: CardRepository<Error = impl Debug + Send>,
    IR: ImageRepository<Error = impl Debug + Send>,
    BC: BotClient<Error = impl Debug + Send>,
>(
    card_repository: Arc<CR>,
    image_repository: Arc<IR>,
    bot_client: Arc<BC>,
) {
    use indoc::formatdoc;
    let now = Utc::now();
    let start = now;
    let end = now + Duration::minutes(1);
    let Ok(cards_with_channels) = card_repository
        .get_card_with_channels_by_date(start, end)
        .await
        .map_err(|e| {
            eprintln!("failed to get cards: {:?}", e);
        })
    else {
        return;
    };
    let sends = cards_with_channels.iter().map(|(card, channels)| async {
        let sends = channels.iter().map(|channel| async {
            let Ok(png) = image_repository.get_png(card.id).await.map_err(|e| {
                eprintln!("failed to get png: {:?}", e);
            }) else {
                return;
            };
            let Some(png) = png else {
                eprintln!("png not found");
                return;
            };
            let Ok(file_id) = bot_client
                .uplodad_file(&UploadFileParams {
                    id: card.id,
                    channel_id: channel.id,
                    content: png,
                    mime_type: "image/png".to_string(),
                })
                .await
                .map(|f| f.id)
                .map_err(|e| {
                    eprintln!("failed to upload file: {:?}", e);
                })
            else {
                return;
            };
            let Ok(user) = bot_client
                .get_user(&card.owner_id.to_string())
                .await
                .map_err(|e| {
                    eprintln!("failed to get user: {:?}", e);
                })
            else {
                return;
            };
            let message = match card.message.as_ref() {
                Some(m) => formatdoc! {
                    r##"
                        !{{"type":"user","raw":"@{}","id":"{}"}} からのQardです！
                        {}

                        https://q.trap.jp/files/{}
                    "##,
                    user.name, user.id, m, file_id
                },
                None => formatdoc! {
                    r##"
                        !{{"type":"user","raw":"@{}","id":"{}"}} からのQardです！

                        https://q.trap.jp/files/{}
                    "##,
                    user.name, user.id, file_id
                },
            };
            let _ = bot_client
                .post_message(&PostMessageParams {
                    content: message,
                    channel_id: channel.id,
                    embed: false,
                })
                .await
                .map_err(|e| {
                    eprintln!("failed to post message: {:?}", e);
                });
        });
        join_all(sends).await
    });
    join_all(sends).await;
}
