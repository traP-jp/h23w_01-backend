pub mod errors;

pub use crate::errors::*;

use bytes::Bytes;
use reqwest::Response;
use traq::apis::channel_api;
use traq::apis::{configuration::Configuration, stamp_api};
use traq::models::{ChannelList, Stamp, User};

#[derive(Debug, Clone)]
pub struct BotClient {
    conf: Configuration,
}

impl BotClient {
    pub fn new(bearer_access_token: String) -> Self {
        let conf = Configuration {
            bearer_access_token: Some(bearer_access_token),
            ..Default::default()
        };
        Self { conf }
    }
}

#[derive(Debug, Clone)]
pub enum ImageData {
    Svg(String),
    Png(Bytes),
    Gif(Bytes),
    Jpeg(Bytes),
}

impl ImageData {
    pub async fn from_response(response: Response) -> Result<Self> {
        let mime_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap();
        match mime_type {
            "image/svg+xml" => {
                let data = response.text().await?;
                Ok(Self::Svg(data))
            }
            "image/png" => {
                let data = response.bytes().await?;
                Ok(Self::Png(data))
            }
            "image/gif" => {
                let data = response.bytes().await?;
                Ok(Self::Gif(data))
            }
            "image/jpeg" => {
                let data = response.bytes().await?;
                Ok(Self::Jpeg(data))
            }
            _ => Err(Error::UnknownImageType),
        }
    }
}

impl BotClient {
    pub async fn get_stamps(&self) -> Result<Vec<Stamp>> {
        Ok(stamp_api::get_stamps(&self.conf, None, None).await?)
    }

    pub async fn get_stamp_image(&self, stamp_id: String) -> Result<ImageData> {
        let conf = &self.conf;
        let token = conf.bearer_access_token.as_ref().unwrap();
        let client = &conf.client;
        let url = format!("{}/stamps/{}/image", conf.base_path, stamp_id);
        let request = client
            .request(reqwest::Method::GET, url.as_str())
            .bearer_auth(token);
        let response = request.send().await?;
        let status = response.status();
        if !status.is_client_error() && !status.is_server_error() {
            Ok(ImageData::from_response(response).await?)
        } else {
            let content = response.text().await?;
            Err(Error::ApiError { status, content })
        }
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        Ok(traq::apis::user_api::get_users(&self.conf, None, None).await?)
    }

    pub async fn get_user_icon(&self, user_id: String) -> Result<ImageData> {
        let conf = &self.conf;
        let token = conf.bearer_access_token.as_ref().unwrap();
        let client = &conf.client;
        let url = format!("{}/users/{}/icon", conf.base_path, user_id);
        let request = client
            .request(reqwest::Method::GET, url.as_str())
            .bearer_auth(token);
        let response = request.send().await?;
        let status = response.status();
        if !status.is_client_error() && !status.is_server_error() {
            Ok(ImageData::from_response(response).await?)
        } else {
            let content = response.text().await?;
            Err(Error::ApiError { status, content })
        }
    }

    pub async fn get_channels(&self) -> Result<ChannelList> {
        Ok(channel_api::get_channels(&self.conf, None).await?)
    }
}
