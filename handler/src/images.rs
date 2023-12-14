use async_trait::async_trait;
use rocket::data::ToByteUnit;
use rocket::form::{self, error::ErrorKind, DataField, Form, FromFormField};
use rocket::http::Status;
use rocket::{routes, FromForm, Route};

use crate::auth::AuthUser;

#[derive(Debug, Clone)]
pub enum FormImage {
    Svg(Vec<u8>),
    Png(Vec<u8>),
    Jpeg(Vec<u8>),
    Gif(Vec<u8>),
}

#[async_trait]
impl<'r> FromFormField<'r> for FormImage {
    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let content_type = field.content_type;
        let data = field
            .data
            .open(2.mebibytes())
            .into_bytes()
            .await?
            .into_inner();
        if content_type.is_svg() {
            return Ok(FormImage::Svg(data));
        }
        if content_type.is_png() {
            return Ok(FormImage::Png(data));
        }
        if content_type.is_jpeg() {
            return Ok(FormImage::Jpeg(data));
        }
        if content_type.is_svg() {
            return Ok(FormImage::Svg(data));
        }
        Err(ErrorKind::Validation(format!("invalid content type {}", content_type).into()).into())
    }
}

#[derive(Debug, FromForm)]
pub struct ImageForm {
    pub id: String,
    pub image: FormImage,
}

#[rocket::get("/<id>")]
pub async fn get_one(id: String, _user: AuthUser<'_>) -> Status {
    println!("get image id {}", id);
    Status::NotImplemented
}

#[rocket::post("/", data = "<form_data>")]
pub async fn post(form_data: Form<ImageForm>, _user: AuthUser<'_>) -> Status {
    println!(
        "post image data id={}, image={:?}",
        form_data.id, form_data.image
    );
    Status::NotImplemented
}

pub fn routes() -> Vec<Route> {
    routes![get_one, post]
}
