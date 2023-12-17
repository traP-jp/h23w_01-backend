use async_trait::async_trait;
use bytes::Bytes;
use mockall::automock;
use shaku::Interface;
pub use traq::models::{Channel, ChannelList, FileInfo, Stamp, User, UserDetail};
use uuid::Uuid;

#[automock(type Error = String;)]
#[async_trait]
pub trait BotClient: Interface {
    type Error;

    async fn get_stamp_image(&self, stamp_id: &str) -> Result<ImageData, Self::Error>;
    async fn get_stamps(&self, r#type: StampType) -> Result<Vec<Stamp>, Self::Error>;
    async fn get_users<'a>(&'a self, name: Option<&'a str>) -> Result<Vec<User>, Self::Error>;
    async fn get_user(&self, id: &str) -> Result<UserDetail, Self::Error>;
    async fn get_user_icon(&self, id: &str) -> Result<ImageData, Self::Error>;
    async fn get_channels(&self) -> Result<ChannelList, Self::Error>;
    async fn post_message(&self, params: &PostMessageParams) -> Result<(), Self::Error>;
    async fn uplodad_file(&self, params: &UploadFileParams) -> Result<UploadFileResp, Self::Error>;
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

#[derive(Debug, Clone)]
pub struct PostMessageParams {
    pub channel_id: Uuid,
    pub content: String,
    pub embed: bool,
}

#[derive(Debug, Clone)]
pub struct UploadFileParams {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub content: Bytes,
    pub mime_type: String,
}

#[derive(Debug, Clone)]
pub struct UploadFileResp {
    pub id: Uuid,
}
