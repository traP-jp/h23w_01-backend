use async_trait::async_trait;
use bytes::Bytes;
use rocket::data::{Data, FromData, Outcome, ToByteUnit};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Request, Route, State};
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use domain::repository::{CardModel, DateTimeUtc};

use crate::auth::AuthUser;
use crate::{UuidParam, CR};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CardResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub publish_channels: Vec<Uuid>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CardRequest {
    pub owner_id: Uuid,
    pub publish_date: DateTimeUtc,
    pub publish_channels: Vec<Uuid>,
    pub message: Option<String>,
    pub images: Vec<Uuid>,
}

async fn mock_card_response() -> CardResponse {
    CardResponse {
        id: uuid!("89d136ad-1ba2-4974-a44a-cc9b5c8c0670"),
        owner_id: uuid!("56df9d96-19b7-4f7a-8695-b157ccb483fa"),
        publish_date: "2023-12-13T08:10:05Z".parse().unwrap(),
        publish_channels: vec![uuid!("0ccb58b0-5300-4842-a7e6-b19c674f7090")],
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
pub async fn get_all(
    card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Result<(Status, Json<Vec<CardResponse>>), Status> {
    let card_models = card_repo.0.get_all_cards().await.map_err(|e| {
        eprintln!("Error in get all cards: {}", e);
        Status::InternalServerError
    })?;
    let mut response = vec![];
    for CardModel {
        id,
        owner_id,
        publish_date,
        message,
    } in card_models.into_iter()
    {
        // WARN: N+1
        let publish_channels = card_repo
            .0
            .get_publish_channels_by_id(id)
            .await
            .map_err(|e| {
                eprintln!("Error in get publish channels: {}", e);
                Status::InternalServerError
            })?;
        let res = CardResponse {
            id,
            owner_id,
            publish_date,
            publish_channels,
            message,
        };
        response.push(res);
    }
    Ok((Status::Ok, Json(response)))
}

#[rocket::post("/", data = "<card>")]
pub async fn post(card: Json<CardRequest>, _card_repo: &State<CR>, _user: AuthUser<'_>) -> String {
    println!("request card: {:?}", card.0);
    "3e20b0e0-5672-4645-bf49-a2b69eafefc6".to_string()
}

#[rocket::get("/me")]
pub async fn get_mine(
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> (Status, Json<Vec<CardResponse>>) {
    (Status::Ok, Json(vec![]))
}

#[rocket::get("/<id>")]
pub async fn get_one(
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Result<(Status, Json<CardResponse>), Status> {
    let mc = mock_card_response().await;
    if id.0 != mc.id {
        Err(Status::NotFound)
    } else {
        Ok((Status::Ok, Json(mc)))
    }
}

#[rocket::delete("/<id>")]
pub async fn delete_one(id: UuidParam, _card_repo: &State<CR>, _user: AuthUser<'_>) -> Status {
    let id = id.0;
    println!("delete card id={id}");
    Status::NoContent
}

const CARD_ID: Uuid = uuid!("89d136ad-1ba2-4974-a44a-cc9b5c8c0670");

#[rocket::get("/<id>/svg")]
pub async fn get_svg(
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> (Status, Option<NamedFile>) {
    let id = id.0;
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
pub async fn post_svg(
    svg: Svg,
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Status {
    let id = id.0;
    println!("post image.svg {} with size {}", id, svg.0.len());
    Status::NoContent
}

#[rocket::patch("/<id>/svg", data = "<svg>")]
pub async fn patch_svg(
    svg: Svg,
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Status {
    let id = id.0;
    println!("patch image.svg {} with size {}", id, svg.0.len());
    Status::NoContent
}

#[rocket::get("/<id>/png")]
pub async fn get_png(
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> (Status, Option<NamedFile>) {
    let id = id.0;
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
pub async fn post_png(
    png: Png,
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Status {
    let id = id.0;
    println!("post image.png {} with size {}", id, png.0.len());
    Status::NoContent
}

#[rocket::patch("/<id>/png", data = "<png>")]
pub async fn patch_png(
    png: Png,
    id: UuidParam,
    _card_repo: &State<CR>,
    _user: AuthUser<'_>,
) -> Status {
    let id = id.0;
    println!("patch image.png {} with size {}", id, png.0.len());
    Status::NoContent
}

pub fn routes() -> Vec<Route> {
    rocket::routes![
        get_all, post, get_mine, get_one, delete_one, get_svg, post_svg, patch_svg, get_png,
        post_png, patch_png
    ]
}
