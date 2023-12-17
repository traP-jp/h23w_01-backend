use async_trait::async_trait;
use bytes::Bytes;
use mockall::automock;
use shaku::Interface;
pub use traq::models::{ChannelList, Stamp, User, UserDetail};

#[automock(type Error = String;)]
#[async_trait]
pub trait BotClient: Interface {
    type Error;

    async fn get_stamp_image(&self, stamp_id: &str) -> Result<ImageData, Self::Error>;
    async fn get_stamps(&self, r#type: StampType) -> Result<Vec<Stamp>, Self::Error>;
    async fn get_users(&self) -> Result<Vec<User>, Self::Error>;
    async fn get_user(&self, id: &str) -> Result<UserDetail, Self::Error>;
    async fn get_user_icon(&self, id: &str) -> Result<ImageData, Self::Error>;
    async fn get_channels(&self) -> Result<ChannelList, Self::Error>;
}

#[derive(Debug, Clone)]
pub enum ImageData {
    Svg(String),
    Png(Bytes),
    Gif(Bytes),
    Jpeg(Bytes),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StampType {
    Original,
    Unicode,
    None,
}
