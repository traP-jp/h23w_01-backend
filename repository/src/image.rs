use crate::error::RepositoryError;
use bytes::Bytes;
use s3::{creds::Credentials, error::S3Error, Bucket, Region};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ImageRepository {
    async fn save_png(&self, card_id: Uuid, content: &Bytes) -> Result<(), RepositoryError>;
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &Bytes,
    ) -> Result<(), RepositoryError>;
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Bytes>, RepositoryError>;
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, RepositoryError>;
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Bytes)>, RepositoryError>;
    // async fn update_png(&self, card_id: Uuid, content: Bytes) -> Result<(), RepositoryError>;
    // async fn update_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn delete_png(&self, card_id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_svg(&self, card_id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_asset(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct ImageRepositoryConfig {
    pub bucket_name: String,
    // endpoint URL
    // pub region: String,
    pub account_id: String,
    pub access_key: String,
    pub secret_key: String,
    pub path_style: bool,
}

impl ImageRepositoryConfig {
    pub fn load_env_with_prefix(prefix: &str) -> Result<Self, std::env::VarError> {
        let var_suff = |suffix: &'static str| std::env::var(format!("{}{}", prefix, suffix));
        Ok(Self {
            bucket_name: var_suff("BUCKET_NAME")?,
            // region: var_suff("REGION")?,
            account_id: var_suff("ACCOUNT_ID")?,
            access_key: var_suff("ACCESS_KEY")?,
            secret_key: var_suff("SECRET_KEY")?,
            path_style: var_suff("PATH_STYLE")?.parse().expect("invalid bool"),
        })
    }
    pub fn backet(&self) -> Result<Bucket, RepositoryError> {
        let bucket = Bucket::new(
            &self.bucket_name,
            // self.region.parse()?,
            Region::R2 {
                account_id: self.account_id.clone(),
            },
            Credentials::new(
                Some(&self.access_key),
                Some(&self.secret_key),
                None,
                None,
                None,
            )
            .map_err(|e| RepositoryError::S3Err(e.into()))?,
        )?;
        if self.path_style {
            Ok(bucket.with_path_style())
        } else {
            Ok(bucket)
        }
    }
}

pub struct ImageRepositoryImpl(Bucket);
impl ImageRepositoryImpl {
    pub fn new(bucket: &Bucket) -> Self {
        Self(bucket.clone())
    }
    pub fn new_with_config(config: ImageRepositoryConfig) -> Result<Self, RepositoryError> {
        let bucket = config.backet()?;
        Ok(Self(bucket))
    }
}

#[async_trait::async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn save_png(&self, card_id: Uuid, content: &Bytes) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.png", card_id);
        bucket
            .put_object_with_content_type(key, content, "image/png")
            .await?;
        Ok(())
    }
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.svg", card_id);
        bucket
            // .put_object_with_content_type(&key, content.as_bytes(), "image/svg+xml")
            .put_object(&key, content.as_bytes())
            .await?;
        Ok(())
    }
    async fn save_asset(
        &self,
        id: Uuid,
        content_type: &str,
        content: &Bytes,
    ) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string();
        bucket
            .put_object_with_content_type(&key, content, content_type)
            .await?;
        Ok(())
    }
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Bytes>, RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.png", card_id);
        let png = bucket.get_object(&key).await;
        match png {
            Ok(x) => Ok(Some(Bytes::from(x.to_vec()))),
            Err(S3Error::Http(404, _)) => Ok(None),
            Err(e) => Err(RepositoryError::S3Err(e)),
        }
    }
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.svg", card_id);
        let svg = bucket.get_object(&key).await;
        match svg {
            Ok(x) => match x.to_string() {
                Ok(s) => Ok(Some(s)),
                Err(e) => Err(RepositoryError::Utf8Err(e)),
            },
            Err(S3Error::Http(404, _)) => Ok(None),
            Err(e) => Err(RepositoryError::S3Err(e)),
        }
    }
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Bytes)>, RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string();
        let image = bucket.get_object(&key).await;
        match image {
            Ok(x) => Ok(Some((
                x.headers().get("content-type").unwrap().to_string(),
                Bytes::from(x.to_vec()),
            ))),
            Err(S3Error::Http(404, _)) => Ok(None),
            Err(e) => Err(RepositoryError::S3Err(e)),
        }
    }
    async fn delete_png(&self, card_id: Uuid) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.png", card_id);
        bucket.delete_object(key).await?;
        Ok(())
    }
    async fn delete_svg(&self, card_id: Uuid) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = format!("{}.svg", card_id);
        bucket.delete_object(key).await?;
        Ok(())
    }
    async fn delete_asset(&self, id: Uuid) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string();
        bucket.delete_object(key).await?;
        Ok(())
    }
}
