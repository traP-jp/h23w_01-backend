use crate::error::RepositoryError;
use s3::{error::S3Error, Bucket};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ImageRepository {
    async fn save_png(&self, card_id: Uuid, content: &[u8]) -> Result<(), RepositoryError>;
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &[u8],
    ) -> Result<(), RepositoryError>;
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Vec<u8>>, RepositoryError>;
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, RepositoryError>;
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Vec<u8>)>, RepositoryError>;
    // async fn update_png(&self, card_id: Uuid, content: &[u8]) -> Result<(), RepositoryError>;
    // async fn update_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn delete_png(&self, card_id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_svg(&self, card_id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_asset(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct ImageRepositoryImpl(Bucket);
impl ImageRepositoryImpl {
    pub fn new(bucket: &Bucket) -> Self {
        Self(bucket.clone())
    }
}

#[async_trait::async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn save_png(&self, card_id: Uuid, content: &[u8]) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = card_id.to_string() + ".png";
        bucket
            .put_object_with_content_type(key, content, "image/png")
            .await?;
        Ok(())
    }
    async fn save_svg(&self, card_id: Uuid, content: &str) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = card_id.to_string() + ".svg";
        bucket
            .put_object_with_content_type(&key, content.as_bytes(), "image/svg+xml")
            .await?;
        Ok(())
    }
    async fn save_asset(
        &self,
        id: Uuid,
        content_type: &str,
        content: &[u8],
    ) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string();
        bucket
            .put_object_with_content_type(&key, content, content_type)
            .await?;
        Ok(())
    }
    async fn get_png(&self, card_id: Uuid) -> Result<Option<Vec<u8>>, RepositoryError> {
        let bucket = &self.0;
        let key = card_id.to_string() + ".png";
        let png = bucket.get_object(&key).await;
        match png {
            Ok(x) => Ok(Some(x.to_vec())),
            Err(S3Error::Http(404, _)) => Ok(None),
            Err(e) => Err(RepositoryError::S3Err(e)),
        }
    }
    async fn get_svg(&self, card_id: Uuid) -> Result<Option<String>, RepositoryError> {
        let bucket = &self.0;
        let key = card_id.to_string() + ".svg";
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
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Vec<u8>)>, RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string();
        let image = bucket.get_object(&key).await;
        match image {
            Ok(x) => Ok(Some((
                x.headers().get("content-type").unwrap().to_string(),
                x.to_vec(),
            ))),
            Err(S3Error::Http(404, _)) => Ok(None),
            Err(e) => Err(RepositoryError::S3Err(e)),
        }
    }

    async fn delete_png(&self, card_id: Uuid) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = card_id.to_string() + ".png";
        bucket.delete_object(key).await?;
        Ok(())
    }
    async fn delete_svg(&self, id: Uuid) -> Result<(), RepositoryError> {
        let bucket = &self.0;
        let key = id.to_string() + ".svg";
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
