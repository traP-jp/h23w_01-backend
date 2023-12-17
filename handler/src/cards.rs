use std::io::Cursor;

use async_trait::async_trait;
use bytes::Bytes;
use rocket::data::{Data, FromData, Outcome, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Request, Response, Route, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain::repository::{CardModel, DateTimeUtc, SaveCardParams};

use crate::auth::AuthUser;
use crate::{UuidParam, CR, IR};

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

impl<'r, 'o: 'r> Responder<'r, 'o> for Svg {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let res = Response::build_from(self.0.respond_to(request)?)
            .status(Status::Ok)
            .header(ContentType::SVG)
            .finalize();
        Ok(res)
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

impl<'r, 'o: 'r> Responder<'r, 'o> for Png {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let res = Response::build()
            .status(Status::Ok)
            .header(ContentType::PNG)
            .sized_body(self.0.len(), Cursor::new(self.0))
            .finalize();
        Ok(res)
    }
}

async fn complete_card_response_one(
    model: &CardModel,
    card_repo: &State<CR>,
) -> anyhow::Result<CardResponse> {
    let CardModel {
        id,
        owner_id,
        publish_date,
        message,
    } = model;
    let publish_channels = card_repo.0.get_publish_channels_by_id(*id).await?;
    let res = CardResponse {
        id: *id,
        owner_id: *owner_id,
        publish_date: *publish_date,
        publish_channels,
        message: message.clone(),
    };
    Ok(res)
}

async fn complete_card_response(
    models: &[CardModel],
    card_repo: &State<CR>,
) -> anyhow::Result<Vec<CardResponse>> {
    let mut completed = vec![];
    for CardModel {
        id,
        owner_id,
        publish_date,
        message,
    } in models
    {
        // WARN: N+1
        let publish_channels = card_repo.0.get_publish_channels_by_id(*id).await?;
        let res = CardResponse {
            id: *id,
            owner_id: *owner_id,
            publish_date: *publish_date,
            publish_channels,
            message: message.clone(),
        };
        completed.push(res);
    }
    Ok(completed)
}

#[rocket::get("/")]
pub async fn get_all(
    card_repo: &State<CR>,
    user: AuthUser,
) -> Result<(Status, Json<Vec<CardResponse>>), Status> {
    let user = user.0.ok_or(Status::Unauthorized)?;
    let now = chrono::Utc::now();

    let card_models: Vec<_> = card_repo
        .0
        .get_all_cards()
        .await
        .map_err(|e| {
            eprintln!("Error in get all cards: {}", e);
            Status::InternalServerError
        })?
        .into_iter()
        .filter(|c| c.owner_id == user.id || c.publish_date < now)
        .collect();
    let response = complete_card_response(&card_models, card_repo)
        .await
        .map_err(|e| {
            eprintln!("Error in fetching publish dates: {}", e);
            Status::InternalServerError
        })?;
    Ok((Status::Ok, Json(response)))
}

#[rocket::post("/", data = "<card>")]
pub async fn post(
    card: Json<CardRequest>,
    card_repo: &State<CR>,
    _user: AuthUser,
) -> Result<(Status, String), Status> {
    // TODO: imagesのIDをDBにcard_idとのrelationで入れたい
    // GCのため
    // TODO: requestのowner_idとuser.idの一致確認
    let CardRequest {
        owner_id,
        publish_date,
        publish_channels,
        message,
        images: _image,
    } = card.0;
    let params = SaveCardParams {
        id: Uuid::new_v4(),
        owner_id,
        publish_date,
        message,
        channels: publish_channels,
    };
    card_repo.0.save_card(&params).await.map_err(|e| {
        eprintln!("error in post card: {}", e);
        Status::InternalServerError
    })?;
    Ok((Status::Ok, params.id.to_string()))
}

#[rocket::get("/me")]
pub async fn get_mine(
    card_repo: &State<CR>,
    user: AuthUser,
) -> Result<(Status, Json<Vec<CardResponse>>), Status> {
    let user = user.0.ok_or(Status::Unauthorized)?;
    let card_models = card_repo.0.get_my_cards(user.id).await.map_err(|e| {
        eprintln!("Error in get my cards: {}", e);
        Status::InternalServerError
    })?;
    let response = complete_card_response(&card_models, card_repo)
        .await
        .map_err(|e| {
            eprintln!("error in completing publish dates: {}", e);
            Status::InternalServerError
        })?;
    Ok((Status::Ok, Json(response)))
}

#[rocket::get("/<id>")]
pub async fn get_one(
    id: UuidParam,
    card_repo: &State<CR>,
    _user: AuthUser,
) -> Result<(Status, Json<CardResponse>), Status> {
    // TODO: NotFoundを返すべき場合がありそう
    let card_model = card_repo
        .0
        .get_card_by_id(id.0)
        .await
        .map_err(|e| {
            eprintln!("error in get card by id: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    let res = complete_card_response_one(&card_model, card_repo)
        .await
        .map_err(|e| {
            eprintln!("error in complete publish date: {}", e);
            Status::InternalServerError
        })?;
    Ok((Status::Ok, Json(res)))
}

#[rocket::patch("/<id>", data = "<card>")]
pub async fn update(
    id: UuidParam,
    card: Json<CardRequest>,
    card_repo: &State<CR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    // TODO: requestのowner_idとuser.idの一致確認
    let CardRequest {
        owner_id,
        publish_date,
        publish_channels,
        message,
        images: _image,
    } = card.0;
    let params = SaveCardParams {
        id: id.0,
        owner_id,
        publish_date,
        message,
        channels: publish_channels,
    };
    card_repo
        .0
        .update_card(&params)
        .await
        .map_err(|e| {
            eprintln!("error in update card: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    Ok(Status::NoContent)
}

#[rocket::delete("/<id>")]
pub async fn delete_one(
    id: UuidParam,
    card_repo: &State<CR>,
    image_repo: &State<IR>,
    user: AuthUser,
) -> Result<Status, Status> {
    let user = user.0.ok_or(Status::Unauthorized)?;

    let id = id.0;
    // 存在確認
    let card = card_repo
        .0
        .get_card_by_id(id)
        .await
        .map_err(|e| {
            eprintln!("error in get card by id: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    // 自分以外のもの || 投稿済みのものは不可
    let now = chrono::Utc::now();
    if card.owner_id != user.id || card.publish_date <= now {
        return Err(Status::Forbidden);
    }

    card_repo
        .0
        .delete_card(id)
        .await
        .map_err(|e| {
            eprintln!("error in delete card: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    let pub_chans = card_repo
        .0
        .get_publish_channels_by_id(id)
        .await
        .map_err(|e| {
            eprintln!("error in get pub_chans: {}", e);
            Status::InternalServerError
        })?;
    for channel_id in pub_chans {
        card_repo
            .0
            .delete_publish_channel(id, channel_id)
            .await
            .map_err(|e| {
                eprintln!("error in delete publish channel: {}", e);
                Status::InternalServerError
            })?
            .ok_or(Status::InternalServerError)?;
    }
    image_repo.0.delete_svg(id).await.map_err(|e| {
        eprintln!("error in delete svg: {}", e);
        Status::InternalServerError
    })?;
    image_repo.0.delete_png(id).await.map_err(|e| {
        eprintln!("error in delete png: {}", e);
        Status::InternalServerError
    })?;
    Ok(Status::NoContent)
}

#[rocket::get("/<id>/svg")]
pub async fn get_svg(
    id: UuidParam,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<Svg, Status> {
    // TODO: 404を返すべき場合がある
    let res = image_repo
        .0
        .get_svg(id.0)
        .await
        .map_err(|e| {
            eprintln!("error in get svg: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    Ok(Svg(res))
}

#[rocket::post("/<id>/svg", data = "<svg>")]
pub async fn post_svg(
    svg: Svg,
    id: UuidParam,
    image_repo: &State<IR>,
    _card_repo: &State<CR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    // TODO: 404
    image_repo.0.save_svg(id.0, &svg.0).await.map_err(|e| {
        eprintln!("error in create svg: {}", e);
        Status::InternalServerError
    })?;
    Ok(Status::NoContent)
}

#[rocket::patch("/<id>/svg", data = "<svg>")]
pub async fn patch_svg(
    svg: Svg,
    id: UuidParam,
    image_repo: &State<IR>,
    _card_repo: &State<CR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    // TODO: requestのowner_idとuser.idの一致確認
    image_repo.0.save_svg(id.0, &svg.0).await.map_err(|e| {
        eprintln!("error in update svg: {}", e);
        Status::InternalServerError
    })?;
    Ok(Status::NoContent)
}

#[rocket::get("/<id>/png")]
pub async fn get_png(
    id: UuidParam,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<Png, Status> {
    // TODO: requestのowner_idとuser.idの一致確認
    let png = image_repo
        .0
        .get_png(id.0)
        .await
        .map_err(|e| {
            eprintln!("error in get png: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    Ok(Png(png))
}

#[rocket::post("/<id>/png", data = "<png>")]
pub async fn post_png(
    png: Png,
    id: UuidParam,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    // TODO: owner_idとuser.idの一致確認
    image_repo.0.save_png(id.0, &png.0).await.map_err(|e| {
        eprintln!("error in create png: {}", e);
        Status::InternalServerError
    })?;
    Ok(Status::NoContent)
}

#[rocket::patch("/<id>/png", data = "<png>")]
pub async fn patch_png(
    png: Png,
    id: UuidParam,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    // TODO: owner_idとuser.idの一致確認
    image_repo.0.save_png(id.0, &png.0).await.map_err(|e| {
        eprintln!("error in update png: {}", e);
        Status::InternalServerError
    })?;
    Ok(Status::NoContent)
}

pub fn routes() -> Vec<Route> {
    rocket::routes![
        get_all, post, get_mine, get_one, delete_one, get_svg, post_svg, patch_svg, get_png,
        post_png, patch_png
    ]
}
