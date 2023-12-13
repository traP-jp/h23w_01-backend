use reqwest::StatusCode;
use traq::apis;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("json parse error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("traq api error: {}", .content)]
    ApiError { status: StatusCode, content: String },
    #[error("unknown image type")]
    UnknownImageType,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<apis::Error<T>> for Error {
    fn from(e: apis::Error<T>) -> Self {
        use apis::Error::*;
        match e {
            Reqwest(e) => e.into(),
            Serde(e) => e.into(),
            Io(e) => e.into(),
            ResponseError(e) => Error::ApiError {
                status: e.status,
                content: e.content,
            },
        }
    }
}
