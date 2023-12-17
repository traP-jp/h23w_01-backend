use async_trait::async_trait;
use bytes::Bytes;
use mockall::automock;
use serde::{Deserialize, Serialize};
use shaku::Interface;
use uuid::Uuid;

#[automock(type Error = String;)]
#[async_trait]
pub trait CardRepository: Interface {
    type Error;

    async fn migrate(&self, strategy: MigrationStrategy) -> Result<(), Self::Error>;
    async fn get_card_with_channels_by_date(
        &self,
        start: DateTimeUtc,
        end: DateTimeUtc,
    ) -> Result<Vec<(CardModel, Vec<PublishChannelModel>)>, Self::Error>;
    async fn save_card(&self, params: &SaveCardParams) -> Result<(), Self::Error>;
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, Self::Error>;
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, Self::Error>;
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, Self::Error>;
    async fn get_publish_channels_by_id(&self, card_id: Uuid) -> Result<Vec<Uuid>, Self::Error>;
    async fn delete_publish_channel(
        &self,
        card_id: Uuid,
        channel_id: Uuid,
    ) -> Result<Option<()>, Self::Error>;
    async fn delete_card(&self, card_id: Uuid) -> Result<Option<()>, Self::Error>;
}

pub type DateTimeUtc = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MigrationStrategy {
    Up,
    Down,
    Refresh,
    #[default]
    None,
}

impl std::str::FromStr for MigrationStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "refresh" => Ok(Self::Refresh),
            "none" => Ok(Self::None),
            s => Err(format!("unknown strategy `{}`", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardModel {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishChannelModel {
    pub id: Uuid,
    pub card_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SaveCardParams {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub message: Option<String>,
    pub channels: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct SaveImageParams {
    pub id: Uuid,
    pub mime_type: String,
    pub content: Vec<u8>,
}

#[automock(type Error = String;)]
#[async_trait]
pub trait ImageRepository: Interface {
    type Error;
    async fn save_png(&self, card_id: Uuid, content: &Bytes) -> Result<(), Self::Error>;
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), Self::Error>;
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &Bytes,
    ) -> Result<(), Self::Error>;
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Bytes>, Self::Error>;
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, Self::Error>;
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Bytes)>, Self::Error>;
    async fn delete_png(&self, card_id: Uuid) -> Result<(), Self::Error>;
    async fn delete_svg(&self, card_id: Uuid) -> Result<(), Self::Error>;
    async fn delete_asset(&self, id: Uuid) -> Result<(), Self::Error>;
}
