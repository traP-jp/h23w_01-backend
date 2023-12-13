use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "card_content")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(Some(16)))"
    )]
    pub id: Uuid,
    pub svg: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
