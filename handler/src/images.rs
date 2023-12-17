use std::io::Cursor;

use async_trait::async_trait;
use bytes::Bytes;
use rocket::data::ToByteUnit;
use rocket::form::{self, error::ErrorKind, DataField, Form, FromFormField};
use rocket::http::hyper::header::CONTENT_TYPE;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{routes, FromForm, Response, Route, State};

use crate::auth::AuthUser;
use crate::{UuidParam, IR};

#[derive(Debug, Clone)]
pub enum FormImage {
    Svg(String),
    Png(Bytes),
    Jpeg(Bytes),
    Gif(Bytes),
}

#[async_trait]
impl<'r> FromFormField<'r> for FormImage {
    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let content_type = field.content_type;
        let data = field.data.open(2.mebibytes());
        if content_type.is_svg() {
            let data = data.into_string().await?.into_inner();
            return Ok(FormImage::Svg(data));
        }
        let data = data.into_bytes().await?.into_inner();
        if content_type.is_png() {
            return Ok(FormImage::Png(data.into()));
        }
        if content_type.is_jpeg() {
            return Ok(FormImage::Jpeg(data.into()));
        }
        if content_type.is_gif() {
            return Ok(FormImage::Gif(data.into()));
        }
        Err(ErrorKind::Validation(format!("invalid content type {}", content_type).into()).into())
    }
}

#[derive(Debug, Clone, FromForm)]
pub struct ImageForm<'r> {
    pub id: &'r str,
    pub image: FormImage,
}

pub struct ImageResponse(pub String, pub Bytes);

impl<'r, 'o: 'r> Responder<'r, 'o> for ImageResponse {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let res = Response::build()
            .status(Status::Ok)
            .raw_header(CONTENT_TYPE.as_str(), self.0)
            .sized_body(self.1.len(), Cursor::new(self.1))
            .finalize();
        Ok(res)
    }
}

#[rocket::get("/<id>")]
pub async fn get_one(
    id: UuidParam,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<ImageResponse, Status> {
    let image = image_repo
        .0
        .get_asset(id.0)
        .await
        .map_err(|e| {
            eprintln!("error in get asset: {}", e);
            Status::InternalServerError
        })?
        .ok_or(Status::NotFound)?;
    Ok(ImageResponse(image.0, image.1))
}

#[rocket::post("/", data = "<form_data>")]
pub async fn post(
    form_data: Form<ImageForm<'_>>,
    image_repo: &State<IR>,
    _user: AuthUser,
) -> Result<Status, Status> {
    let ImageForm { id, image } = form_data.into_inner();
    let id = id.parse().map_err(|_| Status::BadRequest)?;
    match image {
        FormImage::Svg(svg) => image_repo
            .0
            .save_asset(id, "image/svg+xml", &Bytes::from(svg))
            .await
            .map_err(|e| {
                eprintln!("error in create asset svg: {}", e);
                Status::InternalServerError
            })?,
        FormImage::Png(png) => image_repo
            .0
            .save_asset(id, "image/png", &png)
            .await
            .map_err(|e| {
                eprintln!("error in create asset png: {}", e);
                Status::InternalServerError
            })?,
        FormImage::Jpeg(jpeg) => image_repo
            .0
            .save_asset(id, "image/jpeg", &jpeg)
            .await
            .map_err(|e| {
                eprintln!("error in create asset jpeg: {}", e);
                Status::InternalServerError
            })?,
        FormImage::Gif(gif) => image_repo
            .0
            .save_asset(id, "image/gif", &gif)
            .await
            .map_err(|e| {
                eprintln!("error in create asset gif: {}", e);
                Status::InternalServerError
            })?,
    }
    Ok(Status::NoContent)
}

pub fn routes() -> Vec<Route> {
    routes![get_one, post]
}
