use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use domain::repository::CardModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "card")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub message: Option<String>,
}

impl From<CardModel> for Model {
    fn from(value: CardModel) -> Self {
        let CardModel {
            id,
            owner_id,
            publish_date,
            message,
        } = value;
        Self {
            id,
            owner_id,
            publish_date,
            message,
        }
    }
}

impl From<Model> for CardModel {
    fn from(value: Model) -> Self {
        let Model {
            id,
            owner_id,
            publish_date,
            message,
        } = value;
        Self {
            id,
            owner_id,
            publish_date,
            message,
        }
    }
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
