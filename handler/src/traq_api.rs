use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::{Request, Route, State};

use domain::bot_client::StampType as RawStampType;
use domain::bot_client::{ChannelList, ImageData, Stamp, User, UserDetail};

use crate::auth::AuthUser;
use crate::BC;

type Routes = Vec<Route>;

#[derive(Debug, Clone)]
pub struct ResponseImage(pub ImageData);

impl<'r, 'o: 'r> Responder<'r, 'o> for ResponseImage {
    fn respond_to(self: ResponseImage, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        use rocket::http::ContentType;
        use std::io::Cursor;
        let image = self.0;
        match image {
            ImageData::Gif(gif) => Ok(Response::build()
                .header(ContentType::GIF)
                .sized_body(gif.len(), Cursor::new(gif))
                .finalize()),
            ImageData::Jpeg(jpeg) => Ok(Response::build()
                .header(ContentType::JPEG)
                .sized_body(jpeg.len(), Cursor::new(jpeg))
                .finalize()),
            ImageData::Png(png) => Ok(Response::build()
                .header(ContentType::PNG)
                .sized_body(png.len(), Cursor::new(png))
                .finalize()),
            ImageData::Svg(svg) => Ok(Response::build()
                .header(ContentType::SVG)
                .sized_body(svg.len(), Cursor::new(svg))
                .finalize()),
        }
    }
}

pub mod stamps {
    use super::*;

    use rocket::form::{self, FromFormField, ValueField};
    use Stamp;

    type Stamps = Vec<Stamp>;

    #[derive(Debug, Clone, Copy)]
    pub struct StampType(RawStampType);

    impl From<RawStampType> for StampType {
        fn from(t: RawStampType) -> Self {
            Self(t)
        }
    }

    impl<'r> FromFormField<'r> for StampType {
        fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
            use RawStampType::*;
            match field.value {
                "original" => Ok(Self(Original)),
                "unicode" => Ok(Self(Unicode)),
                "" => Ok(Self(None)),
                _ => Err(form::Error::validation("invalid stamp type"))?,
            }
        }
    }

    #[rocket::get("/?<type>")]
    pub async fn get_all(
        r#type: Option<StampType>,
        client: &State<BC>,
        _user: AuthUser<'_>,
    ) -> Result<Json<Stamps>, Status> {
        client
            .0
            .get_stamps(r#type.unwrap_or(RawStampType::None.into()).0)
            .await
            .map(Json)
            .map_err(|e| {
                eprintln!("Error in get_stamps: {}", e);
                Status::InternalServerError
            })
    }

    #[rocket::get("/<id>/image")]
    pub async fn get_one(
        id: &str,
        client: &State<BC>,
        _user: AuthUser<'_>,
    ) -> Result<ResponseImage, Status> {
        client
            .0
            .get_stamp_image(id)
            .await
            .map(ResponseImage)
            .map_err(|e| {
                eprintln!("Error in get_stamp_image: {}", e);
                Status::InternalServerError
            })
    }

    /// `/stamps`
    pub fn routes() -> Routes {
        rocket::routes![get_all, get_one]
    }
}

pub mod users {

    use super::*;

    type Users = Vec<User>;

    #[rocket::get("/")]
    pub async fn get_all(client: &State<BC>, _user: AuthUser<'_>) -> Result<Json<Users>, Status> {
        client.0.get_users().await.map(Json).map_err(|e| {
            eprintln!("Error in get_users: {}", e);
            Status::InternalServerError
        })
    }

    #[rocket::get("/<id>")]
    pub async fn get_detail(
        id: &str,
        client: &State<BC>,
        _user: AuthUser<'_>,
    ) -> Result<Json<UserDetail>, Status> {
        client.0.get_user(id).await.map(Json).map_err(|e| {
            eprintln!("Error in get_user: {}", e);
            Status::InternalServerError
        })
    }

    #[rocket::get("/<id>/icon")]
    pub async fn get_icon(
        id: &str,
        client: &State<BC>,
        _user: AuthUser<'_>,
    ) -> Result<ResponseImage, Status> {
        client
            .0
            .get_user_icon(id)
            .await
            .map(ResponseImage)
            .map_err(|e| {
                eprintln!("Error in get_user_icon: {}", e);
                Status::InternalServerError
            })
    }

    /// `/users`
    pub fn routes() -> Routes {
        rocket::routes![get_all, get_icon]
    }
}

pub mod channels {
    use super::*;

    #[rocket::get("/")]
    pub async fn get_all(
        client: &State<BC>,
        _user: AuthUser<'_>,
    ) -> Result<Json<ChannelList>, Status> {
        client.0.get_channels().await.map(Json).map_err(|e| {
            eprintln!("Error in get_channels: {}", e);
            Status::InternalServerError
        })
    }

    pub fn routes() -> Routes {
        rocket::routes![get_all]
    }
}
