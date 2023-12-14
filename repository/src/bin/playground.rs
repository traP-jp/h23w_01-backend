use std::str::FromStr;

use chrono::Utc;
use entity::{
    card,
    prelude::*,
    publish_channel::{self},
};
use sea_orm::{
    prelude::{DateTimeUtc, Uuid},
    ActiveValue, Database, DatabaseConnection, EntityTrait, TransactionTrait,
};

const DATABASE_URL: &str = "mysql://root:pass@localhost:3306";
const DB_NAME: &str = "/db";
#[tokio::main]
async fn main() {
    let db = Database::connect(DATABASE_URL.to_owned() + DB_NAME)
        .await
        .unwrap();
    save_card(
        &db,
        Uuid::new_v4(),
        Utc::now(),
        Some("oisu---".to_owned()),
        &[
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
        ],
    )
    .await;
    save_card(
        &db,
        Uuid::new_v4(),
        Utc::now(),
        Some("hge".to_owned()),
        &[
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000003").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000004").unwrap(),
        ],
    )
    .await;
    let cards = search_card_from_channel(
        &db,
        Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
    )
    .await;
    println!("{:?}", cards);
}

async fn save_card(
    db: &DatabaseConnection,
    owner_id: Uuid,
    publish_date: DateTimeUtc,
    message: Option<String>,
    channels: &[Uuid],
) {
    let tx = db.begin().await.unwrap();
    let card = card::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        owner_id: ActiveValue::Set(owner_id),
        publish_date: ActiveValue::Set(publish_date.naive_local()),
        message: ActiveValue::Set(message),
    };
    let channels = channels
        .iter()
        .map(|channel_id| publish_channel::ActiveModel {
            id: ActiveValue::Set(*channel_id),
            card_id: ActiveValue::Set(card.id.clone().unwrap()),
        })
        .collect::<Vec<_>>();
    Card::insert(card).exec(&tx).await.unwrap();
    PublishChannel::insert_many(channels)
        .exec(&tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
}

async fn search_card_from_channel(db: &DatabaseConnection, channel_id: Uuid) -> Vec<card::Model> {
    let cards = PublishChannel::find_by_id(channel_id)
        .find_with_related(Card)
        .all(db)
        .await
        .unwrap();
    cards
        .iter()
        .flat_map(|(_a, b)| b.clone())
        .collect::<Vec<_>>()
}
