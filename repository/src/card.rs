use entity::prelude::*;
use sea_orm::{
    prelude::DateTimeUtc, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, TransactionTrait,
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait CardRepository {
    async fn save_card(&self, params: &SaveCardParams) -> Result<(), DbErr>;
    async fn save_image(&self, params: &SaveImageParams) -> Result<(), DbErr>;
    async fn save_png(&self, card_id: Uuid, content: &[u8]) -> Result<(), DbErr>;
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), DbErr>;
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, DbErr>;
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, DbErr>;
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, DbErr>;
    async fn delete_card(&self, card_id: Uuid) -> Result<(), DbErr>;
}

pub struct CardRepositoryImpl(DatabaseConnection);
impl CardRepositoryImpl {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self(db.clone())
    }
}

#[async_trait::async_trait]
impl CardRepository for CardRepositoryImpl {
    async fn save_card(&self, params: &SaveCardParams) -> Result<(), DbErr> {
        // TODO: 画像の保存
        let db = &self.0;
        let tx = db.begin().await?;
        let card = CardActiveModel {
            id: ActiveValue::Set(params.id),
            owner_id: ActiveValue::Set(params.owner_id),
            publish_date: ActiveValue::Set(params.publish_date.naive_utc()),
            message: ActiveValue::Set(params.message.clone()),
        };
        let channels = params
            .channels
            .iter()
            .map(|channel_id| PublishChannelActiveModel {
                id: ActiveValue::Set(*channel_id),
                card_id: ActiveValue::Set(card.id.clone().unwrap()),
            })
            .collect::<Vec<_>>();
        Card::insert(card).exec(&tx).await?;
        PublishChannel::insert_many(channels).exec(&tx).await?;

        tx.commit().await?;
        Ok(())
    }
    async fn save_image(&self, params: &SaveImageParams) -> Result<(), DbErr> {
        let db = &self.0;
        let tx = db.begin().await?;
        let image = ImageActiveModel {
            id: ActiveValue::Set(params.id),
            mime_type: ActiveValue::Set(params.mime_type.clone()),
            content: ActiveValue::Set(params.content.clone()),
        };
        Image::insert(image).exec(&tx).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn get_all_cards(&self) -> Result<Vec<CardModel>, DbErr> {
        let db = &self.0;
        let cards = Card::find().all(db).await?;
        Ok(cards)
    }

    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<CardModel>, DbErr> {
        let db = &self.0;
        let cards = Card::find()
            .filter(CardColumn::Id.contains(user_id))
            .all(db)
            .await?;
        Ok(cards)
    }
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Option<CardModel>, DbErr> {
        let db = &self.0;
        let card = Card::find_by_id(card_id).one(db).await?;
        Ok(card)
    }
    async fn delete_card(&self, card_id: Uuid) -> Result<(), DbErr> {
        let db = &self.0;
        Card::delete_by_id(card_id).exec(db).await?;
        Ok(())
    }
    async fn save_png(&self, card_id: Uuid, content: &[u8]) -> Result<(), DbErr> {
        let db = &self.0;
        let tx = db.begin().await?;
        let card_png = CardPngActiveModel {
            card_id: ActiveValue::Set(card_id),
            content: ActiveValue::Set(content.to_vec()),
        };
        CardPng::insert(card_png).exec(&tx).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), DbErr> {
        let db = &self.0;
        let tx = db.begin().await?;
        let card_svg = CardSvgActiveModel {
            card_id: ActiveValue::Set(card_id),
            content: ActiveValue::Set(content.to_string()),
        };
        CardSvg::insert(card_svg).exec(&tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

pub struct SaveCardParams {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub message: Option<String>,
    pub channels: Vec<Uuid>,
}

pub struct SaveImageParams {
    pub id: Uuid,
    pub mime_type: String,
    pub content: Vec<u8>,
}