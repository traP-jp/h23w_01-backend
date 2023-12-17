use async_trait::async_trait;
use bytes::Bytes;
use rocket::data::ToByteUnit;
use rocket::form::{self, error::ErrorKind, DataField, Form, FromFormField};
use rocket::http::Status;
use rocket::{routes, FromForm, Route};

use crate::auth::AuthUser;
use crate::UuidParam;

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

#[rocket::get("/<id>")]
pub async fn get_one(id: UuidParam, _user: AuthUser<'_>) -> Status {
    let id = id.0;
    println!("get image id {}", id);
    Status::NotImplemented
}

#[rocket::post("/", data = "<form_data>")]
pub async fn post(form_data: Form<ImageForm<'_>>, _user: AuthUser<'_>) -> Status {
    println!(
        "post image data id={}, image={:?}",
        form_data.id, form_data.image
    );
    Status::NotImplemented
}

pub fn routes() -> Vec<Route> {
    routes![get_one, post]
}
