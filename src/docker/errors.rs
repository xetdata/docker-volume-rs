use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::io;
use thiserror::Error;

pub type VolumeResponse<T> = Result<T, VolumeError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum VolumeError {
    #[error("Something went wrong")]
    Unknown,
    #[error("Provided volume wasn't found")]
    NotFound,
    #[error("{0} is not a valid option")]
    NoOption(String),
    #[error("Invalid options: {0}")]
    InvalidOptions(String),
    #[error("Failed to create a location for a volume: {0}")]
    FailedIO(io::Error),
    #[error("Failed to mount the volume due to an internal error: {0}")]
    FailedMount(String),
}

impl IntoResponse for VolumeError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        match self {
            VolumeError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, message),
            VolumeError::NotFound => (StatusCode::NOT_FOUND, message),
            VolumeError::NoOption(_) => (StatusCode::BAD_REQUEST, message),
            VolumeError::InvalidOptions(_) => (StatusCode::BAD_REQUEST, message),
            VolumeError::FailedIO(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            VolumeError::FailedMount(_) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        }
        .into_response()
    }
}
