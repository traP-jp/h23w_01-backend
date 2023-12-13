use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "publish_channel")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(Some(16)))"
    )]
    pub id: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(Some(16)))")]
    pub card_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::card::Entity",
        from = "Column::CardId",
        to = "super::card::Column::Id"
    )]
    Card,
}

impl Related<super::card::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Card.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
