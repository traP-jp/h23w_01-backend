use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::time::{sleep, Duration};
use traq::models::UserAccountState;
use uuid::{uuid, Uuid};

use domain::bot_client::{
    BotClient, ChannelList, ImageData, PostMessageParams, Stamp, StampType, UploadFileParams,
    UploadFileResp, User, UserDetail,
};
use domain::cron::Cron;
use domain::repository::ImageRepository;
use domain::repository::{
    CardModel, CardRepository, DateTimeUtc, MigrationStrategy, PublishChannelModel, SaveCardParams,
};

use cron::CronImpl;

#[derive(Clone, Copy, Debug, Default)]
struct MockBotClient;

impl MockBotClient {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BotClient for MockBotClient {
    type Error = anyhow::Error;

    async fn get_stamp_image(&self, _stamp_id: &str) -> anyhow::Result<ImageData> {
        Err(anyhow::anyhow!("unsupported"))
    }

    async fn get_stamps(&self, _type: StampType) -> anyhow::Result<Vec<Stamp>> {
        Ok(vec![])
    }

    async fn get_users<'a>(&'a self, _name: Option<&'a str>) -> anyhow::Result<Vec<User>> {
        Ok(vec![])
    }

    async fn get_user(&self, id: &str) -> anyhow::Result<UserDetail> {
        println!("get_user: {}", id);
        Ok(UserDetail {
            id: uuid!("00000000-0000-0000-0000-000000000000"),
            state: UserAccountState::Active,
            bot: false,
            icon_file_id: uuid!("00000000-0000-0000-0000-000000000000"),
            display_name: "test".to_string(),
            name: "test".to_string(),
            twitter_id: "test".to_string(),
            last_online: None,
            updated_at: "2021-01-01T00:00:00.000Z".to_string(),
            tags: vec![],
            groups: vec![],
            bio: "test".to_string(),
            home_channel: None,
        })
    }

    async fn get_user_icon(&self, _id: &str) -> anyhow::Result<ImageData> {
        Err(anyhow::anyhow!("unsupported"))
    }

    async fn get_channels(&self) -> anyhow::Result<ChannelList> {
        Err(anyhow::anyhow!("unsupported"))
    }

    async fn post_message(&self, params: &PostMessageParams) -> anyhow::Result<()> {
        println!("post_message: {:?}", params);
        Ok(())
    }

    async fn uplodad_file(&self, params: &UploadFileParams) -> anyhow::Result<UploadFileResp> {
        println!("upload_file: {:?}", params);
        Ok(UploadFileResp {
            id: uuid!("00000000-0000-0000-0000-000000000000"),
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MockCardRepository;

impl MockCardRepository {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CardRepository for MockCardRepository {
    type Error = anyhow::Error;

    async fn migrate(&self, _strategy: MigrationStrategy) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_card_with_channels_by_date(
        &self,
        start: DateTimeUtc,
        end: DateTimeUtc,
    ) -> Result<Vec<(CardModel, Vec<PublishChannelModel>)>, Self::Error> {
        println!("get_card_with_channels_by_date: {:?} - {:?}", start, end);
        let channels = vec![PublishChannelModel {
            id: uuid!("00000000-0000-0000-0000-000000000000"),
            card_id: uuid!("00000000-0000-0000-0000-000000000000"),
        }];
        let card = CardModel {
            id: uuid!("00000000-0000-0000-0000-000000000000"),
            owner_id: uuid!("00000000-0000-0000-0000-000000000000"),
            publish_date: start,
            message: None,
        };
        Ok(vec![(card, channels)])
    }
    async fn save_card(&self, _params: &SaveCardParams) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn update_card(&self, _params: &SaveCardParams) -> Result<Option<()>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_my_cards(&self, _user_id: Uuid) -> Result<Vec<CardModel>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_card_by_id(&self, _card_id: Uuid) -> Result<Option<CardModel>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_publish_channels_by_id(&self, _card_id: Uuid) -> Result<Vec<Uuid>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn delete_publish_channel(
        &self,
        _card_id: Uuid,
        _channel_id: Uuid,
    ) -> Result<Option<()>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn delete_card(&self, _card_id: Uuid) -> Result<Option<()>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MockImageRepository;

impl MockImageRepository {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ImageRepository for MockImageRepository {
    type Error = anyhow::Error;

    async fn save_png(&self, _card_id: Uuid, _content: &Bytes) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn save_svg(&self, _card_id: Uuid, _content: &str) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn save_asset(
        &self,
        _id: Uuid,
        _mime_type: &str,
        _content: &Bytes,
    ) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Bytes>, Self::Error> {
        println!("get_png: {:?}", card_id);
        Ok(Some(Bytes::from("png")))
    }
    async fn get_svg(&self, _card_id: Uuid) -> Result<Option<String>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn get_asset(&self, _id: Uuid) -> Result<Option<(String, Bytes)>, Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn delete_png(&self, _card_id: Uuid) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn delete_svg(&self, _card_id: Uuid) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
    async fn delete_asset(&self, _id: Uuid) -> Result<(), Self::Error> {
        Err(anyhow::anyhow!("unsupported"))
    }
}

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
