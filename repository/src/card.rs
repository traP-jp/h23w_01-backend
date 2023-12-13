use entity::prelude::*;
use sea_orm::{prelude::DateTimeLocal, DatabaseConnection, DbErr};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait CardRepository {
    async fn insert_card(&self, card: &Card) -> Result<Card, DbErr>;
    async fn get_all_cards(&self) -> Result<Vec<Card>, DbErr>;
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<Card>, DbErr>;
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Card, DbErr>;
    async fn delete_card(&self, card_id: Uuid) -> Result<(), DbErr>;
}

pub struct CardRepositoryImpl(DatabaseConnection);

impl CardRepository for CardRepositoryImpl {
    async fn get_all_cards(&self) -> Result<Vec<Card>> {
        let db = &self.0;
        let cards = Card::find().all(db).await?;
        Ok(cards)
    }
    async fn insert_card(&self, card: &Card) -> Result<Card> {
        let db = &self.0;
        let card = card.clone().insert(db).await?;
        Ok(card)
    }
    async fn get_my_cards(&self, user_id: Uuid) -> Result<Vec<Card>> {
        let db = &self.0;
        let cards = Card::find()
            .filter(card::Column::UserId.contains(user_id))
            .all(db)
            .await?;
        Ok(cards)
    }
    async fn get_card_by_id(&self, card_id: Uuid) -> Result<Card> {
        let db = &self.0;
        let card = Card::find_by_id(card_id).one(db).await?;
        Ok(card)
    }
    async fn delete_card(&self, card_id: Uuid) -> Result<(), DbErr> {
        let db = &self.0;
        Card::find_by_id(card_id).delete(db).await?;
        Ok(())
    }
}
