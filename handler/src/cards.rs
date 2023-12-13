use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CardResponse {
    // TODO: UUID
    pub id: String,
    // TOOD: UUID
    pub owner_id: String,
    // TODO: Datetime
    pub publish_date: String,
    // TODO: Vec<Uuid>
    pub publish_channels: Vec<String>,
    pub message: Option<String>,
    // TODO: Vec<Uuid>
    pub images: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CardRequest {
    // TOOD: UUID
    pub owner_id: String,
    // TODO: Datetime
    pub publish_date: String,
    // TODO: Vec<Uuid>
    pub publish_channels: Vec<String>,
    pub message: Option<String>,
}

async fn mock_card_response() -> CardResponse {
    CardResponse {
        id: "89d136ad-1ba2-4974-a44a-cc9b5c8c0670".to_string(),
        owner_id: "56df9d96-19b7-4f7a-8695-b157ccb483fa".to_string(),
        publish_date: "2023-12-13T08:10:05Z".to_string(),
        publish_channels: vec!["0ccb58b0-5300-4842-a7e6-b19c674f7090".to_string()],
        message: None,
        images: vec![],
    }
}

#[rocket::get("/")]
pub async fn get_all() -> (Status, Json<Vec<CardResponse>>) {
    // TODO: まだモックなので実装
    let v = vec![mock_card_response().await];
    (Status::Accepted, Json(v))
}

#[rocket::post("/", data = "<card>")]
pub async fn post(card: Json<CardRequest>) -> String {
    println!("request card: {:?}", card.0);
    "3e20b0e0-5672-4645-bf49-a2b69eafefc6".to_string()
}

#[rocket::get("/me")]
pub async fn get_mine() -> (Status, Json<Vec<CardResponse>>) {
    (Status::Ok, Json(vec![]))
}

#[rocket::get("/<id>")]
pub async fn get_one(id: String) -> (Status, Option<Json<CardResponse>>) {
    let mc = mock_card_response().await;
    if id != mc.id {
        (Status::NotFound, None)
    } else {
        (Status::Ok, Some(Json(mc)))
    }
}

#[rocket::delete("/<id>")]
pub async fn delete_one(id: String) -> Status {
    println!("delete card id={id}");
    Status::NoContent
}

pub fn routes() -> Vec<Route> {
    rocket::routes![get_all, post, get_mine, get_one, delete_one]
}
