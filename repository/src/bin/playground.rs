use chrono::Utc;
use entity::prelude::*;
use repository::{
    card::{CardRepository, CardRepositoryImpl, SaveCardParams},
    image::{ImageRepository, ImageRepositoryImpl},
};
use s3::{creds::Credentials, Bucket, Region};
use sea_orm::{
    prelude::{DateTimeUtc, Uuid},
    ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};

const DATABASE_URL: &str = "mysql://root:pass@localhost:3306";
const DB_NAME: &str = "/db";

#[tokio::main]
async fn main() {
    test_db().await;
    test_object_storage().await;
}

async fn test_object_storage() {
    let account_id = "".to_string();
    let access_key = Some("");
    let secret_key = Some("");

    let bucket = Bucket::new(
        "h23w1",
        Region::R2 { account_id },
        Credentials::new(access_key, secret_key, None, None, None).unwrap(),
    )
    .unwrap();
    let card_id = Uuid::new_v4();
    let image_repo = ImageRepositoryImpl::new(&bucket);
    let sample_svg = "<svg></svg>";
    image_repo.save_svg(card_id, sample_svg).await.unwrap();
    let svg = image_repo.get_svg(card_id).await.unwrap().unwrap();
    assert_eq!(svg, sample_svg);
}

async fn test_db() {
    let db = Database::connect(DATABASE_URL.to_string() + DB_NAME)
        .await
        .unwrap();
    let repo = CardRepositoryImpl::new(&db);
    let id = Uuid::new_v4();
    repo.save_card(&SaveCardParams {
        id,
        owner_id: Uuid::new_v4(),
        publish_date: Utc::now(),
        message: Some("Hello".to_string()),
        channels: vec![Uuid::new_v4(), Uuid::new_v4()],
    })
    .await
    .unwrap();
    println!("{:?}", repo.get_card_by_id(id).await);
}
