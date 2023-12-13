use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "card")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(Some(16)))"
    )]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTime,
    pub message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::publish_channel::Entity")]
    PublishChannel,
}

impl Related<super::publish_channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PublishChannel.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
