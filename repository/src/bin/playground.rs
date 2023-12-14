use chrono::Utc;
use entity::prelude::*;
use sea_orm::{
    prelude::{DateTimeUtc, Uuid},
    ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use std::str::FromStr;

const DATABASE_URL: &str = "mysql://root:pass@localhost:3306";
const DB_NAME: &str = "/db";
#[tokio::main]
async fn main() {
    let db = Database::connect(DATABASE_URL.to_owned() + DB_NAME)
        .await
        .unwrap();
    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    save_card(
        &db,
        id1,
        owner_id,
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
        id2,
        owner_id,
        Utc::now(),
        Some("hge".to_owned()),
        &[
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000003").unwrap(),
            Uuid::from_str("00000000-0000-0000-0000-000000000004").unwrap(),
        ],
    )
    .await;
    println!("{:?}", get_all_cards(&db).await);
    println!("{:?}", get_card_with_channel_and_image(&db, id1).await);
    println!("{:?}", get_card_with_channel_and_image(&db, id1).await);
}

async fn save_card(
    db: &DatabaseConnection,
    id: Uuid,
    owner_id: Uuid,
    publish_date: DateTimeUtc,
    message: Option<String>,
    channels: &[Uuid],
) {
    let tx = db.begin().await.unwrap();
    let card = CardActiveModel {
        id: ActiveValue::Set(id),
        owner_id: ActiveValue::Set(owner_id),
        publish_date: ActiveValue::Set(publish_date.naive_local()),
        message: ActiveValue::Set(message),
    };
    let channels = channels
        .iter()
        .map(|channel_id| PublishChannelActiveModel {
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

async fn get_all_cards(db: &DatabaseConnection) -> Vec<CardModel> {
    Card::find().all(db).await.unwrap()
}

async fn get_card_with_channel_and_image(
    db: &DatabaseConnection,
    card_id: Uuid,
) -> Option<(
    CardModel,
    Vec<PublishChannelModel>,
    Vec<CardSvgModel>,
    Vec<CardPngModel>,
)> {
    let card: Option<CardModel> = Card::find_by_id(card_id).one(db).await.unwrap();
    card.as_ref()?;
    let card = card.unwrap();
    let channels: Vec<PublishChannelModel> = PublishChannel::find()
        .filter(PublishChannelColumn::CardId.eq(card_id))
        .all(db)
        .await
        .unwrap();
    let svgs: Vec<CardSvgModel> = CardSvg::find()
        .filter(CardSvgColumn::CardId.eq(card_id))
        .all(db)
        .await
        .unwrap();
    let pngs: Vec<CardPngModel> = CardPng::find()
        .filter(CardPngColumn::CardId.eq(card_id))
        .all(db)
        .await
        .unwrap();
    Some((card, channels, svgs, pngs))
}
