use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::{Request, Route, State};

use bot_client::{BotClient, ImageData};

type Routes = Vec<Route>;

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

    use traq::models::Stamp;

    type Stamps = Vec<Stamp>;

    #[rocket::get("/")]
    pub async fn get_all(client: &State<BotClient>) -> Result<Json<Stamps>, Status> {
        client.get_stamps().await.map(Json).map_err(|e| {
            eprintln!("Error in get_stamps: {}", e);
            Status::InternalServerError
        })
    }

    #[rocket::get("/<id>/image")]
    pub async fn get_one(id: String, client: &State<BotClient>) -> Result<ResponseImage, Status> {
        client
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

    use traq::models::User;

    type Users = Vec<User>;

    #[rocket::get("/")]
    pub async fn get_all(client: &State<BotClient>) -> Result<Json<Users>, Status> {
        client.get_users().await.map(Json).map_err(|e| {
            eprintln!("Error in get_users: {}", e);
            Status::InternalServerError
        })
    }

    #[rocket::get("/<id>/icon")]
    pub async fn get_icon(id: String, client: &State<BotClient>) -> Result<ResponseImage, Status> {
        client
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

    use traq::models::Channel;

    type Channels = Vec<Channel>;

    #[rocket::get("/")]
    pub async fn get_all(client: &State<BotClient>) -> Result<Json<Channels>, Status> {
        client
            .get_channels()
            .await
            .map(|cl| Json(cl.public))
            .map_err(|e| {
                eprintln!("Error in get_channels: {}", e);
                Status::InternalServerError
            })
    }

    pub fn routes() -> Routes {
        rocket::routes![get_all]
    }
}