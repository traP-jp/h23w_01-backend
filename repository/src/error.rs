use std::str::Utf8Error;

use s3::error::S3Error;
use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("DbErr: {0}")]
    DbErr(#[from] DbErr),
    #[error("S3Err: {0}")]
    S3Err(#[from] S3Error),
    #[error("Utf8Err: {0}")]
    Utf8Err(#[from] Utf8Error),
}
