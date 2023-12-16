use crate::error::RepositoryError;
use entity::prelude::*;
use s3::Bucket;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, TransactionTrait};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait ImageRepository {
    async fn save_png(&self, id: Uuid, content: &[u8]) -> Result<(), RepositoryError>;
    async fn save_svg(&self, id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &[u8],
    ) -> Result<(), RepositoryError>;
    async fn get_png(&self, id: Uuid) -> Result<Option<Vec<u8>>, RepositoryError>;
    async fn get_svg(&self, id: Uuid) -> Result<Option<String>, RepositoryError>;
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Vec<u8>)>, RepositoryError>;
    async fn update_png(&self, id: Uuid, content: &[u8]) -> Result<(), RepositoryError>;
    async fn update_svg(&self, id: Uuid, content: &str) -> Result<(), RepositoryError>;
    async fn delete_png(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_svg(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn delete_asset(&self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct ImageRepositoryImpl(DatabaseConnection, Bucket);
impl ImageRepositoryImpl {
    pub fn new(db: &DatabaseConnection, bucket: &Bucket) -> Self {
        Self(db.clone(), bucket.clone())
    }
}

#[async_trait::async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn save_png(&self, id: Uuid, content: &[u8]) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let png = CardPngActiveModel {
            card_id: ActiveValue::Set(id.clone()),
        };
        CardPng::insert(png).exec(&tx).await?;
        let key = id.to_string();
        bucket.put_object(&key, content).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn save_svg(&self, id: Uuid, content: &str) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let svg = CardSvgActiveModel {
            card_id: ActiveValue::Set(Uuid::new_v4()),
        };
        CardSvg::insert(svg).exec(&tx).await?;
        let key = id.to_string();
        bucket.put_object(&key, content.as_bytes()).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn save_asset(
        &self,
        id: Uuid,
        mime_type: &str,
        content: &[u8],
    ) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let image = ImageActiveModel {
            id: ActiveValue::Set(id.clone()),
            mime_type: ActiveValue::Set(mime_type.to_string()),
        };
        Image::insert(image).exec(&tx).await?;
        let key = id.to_string();
        bucket.put_object(&key, content).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn get_png(&self, id: Uuid) -> Result<Option<Vec<u8>>, RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let png = CardPng::find_by_id(id).one(db).await?;
        if png.is_none() {
            return Ok(None);
        }
        let key = id.to_string();
        let png = bucket.get_object(&key).await?.to_vec();
        Ok(Some(png))
    }
    async fn get_svg(&self, id: Uuid) -> Result<Option<String>, RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let svg = CardSvg::find_by_id(id).one(db).await?;
        if svg.is_none() {
            return Ok(None);
        }
        let key = id.to_string();
        let svg = bucket.get_object(&key).await?.to_string()?;
        Ok(Some(svg))
    }
    async fn get_asset(&self, id: Uuid) -> Result<Option<(String, Vec<u8>)>, RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let image = Image::find_by_id(id).one(db).await?;
        if let Some(image) = image {
            let key = id.to_string();
            let content = bucket.get_object(&key).await?.to_vec();
            Ok(Some((image.mime_type, content)))
        } else {
            Ok(None)
        }
    }
    async fn update_png(&self, id: Uuid, content: &[u8]) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let png = CardPng::find_by_id(id).one(db).await?;
        if let Some(png) = png {
            let key = id.to_string();
            bucket.put_object(&key, content).await?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    async fn update_svg(&self, id: Uuid, content: &str) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let svg = CardSvg::find_by_id(id).one(db).await?;
        if let Some(svg) = svg {
            let key = id.to_string();
            bucket.put_object(&key, content.as_bytes()).await?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    async fn delete_png(&self, id: Uuid) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let png = CardPng::find_by_id(id).one(db).await?;
        let ret = if let Some(png) = png {
            let key = id.to_string();
            bucket.delete_object(&key).await?;
            CardPng::delete_by_id(id).exec(db).await?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        };
        tx.commit().await?;
        ret
    }
    async fn delete_svg(&self, id: Uuid) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let svg = CardSvg::find_by_id(id).one(db).await?;
        let ret = if let Some(svg) = svg {
            let key = id.to_string();
            bucket.delete_object(&key).await?;
            CardSvg::delete_by_id(id).exec(db).await?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        };
        tx.commit().await?;
        ret
    }
    async fn delete_asset(&self, id: Uuid) -> Result<(), RepositoryError> {
        let db = &self.0;
        let bucket = &self.1;
        let tx = db.begin().await?;
        let asset = Image::find_by_id(id).one(db).await?;
        let ret = if let Some(_) = asset {
            let key = id.to_string();
            bucket.delete_object(&key).await?;
            Image::delete_by_id(id).exec(db).await?;
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        };
        tx.commit().await?;
        ret
    }
}
