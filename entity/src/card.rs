use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "card")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTime,
    pub message: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::publish_channel::Entity")]
    PublishChannel,
    #[sea_orm(has_many = "super::card_svg::Entity")]
    CardSvg,
    #[sea_orm(has_many = "super::card_png::Entity")]
    CardPng,
}

impl Related<super::publish_channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PublishChannel.def()
    }
}

impl Related<super::card_svg::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardSvg.def()
    }
}

impl Related<super::card_png::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CardPng.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
