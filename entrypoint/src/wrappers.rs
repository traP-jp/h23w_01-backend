use async_trait::async_trait;

use domain::bot_client::{BotClient, ChannelList, ImageData, Stamp, StampType, User, UserDetail};

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
    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        Ok(self.0.get_users().await?)
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
}
