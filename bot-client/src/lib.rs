pub mod errors;

pub use crate::errors::*;

use bytes::Bytes;
use reqwest::Response;
use traq::apis::{channel_api, configuration::Configuration, stamp_api, user_api};
use traq::models::{ChannelList, Stamp, User, UserDetail};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StampType {
    Original,
    Unicode,
    None,
}

impl StampType {
    fn to_param(self) -> Option<&'static str> {
        match self {
            Self::Original => Some("original"),
            Self::Unicode => Some("unicode"),
            Self::None => None,
        }
    }
}

impl BotClient {
    pub async fn get_stamps(&self, r#type: StampType) -> Result<Vec<Stamp>> {
        Ok(stamp_api::get_stamps(&self.conf, None, r#type.to_param()).await?)
    }

    pub async fn get_stamp_image(&self, stamp_id: &str) -> Result<ImageData> {
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
        Ok(user_api::get_users(&self.conf, None, None).await?)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<UserDetail> {
        Ok(user_api::get_user(&self.conf, user_id).await?)
    }

    pub async fn get_user_icon(&self, user_id: &str) -> Result<ImageData> {
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
