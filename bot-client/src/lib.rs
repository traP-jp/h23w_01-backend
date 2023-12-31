pub mod errors;
pub use crate::errors::*;
use async_trait::async_trait;
use domain::bot_client::{
    BotClient, ImageData, PostMessageParams, StampType, UploadFileParams, UploadFileResp,
};
use reqwest::multipart::{Form, Part};
use reqwest::Response;
use shaku::Component;
use traq::apis::message_api;
use traq::apis::{channel_api, configuration::Configuration, stamp_api, user_api};
use traq::models::{ChannelList, FileInfo, PostMessageRequest, Stamp, User, UserDetail};

#[derive(Debug, Clone, Component)]
#[shaku(interface = BotClient<Error = Error>)]
pub struct BotClientImpl {
    conf: Configuration,
}

impl BotClientImpl {
    pub fn new(bearer_access_token: String) -> Self {
        let conf = Configuration {
            bearer_access_token: Some(bearer_access_token),
            ..Default::default()
        };
        Self { conf }
    }
}

pub async fn image_from_response(response: Response) -> Result<ImageData> {
    let mime_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap();
    match mime_type {
        "image/svg+xml" => {
            let data = response.text().await?;
            Ok(ImageData::Svg(data))
        }
        "image/png" => {
            let data = response.bytes().await?;
            Ok(ImageData::Png(data))
        }
        "image/gif" => {
            let data = response.bytes().await?;
            Ok(ImageData::Gif(data))
        }
        "image/jpeg" => {
            let data = response.bytes().await?;
            Ok(ImageData::Jpeg(data))
        }
        _ => Err(Error::UnknownImageType),
    }
}

fn to_param(st: StampType) -> Option<&'static str> {
    match st {
        StampType::Original => Some("original"),
        StampType::Unicode => Some("unicode"),
        StampType::None => None,
    }
}

#[async_trait]
impl BotClient for BotClientImpl {
    type Error = Error;

    async fn get_stamps(&self, r#type: StampType) -> Result<Vec<Stamp>> {
        Ok(stamp_api::get_stamps(&self.conf, None, to_param(r#type)).await?)
    }

    async fn get_stamp_image(&self, stamp_id: &str) -> Result<ImageData> {
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
            Ok(image_from_response(response).await?)
        } else {
            let content = response.text().await?;
            Err(Error::ApiError { status, content })
        }
    }

    async fn get_users<'a>(&'a self, name: Option<&'a str>) -> Result<Vec<User>> {
        Ok(user_api::get_users(&self.conf, None, name).await?)
    }

    async fn get_user(&self, user_id: &str) -> Result<UserDetail> {
        Ok(user_api::get_user(&self.conf, user_id).await?)
    }

    async fn get_user_icon(&self, user_id: &str) -> Result<ImageData> {
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
            Ok(image_from_response(response).await?)
        } else {
            let content = response.text().await?;
            Err(Error::ApiError { status, content })
        }
    }

    async fn get_channels(&self) -> Result<ChannelList> {
        Ok(channel_api::get_channels(&self.conf, None).await?)
    }
    async fn post_message(&self, params: &PostMessageParams) -> Result<()> {
        message_api::post_message(
            &self.conf,
            &params.channel_id.to_string(),
            Some(PostMessageRequest {
                content: params.content.clone(),
                embed: Some(params.embed),
            }),
        )
        .await?;
        Ok(())
    }
    async fn uplodad_file(&self, params: &UploadFileParams) -> Result<UploadFileResp> {
        let conf = &self.conf;
        let token = conf.bearer_access_token.as_ref().unwrap();
        let client = &conf.client;
        let url = format!("{}/files", conf.base_path);
        let file = Part::bytes(params.content.to_vec())
            .file_name(params.id.to_string())
            .mime_str(&params.mime_type)?;
        let form = Form::new()
            .text("channelId", params.channel_id.to_string())
            .part("file", file);
        let request = client
            .request(reqwest::Method::POST, url.as_str())
            .bearer_auth(token)
            .multipart(form);
        let response = request.send().await?;
        if !response.status().is_client_error() && !response.status().is_server_error() {
            let json = response.json::<FileInfo>().await?;
            Ok(UploadFileResp { id: json.id })
        } else {
            let status = response.status();
            let content = response.text().await?;
            Err(Error::ApiError { status, content })
        }
    }
}
