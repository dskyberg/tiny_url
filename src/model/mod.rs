use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub status: String,
    pub message: Value,
}

impl MessageResponse {
    pub fn new(status: &str, message: Value) -> Self {
        Self {
            status: status.into(),
            message,
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UrlRequest {
    #[validate(url, length(max = 200))]
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListUrlsResponse {
    pub urls: Vec<TinyUrl>,
}

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct TinyUrl {
    pub url: String,
    pub src_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TinyUrl {
    pub fn create(src_url: &str) -> Self {
        let now = Utc::now();
        let url = nanoid!(10);

        Self {
            url,
            src_url: src_url.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
