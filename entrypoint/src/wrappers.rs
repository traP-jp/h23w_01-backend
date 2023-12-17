use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;

use domain::bot_client::{
    BotClient, ChannelList, ImageData, PostMessageParams, Stamp, StampType, UploadFileParams,
    UploadFileResp, User, UserDetail,
};
use domain::repository::{
    CardModel, CardRepository, DateTimeUtc, ImageRepository, MigrationStrategy,
    PublishChannelModel, SaveCardParams,
};

pub struct BotClientWrapper<T: BotClient>(pub T);

#[async_trait]
impl<E, T: BotClient<Error = E>> BotClient for BotClientWrapper<T>
where
    anyhow::Error: From<E>,
{
    type Error = anyhow::Error;

    async fn get_stamps(&self, stamp_type: StampType) -> anyhow::Result<Vec<Stamp>> {
        Ok(self.0.get_stamps(stamp_type).await?)
    }
    async fn get_stamp_image(&self, stamp_id: &str) -> anyhow::Result<ImageData> {
        Ok(self.0.get_stamp_image(stamp_id).await?)
    }
    async fn get_users<'a>(&'a self, name: Option<&'a str>) -> anyhow::Result<Vec<User>> {
        Ok(self.0.get_users(name).await?)
    }
    async fn get_user(&self, user_id: &str) -> anyhow::Result<UserDetail> {
        Ok(self.0.get_user(user_id).await?)
    }
    async fn get_user_icon(&self, user_id: &str) -> anyhow::Result<ImageData> {
        Ok(self.0.get_user_icon(user_id).await?)
    }
    async fn get_channels(&self) -> anyhow::Result<ChannelList> {
        Ok(self.0.get_channels().await?)
    }
    async fn post_message(&self, params: &PostMessageParams) -> Result<(), Self::Error> {
        Ok(self.0.post_message(params).await?)
    }
    async fn uplodad_file(&self, params: &UploadFileParams) -> Result<UploadFileResp, Self::Error> {
        Ok(self.0.uplodad_file(params).await?)
    }
}

pub struct CardRepositoryWrapper<T: CardRepository>(pub T);

#[async_trait]
impl<E, T: CardRepository<Error = E>> CardRepository for CardRepositoryWrapper<T>
where
    anyhow::Error: From<E>,
{
    type Error = anyhow::Error;

    async fn migrate(&self, strategy: MigrationStrategy) -> Result<(), Self::Error> {
        Ok(self.0.migrate(strategy).await?)
    }
    async fn save_card(&self, params: &SaveCardParams) -> Result<(), Self::Error> {
        Ok(self.0.save_card(params).await?)
    }
    async fn update_card(&self, params: &SaveCardParams) -> Result<Option<()>, Self::Error> {
        Ok(self.0.update_card(params).await?)
    }
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, Self::Error> {
        Ok(self.0.get_all_cards().await?)
    }
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, Self::Error> {
        Ok(self.0.get_my_cards(user_id).await?)
    }
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, Self::Error> {
        Ok(self.0.get_card_by_id(card_id).await?)
    }
    async fn get_publish_channels_by_id(&self, card_id: Uuid) -> Result<Vec<Uuid>, Self::Error> {
        Ok(self.0.get_publish_channels_by_id(card_id).await?)
    }
    async fn delete_publish_channel(
        &self,
        card_id: Uuid,
        channel_id: Uuid,
    ) -> Result<Option<()>, Self::Error> {
        Ok(self.0.delete_publish_channel(card_id, channel_id).await?)
    }
    async fn delete_card(&self, card_id: Uuid) -> Result<Option<()>, Self::Error> {
        Ok(self.0.delete_card(card_id).await?)
    }
    async fn get_card_with_channels_by_date(
        &self,
        start: DateTimeUtc,
        end: DateTimeUtc,
    ) -> Result<Vec<(CardModel, Vec<PublishChannelModel>)>, Self::Error> {
        Ok(self.0.get_card_with_channels_by_date(start, end).await?)
    }
}

pub struct ImageRepositoryWrapper<T: ImageRepository>(pub T);

#[async_trait]
impl<E, T: ImageRepository<Error = E>> ImageRepository for ImageRepositoryWrapper<T>
where
    anyhow::Error: From<E>,
{
    type Error = anyhow::Error;

    async fn save_png(&self, card_id: Uuid, content: &Bytes) -> Result<(), Self::Error> {
        Ok(self.0.save_png(card_id, content).await?)
    }
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), Self::Error> {
        Ok(self.0.save_svg(card_id, content).await?)
    }
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &Bytes,
    ) -> Result<(), Self::Error> {
        Ok(self.0.save_asset(id, mime_type, content).await?)
    }
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Bytes>, Self::Error> {
        Ok(self.0.get_png(card_id).await?)
    }
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, Self::Error> {
        Ok(self.0.get_svg(card_id).await?)
    }
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Bytes)>, Self::Error> {
        Ok(self.0.get_asset(id).await?)
    }
    async fn delete_png(&self, card_id: Uuid) -> Result<(), Self::Error> {
        Ok(self.0.delete_png(card_id).await?)
    }
    async fn delete_svg(&self, card_id: Uuid) -> Result<(), Self::Error> {
        Ok(self.0.delete_svg(card_id).await?)
    }
    async fn delete_asset(&self, id: Uuid) -> Result<(), Self::Error> {
        Ok(self.0.delete_asset(id).await?)
    }
}
