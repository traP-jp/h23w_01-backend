use entity::prelude::*;
use sea_orm::{Database, EntityTrait};

const DATABASE_URL: &str = "mysql://root:pass@localhost:3306";
const DB_NAME: &str = "/db";

async fn foo() {
    let db = Database::connect(DATABASE_URL.to_owned() + DB_NAME)
        .await
        .unwrap();
    let card = PublishChannel::find_by_id("00000000-0000-0000-0000-000000000000")
        .find_with_related(Card)
        .all(&db)
        .await
        .unwrap();
    println!("{:?}", card);
}
