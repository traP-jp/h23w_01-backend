use async_trait::async_trait;
use bytes::Bytes;
use rocket::data::{Data, FromData, Outcome, ToByteUnit};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Request, Route};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;

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
    // TODO: Vec<Uuid>
    pub images: Vec<String>,
}

async fn mock_card_response() -> CardResponse {
    CardResponse {
        id: "89d136ad-1ba2-4974-a44a-cc9b5c8c0670".to_string(),
        owner_id: "56df9d96-19b7-4f7a-8695-b157ccb483fa".to_string(),
        publish_date: "2023-12-13T08:10:05Z".to_string(),
        publish_channels: vec!["0ccb58b0-5300-4842-a7e6-b19c674f7090".to_string()],
        message: None,
    }
}

#[derive(Debug, Clone)]
pub struct Svg(String);

#[async_trait]
impl<'a> FromData<'a> for Svg {
    type Error = String;

    async fn from_data(req: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self> {
        let Some(content_type) = req.content_type() else {
            return Outcome::Error((
                Status::BadRequest,
                "content-type must be specified".to_string(),
            ));
        };
        if !content_type.is_svg() {
            return Outcome::Error((
                Status::BadRequest,
                format!(
                    "expected image/svg+xml as content-type, found {}",
                    content_type
                ),
            ));
        }
        let Ok(data) = data.open(5.megabytes()).into_string().await else {
            return Outcome::Error((
                Status::InternalServerError,
                "failed to read request body".to_string(),
            ));
        };
        Outcome::Success(Svg(data.into_inner()))
    }
}

#[derive(Debug, Clone)]
pub struct Png(Bytes);

#[async_trait]
impl<'a> FromData<'a> for Png {
    type Error = String;

    async fn from_data(req: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self> {
        let Some(content_type) = req.content_type() else {
            return Outcome::Error((
                Status::BadRequest,
                "content-type must be specified".to_string(),
            ));
        };
        if !content_type.is_png() {
            return Outcome::Error((
                Status::BadRequest,
                format!("expected image/png as content-type, found {}", content_type),
            ));
        }
        let Ok(data) = data.open(5.megabytes()).into_bytes().await else {
            return Outcome::Error((
                Status::InternalServerError,
                "failed to read request body".to_string(),
            ));
        };
        Outcome::Success(Png(data.into_inner().into()))
    }
}

#[rocket::get("/")]
pub async fn get_all(_user: AuthUser<'_>) -> (Status, Json<Vec<CardResponse>>) {
    // TODO: まだモックなので実装
    let v = vec![mock_card_response().await];
    (Status::Ok, Json(v))
}

#[rocket::post("/", data = "<card>")]
pub async fn post(card: Json<CardRequest>, _user: AuthUser<'_>) -> String {
    println!("request card: {:?}", card.0);
    "3e20b0e0-5672-4645-bf49-a2b69eafefc6".to_string()
}

#[rocket::get("/me")]
pub async fn get_mine(_user: AuthUser<'_>) -> (Status, Json<Vec<CardResponse>>) {
    (Status::Ok, Json(vec![]))
}

#[rocket::get("/<id>")]
pub async fn get_one(id: String, _user: AuthUser<'_>) -> (Status, Option<Json<CardResponse>>) {
    let mc = mock_card_response().await;
    if id != mc.id {
        (Status::NotFound, None)
    } else {
        (Status::Ok, Some(Json(mc)))
    }
}

#[rocket::delete("/<id>")]
pub async fn delete_one(id: String, _user: AuthUser<'_>) -> Status {
    println!("delete card id={id}");
    Status::NoContent
}

const CARD_ID: &str = "89d136ad-1ba2-4974-a44a-cc9b5c8c0670";

#[rocket::get("/<id>/svg")]
pub async fn get_svg(id: String, _user: AuthUser<'_>) -> (Status, Option<NamedFile>) {
    println!("get image.svg {}", id);
    if id != CARD_ID {
        return (Status::NotFound, None);
    }
    (
        Status::Ok,
        NamedFile::open("./mock-assets/sample.svg").await.ok(),
    )
}

#[rocket::post("/<id>/svg", data = "<svg>")]
pub async fn post_svg(svg: Svg, id: String, _user: AuthUser<'_>) -> Status {
    println!("post image.svg {} with size {}", id, svg.0.len());
    Status::NoContent
}

#[rocket::patch("/<id>/svg", data = "<svg>")]
pub async fn patch_svg(svg: Svg, id: String, _user: AuthUser<'_>) -> Status {
    println!("patch image.svg {} with size {}", id, svg.0.len());
    Status::NoContent
}

#[rocket::get("/<id>/png")]
pub async fn get_png(id: String, _user: AuthUser<'_>) -> (Status, Option<NamedFile>) {
    println!("get image.png {}", id);
    if id != CARD_ID {
        return (Status::NotFound, None);
    }
    (
        Status::Ok,
        NamedFile::open("./mock-assets/sample.png").await.ok(),
    )
}

#[rocket::post("/<id>/png", data = "<png>")]
pub async fn post_png(png: Png, id: String, _user: AuthUser<'_>) -> Status {
    println!("post image.png {} with size {}", id, png.0.len());
    Status::NoContent
}

#[rocket::patch("/<id>/png", data = "<png>")]
pub async fn patch_png(png: Png, id: String, _user: AuthUser<'_>) -> Status {
    println!("patch image.png {} with size {}", id, png.0.len());
    Status::NoContent
}

pub fn routes() -> Vec<Route> {
    rocket::routes![
        get_all, post, get_mine, get_one, delete_one, get_svg, post_svg, patch_svg, get_png,
        post_png, patch_png
    ]
}
