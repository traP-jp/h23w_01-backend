use domain::repository::PublishChannelModel;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "publish_channel")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub card_id: Uuid,
}

impl From<PublishChannelModel> for Model {
    fn from(value: PublishChannelModel) -> Self {
        let PublishChannelModel { id, card_id } = value;
        Self { id, card_id }
    }
}

impl From<Model> for PublishChannelModel {
    fn from(value: Model) -> Self {
        let Model { id, card_id } = value;
        Self { id, card_id }
    }
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
